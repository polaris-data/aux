// Generated from pinned native WebSocket specifications.
use aux_rs_derive::*;
use serde_json::{Value, json};
#[test]
fn websocket_public_login() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["public_login"];
    let payload: LoginRequest = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"public/login","params":value});
    assert_eq!(wire["method"], "public/login");
}
#[test]
fn websocket_subscribe() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["subscribe"];
    let payload: SubscribeParams = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"subscribe","params":value});
    assert_eq!(wire["method"], "subscribe");
}
#[test]
fn websocket_unsubscribe() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["unsubscribe"];
    let payload: UnsubscribeParams = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire = json!({"jsonrpc":"2.0","id":1,"method":"unsubscribe","params":value});
    assert_eq!(wire["method"], "unsubscribe");
}
#[test]
fn websocket_private_set_cancel_on_disconnect() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["private_set_cancel_on_disconnect"];
    let payload: SetCancelOnDisconnectRequest =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let wire =
        json!({"jsonrpc":"2.0","id":1,"method":"private/set_cancel_on_disconnect","params":value});
    assert_eq!(wire["method"], "private/set_cancel_on_disconnect");
}
#[test]
fn websocket_auctions_watch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["auctions_watch"];
    let payload: Vec<AuctionResult> = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<AuctionResult> = event.payload().unwrap();
}
#[test]
fn websocket_margin_watch() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["margin_watch"];
    let payload: Vec<MarginWatchResult> = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<MarginWatchResult> = event.payload().unwrap();
}
#[test]
fn websocket_orderbook_instrument_name_group_depth() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["orderbook_instrument_name_group_depth"];
    let payload: OrderbookPayloadDoc = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: OrderbookPayloadDoc = event.payload().unwrap();
}
#[test]
fn websocket_spot_feed_currency() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["spot_feed_currency"];
    let payload: SpotFeedPayload = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: SpotFeedPayload = event.payload().unwrap();
}
#[test]
fn websocket_subaccount_id_balances() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["subaccount_id_balances"];
    let payload: Vec<BalanceUpdate> = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<BalanceUpdate> = event.payload().unwrap();
}
#[test]
fn websocket_subaccount_id_best_quotes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["subaccount_id_best_quotes"];
    let payload: Vec<BestQuoteChannelResult> =
        serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<BestQuoteChannelResult> = event.payload().unwrap();
}
#[test]
fn websocket_subaccount_id_orders() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["subaccount_id_orders"];
    let payload: Vec<Order> = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<Order> = event.payload().unwrap();
}
#[test]
fn websocket_subaccount_id_quotes() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["subaccount_id_quotes"];
    let payload: Vec<QuotePublishResult> = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<QuotePublishResult> = event.payload().unwrap();
}
#[test]
fn websocket_subaccount_id_trades() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["subaccount_id_trades"];
    let payload: Vec<Trade> = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<Trade> = event.payload().unwrap();
}
#[test]
fn websocket_ticker_slim_instrument_name_interval() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["ticker_slim_instrument_name_interval"];
    let payload: TickerSlimPayload = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: TickerSlimPayload = event.payload().unwrap();
}
#[test]
fn websocket_trades_instrument_name() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["trades_instrument_name"];
    let payload: Vec<PublicTrade> = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<PublicTrade> = event.payload().unwrap();
}
#[test]
fn websocket_trades_instrument_type_currency() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["trades_instrument_type_currency"];
    let payload: Vec<PublicTrade> = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<PublicTrade> = event.payload().unwrap();
}
#[test]
fn websocket_wallet_rfqs() {
    let fixtures: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let f = &fixtures["wallet_rfqs"];
    let payload: Vec<PublicRfq> = serde_json::from_value(f["payload"].clone()).unwrap();
    let value = serde_json::to_value(payload).unwrap();
    let specification: Subscription = serde_json::from_value(f["specification"].clone()).unwrap();
    assert_eq!(specification.channel().unwrap(), f["address"]);
    let wire = json!({"method":"subscription","params":{"channel":f["address"],"data":value}});
    let event = Event::decode(&wire.to_string()).unwrap();
    let _: Vec<PublicRfq> = event.payload().unwrap();
}
