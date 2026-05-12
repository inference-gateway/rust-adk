//! Authentication primitives for the A2A HTTP surface.
//!
//! The middleware wired here protects `POST /a2a` (and any other route
//! that opts in via [`auth_middleware`]) by validating an
//! `Authorization: Bearer <token>` JWT against an OIDC issuer's JWKS.
//! `GET /health` and `GET /.well-known/agent.json` are intentionally left
//! public so health probes and discovery clients keep working without a
//! credential.
//!
//! The middleware is only attached when [`crate::config::AuthConfig`]
//! has `enable == true` and an [`AuthVerifier`] is registered on
//! [`AppState`] - otherwise the routes behave exactly as before, which
//! preserves backwards compatibility for callers that have not opted in
//! to authentication.
//!
//! The wire shape (bearer JWT) matches the Go ADK's middleware. Broader
//! `securitySchemes`-driven negotiation (API key, mTLS, OAuth2
//! authorization-code) is intentionally out of scope; this module is the
//! foundation those schemes can build on.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Json, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::RwLock;
use tracing::{debug, warn};

use crate::config::AuthConfig;

use super::protocol::AppState;

/// Subject claims extracted from a validated bearer token.
///
/// Plumbed through Axum request extensions so JSON-RPC handlers can
/// surface or scope behaviour by tenant in a follow-up (e.g. filtering
/// the extended agent card).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedPrincipal {
    /// `sub` claim from the JWT.
    pub subject: String,
    /// Tenant claim, lifted from the first of `tenant`, `tid`, or
    /// `organization` that is present in the token. May be empty when
    /// no claim is set - downstream code should treat this as
    /// "unspecified" rather than "anonymous".
    pub tenant: String,
    /// Issuer (`iss`) claim - already validated against
    /// [`AuthConfig::issuer_url`] before this struct is constructed.
    pub issuer: String,
    /// All claims, retained so handlers can inspect provider-specific
    /// fields (e.g. `groups`, `roles`) without re-decoding the token.
    pub claims: HashMap<String, Value>,
}

impl AuthenticatedPrincipal {
    fn from_claims(issuer: String, claims: HashMap<String, Value>) -> Self {
        let subject = claims
            .get("sub")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let tenant = ["tenant", "tid", "organization"]
            .iter()
            .find_map(|k| claims.get(*k).and_then(|v| v.as_str()))
            .unwrap_or_default()
            .to_string();
        Self {
            subject,
            tenant,
            issuer,
            claims,
        }
    }
}

/// Error surface for token verification. The middleware maps any
/// variant to HTTP 401 - the granularity exists for logs and to give
/// tests something concrete to match on.
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("missing Authorization header")]
    MissingHeader,
    #[error("Authorization header must use the Bearer scheme")]
    MalformedHeader,
    #[error("token is empty")]
    EmptyToken,
    #[error("token validation failed: {0}")]
    InvalidToken(String),
    #[error("OIDC discovery failed: {0}")]
    DiscoveryFailed(String),
    #[error("JWKS fetch failed: {0}")]
    JwksFetchFailed(String),
    #[error("signing key not found for token")]
    UnknownKid,
    #[error("internal auth error: {0}")]
    Internal(String),
}

/// Pluggable bearer-token verifier. Implementing this trait lets callers
/// plug a custom backend (e.g. a static signing key, a mock for tests)
/// in place of the bundled OIDC verifier.
#[async_trait]
pub trait AuthVerifier: Send + Sync + std::fmt::Debug {
    /// Validate a raw bearer token (no `Bearer ` prefix) and return the
    /// authenticated principal on success.
    async fn verify(&self, token: &str) -> Result<AuthenticatedPrincipal, AuthError>;
}

/// JWT verifier that pulls the JWKS from an OIDC issuer's discovery
/// document and caches the keys in memory. Verifies token signature,
/// `iss`, `exp`, and (when `client_id` is configured) `aud`.
#[derive(Debug)]
pub struct OidcJwtVerifier {
    issuer_url: String,
    audience: Option<String>,
    http: reqwest::Client,
    cache: RwLock<JwksCache>,
}

#[derive(Debug, Default)]
struct JwksCache {
    jwks_uri: Option<String>,
    /// `kid -> (DecodingKey, alg)` for keys advertised by the JWKS.
    keys: HashMap<String, (DecodingKey, Algorithm)>,
}

#[derive(Debug, Deserialize)]
struct DiscoveryDocument {
    jwks_uri: String,
}

