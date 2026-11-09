use ts_rs::TS;
fn declaration<T: TS>(config: &ts_rs::Config, extra: bool) -> String {
    let value = T::decl(config);
    if extra {
        format!(
            "{} & {{ [key: string]: LosslessValue }};",
            value.trim_end_matches(";")
        )
    } else {
        value
    }
}
fn main() {
    let config = ts_rs::Config::default();
    let mut result = String::from(
        "// Generated from Rust serde models by ts-rs. Do not edit.\nimport type { LosslessValue } from \"./index.js\";\n",
    );
    result.push_str("export namespace hyperliquid {\n");
    result.push_str("export type Address = string; export type Cloid = string;\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::AccountStateEvent>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Action>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Asset>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::AssetPosition>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::BatchData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::BatchResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::BuilderFee>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Cancel>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::CancelByCloid>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::CancelByCloidRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::CancelRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::CumulativeFunding>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::ExchangeResponse::<String>>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Fill>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::FilledOrder>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Grouping>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::ItemStatus>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::L2Book>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::L2BookRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Level>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Leverage>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::MarginSummary>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::MarginTable>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::MarginTier>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Meta>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::MetaRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::NamedGrouping>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::OpenOrder>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::OpenOrdersEvent>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::OpenOrdersRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Order>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::OrderRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::OrderType>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::OrderUpdate>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Position>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::RestingOrder>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::signing::Signature>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::SignedRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::SubBuilder>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Subscription>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::SubscriptionResponse>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Tif>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Tpsl>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::Trade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::UserFills>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::signing::UserSignedAction>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::UserState>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::hyperliquid::UserStateRequest>(&config, false)
    ));
    result.push_str("}\n");
    result.push_str("export namespace arcus {\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Account>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::CancelOrderRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Channel>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Event>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::MarketInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Markets>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::OpenOrders>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::OrderBook>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::OrderRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::OrderResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::OrderType>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Position>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Positions>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Side>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Subscription>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::TimeInForce>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Trade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::arcus::Trades>(&config, true)
    ));
    result.push_str("}\n");
    result.push_str("export namespace derive {\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AccountFeeInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Action>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ActionPayload>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AggregatedOrdersResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AggregatedTriggerOrdersResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AssetEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AssetUniverse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AuctionBidEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AuctionDetails>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AuctionDetailsSubaccountBalances>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AuctionHistory>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::AuctionResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::BalanceUpdate>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::BestQuoteChannelResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::BurnSharesRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelAlgoOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelAllAlgoOrdersRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelAllRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelAllTriggerOrdersRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelBatchQuotesRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelBatchResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelBatchRfqsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelBatchRfqsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelByInstrumentRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelByInstrumentResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelByLabelRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelByLabelResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelByNonceRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelByNonceResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelQuoteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelRfqRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelTriggerOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CancelVaultRequestRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ChangeSubaccountLabelRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Collateral>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CreateOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::CreateVaultRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Currency>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::DailyTradingStatistics>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::DecodeActionRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::DecodeActionResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::DeleteSubaccountRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::DeleteSubaccountResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::DepositBridgeOrigin>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::DepositEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::DepositHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::EditSessionKeyRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::EmptyRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Erc20Details>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Event>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ExecuteQuoteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ExpirySettlementPrice>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ExternalTransferActionDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ForceBurnRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ForwardFeedDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ForwardedToken>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::FundingFeedDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::FundingRateCandle>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::FundingRateHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetAccountRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetAlgoOrdersRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetAllInstrumentsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetAllInstrumentsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetAllPortfoliosRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetAllReferralCodesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetCollateralsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetCuratedVaultsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetCurrencyRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetDepositHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetErc20TransferHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetFundingHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetFundingRateHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetIndexChartDataRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetInstrumentRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetInterestHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetInterestRateHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetLatestSignedFeedsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetLatestSignedFeedsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetLiquidationHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetLiveBurnRequestsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetLiveIncidentsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetLiveIncidentsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetLiveMintRequestsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetLiveVaultRequestsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetMakerProgramScoresParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetMakerProgramScoresResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetMakerProgramsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetMakerRfqHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetMarginResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetOnchainActionHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetOnchainActionHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetOpenOrdersRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetOptionSettlementHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetOptionSettlementPricesRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetOrderHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetPendingDepositsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetPendingDepositsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetPerpImpactTwapRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetPositionsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetPublicTradeHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetQuotesRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetReferralPerformanceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetReferralPerformanceResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetRfqsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetShareholderVaultsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetSubaccountRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetSubaccountValueHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetSubaccountsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetTickerRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetTickersRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetTickersResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetTradeHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetTradingviewChartDataRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetTransactionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetTransactionResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetTriggerOrdersRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetVaultActionHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetVaultPerformanceHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetVaultRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetVaultRequestHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetVaultSharesRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetVaultsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetWalletsFromSessionKeyRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetWithdrawAddressesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetWithdrawAddressesResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::GetWithdrawalHistoryRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::IncidentResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::IndexCandle>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Instrument>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::InterestHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::InterestPayment>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::InterestRateCandle>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::InterestRateHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::LegUnpricedParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::LendingDetails>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::LiquidationHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::LiveAuction>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::LoginRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ManagerCollateral>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::MarginWatchRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::MarginWatchResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::MintSharesRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::MmpConfigResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::MmpScopeRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ModuleData>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::MultipleVaultRequestsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OffchainAckResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Ohlc>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OnchainActionHistoryEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OpenInterestStats>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OptionDetails>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OptionPricing>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OptionSettlementHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OptionSettlementPricesResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OptionSettlementResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OracleSignatureDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Order>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OrderActionDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OrderActionInputData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OrderCreatedResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OrderDebugResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OrderQuoteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OrderQuoteResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::OrderbookPayloadDoc>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PaginatedOrdersResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PaginatedTradesResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PaginatedVaultActionHistory>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PaginatedVaultRequestHistory>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Pagination>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PendingDepositBridgeOrigin>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PendingDepositEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PerpDetails>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PerpFeedDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PerpImpactTwapResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PerpSettlementEventResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PerpSettlementHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PollQuotesRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PollRfqsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Position>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PricedLegParamsAndResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateChangeSubaccountLabelResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateGetAccountResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateGetCollateralsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateGetMarginRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateGetPositionsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateGetSubaccountsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateLiquidateRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateLiquidateResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateSessionKeysResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateSetSessionKeyResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateTransferSpotExternalRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateTransferSpotExternalResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateTransferSpotRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateTransferSpotResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateWithdrawRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PrivateWithdrawResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ProgramResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ProtocolVault>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicExecuteQuoteDebugRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicGetLiveAuctionsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicGetMarginRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicGetSubaccountsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicGetWalletRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicGetWalletResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicGetWalletsFromSessionKeyResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicMarginWatchResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicQuote>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicRfq>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicSendQuoteDebugRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicTrade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicTradesResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicVaultActionResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::PublicWithdrawDebugRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Quote>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::QuoteExecuteDebugResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::QuoteExecuteResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::QuoteGetResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::QuotePollResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::QuotePublishResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::QuoteReplaceResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::QuoteSendDebugResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RFQGetResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RFQPollResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RPCError>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RateFeedDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RateLimitInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RateLimitResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ReferralPerformanceByInstrumentType>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Referrer>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RegisterBridgeDepositParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RegisterDepositAddressParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RegisterDepositAddressResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RegisterWithdrawAddressParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RegisterWithdrawAddressResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RejectDepositRequestRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ReplaceOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ReplaceOrderResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ReplaceQuoteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RequestVaultDepositRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RequestVaultWithdrawRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Rfq>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RfqGetBestQuoteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RfqGetBestQuoteResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RfqLegResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RfqMakerActionDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RfqTakerActionDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RiskUniverse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::RiskUniverseManager>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::ScoreBreakdown>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SecurityModuleDetails>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SendQuoteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SendRfqRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SessionKey>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SessionKeysRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SetCancelOnDisconnectRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SetMmpConfigRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SetMmpConfigResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SetSessionKeyActionDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SetSessionKeyActionInputData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SetSessionKeyDebugResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SetSessionKeyRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SettledTrade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SignedAction>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SignedTransferQuoteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SigningLeg>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SimulatedCollateralRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SimulatedPositionRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SpotAssetEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SpotFeedDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SpotFeedEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SpotFeedPayload>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SpotFeedPayloadFeeds>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SpotPublicDetails>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SpotUniverse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SrmPerpMarginRequirements>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Subaccount>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SubaccountValueEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SubaccountValueHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SubscribeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::SubscribeResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Subscription>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TickerSlimPayload>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TickerSlimSnapshot>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Trade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TradeHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TradingviewCandle>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferActionDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferPositionsDebugResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferPositionsMakerActionInputData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferPositionsMakerDebugResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferPositionsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferPositionsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferPositionsTakerActionInputData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferPositionsTakerDebugResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferSpotActionInputData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferSpotDebugResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferSpotExternalActionInputData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::TransferSpotExternalDebugResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::UniverseManagers>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::UnsubscribeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::UnsubscribeResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::UpdateVaultInfoRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::UpdateWhitelistedRecipientsActionDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::UpdateWhitelistedRecipientsActionInputData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::UpdateWhitelistedRecipientsDebugResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::UpdateWhitelistedRecipientsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::UpdateWhitelistedRecipientsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::Vault>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultActionResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultCancelResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultConfig>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultCreateResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultDepositHold>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultForceBurnResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultIdsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultPerformanceHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultPerformancePointResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultRequestAckResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultRequestId>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultRequestResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultSettleResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultShareEntryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultSharesResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VaultsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VolFeedDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::VolSVIParamDataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::WithdrawAddressEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::WithdrawalEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::WithdrawalForward>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::derive::WithdrawalHistoryResult>(&config, true)
    ));
    result.push_str("}\n");
    result.push_str("export namespace qfex {\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::AddOrder>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::Balance>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::CancelOrder>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::CancelOrderIdType>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::Channel>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::Contracts>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::Event>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::OrderBook>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::OrderBookUpdate>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::OrderResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::OrderType>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::Position>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::PositionsBalance>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::Refdata>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::Side>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::Symbol>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::TimeInForce>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::Trade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::UserOrders>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::qfex::UserOrdersRequest>(&config, false)
    ));
    result.push_str("}\n");
    result.push_str("export namespace monaco {\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::AccountBalance>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::AccountSubscription>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BalancesRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCancelAllResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCancelError>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCancelOrdersRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCancelOrdersResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCancelResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCreateError>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCreateOrderItem>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCreateOrdersRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCreateOrdersResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchCreateResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchReplaceError>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchReplaceOrderItem>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchReplaceOrdersRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchReplaceOrdersResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::BatchReplaceResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::CancelOrderRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::CancelOrderResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ChallengeRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ChallengeResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::CreateOrderRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::CreateOrderResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::Event>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ExecutionPriceRange>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::FundingHistoryRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::FundingRecord>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::FundingRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetBalancesResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetFundingStateResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetIndexPriceResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetMarginAccountSummaryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetMarkPriceResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetMarketMetadataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetOrderResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetOrderbookResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetTradesResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::GetTradingPairResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::IndexComponent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::IndexPriceRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ListFundingHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ListMarginAccountsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ListOrdersResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ListPositionsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ListTradingPairsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::LoginOptions>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::MarginAccountSummary>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::MarginAccountsRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::MarkPriceRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::MarketMetadataRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::MatchResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::OrderBookRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::OrderRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::OrderbookData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::OrdersRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ParentMarginAccountRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ParentTpSlLeg>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ParentTrailingStopLeg>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::Position>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::PositionsRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::PriceChange>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::PriceLevel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::PublicTrade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::RefreshRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::RefreshResponse>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ReplaceOrderRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::ReplaceOrderResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::RequestSigning>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::RevokeRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::RevokeResponse>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::SessionInfo>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::Subscription>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::TradeData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::TradesRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::TradingPairData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::TradingPairRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::TradingPairsRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::UpdatedFields>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::UserInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::VerifyRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::monaco::VerifyResponse>(&config, true)
    ));
    result.push_str("}\n");
    result.push_str("export namespace paradex {\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::AccountEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ActivateSubkeyParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ArchiveSubAccountParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::BboEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockOffersCancelParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockOffersCreateParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockOffersExecuteParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockOffersGetParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockOffersListParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockTradesCancelParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockTradesCreateParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockTradesExecuteParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockTradesGetParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::BlockTradesListParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::BookEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::BookEventAsksItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::BookEventBidsItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::CancelAlgoOrderParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::CreateAlgoOrderParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::CreateDepositAddressParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::CreateTokenParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::CreateXpTransferV2Params>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::DecodedFrame>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::DeleteAccountMmpParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::Event>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::EvmAuthParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::EvmOnboardingParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::FillEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::FundingDataEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::FundingRateComparisonEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAccountHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAccountMarginParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAccountMmpParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAccountParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAccountProfileParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAccountSettingsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAccountsInfoParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAccountsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAlgoOrderByIdParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAlgoOrdersHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetAnnouncementsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetBalanceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetBboInteractiveParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetBboParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetComplianceActionsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetDepositAddressParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetDimeConfigParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetFillsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetFundingDataParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetFundingParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetGlobalStakingBalanceHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetImpactPriceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetInsuranceFundParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetJwtParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetLiquidationsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetMarginConfigParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetMarketSettlementPriceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetMarketsHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetMarketsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetMarketsSummaryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetOnboardingParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetOpenAlgoOrdersParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetOpenOrdersParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetOrderbookInteractiveParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetOrderbookParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetOrdersParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetPortfolioMarginConfigParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetPositionsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetPublicXpTransfersV2Params>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetReferralQrCodeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetReferralsConfigParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetReferralsSummaryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetStakingBalanceHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetStakingConfigParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetStakingHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetStakingSummaryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetSubAccountsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetSubkeyJwtParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetSubkeyParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetSystemConfigParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetSystemStateParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetSystemStatsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetSystemTimeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetTransferFeeConfigV2Params>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetTransfersParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetVolumeTiersParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetXpBalanceV2Params>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetXpTransferByIdV2Params>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::GetXpTransfersV2Params>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::HeartbeatEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::JwksParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ListDepositNetworksParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ListSubkeysParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ListTokensParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::MarketSummaryEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::MarketsKlinesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::Messagesv1NftPrice>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OnboardingParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::OrderEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OrdersCancelAllParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OrdersCancelBatchParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OrdersCancelByClientIdParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OrdersCancelParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OrdersGetByClientIdParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OrdersGetParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OrdersModifyParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OrdersNewBatchParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::OrdersNewParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::PositionEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::PostAccountTerminalTokenParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RecordComplianceActionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RefreshAccountInventoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RegisterSubkeyParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsAccountMarginRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsAccountMmpConfigRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsAccountMmpResetRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsActivateSubkey>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsActivateSubkeyResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsAlgoOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsBlockExecuteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsBlockOfferInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsBlockOfferRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsBlockTradeInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsBlockTradeRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsCancelOrderBatchRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsCreateDepositAddressReq>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsCreateSubkey>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsCreateToken>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsCreateVault>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsCreateXPTransferRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsModifyOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsOnboarding>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsRecordComplianceActionRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsRfqCreateRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsRfqExecuteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsRfqLeg>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsUpdateAccountMaxSlippageRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsUpdateAccountProfileRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsUpdateAllowedCIDRs>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsUpdateNotificationPreferencesRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsUpdateSizeCurrencyDisplayRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsUpdateSubkey>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsUpdateSubkeyBlockTradeSigning>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsUpdateTradingValueDisplayRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RequestsUtm>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResetAccountMmpParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAPIResults>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountAnalytics>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountDailyPnlPoint>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountHistoricalDataResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountHourlyPnlPoint>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountInfoResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountMarginEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountMmpConfigResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountMmpResetRefusedData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountMmpResetResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountMmpResetResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountMonthlyPnlEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountProfileResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountSettingsResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAccountSummaryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAlgoOrderResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAnnouncement>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesApiError>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesApiToken>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAskBidArray>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesAuthResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesBBOResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesBalanceResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesBatchResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesBlockTradeConstraints>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesBlockTradeDetailFullResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesBlockTradeDetailResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesBlockTradeOrder>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesBlockTradeSignature>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesBridgedToken>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesCancelOrderBatchResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesCancelOrderResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesComplianceActionResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesCreateTokenResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesCreateTransferResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesDelta1CrossMarginParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesDepositAddressDetailsResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesDepositAddressResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesDepositNetwork>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesDepositNetworksResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesDimeAllocationResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesDimeConfigResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesDiscordProfile>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesErrorResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesFeeWithCap>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesFees>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesFillResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesFundingDataResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesFundingPayment>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesGetAccountMarginConfigsResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesGetAccountMmpConfigsResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesGetAccountsInfoResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesGetOnboardingResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesGetSubAccountsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesGetXPBalanceResponseV2>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesGreeks>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesImpactPriceResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesInsuranceAccountResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesJwkKey>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesJwksResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesLiquidationResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesMakerTakerFee>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesMarginParamsResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesMarketEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesMarketFeeConfig>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesMarketHistoryFields>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesMarketHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesMarketResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesMarketSettlementResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesMarketSummaryRestResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesNft>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesNotificationPreferencesResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesOnboardingDerivationInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesOptionCrossMarginParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesOptionMarginParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesOptionSpreadMarginParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesOrderResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesPaginatedAPIResults>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesParadigmEnableResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesPortfolioMarginScenarioResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesPositionResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesReferralConfigResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesReferralsResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRefreshInventoryResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRequestInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRevokeSubkeyResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRevokeTokenResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqAcceptedResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqBboLegResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqBboResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqDraftResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqLegResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqMarketInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqMarketsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqPlatformStateResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRfqResults>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesRollingDetailsResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesShareRateResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesStakingConfigResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesStakingStakeResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesStakingSummaryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesStakingTierResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesStrategy>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesSubkey>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesSystemConfigResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesSystemStateResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesSystemStatsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesSystemTimeResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesTradeResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesTradebustResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesTransactionResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesTransferResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesTwitterProfile>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesUpdateAccountMarginConfigResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultAccountStrategyAccrual>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultAccountSummaryResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultAnalytics>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultAnalyticsResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultHistoricalDataResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultMonthlyPnlEntry>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultSummaryResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultSummaryStrategy>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultTopHolder>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVaultsConfigResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVisibility>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVolShockParamsResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVolumeTierKindInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesVolumeTierResp>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesWindowedMetrics>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesWithdrawalLockRejectionResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesWithdrawalLockResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ResponsesXPTransfer>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RevokeSubkeyParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RevokeTokenParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqBboParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqCreateParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqDeleteParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqDraftCreateParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqDraftDeleteParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqDraftListParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqEnableParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqExecuteParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqListParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqMarketsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::RfqPlatformStateParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::SbeMessage>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::ServicesTransferFeeConfigResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::SubscribedEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::Subscription>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::sbe::TradeEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::TradebustsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::TradesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::TransactionsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountMarketMaxSlippageParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountMaxSlippageParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountMaxSlippageParamsBody>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountNotificationPreferencesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountProfileParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountProfileReferralCodeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountProfileUsernameParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountReferrerParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountSizeCurrencyDisplayParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateAccountTradingValueDisplayParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateLastSeenParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateSubkeyAllowedCidrsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateSubkeyBlockTradeSigningParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateSubkeyParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpdateTokenAllowedCidrsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpsertAccountMarginParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::UpsertAccountMmpParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsCreateParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetAccountSummaryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetAnalyticsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetBalanceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetConfigParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetHistoricalDataParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetMineParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetPositionsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetSummaryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::VaultsGetTransfersParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsAccountAccountSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsAccountChannelsAccountSubscribeWithdrawalLock>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsAccountChannelsAccountSubscribeWithdrawalLockRejection>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsAuthAuthPublish>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsAuthAuthSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsBalanceEventsBalanceEventsSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsBboMarketSymbolBboMarketSymbolSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsFillsMarketSymbolFillsMarketSymbolSubscribe>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsFundingDataMarketSymbolFundingDataMarketSymbolSubscribe>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::paradex::WsFundingPaymentsMarketSymbolFundingPaymentsMarketSymbolSubscribe,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::paradex::WsMarketsMarketSymbolChannelsMarketsMarketSymbolSubscribeChangedFields,
        >(&config, true)
    ));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsMarketsMarketSymbolChannelsMarketsMarketSymbolSubscribeChangedFieldsDelta1CrossMarginParams>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsMarketsMarketSymbolChannelsMarketsMarketSymbolSubscribeChangedFieldsFeeConfig>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsMarketsMarketSymbolChannelsMarketsMarketSymbolSubscribeChangedFieldsOptionCrossMarginParams>( &config, true)));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsMarketsMarketSymbolMarketsMarketSymbolSubscribe>(
            &config, true
        )
    ));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsMarketsSummaryMarketSymbolChannelsMarketsSummaryMarketSymbolSubscribeGreeks>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsMarketsSummaryMarketSymbolChannelsMarketsSummaryMarketSymbolSubscribeRollingDetails>( &config, true)));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::paradex::WsMarketsSummaryMarketSymbolMarketsSummaryMarketSymbolSubscribe,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsMmpChannelsMmpSubscribeConfig>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsMmpMmpSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsModifySubscriptionModifySubscriptionPublish>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsModifySubscriptionModifySubscriptionSubscribe>(
            &config, true
        )
    ));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskApi>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestAskInteractive>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidApi>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeBestBidInteractive>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeDeletesItems>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeInsertsItems>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickChannelsOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribeUpdatesItems>( &config, true)));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribe>( &config, true)));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCancelAllOrderCancelAllPublish>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCancelAllOrderCancelAllSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::paradex::WsOrderCancelBatchChannelsOrderCancelBatchSubscribeResultsItems,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCancelBatchOrderCancelBatchPublish>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCancelBatchOrderCancelBatchSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCancelOnDisconnectOrderCancelOnDisconnectPublish>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCancelOnDisconnectOrderCancelOnDisconnectSubscribe>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCancelOrderCancelPublish>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCancelOrderCancelSubscribe>(&config, true)
    ));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::paradex::WsOrderCreateBatchChannelsOrderCreateBatchPublishOrdersItems>( &config, true)));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::paradex::WsOrderCreateBatchChannelsOrderCreateBatchSubscribeResultsItems,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::paradex::WsOrderCreateBatchChannelsOrderCreateBatchSubscribeResultsItemsOrder,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCreateBatchOrderCreateBatchPublish>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCreateBatchOrderCreateBatchSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCreateChannelsOrderCreateSubscribeOrder>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCreateOrderCreatePublish>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderCreateOrderCreateSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderModifyChannelsOrderModifySubscribeOrder>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderModifyOrderModifyPublish>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrderModifyOrderModifySubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::paradex::WsOrdersMarketSymbolChannelsOrdersMarketSymbolSubscribeRequestInfo,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsOrdersMarketSymbolOrdersMarketSymbolSubscribe>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsPositionsPositionsSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsRfqRfqIdChannelsRfqRfqIdSubscribeDeribitCost>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsRfqRfqIdChannelsRfqRfqIdSubscribeQuote>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsRfqRfqIdChannelsRfqRfqIdSubscribeResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsRfqRfqIdChannelsRfqRfqIdSubscribeRfq>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsRfqRfqIdChannelsRfqRfqIdSubscribeTrade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsRfqRfqIdRfqRfqIdSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsSubscribeSubscribePublish>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsSubscribeSubscribeSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsTradebustsTradebustsSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsTradesMarketSymbolTradesMarketSymbolSubscribe>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsTransactionTransactionSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsTransfersTransfersSubscribe>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsUnsubscribeUnsubscribePublish>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::paradex::WsUnsubscribeUnsubscribeSubscribe>(&config, true)
    ));
    result.push_str("}\n");
    result.push_str("export namespace lighter {\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::Account>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::AccountAsset>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::AccountEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::AccountOrdersEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::Accounts>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::BookEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::BookLevel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::CancelOrder>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::CreateOrder>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::NextNonce>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::Order>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::OrderBook>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::OrderBookOrders>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::OrderBooks>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::Orders>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::Position>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::SendTxResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::SimpleOrder>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::StreamBook>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::Trade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::Trades>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::lighter::TradesEvent>(&config, true)
    ));
    result.push_str("}\n");
    result.push_str("export namespace metric {\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::planning::AdditionPlan>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::planning::AdditionRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::Addresses>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::BatchBidAskRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::BatchBidAskResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::BidAskResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::math::BinData>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::math::BinState>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::planning::BinTarget>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::BuilderOutcome>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::Chain>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::ChainsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::creation::CreatePoolParams>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::liquidity::CursorBounds>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::liquidity::CursorInput>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::Depth>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::DepthLevel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::creation::ExtensionOrders>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::workflows::ExternalSwapFallback>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::workflows::ExternalSwapParams>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::workflows::ExternalSwapRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::creation::FactoryPoolParameters>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::FactorySettingsResponse>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::workflows::FallbackSwapRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::workflows::LiquidityAdditionKind>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::workflows::LiquidityAdditionRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::planning::LiquidityBin>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::liquidity::LiquidityDeltas>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::planning::LiquiditySnapshot>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::MetadataRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::MetadataResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::oracle::OracleLane>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::PammQuoteRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::PammQuoteResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::PendingTransaction>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::creation::PoolCreationRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::PoolMetadata>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::PoolQuote>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::PoolQuoteError>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::planning::PositionShares>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::PreparedCall>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::creation::PreparedPoolCreation>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::PreparedTransaction>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::planning::RemovalSpec>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::RouteLeg>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::RouteOption>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::RoutePreparation>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::RouteRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::RouteResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::SignedTransaction>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::stop_loss::StopLossBinInput>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::stop_loss::StopLossBinStatus>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::stop_loss::StopLossConfig>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::stop_loss::StopLossEvaluation>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::stop_loss::StopLossEvaluationRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::liquidity::StopLossInput>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::stop_loss::StopLossStatus>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::stop_loss::StopLossStatusRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::math::SwapDetails>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::workflows::SwapPayment>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::TradeType>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::creation::UniformBinsRequest>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::metric::planning::WeightedRequest>(&config, false)
    ));
    result.push_str("}\n");
    result.push_str("export namespace phoenix {\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::AckBeforeTimestampBody>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::AckNotificationsBody>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::AcknowledgeNotificationsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::AcknowledgeNotificationsUpToTimestampParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ActivateInviteResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ActivateInviteWithReferralRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ActivateReferralParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ActivateReferralTxRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ActivateReferralTxResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ActivateReferralWithUserFundedTransactionParams>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::AggregatedTrades>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ApiAccountMeta>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ApiCandle>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ApiInstructionResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::AuthResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::AuthoritySetView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildAttachedConditionalOrderTransactionParams>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildCancelConditionalOrderTransactionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildCancelStopLossOrderTransactionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildIsolatedLimitOrderTransactionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildIsolatedLimitOrderTransactionWithConditionalsParams>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::phoenix::BuildIsolatedLimitOrderTransactionWithLiquidationEstimateParams,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildIsolatedMarketOrderTransactionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::phoenix::BuildIsolatedMarketOrderTransactionWithLiquidationEstimateParams,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildPositionConditionalOrderTransactionParams>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildRegisterIxsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildRegisterIxsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildRegisterTraderInstructionsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::BuildStopLossOrderTransactionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CancelConditionalOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CancelStopLossOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CapabilityAccessView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CheckWalletInviteStatusParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CheckWalletResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CollateralEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CollateralHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CommodityMarketCalendarResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CommodityMarketCalendarView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CommodityMarketDaySchedule>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CommodityMarketSessionRange>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CommodityMetadata>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ConditionalTriggerRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CooldownStatus>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CreateServiceLoginChallengeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CreateWalletTransactionChallengeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::CurrentOrderState>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ErrorResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeKeysView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeLeverageTier>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeMarketConfig>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeMarketSnapshot>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeRiskFactors>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeSnapshotView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeStateSnapshot>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeStatusView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeWsCommodityMetadata>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeWsFundingConfig>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeWsLeverageTier>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeWsMarkPriceParameters>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeWsMarketPriceBand>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ExchangeWsRiskFactors>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::FillRecord>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::FillsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::FundingHourlyEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::FundingHourlyHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::FundingOverviewPoint>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::FundingOverviewResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::FundingOverviewSeries>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::FundingRateHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::FundingRatePoint>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetCandlesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetCommodityMarketCalendarParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetExchangeKeysParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetExchangeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetExchangeSnapshotParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetExchangeStatusParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetFundingOverviewParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetFundingRateHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetJwksParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetMarkPriceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetMarketCalendarByIdParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetMarketCalendarParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetMarketFillsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetMarketParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetMarketStatsHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetNextCommodityMarketTransitionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetNextMarketCalendarTransitionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetNotificationsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetOrderHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetOrderHistoryV2Params>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetOrderbookParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetReferralActivationPermissionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTradeHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTraderCapabilitiesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTraderCollateralHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTraderFundingHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTraderMarketPnLParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTraderParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTraderPnLParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTraderPortfolioValuesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTraderStateParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetTraderTradeHistoryV2Params>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetUserCollateralHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetUserHourlyFundingParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetUserLiquidationHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetUserPnLParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetUserTradeHistoryV2Params>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetV1TradersByTraderPubkeyCollateralHistoryParams>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::GetWalletLoginNonceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::HashMapValueValue>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::IntendedOrder>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::JwksResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::LimitOrder>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ListMarketsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ListTraderNotificationsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::LogInWithPrivyTokenParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::LogInWithServiceSignatureParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::LogInWithSignedWalletTransactionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::LogInWithWalletSignatureParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::LogOutParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::LookupTable>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::MarkPriceResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::MarketCalendar>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::MarketCalendarRecord>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::MarketCalendarResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::MarketPriceBand>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::MarketPublicMetadata>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::MarketStatsHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::MarketStatsPoint>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::MarketStatsSnapshot>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::NativeNumber>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::NextCommodityMarketTransition>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::NextMarketCalendarTransition>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::OrderbookView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PaginatedResponseVecOrderHistoryItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PaginatedResponseVecOrderHistoryItemDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PaginatedResponseVecOrderHistoryV2Item>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PaginatedResponseVecOrderHistoryV2ItemDataItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PaginatedResponseVecTradeHistoryV2Item>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PaginatedResponseVecTradeHistoryV2ItemDataItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PaginatedResponseVecUserLiquidationHistoryPoint>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PaginatedResponseVecUserLiquidationHistoryPointDataItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PlaceAttachedConditionalOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PlaceIsolatedLimitOrderEnhancedResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PlaceIsolatedLimitOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PlaceIsolatedLimitOrderWithConditionalsRequest>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PlaceIsolatedMarketOrderEnhancedResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PlaceIsolatedMarketOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PlacePositionConditionalOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PlacedOrder>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PnLPoint>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PortfolioValuePoint>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PreparedTransaction>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::Price>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PrivyLoginRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::PrivyWalletProof>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ReferralActivationPermissionResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::RefreshRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::RefreshSessionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::SendRegisterIxsRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::SendRegisterIxsResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ServiceChallengeRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ServiceChallengeResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ServiceLoginRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::SignedTransaction>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::Spline>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::SubmitRegisterTraderTransactionParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TickRegion>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TpSlOrderConfig>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderCapabilitiesMetadata>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderCapabilitiesView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderCapabilityDescriptor>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderFundingHistoryEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderFundingHistoryResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderMarketPnLPoint>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderMarketPnLSeries>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderPositionView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateCapabilities>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateConditionalStopLossTrigger>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateConditionalTakeProfitTrigger>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateLimitOrderEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateMarketLimitOrderEvent>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStatePositionRow>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateSnapshot>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateSnapshotResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateSplineRow>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateStopLossTrigger>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateSubaccountSnapshot>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateTakeProfitTrigger>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateTickRegion>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateTrigger>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderStateTriggerRow>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::TraderView>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ValidateInviteParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ValidateInviteRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::ValidateInviteResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::WalletLoginRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::WalletNonceResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::WalletTransactionChallengeRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::WalletTransactionChallengeResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::phoenix::WalletTransactionLoginRequest>(&config, true)
    ));
    result.push_str("}\n");
    result.push_str("export namespace extended {\n");
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AccountInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AccountLeverage>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AccountLeverageModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AccountModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AccountStreamDataModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AccountTradeModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AccountUpdateRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ApiKey>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ApiKeyRequestModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ApiKeyResponseModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ApiResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::Asset>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AssetIndexPrice>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AssetModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AssetOperation>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AssetOperationModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AssetOperationsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::AssetOperationsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::Balance>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::BalanceModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::BridgeConfig>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::BridgeConfigChainsItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::BridgeQuote>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::BridgesConfigModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::BuilderDashboard>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::BuilderTradeModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CancelOrderByExternalIdParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CancelOrderByExternalIdResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CancelOrderParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CandleModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ChainConfigModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::Claim>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ClaimResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ClaimResponseModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ClaimTestingFundsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ClaimTestingFundsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ClientInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ClientModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ClientRankings>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ClientRankingsRankingsItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ClientTradingFeeStats>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CollateralSlowWithdrawal>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CommitBridgeQuoteParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CommitBridgeQuoteResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateApiKeyParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateApiKeyRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateApiKeyResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateApiKeyResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateDepositParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateDepositRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateOrderConditionalTriggerModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateOrderRfqModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateOrderTpslTriggerModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateRFQOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateReferralLinkCodeRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateReferralLinkParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateScaledOrderParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::CreateScaledOrderRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::DeleteApiKeyParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::DeleteUserOrderOrderIdParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::DeleteUserOrderOrderIdResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::DeleverageLevels>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::DepositRequestModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::DepositStatusUpdateModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::DepositToVaultParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::DepositToVaultResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::EmptyModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ErrorResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ExchangeStatus>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ExportCsvZipParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::FundingFee>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::FundingRate>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::FundingRateConfigRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::FundingRateModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GeckoContract>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GeckoOrderBook>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountEquityHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountEquityHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountEquityHistoryResultDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingChartParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingChartResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingChartResultDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingHistoryResultDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingHistoryResultPagination>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingStatsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingStatsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingStatsResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountFundingStatsResultDataFundingPerMarketItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountHealthParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountHealthResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountHealthResultDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountMaxDrawdownHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountMaxDrawdownHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountMaxDrawdownHistoryResultDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountMaxDrawdownHistoryResultDataItemDataItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPerformanceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPerformanceResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPerformanceResultDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPnLHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPnLHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPnLHistoryResultDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPnLPercentageHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPnLPercentageHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPnLPercentageHistoryResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPnLPercentageHistoryResultDataPnlItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPnLPercentageHistoryResultDataPriceChanges>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::extended::GetAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPortfolioSummaryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPortfolioSummaryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPortfolioSummaryResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountPortfolioSummaryResultDataAccountValue>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountVaultEquityHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountVaultEquityHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountVaultEquityHistoryResultDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAccountsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAffiliateParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAffiliateRewardsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetApiKeysParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAssetOperationsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAssetPriceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAssetsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetAssetsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetBalanceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetBalanceResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetBridgeConfigParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetBridgeConfigResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetBridgeQuoteParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetBridgeQuoteResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetBuilderDashboardParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetBuilderTradesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetBuilderTradesResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetCandlesHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetCandlesHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetClaimParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetClaimsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetClientParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetClientResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetCumulativeAccountPnLHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetCumulativeAccountPnLHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetCumulativeAccountPnLHistoryResultDataItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetCumulativeAccountPnLPercentageHistoryParams>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetCumulativeAccountPnLPercentageHistoryResult>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetCumulativeAccountPnLPercentageHistoryResultData>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetCumulativeAccountPnLPercentageHistoryResultDataPnlItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::extended::GetCumulativeAccountPnLPercentageHistoryResultDataPriceChanges,
        >(&config, true)
    ));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::extended::GetCumulativeAccountPnLPercentageHistoryResultDataPriceChangesBTCUSDItem>( &config, true)));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetEarnedRewardsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetExchangeStatusParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetFeesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetFeesResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetFundingRatesHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetFundingRatesHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetIndexPriceCandlesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInfoAssetsAssetNamePriceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInfoAssetsAssetNamePriceResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInfoCandlesMarketCandleTypeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInfoCandlesMarketCandleTypeResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInfoCandlesMarketCandleTypeResultDataItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestDailyMetricsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestDailyMetricsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestDailyMetricsResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestDailyMetricsResultData1000Item>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestKeyMetricsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestKeyMetricsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestKeyMetricsResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestPaymentChartParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestPaymentChartResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestPaymentChartResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestPaymentChartResultDataPointsItem>(&config, true)
    ));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::extended::GetInterestPaymentChartResultDataPointsItemBorrowingByCollateralAssetItem>( &config, true)));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestPaymentsHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestPaymentsHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestPaymentsHistoryResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestPaymentsHistoryResultDataItemsItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestRateCurvesHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestRateCurvesHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestRateCurvesHistoryResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestRateCurvesHistoryResultDataItemsItem>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::extended::GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItem,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<
            aux_rs::extended::GetInterestRateCurvesHistoryResultDataItemsItemAssetCurvesItemCurve,
        >(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetInterestRateCurvesHistoryResultDataItemsItemGlobalCurve>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetLatestInterestRateCurveParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetLatestInterestRateCurveResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetLatestInterestRateCurveResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetLatestInterestRateCurveResultDataItem>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetLatestInterestRateCurveResultDataItemAssetCurvesItem>(
            &config, true
        )
    ));
    result.push_str(&format!("export {}\n",declaration::<aux_rs::extended::GetLatestInterestRateCurveResultDataItemAssetCurvesItemCurve>( &config, true)));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetLatestInterestRateCurveResultDataItemGlobalCurve>(
            &config, true
        )
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetLeverageParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetLeverageResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetMarkPriceCandlesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetMarketStatisticsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetMarketStatisticsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetMarketTradesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetMarketsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetMarketsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetMarketsStatsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetNextClaimTimeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOpenInterestsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOpenOrdersParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOpenOrdersResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOrderByExternalIdParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOrderByIdParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOrderByIdResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOrderParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOrderbookSnapshotParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOrderbookSnapshotResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOrdersHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetOrdersHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetPartnerRewardsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetPositionsHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetPositionsHistoryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetPositionsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetPositionsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetPublicLeaderboardParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetPublicReferralLinkParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetRebateStatsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetRebatesCsvParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetReferralDashboardParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetReferralLinksParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetReferralRebatesByLinkCsvParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetReferralRebatesByPeriodCsvParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetReferralRebatesBySubaffiliateCsvParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetReferralRebatesByUserCsvParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetReferralStatusParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetRewardLeaderboardStatsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetSettingsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetSettingsResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetSpotBalancesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetSpotBalancesResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetStarknetDomainParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetTradesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetTradesResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetTradingCandlesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetTradingFeeStatsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetUserFundingHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetUserLeaderboardParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetUserOrdersExternalExternalIdParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetUserOrdersExternalExternalIdResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultFundingHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultOrdersParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultPerformanceParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultPerformanceResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultPerformanceResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultPerformanceResultDataAprBreakdown>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultPositionHistoryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultPositionsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultSummaryParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultSummaryResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultSummaryResultData>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultSummaryResultDataAprBreakdown>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetVaultTradesParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetWalletValidationParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetWithdrawalsParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::GetWrapperParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::InternalAmount>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::L1Amount>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::L2ConfigModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::LimitOrderSettlementModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MarkWalletCheckedParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MarketInfo>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MarketL2Config>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MarketModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MarketStats>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MarketStatsModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MarketTradingConfig>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MassCancelParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MassCancelRequestModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::MassCancelResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::NativeNumber>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::NewOrderModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::NewOrderResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OnChainPerpetualTransferModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OnChainTransfer>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OnboardClientParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OnboardClientResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OnboardSubaccountParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OnboardSubaccountResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OnboardedClientModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OnchainWithdrawalParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OpenOrderModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OpenOrderTpslTriggerModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::Order>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OrderBook>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OrderCancelRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OrderConditionalTrigger>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OrderContext>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OrderTpslTrigger>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OrderbookQuantityModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::OrderbookUpdateModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PaginatedResponse>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PaginationModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PerpetualTransferModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PlaceOrderParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PlaceOrderResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PlaceRfqOrderParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PlaceRfqOrderResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PlacedOrderModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::Position>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PositionHistory>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PositionHistoryModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PositionModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PostUserTransferOnchainParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PostUserTransferOnchainResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PreparedOrder>(&config, false)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PriceCandle>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PublicTrade>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::PublicTradeModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::QuoteModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::RealisedPnlBreakdownModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ResponseErrorModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::RiskFactorConfigModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::RiskFactorSegmentModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::SetDeadmanSwitchParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::Settings>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::SettingsModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::SettlementAsset>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::SettlementSignatureModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::SpotBalance>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::SpotBalanceModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StarkAmount>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StarkDebuggingOrderAmountsModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StarkSettlementModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StarkTransferSettlementModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StarkWithdrawalSettlementModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StarknetDomain>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcAccountBalanceModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcAccountDepositUpdateModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcAccountOrdersModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcAccountPositionsModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcAccountSpotBalancesModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcAccountTradesModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcAccountWithdrawalUpdateModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcOrderbookUpdateModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcPriceModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::StreamRpcResponseModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::TimestampModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::TradeCandle>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::TradingConfigModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::TradingFee>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::TradingFeeModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::TransferParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::TransferPatchParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::TransferRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::TransferResponseModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::Twap>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::UpdateAccountInfoParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::UpdateFundingRateConfigParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::UpdateLeverageParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::UpdateLeverageResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::UpdateReferralLinkCodeRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::UpdateReferralLinkParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::UseReferralCodeParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::UseReferralCodeRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::ValidateWalletParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WalletValidation>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WalletValidationRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WithdrawFromVaultParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WithdrawFromVaultResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WithdrawParams>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WithdrawRequestModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WithdrawResult>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WithdrawalRequest>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WithdrawalRequestModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WithdrawalStatusUpdateModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WrappedApiResponseModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::WrappedStreamResponseModel>(&config, true)
    ));
    result.push_str(&format!(
        "export {}\n",
        declaration::<aux_rs::extended::X10BaseModel>(&config, true)
    ));
    result.push_str("}\n");
    let output = result.replace("JsonValue", "LosslessValue");
    for line in output.lines() {
        println!("{}", line.trim_end());
    }
}
