use super::agent::Agent;
use super::artifact_service::ArtifactService;
use super::artifacts_server::{ArtifactsServer, spawn_retention_task};
use super::auth::{AuthVerifier, auth_middleware};
use super::protocol::{AppState, a2a_handler};
use super::storage::Storage;
use super::task_handler::{StreamableTaskHandler, TaskHandler};
use super::task_manager::DefaultTaskManager;
use super::tls::{MtlsAcceptor, build_server_config};
use crate::a2a_types::AgentCard;
use crate::client::HealthStatus;
use crate::config::Config;
use anyhow::{Result, anyhow};
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, post},
};
use axum_server::Handle;
use inference_gateway_sdk::{InferenceGatewayAPI, InferenceGatewayClient};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct A2AServer {
    #[allow(dead_code)]
    pub(super) config: Config,
    pub(super) agent_card: Option<AgentCard>,
    pub(super) agent: Option<Arc<Agent>>,
    pub(super) gateway_url: String,
    pub(super) storage: Arc<dyn Storage>,
    pub(super) background_task_handler: Option<Arc<dyn TaskHandler>>,
    pub(super) streaming_task_handler: Option<Arc<dyn StreamableTaskHandler>>,
    pub(super) task_manager: Option<DefaultTaskManager>,
    /// When `Some`, the JSON-RPC route (`POST /a2a`) is wrapped with an
    /// auth middleware that requires a valid bearer token. `GET /health`
    /// and `GET /.well-known/agent.json` are always public.
    pub(super) auth_verifier: Option<Arc<dyn AuthVerifier>>,
    /// Optional artifact service used to mint and serve file/data
    /// artifacts. When `Some` and `config.artifacts_config.enable` is
    /// true, [`serve`](Self::serve) spawns a dedicated artifacts HTTP
    /// server alongside the main A2A listener.
    pub(super) artifact_service: Option<Arc<dyn ArtifactService>>,
}

impl A2AServer {
    /// Access the storage backing this server. Useful for tests and
    /// callers that need to inspect or pre-populate state.
    pub fn storage(&self) -> Arc<dyn Storage> {
        Arc::clone(&self.storage)
    }

    /// Access the artifact service backing this server, if one is wired
    /// up. Useful for task handlers that want to mint file artifacts.
    pub fn artifact_service(&self) -> Option<Arc<dyn ArtifactService>> {
        self.artifact_service.clone()
    }

    pub async fn serve(mut self, addr: SocketAddr) -> Result<()> {
        let runner = self.task_manager.take().map(|m| m.start());
        let auth_verifier = self.auth_verifier.clone();
        let tls_config = self.config.tls_config.clone();
        let artifacts_config = self.config.artifacts_config.clone();
        let artifact_service = self.artifact_service.clone();

        let (artifacts_handle, retention_handle) =
            spawn_artifacts_subsystem(&artifacts_config, artifact_service.clone());

        let state = Arc::new(match auth_verifier {
            Some(v) => AppState::with_auth(self, v),
            None => AppState::new(self),
        });

        // Public routes - never gated by the auth middleware so health
        // probes and discovery clients keep working without a token.
        let public = Router::new()
            .route("/health", get(health_handler))
            .route("/.well-known/agent.json", get(agent_card_handler))
            .with_state(Arc::clone(&state));

        // Protected JSON-RPC route. The middleware is a no-op when
        // `AppState::auth_verifier` is `None`, but we attach it
        // unconditionally so the protected sub-router has a consistent
        // type regardless of configuration.
        let protected = Router::new()
            .route("/a2a", post(a2a_handler))
            .route_layer(middleware::from_fn_with_state(
                Arc::clone(&state),
                auth_middleware,
            ))
            .with_state(Arc::clone(&state));

        let app = public.merge(protected).layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::permissive()),
        );

        let result = if tls_config.enable {
            serve_tls(app, addr, &tls_config).await
        } else {
            serve_plain(app, addr).await
        };

        if let Some(runner) = runner {
            runner.shutdown().await;
        }

        if let Some(handle) = retention_handle {
            handle.abort();
        }
        if let Some(handle) = artifacts_handle {
            handle.abort();
            let _ = handle.await;
        }

        result
    }
}