#[derive(Debug, Deserialize)]
struct JwksDocument {
    keys: Vec<jsonwebtoken::jwk::Jwk>,
}

impl OidcJwtVerifier {
    /// Build a verifier from the [`AuthConfig`]. The HTTP client uses a
    /// 5s timeout for discovery + JWKS fetches.
    pub fn from_config(config: &AuthConfig) -> Result<Self> {
        if config.issuer_url.trim().is_empty() {
            return Err(anyhow!("AUTH_ISSUER_URL is required when AUTH_ENABLE=true"));
        }
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|e| anyhow!("failed to build OIDC HTTP client: {e}"))?;
        let audience = if config.client_id.trim().is_empty() {
            None
        } else {
            Some(config.client_id.clone())
        };
        Ok(Self {
            issuer_url: config.issuer_url.trim_end_matches('/').to_string(),
            audience,
            http,
            cache: RwLock::new(JwksCache::default()),
        })
    }

    /// Discovery URL: `<issuer>/.well-known/openid-configuration`.
    fn discovery_url(&self) -> String {
        format!("{}/.well-known/openid-configuration", self.issuer_url)
    }

    /// Returns the cached `jwks_uri`, fetching the discovery document
    /// if the cache is cold.
    async fn jwks_uri(&self) -> Result<String, AuthError> {
        if let Some(uri) = self.cache.read().await.jwks_uri.clone() {
            return Ok(uri);
        }
        let url = self.discovery_url();
        debug!("fetching OIDC discovery document from {url}");
        let doc: DiscoveryDocument = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| AuthError::DiscoveryFailed(e.to_string()))?
            .error_for_status()
            .map_err(|e| AuthError::DiscoveryFailed(e.to_string()))?
            .json()
            .await
            .map_err(|e| AuthError::DiscoveryFailed(e.to_string()))?;
        let mut cache = self.cache.write().await;
        cache.jwks_uri = Some(doc.jwks_uri.clone());
        Ok(doc.jwks_uri)
    }

    /// Refresh the cached JWKS by fetching `jwks_uri`. Existing entries
    /// are replaced wholesale.
    async fn refresh_jwks(&self) -> Result<(), AuthError> {
        let uri = self.jwks_uri().await?;
        debug!("fetching JWKS from {uri}");
        let doc: JwksDocument = self
            .http
            .get(&uri)
            .send()
            .await
            .map_err(|e| AuthError::JwksFetchFailed(e.to_string()))?
            .error_for_status()
            .map_err(|e| AuthError::JwksFetchFailed(e.to_string()))?
            .json()
            .await
            .map_err(|e| AuthError::JwksFetchFailed(e.to_string()))?;

        let mut keys = HashMap::new();
        for jwk in &doc.keys {
            let Some(kid) = jwk.common.key_id.clone() else {
                continue;
            };
            let Some(alg) = jwk.common.key_algorithm.and_then(map_key_algorithm) else {
                continue;
            };
            let Ok(decoding) = DecodingKey::from_jwk(jwk) else {
                continue;
            };
            keys.insert(kid, (decoding, alg));
        }

        let mut cache = self.cache.write().await;
        cache.keys = keys;
        Ok(())
    }

    async fn key_for_kid(&self, kid: &str) -> Option<(DecodingKey, Algorithm)> {
        self.cache.read().await.keys.get(kid).cloned()
    }
}

fn map_key_algorithm(alg: jsonwebtoken::jwk::KeyAlgorithm) -> Option<Algorithm> {
    use jsonwebtoken::jwk::KeyAlgorithm as K;
    match alg {
        K::HS256 => Some(Algorithm::HS256),
        K::HS384 => Some(Algorithm::HS384),
        K::HS512 => Some(Algorithm::HS512),
        K::ES256 => Some(Algorithm::ES256),
        K::ES384 => Some(Algorithm::ES384),
        K::RS256 => Some(Algorithm::RS256),
        K::RS384 => Some(Algorithm::RS384),
        K::RS512 => Some(Algorithm::RS512),
        K::PS256 => Some(Algorithm::PS256),
        K::PS384 => Some(Algorithm::PS384),
        K::PS512 => Some(Algorithm::PS512),
        K::EdDSA => Some(Algorithm::EdDSA),
        _ => None,
    }
}

