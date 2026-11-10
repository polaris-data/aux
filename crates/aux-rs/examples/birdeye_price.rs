//! Opt-in example: Birdeye API requests consume compute units.
use aux_rs::birdeye::{BirdeyeClient, PriceRequest};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "So11111111111111111111111111111111111111112".into());
    let client = BirdeyeClient::new(std::env::var("BIRDEYE_API_KEY")?)?
        .with_chain(std::env::var("BIRDEYE_CHAIN").unwrap_or_else(|_| "solana".into()))?;
    let result = client
        .price(PriceRequest {
            address,
            ..Default::default()
        })
        .await;
    client.transport().shutdown().await?;
    println!("{}", serde_json::to_string_pretty(&result?)?);
    Ok(())
}
