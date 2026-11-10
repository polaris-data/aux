import { BirdeyeClient, exactNumber, type birdeye } from "../js/index.js";

async function price(apiKey: string): Promise<birdeye.ApiResponse> {
  const client = new BirdeyeClient({ apiKey, chain: "solana" });
  try {
    await client.multiPricePost({ request: {
      include_liquidity: true,
      body: { list_address: "TOKEN_A,TOKEN_B" },
    }});
    return await client.price({ request: {
      address: "So11111111111111111111111111111111111111112",
      check_liquidity: exactNumber("0.1000000000000000001"),
    }});
  } finally {
    await client.close();
  }
}
void price; // Compute-unit-consuming requests are opt-in.
