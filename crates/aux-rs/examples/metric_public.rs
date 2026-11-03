//! Explicit read-only Metric API check. No credentials or transactions.
use aux_rs::metric::{MetadataRequest, Metric, MetricClient};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chain_id = std::env::var("METRIC_CHAIN_ID")
        .unwrap_or_else(|_| "1".into())
        .parse()?;
    let client = MetricClient::new(Metric::new(chain_id)?)?;
    let chains = client.chains().await?;
    let metadata = client
        .metadata(MetadataRequest {
            count: Some(1),
            ..Default::default()
        })
        .await?;
    println!(
        "{} chains; {} pools on chain {}",
        chains.chains.len(),
        metadata.total,
        chain_id
    );
    if let Some(pool) = metadata.data.first() {
        let quote = client.bid_ask(&pool.pool_address, Some(false)).await?;
        println!(
            "{}: status={}, bidAdj={:?}, askAdj={:?}",
            pool.pair, quote.price_provider_status, quote.bid_adj, quote.ask_adj
        );
    }
    client.transport().shutdown().await?;
    Ok(())
}
