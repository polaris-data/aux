import {HyperliquidClient, MetricClient, HyperliquidSigner, type hyperliquid} from "aux-ts";
async function publicRead() {
  const client = new HyperliquidClient();
  try {
    const book: hyperliquid.L2Book = await client.l2Book({coin:"ETH"});
    const timestamp: bigint = book.time;
    console.log(timestamp, book.levels[0][0]?.px);
    const stream = await client.subscribeL2Book({coin:"ETH"});
    try { for await (const update of stream) { console.log(update); break; } }
    finally { await stream.close(); }
  } finally { await client.close(); }
}
const metric = new MetricClient({chainId:1n});
const signer = new HyperliquidSigner({privateKey:"0x"+"01".repeat(32)});
void publicRead;
void metric.close();
void signer;
// @ts-expect-error Financial decimal input must be a string.
const invalid: hyperliquid.Level = {px:1.25,sz:"1",n:1n};
void invalid;
async function typedOrder(client: HyperliquidClient) {
  const request: hyperliquid.OrderRequest = {
    orders:[{a:0,b:true,p:"3000",s:"0.01",r:false,t:{limit:{tif:"Gtc"}}}],
    grouping:"na",
  };
  return client.order({request});
}
void typedOrder;
async function typedDeriveOrder(client: import("aux-ts").DeriveClient) {
  // The native signer fills nonce, signer, signature and signature_expiry_sec.
  return client.privateOrder({request:{
    amount:"0.1", direction:"buy", instrument_name:"ETH-PERP",
    limit_price:"3000", max_fee:"0.1", subaccount_id:1n,
  }});
}
void typedDeriveOrder;
