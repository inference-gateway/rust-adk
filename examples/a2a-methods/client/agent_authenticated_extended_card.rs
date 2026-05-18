//! `agent/getAuthenticatedExtendedCard` - fetch the authenticated extended
//! [`AgentCard`] for the calling tenant.
//!
//! The example server advertises `supportsExtendedAgentCard: true` on its
//! agent card, so the JSON-RPC call returns the configured card. When the
//! flag is absent or `false`, the server responds with
//! `METHOD_NOT_FOUND` so clients can fall back to the unauthenticated
//! card served at `/.well-known/agent.json`.
//!
//! For contrast, the example also fetches the unauthenticated card from
//! the discovery endpoint and logs both side-by-side.
//!
//! ```bash
//! cargo run -p a2a-methods-server
//! cargo run -p a2a-methods-agent-authenticated-extended-card
//! ```
use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::GetExtendedAgentCardRequest;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8085".to_string());
    let client = A2AClient::new(&server_url)?;

    // 1. Unauthenticated card (discovery endpoint) for comparison.
    let discovery_card = client.get_agent_card().await?;
    info!(
        "/.well-known/agent.json → name={:?} version={:?} supportsExtendedAgentCard={:?}",
        discovery_card.name, discovery_card.version, discovery_card.supports_extended_agent_card
    );

    // 2. Authenticated extended card via JSON-RPC.
    match client
        .get_authenticated_extended_card(GetExtendedAgentCardRequest {
            tenant: "example".to_string(),
        })
        .await
    {
        Ok(extended_card) => {
            info!(
                "agent/getAuthenticatedExtendedCard → name={:?} version={:?} skills={} description={:?}",
                extended_card.name,
                extended_card.version,
                extended_card.skills.len(),
                extended_card.description,
            );
        }
        Err(err) => {
            // The server returns METHOD_NOT_FOUND when the underlying
            // agent card does not advertise supportsExtendedAgentCard=true.
            // Clients should fall back to the discovery card in that
            // case - the example server opts in, so this branch should
            // not fire when running against `a2a-methods-server`.
            info!(
                "agent/getAuthenticatedExtendedCard not available ({err}); \
                 falling back to the unauthenticated card from /.well-known/agent.json"
            );
        }
    }

    Ok(())
}
