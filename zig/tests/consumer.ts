import { DeriveClient, EvmReader, type DeriveModels } from "aux-zig";

export async function getOrders(client: DeriveClient, request: DeriveModels.GetOpenOrdersRequest): Promise<DeriveModels.AggregatedOrdersResult> {
  return client.privateGetOpenOrders(request);
}

export async function blockHeight(reader: EvmReader): Promise<bigint> {
  return reader.blockNumber();
}

import { ExtendedClient, PhoenixClient, type ExtendedModels, type PhoenixModels } from "aux-zig";

export function markets(client: ExtendedClient): Promise<ExtendedModels.GetMarketsResult> {
  return client.getMarkets();
}

export function exchange(client: PhoenixClient): Promise<PhoenixModels.ExchangeView> {
  return client.getExchange();
}

import { ParadexClient, type ParadexModels, PolymarketPerpsClient, solana, type DepositParams, type SignedDeposit } from "aux-zig";

export function paradexAccount(token: string): Promise<ParadexModels.ResponsesAccountSummaryResponse> {
  return new ParadexClient({bearer:{token, read_only:true}}).getAccount();
}

export function proxyBalances(proxy: string, secret: string) {
  return new PolymarketPerpsClient({perps:{proxy, secret}}).getBalances();
}

export function signDeposit(seed: string, params: DepositParams): SignedDeposit {
  return solana.signDeposit(seed, solana.prepareDeposit(params));
}
