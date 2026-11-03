// Generated from pinned native WebSocket specifications.
use aux_rs_paradex::*;
use serde_json::{Value, json};
#[test]
fn websocket_account() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["account"];
    let payload: WsAccountAccountSubscribe = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsAccountAccountSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_auth() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["auth"];
    let payload: WsAuthAuthPublish = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"auth","params":value});
    assert_eq!(wire["method"], "auth");
    let _response: WsAuthAuthSubscribe = serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_balance_events() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["balance_events"];
    let payload: WsBalanceEventsBalanceEventsSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsBalanceEventsBalanceEventsSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_bbo_market_symbol() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["bbo_market_symbol"];
    let payload: WsBboMarketSymbolBboMarketSymbolSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsBboMarketSymbolBboMarketSymbolSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_fills_market_symbol() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["fills_market_symbol"];
    let payload: WsFillsMarketSymbolFillsMarketSymbolSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsFillsMarketSymbolFillsMarketSymbolSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_funding_data_market_symbol() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["funding_data_market_symbol"];
    let payload: WsFundingDataMarketSymbolFundingDataMarketSymbolSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsFundingDataMarketSymbolFundingDataMarketSymbolSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_funding_payments_market_symbol() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["funding_payments_market_symbol"];
    let payload: WsFundingPaymentsMarketSymbolFundingPaymentsMarketSymbolSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsFundingPaymentsMarketSymbolFundingPaymentsMarketSymbolSubscribe =
        event.payload().unwrap();
}
#[test]
fn websocket_markets_market_symbol() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["markets_market_symbol"];
    let payload: WsMarketsMarketSymbolMarketsMarketSymbolSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsMarketsMarketSymbolMarketsMarketSymbolSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_markets_summary_market_symbol() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["markets_summary_market_symbol"];
    let payload: WsMarketsSummaryMarketSymbolMarketsSummaryMarketSymbolSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsMarketsSummaryMarketSymbolMarketsSummaryMarketSymbolSubscribe =
        event.payload().unwrap();
}
#[test]
fn websocket_mmp() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["mmp"];
    let payload: WsMmpMmpSubscribe = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsMmpMmpSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_modify_subscription() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["modify_subscription"];
    let payload: WsModifySubscriptionModifySubscriptionPublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"modify_subscription","params":value});
    assert_eq!(wire["method"], "modify_subscription");
    let _response: WsModifySubscriptionModifySubscriptionSubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_order_book_market_symbol_feed_type_15_refresh_rate_price_tick() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["order_book_market_symbol_feed_type_15_refresh_rate_price_tick"];
    let payload:WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribe=serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _:WsOrderBookMarketSymbolFeedType15RefreshRatePriceTickOrderBookMarketSymbolFeedType15RefreshRatePriceTickSubscribe=event.payload().unwrap();
}
#[test]
fn websocket_order_cancel_all() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["order_cancel_all"];
    let payload: WsOrderCancelAllOrderCancelAllPublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"order.cancel_all","params":value});
    assert_eq!(wire["method"], "order.cancel_all");
    let _response: WsOrderCancelAllOrderCancelAllSubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_order_cancel_batch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["order_cancel_batch"];
    let payload: WsOrderCancelBatchOrderCancelBatchPublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"order.cancel_batch","params":value});
    assert_eq!(wire["method"], "order.cancel_batch");
    let _response: WsOrderCancelBatchOrderCancelBatchSubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_order_cancel_on_disconnect() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["order_cancel_on_disconnect"];
    let payload: WsOrderCancelOnDisconnectOrderCancelOnDisconnectPublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"order.cancel_on_disconnect","params":value});
    assert_eq!(wire["method"], "order.cancel_on_disconnect");
    let _response: WsOrderCancelOnDisconnectOrderCancelOnDisconnectSubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_order_cancel() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["order_cancel"];
    let payload: WsOrderCancelOrderCancelPublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"order.cancel","params":value});
    assert_eq!(wire["method"], "order.cancel");
    let _response: WsOrderCancelOrderCancelSubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_order_create_batch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["order_create_batch"];
    let payload: WsOrderCreateBatchOrderCreateBatchPublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"order.create_batch","params":value});
    assert_eq!(wire["method"], "order.create_batch");
    let _response: WsOrderCreateBatchOrderCreateBatchSubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_order_create() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["order_create"];
    let payload: WsOrderCreateOrderCreatePublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"order.create","params":value});
    assert_eq!(wire["method"], "order.create");
    let _response: WsOrderCreateOrderCreateSubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_order_modify() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["order_modify"];
    let payload: WsOrderModifyOrderModifyPublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"order.modify","params":value});
    assert_eq!(wire["method"], "order.modify");
    let _response: WsOrderModifyOrderModifySubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_orders_market_symbol() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["orders_market_symbol"];
    let payload: WsOrdersMarketSymbolOrdersMarketSymbolSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsOrdersMarketSymbolOrdersMarketSymbolSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_positions() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["positions"];
    let payload: WsPositionsPositionsSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsPositionsPositionsSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_rfq_rfq_id() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["rfq_rfq_id"];
    let payload: WsRfqRfqIdRfqRfqIdSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsRfqRfqIdRfqRfqIdSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_subscribe() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["subscribe"];
    let payload: WsSubscribeSubscribePublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"subscribe","params":value});
    assert_eq!(wire["method"], "subscribe");
    let _response: WsSubscribeSubscribeSubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_tradebusts() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["tradebusts"];
    let payload: WsTradebustsTradebustsSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsTradebustsTradebustsSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_trades_market_symbol() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["trades_market_symbol"];
    let payload: WsTradesMarketSymbolTradesMarketSymbolSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsTradesMarketSymbolTradesMarketSymbolSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_transaction() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["transaction"];
    let payload: WsTransactionTransactionSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsTransactionTransactionSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_transfers() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["transfers"];
    let payload: WsTransfersTransfersSubscribe =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: WsTransfersTransfersSubscribe = event.payload().unwrap();
}
#[test]
fn websocket_unsubscribe() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["unsubscribe"];
    let payload: WsUnsubscribeUnsubscribePublish =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"unsubscribe","params":value});
    assert_eq!(wire["method"], "unsubscribe");
    let _response: WsUnsubscribeUnsubscribeSubscribe =
        serde_json::from_value(f["response"].clone()).unwrap();
}
#[test]
fn websocket_funding_rate_comparison() {
    let spec: Subscription = serde_json::from_value(
        serde_json::json!({"type": "funding_rate_comparison", "refresh_rate": "1000ms"}),
    )
    .unwrap();
    assert_eq!(
        spec.channel().unwrap(),
        "funding_rate_comparison.ALL@1000ms"
    );
}
#[test]
fn websocket_block_trades() {
    let spec: Subscription =
        serde_json::from_value(serde_json::json!({"type": "block_trades"})).unwrap();
    assert_eq!(spec.channel().unwrap(), "block_trades");
}
