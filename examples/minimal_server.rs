use inference_gateway_adk::A2AServerBuilder;
use tokio;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt().init();

    // Create the simplest A2A server
    let server = A2AServerBuilder::new()
        .build()
        .await?;

    // Start server
    let addr = "0.0.0.0:8080".parse()?;
    info!("Minimal A2A server running on port 8080");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}