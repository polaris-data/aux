//! Review a caller-supplied native transaction; --execute explicitly signs and submits it once.
use aux_rs::metric::{ChainConfig, LocalEvmSigner, MetricChainClient, PreparedTransaction};
use zeroize::Zeroizing;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let file = args
        .iter()
        .find(|s| s.as_str() != "--execute")
        .ok_or("usage: metric_contract transaction.json [--execute]")?;
    let request: PreparedTransaction = serde_json::from_slice(&std::fs::read(file)?)?;
    request.validate()?;
    if !args.iter().any(|s| s == "--execute") {
        println!("{}", serde_json::to_string_pretty(&request)?);
        return Ok(());
    }
    let read_rpc = std::env::var("METRIC_READ_RPC")?;
    let write_rpc = std::env::var("METRIC_WRITE_RPC").unwrap_or_else(|_| read_rpc.clone());
    let mut config = ChainConfig::new(request.chain_id, read_rpc, write_rpc)?;
    if let Ok(hash) = std::env::var("METRIC_CODE_HASH") {
        config = config.with_code_hash(request.to, hash.parse()?);
    }
    let signer = LocalEvmSigner::from_hex(Zeroizing::new(std::env::var("METRIC_PRIVATE_KEY")?))?;
    let client = MetricChainClient::new(config)?;
    let hash = client.send_transaction(request, &signer).await?;
    println!("submitted {hash:#x}");
    for _ in 0..50 {
        if let Some(receipt) = client.receipt(hash).await? {
            println!("{}", serde_json::to_string_pretty(&receipt)?);
            client.transport().shutdown().await?;
            return Ok(());
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    println!("receipt not observed; reconcile {hash:#x}");
    client.transport().shutdown().await?;
    Ok(())
}
