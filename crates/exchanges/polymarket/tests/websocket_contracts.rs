use aux_rs_polymarket::ws_types::*;
use serde_json::Value;
#[test]
fn market_market_subscription_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketSubscriptionRequest =
        serde_json::from_value(f["market.market.subscriptionRequest"].clone()).unwrap();
}
#[test]
fn market_market_subscription_request_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketSubscriptionRequestUpdate =
        serde_json::from_value(f["market.market.subscriptionRequestUpdate"].clone()).unwrap();
}
#[test]
fn market_market_ping() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketPing =
        serde_json::from_value(f["market.market.ping"].clone()).unwrap();
}
#[test]
fn market_market_pong() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketPong =
        serde_json::from_value(f["market.market.pong"].clone()).unwrap();
}
#[test]
fn market_market_book() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketBook =
        serde_json::from_value(f["market.market.book"].clone()).unwrap();
}
#[test]
fn market_market_price_change() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketPriceChange =
        serde_json::from_value(f["market.market.priceChange"].clone()).unwrap();
}
#[test]
fn market_market_last_trade_price() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketLastTradePrice =
        serde_json::from_value(f["market.market.lastTradePrice"].clone()).unwrap();
}
#[test]
fn market_market_tick_size_change() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketTickSizeChange =
        serde_json::from_value(f["market.market.tickSizeChange"].clone()).unwrap();
}
#[test]
fn market_market_best_bid_ask() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketBestBidAsk =
        serde_json::from_value(f["market.market.bestBidAsk"].clone()).unwrap();
}
#[test]
fn market_market_new_market() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketNewMarket =
        serde_json::from_value(f["market.market.newMarket"].clone()).unwrap();
}
#[test]
fn market_market_market_resolved() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: market::MarketMarketMarketResolved =
        serde_json::from_value(f["market.market.marketResolved"].clone()).unwrap();
}
#[test]
fn user_user_user_subscription_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: user::UserUserUserSubscriptionRequest =
        serde_json::from_value(f["user.user.userSubscriptionRequest"].clone()).unwrap();
}
#[test]
fn user_user_user_subscription_request_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: user::UserUserUserSubscriptionRequestUpdate =
        serde_json::from_value(f["user.user.userSubscriptionRequestUpdate"].clone()).unwrap();
}
#[test]
fn user_user_ping() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: user::UserUserPing = serde_json::from_value(f["user.user.ping"].clone()).unwrap();
}
#[test]
fn user_user_pong() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: user::UserUserPong = serde_json::from_value(f["user.user.pong"].clone()).unwrap();
}
#[test]
fn user_user_order() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: user::UserUserOrder = serde_json::from_value(f["user.user.order"].clone()).unwrap();
}
#[test]
fn user_user_trade() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: user::UserUserTrade = serde_json::from_value(f["user.user.trade"].clone()).unwrap();
}
#[test]
fn sports_sports_ping() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: sports::SportsSportsPing =
        serde_json::from_value(f["sports.sports.ping"].clone()).unwrap();
}
#[test]
fn sports_sports_pong() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: sports::SportsSportsPong =
        serde_json::from_value(f["sports.sports.pong"].clone()).unwrap();
}
#[test]
fn sports_sports_sports_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: sports::SportsSportsSportsUpdate =
        serde_json::from_value(f["sports.sports.sportsUpdate"].clone()).unwrap();
}
#[test]
fn rfq_quoter_auth() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterAuth = serde_json::from_value(f["rfq.quoter.auth"].clone()).unwrap();
}
#[test]
fn rfq_quoter_auth_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterAuthResponse =
        serde_json::from_value(f["rfq.quoter.authResponse"].clone()).unwrap();
}
#[test]
fn rfq_quoter_rfq_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterRfqRequest =
        serde_json::from_value(f["rfq.quoter.rfqRequest"].clone()).unwrap();
}
#[test]
fn rfq_quoter_rfq_quote() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterRfqQuote =
        serde_json::from_value(f["rfq.quoter.rfqQuote"].clone()).unwrap();
}
#[test]
fn rfq_quoter_ack_rfq_quote() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterAckRfqQuote =
        serde_json::from_value(f["rfq.quoter.ackRfqQuote"].clone()).unwrap();
}
#[test]
fn rfq_quoter_rfq_quote_cancel() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterRfqQuoteCancel =
        serde_json::from_value(f["rfq.quoter.rfqQuoteCancel"].clone()).unwrap();
}
#[test]
fn rfq_quoter_ack_rfq_quote_cancel() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterAckRfqQuoteCancel =
        serde_json::from_value(f["rfq.quoter.ackRfqQuoteCancel"].clone()).unwrap();
}
#[test]
fn rfq_quoter_rfq_confirmation_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterRfqConfirmationRequest =
        serde_json::from_value(f["rfq.quoter.rfqConfirmationRequest"].clone()).unwrap();
}
#[test]
fn rfq_quoter_rfq_confirmation_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterRfqConfirmationResponse =
        serde_json::from_value(f["rfq.quoter.rfqConfirmationResponse"].clone()).unwrap();
}
#[test]
fn rfq_quoter_ack_rfq_confirmation_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterAckRfqConfirmationResponse =
        serde_json::from_value(f["rfq.quoter.ackRfqConfirmationResponse"].clone()).unwrap();
}
#[test]
fn rfq_quoter_rfq_execution_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterRfqExecutionUpdate =
        serde_json::from_value(f["rfq.quoter.rfqExecutionUpdate"].clone()).unwrap();
}
#[test]
fn rfq_quoter_rfq_trade() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterRfqTrade =
        serde_json::from_value(f["rfq.quoter.rfqTrade"].clone()).unwrap();
}
#[test]
fn rfq_quoter_rfq_error() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rfq::RfqQuoterRfqError =
        serde_json::from_value(f["rfq.quoter.rfqError"].clone()).unwrap();
}
#[test]
fn perps_ping_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsPingRequest =
        serde_json::from_value(f["perps.ping.Request"].clone()).unwrap();
}
#[test]
fn perps_ping_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsPingResponse =
        serde_json::from_value(f["perps.ping.Response"].clone()).unwrap();
}
#[test]
fn perps_place_orders_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsPlaceOrdersRequest =
        serde_json::from_value(f["perps.placeOrders.Request"].clone()).unwrap();
}
#[test]
fn perps_place_orders_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsPlaceOrdersResponse =
        serde_json::from_value(f["perps.placeOrders.Response"].clone()).unwrap();
}
#[test]
fn perps_cancel_orders_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsCancelOrdersRequest =
        serde_json::from_value(f["perps.cancelOrders.Request"].clone()).unwrap();
}
#[test]
fn perps_cancel_orders_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsCancelOrdersResponse =
        serde_json::from_value(f["perps.cancelOrders.Response"].clone()).unwrap();
}
#[test]
fn perps_cancel_orders_coid_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsCancelOrdersCOIDRequest =
        serde_json::from_value(f["perps.cancelOrdersCOID.Request"].clone()).unwrap();
}
#[test]
fn perps_cancel_orders_coid_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsCancelOrdersCOIDResponse =
        serde_json::from_value(f["perps.cancelOrdersCOID.Response"].clone()).unwrap();
}
#[test]
fn perps_cancel_all_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsCancelAllRequest =
        serde_json::from_value(f["perps.cancelAll.Request"].clone()).unwrap();
}
#[test]
fn perps_cancel_all_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsCancelAllResponse =
        serde_json::from_value(f["perps.cancelAll.Response"].clone()).unwrap();
}
#[test]
fn perps_auto_cancel_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsAutoCancelRequest =
        serde_json::from_value(f["perps.autoCancel.Request"].clone()).unwrap();
}
#[test]
fn perps_auto_cancel_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsAutoCancelResponse =
        serde_json::from_value(f["perps.autoCancel.Response"].clone()).unwrap();
}
#[test]
fn perps_update_leverage_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsUpdateLeverageRequest =
        serde_json::from_value(f["perps.updateLeverage.Request"].clone()).unwrap();
}
#[test]
fn perps_update_leverage_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsUpdateLeverageResponse =
        serde_json::from_value(f["perps.updateLeverage.Response"].clone()).unwrap();
}
#[test]
fn perps_update_leverages_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsUpdateLeveragesRequest =
        serde_json::from_value(f["perps.updateLeverages.Request"].clone()).unwrap();
}
#[test]
fn perps_update_leverages_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsUpdateLeveragesResponse =
        serde_json::from_value(f["perps.updateLeverages.Response"].clone()).unwrap();
}
#[test]
fn perps_update_margin_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsUpdateMarginRequest =
        serde_json::from_value(f["perps.updateMargin.Request"].clone()).unwrap();
}
#[test]
fn perps_update_margin_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsUpdateMarginResponse =
        serde_json::from_value(f["perps.updateMargin.Response"].clone()).unwrap();
}
#[test]
fn perps_auth_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsAuthRequest =
        serde_json::from_value(f["perps.auth.Request"].clone()).unwrap();
}
#[test]
fn perps_auth_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsAuthResponse =
        serde_json::from_value(f["perps.auth.Response"].clone()).unwrap();
}
#[test]
fn perps_trades_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTradesSubscribeRequest =
        serde_json::from_value(f["perps.trades.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_trades_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTradesSubscribeResponse =
        serde_json::from_value(f["perps.trades.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_trades_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTradesUnsubscribeRequest =
        serde_json::from_value(f["perps.trades.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_trades_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTradesUnsubscribeResponse =
        serde_json::from_value(f["perps.trades.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_trades_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTradesUpdate =
        serde_json::from_value(f["perps.trades.Update"].clone()).unwrap();
}
#[test]
fn perps_bbo_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBboSubscribeRequest =
        serde_json::from_value(f["perps.bbo.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_bbo_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBboSubscribeResponse =
        serde_json::from_value(f["perps.bbo.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_bbo_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBboUnsubscribeRequest =
        serde_json::from_value(f["perps.bbo.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_bbo_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBboUnsubscribeResponse =
        serde_json::from_value(f["perps.bbo.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_bbo_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBboUpdate = serde_json::from_value(f["perps.bbo.Update"].clone()).unwrap();
}
#[test]
fn perps_book_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBookSubscribeRequest =
        serde_json::from_value(f["perps.book.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_book_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBookSubscribeResponse =
        serde_json::from_value(f["perps.book.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_book_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBookUnsubscribeRequest =
        serde_json::from_value(f["perps.book.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_book_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBookUnsubscribeResponse =
        serde_json::from_value(f["perps.book.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_book_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBookUpdate = serde_json::from_value(f["perps.book.Update"].clone()).unwrap();
}
#[test]
fn perps_klines_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsKlinesSubscribeRequest =
        serde_json::from_value(f["perps.klines.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_klines_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsKlinesSubscribeResponse =
        serde_json::from_value(f["perps.klines.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_klines_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsKlinesUnsubscribeRequest =
        serde_json::from_value(f["perps.klines.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_klines_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsKlinesUnsubscribeResponse =
        serde_json::from_value(f["perps.klines.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_klines_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsKlinesUpdate =
        serde_json::from_value(f["perps.klines.Update"].clone()).unwrap();
}
#[test]
fn perps_tickers_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTickersSubscribeRequest =
        serde_json::from_value(f["perps.tickers.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_tickers_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTickersSubscribeResponse =
        serde_json::from_value(f["perps.tickers.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_tickers_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTickersUnsubscribeRequest =
        serde_json::from_value(f["perps.tickers.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_tickers_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTickersUnsubscribeResponse =
        serde_json::from_value(f["perps.tickers.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_tickers_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTickersUpdate =
        serde_json::from_value(f["perps.tickers.Update"].clone()).unwrap();
}
#[test]
fn perps_statistics_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsStatisticsSubscribeRequest =
        serde_json::from_value(f["perps.statistics.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_statistics_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsStatisticsSubscribeResponse =
        serde_json::from_value(f["perps.statistics.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_statistics_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsStatisticsUnsubscribeRequest =
        serde_json::from_value(f["perps.statistics.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_statistics_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsStatisticsUnsubscribeResponse =
        serde_json::from_value(f["perps.statistics.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_statistics_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsStatisticsUpdate =
        serde_json::from_value(f["perps.statistics.Update"].clone()).unwrap();
}
#[test]
fn perps_fills_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFillsSubscribeRequest =
        serde_json::from_value(f["perps.fills.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_fills_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFillsSubscribeResponse =
        serde_json::from_value(f["perps.fills.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_fills_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFillsUnsubscribeRequest =
        serde_json::from_value(f["perps.fills.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_fills_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFillsUnsubscribeResponse =
        serde_json::from_value(f["perps.fills.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_fills_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFillsUpdate =
        serde_json::from_value(f["perps.fills.Update"].clone()).unwrap();
}
#[test]
fn perps_backstop_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBackstopSubscribeRequest =
        serde_json::from_value(f["perps.backstop.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_backstop_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBackstopSubscribeResponse =
        serde_json::from_value(f["perps.backstop.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_backstop_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBackstopUnsubscribeRequest =
        serde_json::from_value(f["perps.backstop.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_backstop_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBackstopUnsubscribeResponse =
        serde_json::from_value(f["perps.backstop.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_backstop_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBackstopUpdate =
        serde_json::from_value(f["perps.backstop.Update"].clone()).unwrap();
}
#[test]
fn perps_orders_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsOrdersSubscribeRequest =
        serde_json::from_value(f["perps.orders.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_orders_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsOrdersSubscribeResponse =
        serde_json::from_value(f["perps.orders.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_orders_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsOrdersUnsubscribeRequest =
        serde_json::from_value(f["perps.orders.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_orders_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsOrdersUnsubscribeResponse =
        serde_json::from_value(f["perps.orders.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_orders_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsOrdersUpdate =
        serde_json::from_value(f["perps.orders.Update"].clone()).unwrap();
}
#[test]
fn perps_funding_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFundingSubscribeRequest =
        serde_json::from_value(f["perps.funding.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_funding_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFundingSubscribeResponse =
        serde_json::from_value(f["perps.funding.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_funding_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFundingUnsubscribeRequest =
        serde_json::from_value(f["perps.funding.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_funding_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFundingUnsubscribeResponse =
        serde_json::from_value(f["perps.funding.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_funding_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsFundingUpdate =
        serde_json::from_value(f["perps.funding.Update"].clone()).unwrap();
}
#[test]
fn perps_balances_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBalancesSubscribeRequest =
        serde_json::from_value(f["perps.balances.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_balances_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBalancesSubscribeResponse =
        serde_json::from_value(f["perps.balances.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_balances_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBalancesUnsubscribeRequest =
        serde_json::from_value(f["perps.balances.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_balances_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBalancesUnsubscribeResponse =
        serde_json::from_value(f["perps.balances.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_balances_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBalancesUpdate =
        serde_json::from_value(f["perps.balances.Update"].clone()).unwrap();
}
#[test]
fn perps_portfolio_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsPortfolioSubscribeRequest =
        serde_json::from_value(f["perps.portfolio.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_portfolio_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsPortfolioSubscribeResponse =
        serde_json::from_value(f["perps.portfolio.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_portfolio_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsPortfolioUnsubscribeRequest =
        serde_json::from_value(f["perps.portfolio.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_portfolio_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsPortfolioUnsubscribeResponse =
        serde_json::from_value(f["perps.portfolio.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_portfolio_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsPortfolioUpdate =
        serde_json::from_value(f["perps.portfolio.Update"].clone()).unwrap();
}
#[test]
fn perps_deposits_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsDepositsSubscribeRequest =
        serde_json::from_value(f["perps.deposits.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_deposits_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsDepositsSubscribeResponse =
        serde_json::from_value(f["perps.deposits.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_deposits_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsDepositsUnsubscribeRequest =
        serde_json::from_value(f["perps.deposits.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_deposits_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsDepositsUnsubscribeResponse =
        serde_json::from_value(f["perps.deposits.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_deposits_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsDepositsUpdate =
        serde_json::from_value(f["perps.deposits.Update"].clone()).unwrap();
}
#[test]
fn perps_withdrawals_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsWithdrawalsSubscribeRequest =
        serde_json::from_value(f["perps.withdrawals.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_withdrawals_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsWithdrawalsSubscribeResponse =
        serde_json::from_value(f["perps.withdrawals.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_withdrawals_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsWithdrawalsUnsubscribeRequest =
        serde_json::from_value(f["perps.withdrawals.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_withdrawals_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsWithdrawalsUnsubscribeResponse =
        serde_json::from_value(f["perps.withdrawals.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_withdrawals_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsWithdrawalsUpdate =
        serde_json::from_value(f["perps.withdrawals.Update"].clone()).unwrap();
}
#[test]
fn perps_notifications_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsNotificationsSubscribeRequest =
        serde_json::from_value(f["perps.notifications.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_notifications_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsNotificationsSubscribeResponse =
        serde_json::from_value(f["perps.notifications.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_notifications_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsNotificationsUnsubscribeRequest =
        serde_json::from_value(f["perps.notifications.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_notifications_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsNotificationsUnsubscribeResponse =
        serde_json::from_value(f["perps.notifications.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_notifications_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsNotificationsUpdate =
        serde_json::from_value(f["perps.notifications.Update"].clone()).unwrap();
}
#[test]
fn perps_modify_orders_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsModifyOrdersRequest =
        serde_json::from_value(f["perps.modifyOrders.Request"].clone()).unwrap();
}
#[test]
fn perps_modify_orders_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsModifyOrdersResponse =
        serde_json::from_value(f["perps.modifyOrders.Response"].clone()).unwrap();
}
#[test]
fn perps_modify_orders_coid_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsModifyOrdersCOIDRequest =
        serde_json::from_value(f["perps.modifyOrdersCOID.Request"].clone()).unwrap();
}
#[test]
fn perps_modify_orders_coid_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsModifyOrdersCOIDResponse =
        serde_json::from_value(f["perps.modifyOrdersCOID.Response"].clone()).unwrap();
}
#[test]
fn perps_builder_fills_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBuilderFillsSubscribeRequest =
        serde_json::from_value(f["perps.builderFills.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_builder_fills_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBuilderFillsSubscribeResponse =
        serde_json::from_value(f["perps.builderFills.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_builder_fills_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBuilderFillsUnsubscribeRequest =
        serde_json::from_value(f["perps.builderFills.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_builder_fills_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBuilderFillsUnsubscribeResponse =
        serde_json::from_value(f["perps.builderFills.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_builder_fills_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsBuilderFillsUpdate =
        serde_json::from_value(f["perps.builderFills.Update"].clone()).unwrap();
}
#[test]
fn perps_tpsl_subscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTpslSubscribeRequest =
        serde_json::from_value(f["perps.tpsl.SubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_tpsl_subscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTpslSubscribeResponse =
        serde_json::from_value(f["perps.tpsl.SubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_tpsl_unsubscribe_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTpslUnsubscribeRequest =
        serde_json::from_value(f["perps.tpsl.UnsubscribeRequest"].clone()).unwrap();
}
#[test]
fn perps_tpsl_unsubscribe_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTpslUnsubscribeResponse =
        serde_json::from_value(f["perps.tpsl.UnsubscribeResponse"].clone()).unwrap();
}
#[test]
fn perps_tpsl_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: perps::PerpsTpslUpdate = serde_json::from_value(f["perps.tpsl.Update"].clone()).unwrap();
}
#[test]
fn polybolt_ws_subscribe() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsSubscribe =
        serde_json::from_value(f["polybolt.ws.subscribe"].clone()).unwrap();
}
#[test]
fn polybolt_ws_unsubscribe() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsUnsubscribe =
        serde_json::from_value(f["polybolt.ws.unsubscribe"].clone()).unwrap();
}
#[test]
fn polybolt_ws_auth() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsAuth =
        serde_json::from_value(f["polybolt.ws.auth"].clone()).unwrap();
}
#[test]
fn polybolt_ws_ping() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsPing =
        serde_json::from_value(f["polybolt.ws.ping"].clone()).unwrap();
}
#[test]
fn polybolt_ws_subscribed() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsSubscribed =
        serde_json::from_value(f["polybolt.ws.subscribed"].clone()).unwrap();
}
#[test]
fn polybolt_ws_unsubscribed() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsUnsubscribed =
        serde_json::from_value(f["polybolt.ws.unsubscribed"].clone()).unwrap();
}
#[test]
fn polybolt_ws_authed() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsAuthed =
        serde_json::from_value(f["polybolt.ws.authed"].clone()).unwrap();
}
#[test]
fn polybolt_ws_pong() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsPong =
        serde_json::from_value(f["polybolt.ws.pong"].clone()).unwrap();
}
#[test]
fn polybolt_ws_error() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsError =
        serde_json::from_value(f["polybolt.ws.error"].clone()).unwrap();
}
#[test]
fn polybolt_ws_price_polymarket() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsPricePolymarket =
        serde_json::from_value(f["polybolt.ws.pricePolymarket"].clone()).unwrap();
}
#[test]
fn polybolt_ws_price_crypto() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsPriceCrypto =
        serde_json::from_value(f["polybolt.ws.priceCrypto"].clone()).unwrap();
}
#[test]
fn polybolt_ws_price_equity() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsPriceEquity =
        serde_json::from_value(f["polybolt.ws.priceEquity"].clone()).unwrap();
}
#[test]
fn polybolt_ws_price_crypto_twap() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsPriceCryptoTwap =
        serde_json::from_value(f["polybolt.ws.priceCryptoTwap"].clone()).unwrap();
}
#[test]
fn polybolt_ws_price_equity_twap() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsPriceEquityTwap =
        serde_json::from_value(f["polybolt.ws.priceEquityTwap"].clone()).unwrap();
}
#[test]
fn polybolt_ws_esports_series() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsEsportsSeries =
        serde_json::from_value(f["polybolt.ws.esportsSeries"].clone()).unwrap();
}
#[test]
fn polybolt_ws_challenge() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsChallenge =
        serde_json::from_value(f["polybolt.ws.challenge"].clone()).unwrap();
}
#[test]
fn polybolt_ws_challenge_request() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: polybolt::PolyboltWsChallengeRequest =
        serde_json::from_value(f["polybolt.ws.challengeRequest"].clone()).unwrap();
}
#[test]
fn rtds_crypto_prices_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsCryptoPricesUpdate =
        serde_json::from_value(f["rtds.crypto_prices.update"].clone()).unwrap();
}
#[test]
fn rtds_crypto_prices_chainlink_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsCryptoPricesChainlinkUpdate =
        serde_json::from_value(f["rtds.crypto_prices_chainlink.update"].clone()).unwrap();
}
#[test]
fn rtds_crypto_prices_twap_thirty_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsCryptoPricesTwapThirtyUpdate =
        serde_json::from_value(f["rtds.crypto_prices_twap_thirty.update"].clone()).unwrap();
}
#[test]
fn rtds_crypto_prices_twap_sixty_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsCryptoPricesTwapSixtyUpdate =
        serde_json::from_value(f["rtds.crypto_prices_twap_sixty.update"].clone()).unwrap();
}
#[test]
fn rtds_equity_prices_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsEquityPricesUpdate =
        serde_json::from_value(f["rtds.equity_prices.update"].clone()).unwrap();
}
#[test]
fn rtds_equity_prices_subscribe() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsEquityPricesSubscribe =
        serde_json::from_value(f["rtds.equity_prices.subscribe"].clone()).unwrap();
}
#[test]
fn rtds_comments_comment_created() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsCommentsCommentCreated =
        serde_json::from_value(f["rtds.comments.comment_created"].clone()).unwrap();
}
#[test]
fn rtds_comments_comment_removed() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsCommentsCommentRemoved =
        serde_json::from_value(f["rtds.comments.comment_removed"].clone()).unwrap();
}
#[test]
fn rtds_comments_reaction_created() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsCommentsReactionCreated =
        serde_json::from_value(f["rtds.comments.reaction_created"].clone()).unwrap();
}
#[test]
fn rtds_comments_reaction_removed() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: rtds::RtdsCommentsReactionRemoved =
        serde_json::from_value(f["rtds.comments.reaction_removed"].clone()).unwrap();
}
