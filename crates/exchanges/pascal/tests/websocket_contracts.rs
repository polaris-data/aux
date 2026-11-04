use aux_rs_pascal::ws_types::*;
use serde_json::Value;
#[test]
fn data_client_requests_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataClientRequestsExample1 =
        serde_json::from_value(f["data.client-requests.example_1"].clone()).unwrap();
}
#[test]
fn data_client_requests_example_2() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataClientRequestsExample2 =
        serde_json::from_value(f["data.client-requests.example_2"].clone()).unwrap();
}
#[test]
fn data_client_requests_example_3() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataClientRequestsExample3 =
        serde_json::from_value(f["data.client-requests.example_3"].clone()).unwrap();
}
#[test]
fn data_client_requests_example_4() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataClientRequestsExample4 =
        serde_json::from_value(f["data.client-requests.example_4"].clone()).unwrap();
}
#[test]
fn data_server_responses_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataServerResponsesExample1 =
        serde_json::from_value(f["data.server-responses.example_1"].clone()).unwrap();
}
#[test]
fn data_server_responses_example_2() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataServerResponsesExample2 =
        serde_json::from_value(f["data.server-responses.example_2"].clone()).unwrap();
}
#[test]
fn data_server_responses_example_3() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataServerResponsesExample3 =
        serde_json::from_value(f["data.server-responses.example_3"].clone()).unwrap();
}
#[test]
fn data_server_responses_example_4() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataServerResponsesExample4 =
        serde_json::from_value(f["data.server-responses.example_4"].clone()).unwrap();
}
#[test]
fn data_client_messages_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataClientMessagesExample1 =
        serde_json::from_value(f["data.client-messages.example_1"].clone()).unwrap();
}
#[test]
fn data_client_messages_example_2() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataClientMessagesExample2 =
        serde_json::from_value(f["data.client-messages.example_2"].clone()).unwrap();
}
#[test]
fn data_client_messages_example_3() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataClientMessagesExample3 =
        serde_json::from_value(f["data.client-messages.example_3"].clone()).unwrap();
}
#[test]
fn data_server_messages_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataServerMessagesExample1 =
        serde_json::from_value(f["data.server-messages.example_1"].clone()).unwrap();
}
#[test]
fn data_book_channel_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataBookChannelExample1 =
        serde_json::from_value(f["data.book-channel.example_1"].clone()).unwrap();
}
#[test]
fn data_book_channel_example_2() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataBookChannelExample2 =
        serde_json::from_value(f["data.book-channel.example_2"].clone()).unwrap();
}
#[test]
fn data_trades_channel_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataTradesChannelExample1 =
        serde_json::from_value(f["data.trades-channel.example_1"].clone()).unwrap();
}
#[test]
fn data_all_trades_channel_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataAllTradesChannelExample1 =
        serde_json::from_value(f["data.all-trades-channel.example_1"].clone()).unwrap();
}
#[test]
fn data_all_trades_channel_example_2() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataAllTradesChannelExample2 =
        serde_json::from_value(f["data.all-trades-channel.example_2"].clone()).unwrap();
}
#[test]
fn data_candles_channel_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataCandlesChannelExample1 =
        serde_json::from_value(f["data.candles-channel.example_1"].clone()).unwrap();
}
#[test]
fn data_mark_price_candles_channel_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataMarkPriceCandlesChannelExample1 =
        serde_json::from_value(f["data.mark-price-candles-channel.example_1"].clone()).unwrap();
}
#[test]
fn data_account_channel_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataAccountChannelExample1 =
        serde_json::from_value(f["data.account-channel.example_1"].clone()).unwrap();
}
#[test]
fn data_account_channel_example_2() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataAccountChannelExample2 =
        serde_json::from_value(f["data.account-channel.example_2"].clone()).unwrap();
}
#[test]
fn data_account_channel_example_3() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataAccountChannelExample3 =
        serde_json::from_value(f["data.account-channel.example_3"].clone()).unwrap();
}
#[test]
fn data_market_update_channel_example_1() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataMarketUpdateChannelExample1 =
        serde_json::from_value(f["data.market-update-channel.example_1"].clone()).unwrap();
}
#[test]
fn data_market_update_channel_example_2() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataMarketUpdateChannelExample2 =
        serde_json::from_value(f["data.market-update-channel.example_2"].clone()).unwrap();
}
#[test]
fn data_market_update_channel_example_3() {
    let f: Value = serde_json::from_str(include_str!("fixtures/ws.json")).unwrap();
    let _: data::DataMarketUpdateChannelExample3 =
        serde_json::from_value(f["data.market-update-channel.example_3"].clone()).unwrap();
}