/// Spawn the artifacts HTTP server + retention loop based on
/// [`ArtifactsConfig`]. Returns `(server_handle, retention_handle)` -
/// both `None` when artifacts are disabled or the bind address is
/// invalid.
fn spawn_artifacts_subsystem(
    artifacts_config: &crate::config::ArtifactsConfig,
    artifact_service: Option<Arc<dyn ArtifactService>>,
) -> (
    Option<tokio::task::JoinHandle<()>>,
    Option<tokio::task::JoinHandle<()>>,
) {
    let Some(service) = artifact_service else {
        if artifacts_config.enable {
            warn!(
                "ARTIFACTS_ENABLE=true but no artifact service is configured; skipping \
                 artifacts server startup"
            );
        }
        return (None, None);
    };
    if !artifacts_config.enable {
        return (None, None);
    }
    let server_cfg = artifacts_config.server.clone();
    let bind_addr: SocketAddr = match format!("{}:{}", server_cfg.host, server_cfg.port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            warn!(
                "invalid artifacts bind address `{}:{}`: {e}; artifacts server disabled",
                server_cfg.host, server_cfg.port,
            );
            return (None, None);
        }
    };
    let artifacts_server = ArtifactsServer::new(server_cfg.clone(), Arc::clone(&service))
        .with_tls(server_cfg.tls.clone());
    info!(
        "spawning artifacts server on {}:{}",
        server_cfg.host, server_cfg.port,
    );
    let server_handle = tokio::spawn(async move {
        if let Err(e) = artifacts_server.serve(bind_addr).await {
            error!("artifacts server exited with error: {e}");
        }
    });
    let retention_handle = spawn_retention_task(
        Arc::clone(&service),
        artifacts_config.retention.cleanup_interval,
        artifacts_config.retention.max_age,
        Some(artifacts_config.retention.max_artifacts),
    );
    (Some(server_handle), Some(retention_handle))
}

async fn serve_plain(app: Router, addr: SocketAddr) -> Result<()> {
    info!("A2A Server starting on http://{}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow!("Failed to bind to address {}: {}", addr, e))?;

    let serve = axum::serve(listener, app);

    tokio::select! {
        res = serve => res.map_err(|e| anyhow!("Server error: {}", e)),
        _ = tokio::signal::ctrl_c() => {
            info!("SIGINT received, draining workers");
            Ok(())
        }
    }
}

async fn serve_tls(app: Router, addr: SocketAddr, tls: &crate::config::TlsConfig) -> Result<()> {
    let server_config =
        build_server_config(tls).map_err(|e| anyhow!("TLS configuration is invalid: {}", e))?;

    let mtls_enabled = tls.client_ca_path.is_some();
    if mtls_enabled {
        info!(
            cert = %tls.cert_path,
            client_ca = ?tls.client_ca_path,
            "A2A Server starting on https://{} (mTLS required)",
            addr,
        );
    } else {
        info!(
            cert = %tls.cert_path,
            "A2A Server starting on https://{} (TLS, no client auth)",
            addr,
        );
    }

    let acceptor = MtlsAcceptor::new(server_config);
    let handle = Handle::new();

    let server = axum_server::bind(addr)
        .acceptor(acceptor)
        .handle(handle.clone())
        .serve(app.into_make_service());

    tokio::select! {
        res = server => res.map_err(|e| anyhow!("TLS server error: {}", e)),
        _ = tokio::signal::ctrl_c() => {
            info!("SIGINT received, draining workers");
            handle.graceful_shutdown(Some(std::time::Duration::from_secs(5)));
            Ok(())
        }
    }
}

async fn health_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HealthStatus>, StatusCode> {
    debug!("Health check requested");

    let gateway_healthy = if state.server.agent.is_some() {
        let gateway_client = InferenceGatewayClient::new(&state.server.gateway_url);
        gateway_client.health_check().await.unwrap_or(false)
    } else {
        false
    };

    let status = if gateway_healthy && state.server.agent.is_some() {
        "healthy"
    } else if state.server.agent.is_some() {
        "degraded"
    } else {
        "healthy"
    };

    let health = HealthStatus {
        status: status.to_string(),
        timestamp: chrono::Utc::now(),
        details: Some(serde_json::json!({
            "has_agent": state.server.agent.is_some(),
            "gateway_healthy": gateway_healthy,
            "version": env!("CARGO_PKG_VERSION"),
        })),
    };

    debug!("Health status: {}", health.status);
    Ok(Json(health))
}

async fn agent_card_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AgentCard>, StatusCode> {
    debug!("Agent card requested");

    if let Some(ref agent_card) = state.server.agent_card {
        debug!("Returning configured agent card");
        return Ok(Json(agent_card.clone()));
    }

    error!("No agent card configured - server should not have started without one");
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
