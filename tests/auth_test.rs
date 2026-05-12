//! Integration tests for the bearer-token auth middleware.
//!
//! Covers the four scenarios called out in issue #25:
//! - (a) auth off → existing behaviour is unchanged.
//! - (b) auth on + valid token → `POST /a2a` returns the response.
//! - (c) auth on + missing/invalid token → HTTP 401.
//! - (d) public endpoints (`/health`, `/.well-known/agent.json`) stay
//!   reachable without a token in both modes.
//!
//! A trait-stub `AuthVerifier` is used so the suite does not depend on
//! a real OIDC issuer.

use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use inference_gateway_adk::{
    A2AClient, A2AServerBuilder, AuthError, AuthVerifier, AuthenticatedPrincipal, TaskHandler,
    a2a_types,
};
use serde_json::{Value, json};

/// Background handler that leaves tasks in the SUBMITTED state - enough
/// for `message/send` to succeed without spinning the queue runner.
#[derive(Debug)]
struct SubmittedTaskHandler;

#[async_trait]
impl TaskHandler for SubmittedTaskHandler {
    async fn handle_task(
        &self,
        task: a2a_types::Task,
        _message: Option<a2a_types::Message>,
    ) -> anyhow::Result<a2a_types::Task> {
        Ok(task)
    }
}

/// Verifier that accepts exactly one bearer token. Returns a deterministic
/// principal so the test can assert tenant plumbing.
#[derive(Debug)]
struct AcceptOneToken {
    token: &'static str,
}

#[async_trait]
impl AuthVerifier for AcceptOneToken {
    async fn verify(&self, token: &str) -> Result<AuthenticatedPrincipal, AuthError> {
        if token == self.token {
            let mut claims = HashMap::new();
            claims.insert("sub".to_string(), Value::String("user-1".to_string()));
            claims.insert("tenant".to_string(), Value::String("acme".to_string()));
            Ok(AuthenticatedPrincipal {
                subject: "user-1".to_string(),
                tenant: "acme".to_string(),
                issuer: "https://example.test".to_string(),
                claims,
            })
        } else {
            Err(AuthError::InvalidToken("rejected by stub".to_string()))
        }
    }
}

fn allocate_port() -> u16 {
    let listener = StdTcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener.local_addr().expect("local_addr available").port()
}

fn agent_card(addr: SocketAddr, supports_extended: bool) -> a2a_types::AgentCard {
    let value = json!({
        "name": "Auth Test Agent",
        "description": "agent/getAuthenticatedExtendedCard auth tests",
        "version": "1.0.0",
        "protocolVersion": "0.2.6",
        "url": format!("http://{addr}/a2a"),
        "preferredTransport": "JSONRPC",
        "capabilities": {
            "streaming": true,
            "pushNotifications": false,
            "stateTransitionHistory": false
        },
        "defaultInputModes": ["text/plain"],
        "defaultOutputModes": ["text/plain"],
        "skills": [
            {"id": "x", "name": "x", "description": "x", "tags": ["x"]}
        ],
        "supportsExtendedAgentCard": supports_extended
    });
    serde_json::from_value(value).expect("agent card parses")
}

/// Boot an `A2AServer` on its own thread + tokio runtime and return the
/// bound address. The server outlives the per-test runtime that
/// `#[tokio::test]` creates.
fn spawn_server(verifier: Option<Arc<dyn AuthVerifier>>, supports_extended: bool) -> SocketAddr {
    let port = allocate_port();
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse().expect("addr parses");

    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let addr_clone = addr;
    let verifier_clone = verifier;
    std::thread::Builder::new()
        .name(format!("a2a-auth-test-server-{port}"))
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("runtime builds");
            rt.block_on(async move {
                let mut builder = A2AServerBuilder::new()
                    .with_gateway_url("http://127.0.0.1:1/v1")
                    .with_agent_card(agent_card(addr_clone, supports_extended))
                    .with_background_task_handler(SubmittedTaskHandler)
                    .with_default_streaming_task_handler();
                if let Some(v) = verifier_clone {
                    builder = builder.with_auth_verifier(v);
                }
                let server = builder.build().await.expect("server builds");
                let _ = ready_tx.send(());
                if let Err(e) = server.serve(addr_clone).await {
                    eprintln!("auth test server stopped: {e}");
                }
            });
        })
        .expect("spawn server thread");

    ready_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("server became ready");

    for _ in 0..100 {
        if std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(50)).is_ok() {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    addr
}