#[async_trait]
impl AuthVerifier for OidcJwtVerifier {
    async fn verify(&self, token: &str) -> Result<AuthenticatedPrincipal, AuthError> {
        if token.is_empty() {
            return Err(AuthError::EmptyToken);
        }
        let header = decode_header(token).map_err(|e| AuthError::InvalidToken(e.to_string()))?;
        let Some(kid) = header.kid else {
            return Err(AuthError::InvalidToken(
                "token header missing `kid`".to_string(),
            ));
        };

        // Try the cached key. If absent, refresh the JWKS once and try
        // again - this covers the common case of an issuer rotating keys
        // between requests.
        let key_and_alg = match self.key_for_kid(&kid).await {
            Some(found) => found,
            None => {
                self.refresh_jwks().await?;
                self.key_for_kid(&kid).await.ok_or(AuthError::UnknownKid)?
            }
        };
        let (decoding, alg) = key_and_alg;

        let mut validation = Validation::new(alg);
        validation.set_issuer(&[self.issuer_url.as_str()]);
        if let Some(aud) = self.audience.as_ref() {
            validation.set_audience(&[aud.as_str()]);
        } else {
            validation.validate_aud = false;
        }

        let data = decode::<HashMap<String, Value>>(token, &decoding, &validation)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        Ok(AuthenticatedPrincipal::from_claims(
            self.issuer_url.clone(),
            data.claims,
        ))
    }
}

/// Axum middleware that enforces a valid bearer token before allowing
/// the request to reach the wrapped handler. The middleware is a
/// no-op when [`AppState::auth_verifier`] is `None`, which is what the
/// builder produces when `AuthConfig.enable == false`.
pub(crate) async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, Response> {
    let Some(verifier) = state.auth_verifier.as_ref().cloned() else {
        return Ok(next.run(req).await);
    };

    let raw = match req.headers().get(header::AUTHORIZATION) {
        Some(value) => value
            .to_str()
            .map_err(|_| reject(AuthError::MalformedHeader))?,
        None => return Err(reject(AuthError::MissingHeader)),
    };
    let Some(token) = raw
        .strip_prefix("Bearer ")
        .or_else(|| raw.strip_prefix("bearer "))
    else {
        return Err(reject(AuthError::MalformedHeader));
    };
    let token = token.trim();

    match verifier.verify(token).await {
        Ok(principal) => {
            req.extensions_mut().insert(principal);
            Ok(next.run(req).await)
        }
        Err(e) => {
            warn!("auth middleware rejected request: {e}");
            Err(reject(e))
        }
    }
}

fn reject(err: AuthError) -> Response {
    let body = Json(serde_json::json!({
        "error": "Unauthorized",
        "message": err.to_string(),
    }));
    let mut response = (StatusCode::UNAUTHORIZED, body).into_response();
    response.headers_mut().insert(
        header::WWW_AUTHENTICATE,
        axum::http::HeaderValue::from_static("Bearer realm=\"a2a\""),
    );
    // Suppress unused-import warning when this helper is the only
    // consumer of `Body` in some compilation modes.
    let _ = std::marker::PhantomData::<Body>;
    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct AcceptToken(&'static str);

    #[async_trait]
    impl AuthVerifier for AcceptToken {
        async fn verify(&self, token: &str) -> Result<AuthenticatedPrincipal, AuthError> {
            if token == self.0 {
                let mut claims = HashMap::new();
                claims.insert("sub".to_string(), Value::String("test-user".to_string()));
                claims.insert(
                    "tenant".to_string(),
                    Value::String("test-tenant".to_string()),
                );
                Ok(AuthenticatedPrincipal::from_claims(
                    "https://example.test".to_string(),
                    claims,
                ))
            } else {
                Err(AuthError::InvalidToken("nope".to_string()))
            }
        }
    }

    #[tokio::test]
    async fn principal_extracts_known_claims() {
        let verifier = AcceptToken("good");
        let p = verifier.verify("good").await.expect("ok");
        assert_eq!(p.subject, "test-user");
        assert_eq!(p.tenant, "test-tenant");
        assert_eq!(p.issuer, "https://example.test");
        assert!(p.claims.contains_key("sub"));
    }

    #[tokio::test]
    async fn rejects_unknown_token() {
        let verifier = AcceptToken("good");
        let err = verifier.verify("bad").await.expect_err("must reject");
        assert!(matches!(err, AuthError::InvalidToken(_)));
    }

    #[test]
    fn from_config_requires_issuer_url() {
        let cfg = AuthConfig {
            enable: true,
            issuer_url: String::new(),
            client_id: String::new(),
            client_secret: String::new(),
        };
        let err = OidcJwtVerifier::from_config(&cfg).expect_err("issuer required");
        assert!(err.to_string().contains("AUTH_ISSUER_URL"));
    }
}
