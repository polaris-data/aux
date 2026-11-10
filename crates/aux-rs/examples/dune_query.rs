//! Opt-in example: submitting a query consumes Dune credits.
use aux_rs::dune::{DuneClient, ExecuteQueryRequest, ResultOptions};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query_id: u64 = std::env::args()
        .nth(1)
        .ok_or("usage: dune_query QUERY_ID")?
        .parse()?;
    let client = DuneClient::new(std::env::var("DUNE_API_KEY")?)?;
    let result = async {
        let execution = client
            .execute_query(query_id, ExecuteQueryRequest::default())
            .await?;
        println!("Execution: {}", execution.execution_id);
        let status = client.get_execution_status(&execution.execution_id).await?;
        println!("Status: {}", status.state);
        if status.state == "QUERY_STATE_COMPLETED" {
            let results = client
                .get_execution_results(
                    &execution.execution_id,
                    ResultOptions {
                        limit: Some(100),
                        ..Default::default()
                    },
                )
                .await?;
            println!("{}", serde_json::to_string_pretty(&results)?);
        } else {
            println!("Poll this execution explicitly to retrieve results when ready.");
        }
        Ok::<_, Box<dyn std::error::Error>>(())
    }
    .await;
    client.transport().shutdown().await?;
    result
}