fn extended_card_request() -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": "auth-test-extended-card",
        "method": "agent/getAuthenticatedExtendedCard",
        "params": { "tenant": "acme" }
    })
}

#[tokio::test]
async fn auth_off_extended_card_is_reachable_without_token() {
    let addr = spawn_server(None, true);
    let url = format!("http://{addr}/a2a");
    let client = reqwest::Client::new();
    let response: Value = client
        .post(&url)
        .json(&extended_card_request())
        .send()
        .await
        .expect("request succeeds")
        .json()
        .await
        .expect("body decodes");
    assert!(
        response.get("error").is_none(),
        "expected success, got {response}"
    );
    let result = response.get("result").expect("result present");
    assert_eq!(
        result.get("name").and_then(|v| v.as_str()),
        Some("Auth Test Agent")
    );
}

#[tokio::test]
async fn auth_on_with_valid_token_returns_extended_card() {
    let verifier: Arc<dyn AuthVerifier> = Arc::new(AcceptOneToken { token: "good" });
    let addr = spawn_server(Some(verifier), true);
    let url = format!("http://{addr}/a2a");
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth("good")
        .json(&extended_card_request())
        .send()
        .await
        .expect("request succeeds");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let body: Value = response.json().await.expect("json body");
    assert!(
        body.get("error").is_none(),
        "expected success result, got {body}"
    );
    let result = body.get("result").expect("result present");
    assert_eq!(
        result.get("name").and_then(|v| v.as_str()),
        Some("Auth Test Agent")
    );
}

#[tokio::test]
async fn auth_on_with_missing_token_returns_401() {
    // (c1) auth on + missing token → HTTP 401 from the middleware.
    let verifier: Arc<dyn AuthVerifier> = Arc::new(AcceptOneToken { token: "good" });
    let addr = spawn_server(Some(verifier), true);
    let url = format!("http://{addr}/a2a");
    let response = reqwest::Client::new()
        .post(&url)
        .json(&extended_card_request())
        .send()
        .await
        .expect("request succeeds");
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
    assert!(
        response
            .headers()
            .get(reqwest::header::WWW_AUTHENTICATE)
            .is_some(),
        "401 should carry WWW-Authenticate: Bearer challenge"
    );
}

#[tokio::test]
async fn auth_on_with_invalid_token_returns_401() {
    let verifier: Arc<dyn AuthVerifier> = Arc::new(AcceptOneToken { token: "good" });
    let addr = spawn_server(Some(verifier), true);
    let url = format!("http://{addr}/a2a");
    let response = reqwest::Client::new()
        .post(&url)
        .bearer_auth("bad")
        .json(&extended_card_request())
        .send()
        .await
        .expect("request succeeds");
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_on_malformed_header_returns_401() {
    let verifier: Arc<dyn AuthVerifier> = Arc::new(AcceptOneToken { token: "good" });
    let addr = spawn_server(Some(verifier), true);
    let url = format!("http://{addr}/a2a");
    let response = reqwest::Client::new()
        .post(&url)
        .header(reqwest::header::AUTHORIZATION, "Basic Zm9vOmJhcg==")
        .json(&extended_card_request())
        .send()
        .await
        .expect("request succeeds");
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn auth_on_health_and_card_endpoints_remain_public() {
    // (d) /health and /.well-known/agent.json must stay reachable
    // without a token even when auth is enabled.
    let verifier: Arc<dyn AuthVerifier> = Arc::new(AcceptOneToken { token: "good" });
    let addr = spawn_server(Some(verifier), true);
    let base = format!("http://{addr}");

    let client = A2AClient::new(&base).expect("client builds");
    let health = client
        .get_health()
        .await
        .expect("health works without auth");
    assert!(!health.status.is_empty());

    let card = client
        .get_agent_card()
        .await
        .expect("public agent card works without auth");
    assert_eq!(card.name, "Auth Test Agent");
}

#[tokio::test]
async fn auth_on_method_not_found_still_reaches_handler() {
    let verifier: Arc<dyn AuthVerifier> = Arc::new(AcceptOneToken { token: "good" });
    let addr = spawn_server(Some(verifier), false);
    let url = format!("http://{addr}/a2a");
    let response = reqwest::Client::new()
        .post(&url)
        .bearer_auth("good")
        .json(&extended_card_request())
        .send()
        .await
        .expect("request succeeds");
    assert_eq!(response.status(), reqwest::StatusCode::OK);
    let body: Value = response.json().await.expect("body");
    let code = body
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_i64());
    assert_eq!(code, Some(-32601), "expected METHOD_NOT_FOUND, got {body}");
}
