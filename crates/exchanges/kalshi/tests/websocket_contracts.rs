use aux_rs_kalshi::ws_types::*;
use serde_json::Value;
#[test]
fn predictions_root_subscribe_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootSubscribeCommand =
        serde_json::from_value(f["predictions.root.subscribeCommand"].clone()).unwrap();
}
#[test]
fn predictions_root_unsubscribe_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootUnsubscribeCommand =
        serde_json::from_value(f["predictions.root.unsubscribeCommand"].clone()).unwrap();
}
#[test]
fn predictions_root_update_subscription_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootUpdateSubscriptionCommand =
        serde_json::from_value(f["predictions.root.updateSubscriptionCommand"].clone()).unwrap();
}
#[test]
fn predictions_root_update_subscription_delete_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootUpdateSubscriptionDeleteCommand =
        serde_json::from_value(f["predictions.root.updateSubscriptionDeleteCommand"].clone())
            .unwrap();
}
#[test]
fn predictions_root_update_subscription_single_sid_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootUpdateSubscriptionSingleSidCommand =
        serde_json::from_value(f["predictions.root.updateSubscriptionSingleSidCommand"].clone())
            .unwrap();
}
#[test]
fn predictions_root_cfbenchmarks_update_subscription_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootCfbenchmarksUpdateSubscriptionCommand =
        serde_json::from_value(f["predictions.root.cfbenchmarksUpdateSubscriptionCommand"].clone())
            .unwrap();
}
#[test]
fn predictions_root_pyth_update_subscription_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootPythUpdateSubscriptionCommand =
        serde_json::from_value(f["predictions.root.pythUpdateSubscriptionCommand"].clone())
            .unwrap();
}
#[test]
fn predictions_root_list_subscriptions_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootListSubscriptionsCommand =
        serde_json::from_value(f["predictions.root.listSubscriptionsCommand"].clone()).unwrap();
}
#[test]
fn predictions_root_subscribed_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootSubscribedResponse =
        serde_json::from_value(f["predictions.root.subscribedResponse"].clone()).unwrap();
}
#[test]
fn predictions_root_list_subscriptions_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootListSubscriptionsResponse =
        serde_json::from_value(f["predictions.root.listSubscriptionsResponse"].clone()).unwrap();
}
#[test]
fn predictions_root_unsubscribed_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootUnsubscribedResponse =
        serde_json::from_value(f["predictions.root.unsubscribedResponse"].clone()).unwrap();
}
#[test]
fn predictions_root_subscribed_indices_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootSubscribedIndicesResponse =
        serde_json::from_value(f["predictions.root.subscribedIndicesResponse"].clone()).unwrap();
}
#[test]
fn predictions_root_subscribed_underlyings_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootSubscribedUnderlyingsResponse =
        serde_json::from_value(f["predictions.root.subscribedUnderlyingsResponse"].clone())
            .unwrap();
}
#[test]
fn predictions_root_ok_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootOkResponse =
        serde_json::from_value(f["predictions.root.okResponse"].clone()).unwrap();
}
#[test]
fn predictions_root_error_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsRootErrorResponse =
        serde_json::from_value(f["predictions.root.errorResponse"].clone()).unwrap();
}
#[test]
fn predictions_control_frames_incoming_ping() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsControlFramesIncomingPing =
        serde_json::from_value(f["predictions.control_frames.incomingPing"].clone()).unwrap();
}
#[test]
fn predictions_control_frames_incoming_pong() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsControlFramesIncomingPong =
        serde_json::from_value(f["predictions.control_frames.incomingPong"].clone()).unwrap();
}
#[test]
fn predictions_control_frames_outgoing_ping() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsControlFramesOutgoingPing =
        serde_json::from_value(f["predictions.control_frames.outgoingPing"].clone()).unwrap();
}
#[test]
fn predictions_control_frames_outgoing_pong() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsControlFramesOutgoingPong =
        serde_json::from_value(f["predictions.control_frames.outgoingPong"].clone()).unwrap();
}
#[test]
fn predictions_orderbook_delta_orderbook_snapshot() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsOrderbookDeltaOrderbookSnapshot =
        serde_json::from_value(f["predictions.orderbook_delta.orderbookSnapshot"].clone()).unwrap();
}
#[test]
fn predictions_orderbook_delta_orderbook_delta() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsOrderbookDeltaOrderbookDelta =
        serde_json::from_value(f["predictions.orderbook_delta.orderbookDelta"].clone()).unwrap();
}
#[test]
fn predictions_ticker_ticker() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsTickerTicker =
        serde_json::from_value(f["predictions.ticker.ticker"].clone()).unwrap();
}
#[test]
fn predictions_trade_trade() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsTradeTrade =
        serde_json::from_value(f["predictions.trade.trade"].clone()).unwrap();
}
#[test]
fn predictions_fill_fill() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsFillFill =
        serde_json::from_value(f["predictions.fill.fill"].clone()).unwrap();
}
#[test]
fn predictions_market_positions_market_position() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsMarketPositionsMarketPosition =
        serde_json::from_value(f["predictions.market_positions.marketPosition"].clone()).unwrap();
}
#[test]
fn predictions_market_lifecycle_v2_market_lifecycle_v2() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsMarketLifecycleV2MarketLifecycleV2 =
        serde_json::from_value(f["predictions.market_lifecycle_v2.marketLifecycleV2"].clone())
            .unwrap();
}
#[test]
fn predictions_market_lifecycle_v2_market_metadata_updated() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsMarketLifecycleV2MarketMetadataUpdated =
        serde_json::from_value(f["predictions.market_lifecycle_v2.marketMetadataUpdated"].clone())
            .unwrap();
}
#[test]
fn predictions_market_lifecycle_v2_event_lifecycle() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsMarketLifecycleV2EventLifecycle =
        serde_json::from_value(f["predictions.market_lifecycle_v2.eventLifecycle"].clone())
            .unwrap();
}
#[test]
fn predictions_market_lifecycle_v2_event_fee_update() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsMarketLifecycleV2EventFeeUpdate =
        serde_json::from_value(f["predictions.market_lifecycle_v2.eventFeeUpdate"].clone())
            .unwrap();
}
#[test]
fn predictions_multivariate_market_lifecycle_multivariate_market_lifecycle() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsMultivariateMarketLifecycleMultivariateMarketLifecycle =
        serde_json::from_value(
            f["predictions.multivariate_market_lifecycle.multivariateMarketLifecycle"].clone(),
        )
        .unwrap();
}
#[test]
fn predictions_multivariate_market_lifecycle_event_lifecycle() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsMultivariateMarketLifecycleEventLifecycle =
        serde_json::from_value(
            f["predictions.multivariate_market_lifecycle.eventLifecycle"].clone(),
        )
        .unwrap();
}
#[test]
fn predictions_communications_rfq_created() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsRfqCreated =
        serde_json::from_value(f["predictions.communications.rfqCreated"].clone()).unwrap();
}
#[test]
fn predictions_communications_rfq_deleted() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsRfqDeleted =
        serde_json::from_value(f["predictions.communications.rfqDeleted"].clone()).unwrap();
}
#[test]
fn predictions_communications_quote_created() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsQuoteCreated =
        serde_json::from_value(f["predictions.communications.quoteCreated"].clone()).unwrap();
}
#[test]
fn predictions_communications_quote_accepted() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsQuoteAccepted =
        serde_json::from_value(f["predictions.communications.quoteAccepted"].clone()).unwrap();
}
#[test]
fn predictions_communications_quote_executed() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsQuoteExecuted =
        serde_json::from_value(f["predictions.communications.quoteExecuted"].clone()).unwrap();
}
#[test]
fn predictions_communications_rfq_batch_created() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsRfqBatchCreated =
        serde_json::from_value(f["predictions.communications.rfqBatchCreated"].clone()).unwrap();
}
#[test]
fn predictions_communications_rfq_batch_deleted() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsRfqBatchDeleted =
        serde_json::from_value(f["predictions.communications.rfqBatchDeleted"].clone()).unwrap();
}
#[test]
fn predictions_communications_batch_quote_created() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsBatchQuoteCreated =
        serde_json::from_value(f["predictions.communications.batchQuoteCreated"].clone()).unwrap();
}
#[test]
fn predictions_communications_rfq_batch_best_quotes() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsRfqBatchBestQuotes =
        serde_json::from_value(f["predictions.communications.rfqBatchBestQuotes"].clone()).unwrap();
}
#[test]
fn predictions_communications_batch_quote_accepted() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsBatchQuoteAccepted =
        serde_json::from_value(f["predictions.communications.batchQuoteAccepted"].clone()).unwrap();
}
#[test]
fn predictions_communications_rfq_batch_executed() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsRfqBatchExecuted =
        serde_json::from_value(f["predictions.communications.rfqBatchExecuted"].clone()).unwrap();
}
#[test]
fn predictions_communications_batch_quote_executed() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCommunicationsBatchQuoteExecuted =
        serde_json::from_value(f["predictions.communications.batchQuoteExecuted"].clone()).unwrap();
}
#[test]
fn predictions_order_group_updates_order_group_updates() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsOrderGroupUpdatesOrderGroupUpdates =
        serde_json::from_value(f["predictions.order_group_updates.orderGroupUpdates"].clone())
            .unwrap();
}
#[test]
fn predictions_user_orders_user_order() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsUserOrdersUserOrder =
        serde_json::from_value(f["predictions.user_orders.userOrder"].clone()).unwrap();
}
#[test]
fn predictions_cfbenchmarks_value_cfbenchmarks_value() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCfbenchmarksValueCfbenchmarksValue =
        serde_json::from_value(f["predictions.cfbenchmarks_value.cfbenchmarksValue"].clone())
            .unwrap();
}
#[test]
fn predictions_cfbenchmarks_value_cfbenchmarks_index_list() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCfbenchmarksValueCfbenchmarksIndexList =
        serde_json::from_value(f["predictions.cfbenchmarks_value.cfbenchmarksIndexList"].clone())
            .unwrap();
}
#[test]
fn predictions_cfbenchmarks_value_5hz_cfbenchmarks_value5_hz() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCfbenchmarksValue5hzCfbenchmarksValue5Hz =
        serde_json::from_value(
            f["predictions.cfbenchmarks_value_5hz.cfbenchmarksValue5Hz"].clone(),
        )
        .unwrap();
}
#[test]
fn predictions_cfbenchmarks_value_5hz_cfbenchmarks5_hz_index_list() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsCfbenchmarksValue5hzCfbenchmarks5HzIndexList =
        serde_json::from_value(
            f["predictions.cfbenchmarks_value_5hz.cfbenchmarks5HzIndexList"].clone(),
        )
        .unwrap();
}
#[test]
fn predictions_pyth_value_pyth_value() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsPythValuePythValue =
        serde_json::from_value(f["predictions.pyth_value.pythValue"].clone()).unwrap();
}
#[test]
fn predictions_pyth_value_pyth_underlying_list() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: predictions::PredictionsPythValuePythUnderlyingList =
        serde_json::from_value(f["predictions.pyth_value.pythUnderlyingList"].clone()).unwrap();
}
#[test]
fn margin_root_subscribe_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootSubscribeCommand =
        serde_json::from_value(f["margin.root.subscribeCommand"].clone()).unwrap();
}
#[test]
fn margin_root_unsubscribe_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootUnsubscribeCommand =
        serde_json::from_value(f["margin.root.unsubscribeCommand"].clone()).unwrap();
}
#[test]
fn margin_root_update_subscription_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootUpdateSubscriptionCommand =
        serde_json::from_value(f["margin.root.updateSubscriptionCommand"].clone()).unwrap();
}
#[test]
fn margin_root_update_subscription_delete_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootUpdateSubscriptionDeleteCommand =
        serde_json::from_value(f["margin.root.updateSubscriptionDeleteCommand"].clone()).unwrap();
}
#[test]
fn margin_root_update_subscription_single_sid_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootUpdateSubscriptionSingleSidCommand =
        serde_json::from_value(f["margin.root.updateSubscriptionSingleSidCommand"].clone())
            .unwrap();
}
#[test]
fn margin_root_list_subscriptions_command() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootListSubscriptionsCommand =
        serde_json::from_value(f["margin.root.listSubscriptionsCommand"].clone()).unwrap();
}
#[test]
fn margin_root_subscribed_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootSubscribedResponse =
        serde_json::from_value(f["margin.root.subscribedResponse"].clone()).unwrap();
}
#[test]
fn margin_root_unsubscribed_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootUnsubscribedResponse =
        serde_json::from_value(f["margin.root.unsubscribedResponse"].clone()).unwrap();
}
#[test]
fn margin_root_ok_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootOkResponse =
        serde_json::from_value(f["margin.root.okResponse"].clone()).unwrap();
}
#[test]
fn margin_root_list_subscriptions_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootListSubscriptionsResponse =
        serde_json::from_value(f["margin.root.listSubscriptionsResponse"].clone()).unwrap();
}
#[test]
fn margin_root_error_response() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginRootErrorResponse =
        serde_json::from_value(f["margin.root.errorResponse"].clone()).unwrap();
}
#[test]
fn margin_control_frames_incoming_ping() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginControlFramesIncomingPing =
        serde_json::from_value(f["margin.control_frames.incomingPing"].clone()).unwrap();
}
#[test]
fn margin_control_frames_incoming_pong() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginControlFramesIncomingPong =
        serde_json::from_value(f["margin.control_frames.incomingPong"].clone()).unwrap();
}
#[test]
fn margin_control_frames_outgoing_ping() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginControlFramesOutgoingPing =
        serde_json::from_value(f["margin.control_frames.outgoingPing"].clone()).unwrap();
}
#[test]
fn margin_control_frames_outgoing_pong() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginControlFramesOutgoingPong =
        serde_json::from_value(f["margin.control_frames.outgoingPong"].clone()).unwrap();
}
#[test]
fn margin_orderbook_delta_orderbook_snapshot() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginOrderbookDeltaOrderbookSnapshot =
        serde_json::from_value(f["margin.orderbook_delta.orderbookSnapshot"].clone()).unwrap();
}
#[test]
fn margin_orderbook_delta_orderbook_delta() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginOrderbookDeltaOrderbookDelta =
        serde_json::from_value(f["margin.orderbook_delta.orderbookDelta"].clone()).unwrap();
}
#[test]
fn margin_corporate_actions_corporate_actions_delta() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginCorporateActionsCorporateActionsDelta =
        serde_json::from_value(f["margin.corporate_actions.corporateActionsDelta"].clone())
            .unwrap();
}
#[test]
fn margin_market_lifecycle_v2_market_lifecycle_v2() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginMarketLifecycleV2MarketLifecycleV2 =
        serde_json::from_value(f["margin.market_lifecycle_v2.marketLifecycleV2"].clone()).unwrap();
}
#[test]
fn margin_ticker_ticker() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginTickerTicker =
        serde_json::from_value(f["margin.ticker.ticker"].clone()).unwrap();
}
#[test]
fn margin_trade_trade() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginTradeTrade =
        serde_json::from_value(f["margin.trade.trade"].clone()).unwrap();
}
#[test]
fn margin_fill_fill() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginFillFill = serde_json::from_value(f["margin.fill.fill"].clone()).unwrap();
}
#[test]
fn margin_user_orders_user_order() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginUserOrdersUserOrder =
        serde_json::from_value(f["margin.user_orders.userOrder"].clone()).unwrap();
}
#[test]
fn margin_order_group_updates_order_group_updates() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: margin::MarginOrderGroupUpdatesOrderGroupUpdates =
        serde_json::from_value(f["margin.order_group_updates.orderGroupUpdates"].clone()).unwrap();
}
