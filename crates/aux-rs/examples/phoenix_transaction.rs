//! Explicit RPC preparation and signing. Submission requires caller opt-in and is disabled in CI.
use aux_rs::phoenix::*;
use std::collections::BTreeMap;
use zeroize::Zeroizing;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc = std::env::var("AUX_PHOENIX_RPC_URL")?;
    let instructions: Vec<instructions::types::Instruction> =
        serde_json::from_str(&std::env::var("AUX_PHOENIX_INSTRUCTIONS")?)?;
    let payer = std::env::var("AUX_PHOENIX_FEE_PAYER")?;
    let client = PhoenixChainClient::new(&rpc)?;
    let prepared = client.prepare(&instructions, &payer).await?;
    println!(
        "Prepared reviewed transaction with {} required signers",
        prepared.required_signers.len()
    );
    if std::env::var("AUX_PHOENIX_SUBMIT").as_deref() == Ok("1") {
        if std::env::var_os("CI").is_some() {
            return Err("live submission is disabled in CI".into());
        }
        let signer =
            LocalSigner::from_hex(Zeroizing::new(std::env::var("AUX_PHOENIX_SIGNER_KEY")?))?;
        let additional: BTreeMap<String, String> =
            std::env::var("AUX_PHOENIX_ADDITIONAL_SIGNATURES")
                .ok()
                .map(|v| serde_json::from_str(&v))
                .transpose()?
                .unwrap_or_default();
        let signatures = prepared.sign(&signer, additional).await?;
        let signed = prepared.assemble(&signatures)?;
        let simulation = client.simulate(&signed).await?;
        if !simulation["value"]["err"].is_null() {
            return Err("simulation failed; no transaction submitted".into());
        }
        let signature = client.submit(signed).await?;
        println!(
            "Submitted {signature}; query signature_status and reconcile Phoenix account state explicitly"
        );
    }
    client.transport().shutdown().await?;
    Ok(())
}
