use inference_gateway_adk::A2AServerBuilder;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server = A2AServerBuilder::new()
        .with_gateway_url("http://localhost:8080/v1")
        .build()
        .await?;

    let addr = "0.0.0.0:8081".parse()?;
    info!("Minimal A2A server with Inference Gateway SDK running on port 8081");
    info!("Using Inference Gateway at: http://localhost:8080/v1");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}
