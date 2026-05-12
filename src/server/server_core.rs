use super::agent::Agent;
use super::protocol::{AppState, a2a_handler};
use super::task_handler::{StreamableTaskHandler, TaskHandler};
use crate::a2a_types::AgentCard;
use crate::client::HealthStatus;
use crate::config::Config;
use crate::storage::InMemoryStorage;
use anyhow::{Result, anyhow};
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use inference_gateway_sdk::{InferenceGatewayAPI, InferenceGatewayClient};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, error, info};

#[derive(Debug)]
pub struct A2AServer {
    #[allow(dead_code)]
    pub(super) config: Config,
    pub(super) agent_card: Option<AgentCard>,
    pub(super) agent: Option<Arc<Agent>>,
    pub(super) gateway_url: String,
    pub(super) storage: Arc<InMemoryStorage>,
    pub(super) background_task_handler: Option<Arc<dyn TaskHandler>>,
    pub(super) streaming_task_handler: Option<Arc<dyn StreamableTaskHandler>>,
}

impl A2AServer {
    /// Access the in-memory storage backing this server. Useful for tests
    /// and callers that need to inspect or pre-populate state.
    pub fn storage(&self) -> Arc<InMemoryStorage> {
        Arc::clone(&self.storage)
    }

    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        let state = AppState { server: self };

        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/.well-known/agent.json", get(agent_card_handler))
            .route("/a2a", post(a2a_handler))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive()),
            )
            .with_state(Arc::new(state));

        info!("A2A Server starting on {}", addr);

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| anyhow!("Failed to bind to address {}: {}", addr, e))?;

        axum::serve(listener, app)
            .await
            .map_err(|e| anyhow!("Server error: {}", e))?;

        Ok(())
    }
}

async fn health_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HealthStatus>, StatusCode> {
    debug!("Health check requested");

    let gateway_client = InferenceGatewayClient::new(&state.server.gateway_url);
    let gateway_healthy = gateway_client.health_check().await.unwrap_or(false);

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
            "sdk_version": "0.13.3"
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
