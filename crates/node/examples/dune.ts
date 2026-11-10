import { DuneClient, exactNumber, type dune } from "../js/index.js";

async function query(apiKey: string): Promise<dune.QueryResults | undefined> {
  const client = new DuneClient({ apiKey });
  try {
    const execution = await client.executeQuery({
      queryId: 123n,
      request: { query_parameters: { amount: exactNumber("0.1000000000000000001") } },
    });
    const status = await client.getExecutionStatus({ executionId: execution.execution_id });
    if (status.state === "QUERY_STATE_COMPLETED") {
      return await client.getExecutionResults({ executionId: execution.execution_id, request: { limit: 100n } });
    }
  } finally {
    await client.close();
  }
}
void query; // Example only; executing paid queries is opt-in.
