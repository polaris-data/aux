// Compiled in a separate Cargo project against the packaged crate.
use aux_rs::{
    ExtendedClient,
    http::{OutboundRequest, TransportResponse},
};

/// The local peer stands in for the application side of a verified channel.
struct FixtureTransport(String);
impl aux_rs::http::Transport for FixtureTransport {
    fn send(&self, _: OutboundRequest) -> aux_rs::Result<TransportResponse> {
        panic!("streaming must use connect_stream")
    }
    fn connect_stream(&self, host:&str, port:u16, tls:bool) -> aux_rs::Result<std::os::fd::OwnedFd> {
        assert_eq!((port,tls),(443,true));
        if host != "policy.invalid" { return Err(aux_rs::Error::AuthenticationRequired); }
        std::net::TcpStream::connect(&self.0).map(Into::into).map_err(|_|aux_rs::Error::Network)
    }
}

fn main() -> aux_rs::Result<()> {
    for row in serde_json::from_str::<Vec<serde_json::Value>>(include_str!("../polymarket-helpers.json")).unwrap() {
        let op=row["operation"].as_str().unwrap();
        let result:serde_json::Value=aux_rs::polymarket::evaluate(op,&row["request"]).unwrap_or_else(|e|panic!("{op}: {e:?}"));
        assert_eq!(result,row["expected"],"{op}");
    }
    for row in serde_json::from_str::<Vec<serde_json::Value>>(include_str!("../metric-helpers.json")).unwrap() {
        let op=row["operation"].as_str().unwrap();
        let actual=aux_rs::metric::evaluate(op,&row["request"]).unwrap_or_else(|e| panic!("{op} failed for {}: {e:?}", row["request"]));
        assert_eq!(actual,row["expected"],"{op}");
    }
    let d: serde_json::Value = serde_json::from_str(include_str!("../derive-signing.json")).unwrap();
    for row in d["actions"].as_array().unwrap() {
        let hex = row["encoded"].as_str().unwrap();
        let bytes: Vec<u8> = (2..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i+2],16).unwrap()).collect();
        for proof in row["vectors"].as_array().unwrap() {
            let client = aux_rs::DeriveClient::new().with_origin("https://fixture.invalid")
                .with_credentials(d["owner"].as_str().unwrap(),d["key"].as_str().unwrap())
                .with_signing_chain_id(proof["chain_id"].as_u64().unwrap());
            let request = serde_json::json!({"nonce":d["nonce"],"subaccount_id":d["subaccount_id"],"signature_expiry_sec":d["expiry"]});
            assert_eq!(client.prepare_signed(&request,&serde_json::from_value(row["data"].clone()).unwrap())?["signature"],proof["signature"]);
            let mut action: aux_rs::derive::ActionPayload = serde_json::from_value(serde_json::json!({"module":row["module"],"encoded_data":bytes,"nonce":d["nonce"],
                "subaccount_id":d["subaccount_id"],"expiry":d["expiry"],"owner":d["owner"],"signer":d["signer"]})).unwrap();
            assert_eq!(client.sign_action(&action)?,proof["signature"].as_str().unwrap());
            action.owner = d["signer"].as_str().unwrap().into();
            assert!(client.sign_action(&action).is_err());
        }
    }
    let proofs: serde_json::Value = serde_json::from_str(include_str!("../paradex-signing.json")).unwrap();
    let blocks: serde_json::Value = serde_json::from_str(include_str!("../paradex-blocks.json")).unwrap();
    for vector in proofs["vectors"].as_array().unwrap() {
        let Some(block) = blocks.get(vector["name"].as_str().unwrap()) else { continue };
        for evm in [false,true] {
            let signer = if evm { aux_rs::ParadexSigner::Evm(proofs["evm_key"].as_str().unwrap().into()) }
                         else { aux_rs::ParadexSigner::Stark(proofs["key"].as_str().unwrap().into()) };
            let response = aux_rs::ParadexClient::new().with_origin("https://fixture.invalid")
                .with_signer(proofs["account"].as_str().unwrap(),vector["network"].as_str().unwrap(),signer)
                .block_signature(block["nonce"].as_str().unwrap(),block["expiration"].as_u64().unwrap(),&block["trades"],block["parent"].as_str())?;
            assert_eq!(serde_json::from_str::<serde_json::Value>(response["signature_data"].as_str().unwrap()).unwrap(),
                       vector[if evm {"evm_signature"} else {"signature"}]);
        }
    }
    let price = ExtendedClient::new()
        .with_transport(|request: OutboundRequest| {
            assert_eq!(
                request.url,
                "https://api.starknet.extended.exchange/api/v1/info/assets/ETH/price"
            );
            Ok(TransportResponse {
                status: 200,
                headers: Vec::new(),
                body: r#"{"price":12345678901234567890.12345}"#.into(),
            })
        })
        .get_asset_price(&aux_rs::http::decode(
            "extended",
            200,
            r#"{"asset":"ETH"}"#,
        )?)?;
    assert_eq!(price["price"].to_string(), "12345678901234567890.12345");
    let order = serde_json::json!({
        "position_id":{"value":100},"base_asset_id":{"value":"2"},"base_amount":100,
        "quote_asset_id":{"value":"1"},"quote_amount":-156,"fee_asset_id":{"value":"1"},
        "fee_amount":74,"expiration":{"seconds":100},"salt":"123"
    });
    let settlement = aux_rs::extended::settlement("order", "SN_SEPOLIA",
        "0x5d05989e9302dcebc74e241001e3e3ac3f4402ccf2f8e6f74b034b07ad6a904", &order, None)?;
    assert_eq!(settlement["hash"], "0x4de4c009e0d0c5a70a7da0e2039fb2b99f376d53496f89d9f437e736add6b48");
    let origin = std::env::var("AUX_FIXTURE_ORIGIN").unwrap();
    let catalog: Vec<serde_json::Value> = serde_json::from_str(include_str!("../exchange-http.json")).unwrap();
    let books: Vec<serde_json::Value> = serde_json::from_str(include_str!("../exchange-ws.json")).unwrap();
    macro_rules! native_exchanges {
        ($($client:ident => $venue:literal),+ $(,)?) => {{
            $(let market = catalog.iter().find(|r| r["venue"] == $venue && r["operation"] == "markets").unwrap();
              let book = books.iter().find(|r| r["venue"] == $venue).unwrap();
              let rest = format!("{origin}/native-exchanges/{}/rest/public", $venue);
              let endpoint = format!("{}/native-exchanges/{}/ws/book", origin.replacen("http:", "ws:", 1), $venue);
              let client = aux_rs::$client::new().with_origin(rest);
              assert_eq!(client.markets(&market["request"])?, market["response"]);
              let stream = client.subscribe(&book["params"], Some(&endpoint))?;
              assert_eq!(stream.next_event()?.unwrap(), book["event"]);
              assert!(stream.next_event()?.is_none());)+
        }};
    }
    native_exchanges!(AevoClient => "aevo", BackpackClient => "backpack", BinanceSpotClient => "binance_spot",
        BybitClient => "bybit", DeribitClient => "deribit", DydxV4Client => "dydx_v4", HibachiClient => "hibachi",
        KrakenSpotClient => "kraken_spot", OkxClient => "okx", OrderlyClient => "orderly");
    let market_streams: Vec<serde_json::Value> = serde_json::from_str(include_str!("../market-data-ws.json")).unwrap();
    macro_rules! market_data {
        ($($client:ident => $venue:literal),+ $(,)?) => {$(
            let rest = format!("{origin}/native-exchanges/{}/rest/public", $venue);
            let client = aux_rs::$client::new().with_origin(rest);
            let market = catalog.iter().find(|r| r["venue"] == $venue && r["operation"] == "markets").unwrap();
            assert_eq!(client.markets(&market["request"])?, market["response"]);
            for row in market_streams.iter().filter(|r| r["venue"] == $venue) {
                let endpoint = format!("{}/native-exchanges/{}/ws/{}", origin.replacen("http:", "ws:", 1),
                    $venue, row["case"].as_str().unwrap());
                let stream = client.subscribe(&row["params"], Some(&endpoint))?;
                for event in row["events"].as_array().unwrap() {
                    assert_eq!(&stream.next_event()?.unwrap(), event);
                }
                if row["keep_open"] != true {
                    assert!(stream.next_event()?.is_none());
                }
                stream.close();
            }
        )+};
    }
    market_data!(ApexOmniClient => "apex_omni", KuruClient => "kuru", ReyaClient => "reya");
    let commands = origin.replacen("http:", "ws:", 1) + "/native-exchanges/binance_spot/ws/orders";
    let trader = aux_rs::BinanceSpotClient::new().with_credentials("fixture-key", "fixture-secret");
    let order_session = trader.open_rpc(Some(&commands), &serde_json::json!({}))?;
    let receipt = order_session.place_order(&serde_json::json!({"symbol":"ETHUSDT","side":"BUY","type":"LIMIT","price":"100","quantity":"0.01"}))?;
    assert_eq!(receipt["result"]["orderId"], 9007199254740993u64);
    order_session.close();
    let perps: Vec<serde_json::Value> = serde_json::from_str(include_str!("../perps-http.json")).unwrap();
    macro_rules! perpetuals {
        ($($client:ident => $venue:literal),+ $(,)?) => {{
            $(let row = perps.iter().find(|r| r["venue"] == $venue && r["operation"] == "markets").unwrap();
              let client = aux_rs::$client::new().with_origin(format!("{origin}/perps/{}", $venue));
              assert_eq!(client.markets(&row["request"])?, row["response"]);)+
        }};
    }
    perpetuals!(InjectiveClient => "injective", NadoClient => "nado", OndoPerpsClient => "ondo_perps",
        PacificaClient => "pacifica", ParagonClient => "paragon", TradeXyzClient => "trade_xyz", AsterClient => "aster");
    let added = format!("{origin}/new-exchanges");
    macro_rules! market_discovery {
        ($($client:ident => $venue:literal),+ $(,)?) => {{
            $(let request = perps.iter().find(|row| row["venue"] == $venue && row["operation"] == "markets").unwrap();
              let client = aux_rs::$client::new()
                .with_origin(format!("{added}/{}", $venue))
                .with_credentials("fixture-key", "fixture-secret");
              let reply = client.markets(&request["request"])?;
              if $venue == "edgex" {
                  assert_eq!(reply["data"]["contractList"][0]["contractId"], "10000001");
              } else {
                  assert_eq!(reply, request["response"]);
              })+
        }};
    }
    market_discovery!(PerplClient => "perpl", RisexClient => "risex", DecibelClient => "decibel",
        EdgeXClient => "edgex", GrvtClient => "grvt", KatanaClient => "katana");
    let stats = aux_rs::VariationalClient::new().with_origin(format!("{added}/variational"))
        .market_stats(&serde_json::json!({}))?;
    assert_eq!(stats["listings"][0]["contract_type"], "perpetual");
    assert_eq!(stats["listings"][1]["funding"], serde_json::Value::Null);
    let seed = (0..32).map(|i| format!("{i:02x}")).collect::<String>();
    let perpl = aux_rs::PerplClient::new().with_origin(format!("{added}/perpl")).with_credentials("fixture-key", seed);
    let order = perps.iter().find(|row| row["venue"] == "perpl" && row["operation"] == "place_order").unwrap();
    assert_eq!(perpl.place_order(&order["request"] )?, order["response"]);
    let endpoint = added.replacen("http:", "ws:", 1) + "/perpl/ws/book";
    let book = perpl.subscribe(&serde_json::json!({"channel":"order_book","market_id":1}), Some(&endpoint))?;
    assert_eq!(book.next_event()?.unwrap()["bid"][0]["p"], 9007199254740993u64);
    book.close();
    let injective = aux_rs::InjectiveClient::new().with_origin(format!("{origin}/perps/injective"));
    let book = injective.subscribe(&serde_json::json!({"channel":"order_book","market_id":format!("0x{}", "33".repeat(32))}), None)?;
    assert_eq!(book.next_event()?.unwrap()["orderbook"]["sequence"], 9007199254740993u64);
    assert!(book.next_event()?.is_none());
    let hermes = aux_rs::PythHermesClient::new().with_origin(format!("{origin}/stream-refused")).with_api_key("fixture-stream-credential");
    let rejected = hermes.stream_price_updates(&serde_json::json!({"ids":["ab".repeat(32)]}));
    assert!(matches!(rejected,Err(aux_rs::Error::ExchangeResponse(e)) if e.venue == "pyth_hermes" && e.status == 403 && e.body == serde_json::json!({"message":"invalid Bearer [redacted]", "seq":9007199254740993u64})));
    let endpoint = origin.replace("http:", "ws:") + "/stream-refused/ws";
    let rejected = aux_rs::QfexClient::new().subscribe(&serde_json::json!({"channel":"bbo", "symbol":"BTC"}), Some(&endpoint));
    assert!(matches!(rejected,Err(aux_rs::Error::ExchangeResponse(e)) if e.venue == "qfex" && e.status == 403 && e.body["seq"] == 9007199254740993u64));
    let hl = aux_rs::HyperliquidClient::new();
    let scope = origin.replace("http:", "ws:") + "/account-scope/rust";
    let first = format!("0x{}", "11".repeat(20));
    let second = format!("0x{}", "22".repeat(20));
    let updates = hl.subscribe(&serde_json::json!({"type":"orderUpdates","user":first}), Some(&scope))?;
    let orders = hl.subscribe(&serde_json::json!({"type":"openOrders","user":first}), Some(&scope))?;
    let other = hl.subscribe(&serde_json::json!({"type":"orderUpdates","user":second}), Some(&scope))?;
    let fills = hl.subscribe(&serde_json::json!({"type":"userFills","user":first,"aggregateByTime":true}), Some(&scope))?;
    assert_eq!(updates.next_event()?.unwrap()["data"][0]["order"]["oid"], 1);
    assert_eq!(orders.next_event()?.unwrap()["data"]["user"], first);
    assert_eq!(other.next_event()?.unwrap()["data"][0]["order"]["oid"], 2);
    assert_eq!(fills.next_event()?.unwrap()["data"]["fills"], serde_json::json!([]));
    for stream in [updates,orders,other,fills] { stream.close(); }
    let qfex = aux_rs::QfexClient::new();
    let qfex_mux = origin.replace("http:", "ws:") + "/qfex-multiplex/rust";
    let trades = qfex.subscribe(&serde_json::json!({"channel":"trade","symbol":"*"}), Some(&qfex_mux))?;
    let book = qfex.subscribe(&serde_json::json!({"channel":"bbo","symbol":"BTC"}), Some(&qfex_mux))?;
    for generation in [1, 2] { assert_eq!(trades.next_event()?.unwrap()["seq"], generation); }
    assert_eq!(book.next_event()?.unwrap()["seq"], 9007199254740993u64);
    trades.close();
    assert_eq!(book.next_event()?.unwrap()["seq"], 3);
    book.close();
    let instructions:serde_json::Value=serde_json::from_str(include_str!("../phoenix-instructions.json")).unwrap();
    let phoenix=aux_rs::PhoenixClient::new();
    for (name,row) in instructions.as_object().unwrap() {
        assert_eq!(phoenix.build_instruction(name,&row["request"])?, row["instruction"],"{name}");
    }
    let mut approve=instructions["create_spl_approve_ix"]["request"].clone();
    approve["amount"]=serde_json::json!(u64::MAX);
    let built=phoenix.build_instruction("create_spl_approve_ix",&approve)?;
    assert_eq!(built["data"],serde_json::json!([4,255,255,255,255,255,255,255,255]));
    approve["source"]="bad-key".into();
    assert!(phoenix.build_instruction("create_spl_approve_ix",&approve).is_err());
    let mut cancels=instructions["create_cancel_orders_by_id_ix"]["request"].clone();
    cancels["order_ids"]=serde_json::json!([]);
    assert!(phoenix.build_instruction("create_cancel_orders_by_id_ix",&cancels).is_err());
    let tx_rows:Vec<serde_json::Value>=serde_json::from_str(include_str!("../phoenix-transactions.json")).unwrap();
    for row in &tx_rows {
        let prepared=aux_rs::PhoenixClient::new().prepare_transaction(&row["request"])?;
        assert_eq!(serde_json::json!(prepared),row["prepared"]);
        let sigs=prepared.sign(&"01".repeat(32),&Default::default())?;
        assert!(prepared.assemble(&sigs).is_err());
        let sigs=prepared.sign(&"02".repeat(32),&sigs)?;
        assert_eq!(serde_json::json!(sigs),row["signatures"]);
        let signed=prepared.assemble(&sigs)?;
        assert_eq!(serde_json::json!(signed),row["signed"]);
        signed.validate()?;prepared.validate_server_transaction(&signed.transaction_base64)?;
        let mut malformed=signed.clone();malformed.signature="1".repeat(64);
        assert!(malformed.validate().is_err());
    }
    let tx=&tx_rows[0];
    let signed:aux_rs::phoenix::SignedTransaction=serde_json::from_value(tx["signed"].clone()).unwrap();
    let phoenix=aux_rs::PhoenixChainClient::new(&format!("{origin}/phoenix-rpc/rust"))?;
    assert_eq!(serde_json::json!(phoenix.prepare(&tx["request"]["instructions"],tx["request"]["fee_payer"].as_str().unwrap())?),tx["prepared"]);
    assert!(phoenix.simulate(&signed)?["value"]["err"].is_null());
    assert_eq!(phoenix.submit(&signed)?,signed.signature);
    assert_eq!(phoenix.pending()?.as_ref(),Some(&signed));
    assert_eq!(phoenix.signature_status(&signed.signature)?["value"][0]["confirmationStatus"],"confirmed");
    assert_eq!(phoenix.transaction(&signed.signature)?["slot"].as_u64(),Some(9007199254740993));
    phoenix.clear_reconciled_submission()?;assert!(phoenix.pending()?.is_none());
    let lost=aux_rs::PhoenixChainClient::new(&format!("{origin}/phoenix-rpc/rust/lost"))?;
    for _ in 0..2 {assert!(matches!(lost.submit(&signed),Err(aux_rs::Error::PendingOutcome)));assert_eq!(lost.pending()?.as_ref(),Some(&signed));}
    lost.clear_reconciled_submission()?;
    let rpc_url=origin.replace("http:","ws:")+"/rpc/";
    let mux_url=origin.replace("http:","ws:")+"/multiplex/rust";
    let feed=aux_rs::HyperliquidClient::new();
    let eth=feed.subscribe(&serde_json::json!({"type":"l2Book","coin":"ETH"}),Some(&mux_url))?;
    let btc=feed.subscribe(&serde_json::json!({"type":"trades","coin":"BTC"}),Some(&mux_url))?;
    let duplicate=feed.subscribe(&serde_json::json!({"type":"trades","coin":"BTC"}),Some(&mux_url))?;
    let sol=feed.subscribe(&serde_json::json!({"type":"l2Book","coin":"SOL"}),Some(&mux_url))?;
    let read=|stream:&aux_rs::EventStream|->aux_rs::Result<serde_json::Value>{ Ok(serde_json::from_str(&stream.next_json()?.unwrap()).unwrap()) };
    assert_eq!(read(&eth)?["data"]["time"],1);
    assert_eq!(read(&btc)?["data"][0]["coin"],"BTC");
    assert_eq!(read(&duplicate)?["data"][0]["coin"],"BTC");
    assert_eq!(read(&sol)?["data"]["time"],2);
    btc.close();eth.close();duplicate.close();
    assert_eq!(read(&sol)?["data"]["time"],3);
    sol.close();
    let rejected_url = origin.replace("http:", "ws:") + "/rejected-feed/rust";
    let rejected_book = aux_rs::LighterClient::new().subscribe(&serde_json::json!({"channel":"order_book","market_id":4}), Some(&rejected_url))?;
    let rejected_quote = aux_rs::QfexClient::new().subscribe(&serde_json::json!({"channel":"bbo","symbol":"BTC"}), Some(&rejected_url))?;
    for (venue, stream) in [("lighter",rejected_book),("qfex",rejected_quote)] {
        assert!(matches!(stream.next_json(),Err(aux_rs::Error::SessionResponse(detail)) if detail.venue == venue && detail.body["error"]["sequence"] == 9007199254740993u64));
    }
    let lighter_mux = origin.replace("http:", "ws:") + "/lighter-multiplex/rust";
    let lighter_feed = aux_rs::LighterClient::new();
    let book_spec = serde_json::json!({"channel":"order_book","market_id":4});
    let book = lighter_feed.subscribe(&book_spec, Some(&lighter_mux))?;
    let account = lighter_feed.subscribe(&serde_json::json!({"channel":"account_all","account_index":18}), Some(&lighter_mux))?;
    for generation in [1, 2] {
        assert_eq!(read(&book)?["sequence"],generation);
        assert_eq!(read(&account)?["sequence"],generation);
    }
    let late = lighter_feed.subscribe(&book_spec, Some(&lighter_mux))?;
    assert_eq!(read(&late)?["sequence"],3);
    late.close();
    book.close();
    assert_eq!(read(&account)?["sequence"],4);
    account.close();
    let external_kalshi=aux_rs::KalshiClient::new().with_origin(format!("{origin}/external-kalshi")).with_request_signer("fixture-k",|message| {
        let text=std::str::from_utf8(message).unwrap();
        assert!(text.ends_with("GET/external-kalshi/portfolio/balance") && !text.contains('?'));
        let sig=aux_rs::auth::arcus_sign(&"07".repeat(32),text)?;
        let hex=sig["signature"].as_str().unwrap();
        Ok((0..hex.len()).step_by(2).map(|i|u8::from_str_radix(&hex[i..i+2],16).unwrap()).collect())
    });
    assert_eq!(external_kalshi.get_balance(&serde_json::json!({"subaccount":1}))?["balance"],123);
    let mr:serde_json::Value=serde_json::from_str(include_str!("../metric-chain.json")).unwrap();
    let endpoint=format!("{origin}/metric-chain/rust");
    let metric_chain=aux_rs::MetricChainClient::new(1,&endpoint,&endpoint);
    let url=format!("{origin}/poly-contract/rust");
    let poly_chain=aux_rs::PolymarketChainClient::new(1,&url,&url);
    let cases:serde_json::Value=serde_json::from_str(include_str!("../polymarket-abi.json")).unwrap();
    for entry in cases.as_array().unwrap() {
        let name=entry["interface"].as_str().unwrap();
        let f=entry["function"].as_str().unwrap();
        let to="0x1111111111111111111111111111111111111111";
        assert_eq!(poly_chain.prepare_call(name,f,to,&entry["args"],"0x0")?["data"],entry["calldata"]);
        if f=="allowance" {
            assert_eq!(poly_chain.call(name,f,to,&entry["args"],"safe",None,"0x0")?.to_string(),format!("[{}]",(1u128<<88)+9));
        }
    }
    let ce=format!("{origin}/metric-contract/rust");
    let contract=aux_rs::MetricChainClient::new(1,&ce,&ce);
    let address=mr["request"]["pool"].as_str().unwrap();
    let empty_args=serde_json::json!([]);
    assert_eq!(contract.prepare_call("AnchoredPriceProvider","BPS_BASE()",address,&empty_args,"0x10")?["data"],"0x9eaa1b09");
    assert_eq!(contract.call("AnchoredPriceProvider","BPS_BASE()",address,&empty_args,"safe",None,"0x0")?,serde_json::json!([-3]));
    assert_eq!(metric_chain.oracle_stop_loss_status(&mr["request"])?,mr["expected"]);
    let state=&mr["request"];
    assert_eq!(metric_chain.liquidity_snapshot(state["pool"].as_str().unwrap(),state["dataProvider"].as_str().unwrap(),&[-1,0,1],"finalized")?,mr["snapshot"]);
    for path in mr["routes"].as_array().unwrap() { assert_eq!(metric_chain.prepare_route(&path["request"])?,path["expected"]); }
    let mut invalid=mr["routes"][0]["request"].clone();
    invalid["route"]["routes"][0]["legs"][0]["zeroForOne"]=false.into();
    assert!(metric_chain.prepare_route(&invalid).is_err());
    let eip:Vec<serde_json::Value>=serde_json::from_str(include_str!("../eip712.json")).unwrap();
    for row in eip {
        if row.get("order").is_some() {
            assert_eq!(aux_rs::polymarket::order_typed_data(&row["order"],&row["domain"])?,row["typed_data"]);
        }
        assert_eq!(aux_rs::polymarket::typed_data_hash(&row["typed_data"])?,row["digest"]);
    }
    let x:serde_json::Value=serde_json::from_str(include_str!("../extended-onboarding.json")).unwrap();
    let ext=aux_rs::ExtendedClient::new().with_origin(&origin);
    assert_eq!(ext.onboarding_payload(&x["registration"])?,x["AccountRegistration"]);
    assert_eq!(ext.key_derivation_payload(&x["registration"])?,x["AccountCreation"]);
    assert_eq!(aux_rs::ExtendedSigner::from_wallet_signature(x["wallet_signature"].as_str().unwrap().into())?.public_key(),x["public_key"]);
    let siwe:serde_json::Value=serde_json::from_str(include_str!("../paradex-siwe.json")).unwrap();
    let mut pd_auth=aux_rs::ParadexClient::new().with_origin(&origin).with_signer("0x1234","PRIVATE_SN_PARACLEAR_TESTNET",aux_rs::ParadexSigner::Evm(siwe["key"].as_str().unwrap().into()));
    assert_eq!(pd_auth.evm_account_address(siwe["address"].as_str().unwrap(),siwe["class_hash"].as_str().unwrap())?,siwe["account"]);
    pd_auth.authenticate()?;
    let opts=serde_json::json!({});
    let derive=aux_rs::DeriveClient::new().open_public_rpc(Some(&(rpc_url.clone()+"derive-public")),&opts)?;
    let denied=derive.subscribe_channels(&serde_json::json!({"channels":["rejection-fixture"]}));
    assert!(matches!(denied,Err(aux_rs::Error::SessionResponse(detail)) if detail.venue=="derive" && detail.body["error"]["data"]["seq"]==9007199254740993u64));
    assert!(derive.private_cancel(&opts).is_err());
    derive.unsubscribe_channels(&serde_json::json!({"channels":[]}))?;
    derive.close();
    let proxy="0x4a62316623ad457F02cDC5D997deD67a383EC569";
    let perps_rpc=aux_rs::PolymarketPerpsClient::new().with_perps_auth(aux_rs::PerpsAuth{proxy:Some(proxy.into()),secret:Some("fixture".into()),..Default::default()}).open_rpc(Some(&(rpc_url.clone()+"perps")),&opts)?;
    assert_eq!(perps_rpc.ping()?["data"],"ping");perps_rpc.close();
    let rfq_rpc=aux_rs::PolymarketClient::new().with_clob_credentials(aux_rs::ClobCredentials{address:proxy.into(),key:"key".into(),secret:"c2VjcmV0".into(),passphrase:"pass".into()}).open_rpc(Some(&(rpc_url.clone()+"rfq")),&opts)?;
    assert_eq!(rfq_rpc.cancel_quote(&serde_json::json!({"rfq_id":"r","quote_id":"q"}))?["rfq_id"],"r");
    assert_eq!(rfq_rpc.next_event()?.unwrap()["rfq_id"],"other");rfq_rpc.close();
    let token="fixture.eyJleHAiOjQxMDI0NDQ4MDB9.fixture";
    let pd=aux_rs::ParadexClient::new().with_bearer_token(token,false).with_signer("0x1234","PRIVATE_SN_PARACLEAR_TESTNET",aux_rs::ParadexSigner::Stark("0x1".into()));
    let orders_ch=serde_json::json!({"type":"orders","market":"ETH-USD-PERP"});
    let pd_session=pd.open_rpc(Some(&(rpc_url.clone()+"paradex")),&serde_json::json!({}))?;
    assert_eq!(pd_session.subscribe_channel(&orders_ch)?["ok"],"subscribe");
    assert_eq!(pd_session.next_event()?.unwrap()["params"]["seq"].as_u64(),Some(9007199254740993));
    pd_session.unsubscribe_channel(&orders_ch)?;
    let order=serde_json::json!({"market":"ETH-USD-PERP","side":"BUY","type":"LIMIT","size":"0.12345678","price":"3100.25","signature_timestamp":1793710000123i64});
    pd_session.create_order(&order)?;
    pd_session.create_batch(std::slice::from_ref(&order))?;
    pd_session.modify_order("123456",&order)?;
    pd_session.cancel_order("order")?;
    pd_session.cancel_by_client_id("client","ETH")?;
    pd_session.cancel_all(None)?;
    pd_session.cancel_batch(&["a".into(),"b".into()])?;
    pd_session.set_cancel_on_disconnect(true)?;
    pd_session.modify_subscription(&serde_json::json!({"channel":"orders","add":["BTC"]}))?;
    pd_session.close();
    let ro=aux_rs::ParadexClient::new().with_bearer_token(token,true).open_rpc(Some(&(rpc_url.clone()+"paradex/readonly")),&serde_json::json!({}))?;
    assert!(ro.cancel_order("forbidden").is_err());
    assert!(ro.subscribe_channel(&serde_json::json!({"type":"trades","market":"BTC"})).is_err());
    ro.subscribe_channel(&orders_ch)?;ro.unsubscribe_channel(&orders_ch)?;ro.close();
    let rpc_opts=serde_json::json!({"timeout_ms":80});
    for session in [
        aux_rs::KalshiClient::new().with_credentials("fixture-k","07".repeat(32)).open_rpc(Some(&(rpc_url.clone()+"kalshi")),&rpc_opts)?,
        aux_rs::KalshiPerpsClient::new().with_credentials("fixture-k","07".repeat(32)).open_rpc(Some(&(rpc_url.clone()+"kalshi_perps")),&rpc_opts)?
    ] {
        assert!(matches!(session.subscribe(&serde_json::json!({"step":1})), Err(aux_rs::Error::PendingOutcome)));
        assert_eq!(session.pending()?.unwrap()["method"],"subscribe");
        assert!(matches!(session.unsubscribe(&serde_json::json!({"step":2})), Err(aux_rs::Error::PendingOutcome)));
        assert_eq!(session.next_event()?.unwrap()["value"].as_u64(),Some(9007199254740993));
        std::thread::sleep(std::time::Duration::from_millis(300));
        assert_eq!(session.reconcile()?["ack"],"subscribe");
        assert!(session.pending()?.is_none());
        assert_eq!(session.list_subscriptions(&serde_json::json!({"step":2}))?["id"],"2");
        assert_eq!(session.update_subscription(&serde_json::json!({"step":3}))?["id"],"3");
        assert_eq!(session.unsubscribe(&serde_json::json!({"step":4}))?["id"],"4");
        session.close(); session.close();
    }
    let permits:Vec<serde_json::Value>=serde_json::from_str(include_str!("../pascal-permits.json")).unwrap();
    let owner=permits[0]["auth"]["owner"].as_str().unwrap();
    let local=aux_rs::PascalClient::new().with_permit(aux_rs::PascalPermit{
        owner:owner.into(), seed:"08".repeat(32), deployment_id:3,recv_window_ms:5000
    });
    let remote=|row: &serde_json::Value| {
      let seed = row["auth"]["seed"].as_str().unwrap().to_owned();
      aux_rs::PascalClient::new().with_external_permit_signer(owner,row["signer"].as_str().unwrap(),3,5000,move |message|{
        let args=serde_json::json!({"operation":"solana_sign","key":seed,"message":message});
        let proof=aux_rs::native::invoke(59,args.to_string().as_bytes())?;
        serde_json::from_str(&proof).map_err(|_|aux_rs::Error::Decode("fixture signature"))
      })
    };
    for row in &permits {
        let permit=remote(row).sign_permit_at(row["operation"].as_str().unwrap(),&row["body"],1731536000000)?;
        assert_eq!(permit["auth"]["signature"],row["signature"]);
    }
    let mut wrong_key=permits[0].clone();
    wrong_key["auth"]["seed"]="07".repeat(32).into();
    assert!(remote(&wrong_key).sign_permit_at("place_orders",&permits[0]["body"],1731536000000).is_err());
    for client in [local,remote(&permits[0])] {
        let rpc=client.open_rpc(Some(&format!("{rpc_url}pascal")),&serde_json::json!({}))?;
        assert_eq!(rpc.ping()?["ack"],"ping");
        assert_eq!(rpc.next_event()?.unwrap()["event"],"queued");
        assert_eq!(rpc.place_orders(&permits[0]["body"])?["ack"],"place");
        assert_eq!(rpc.cancel_orders(&permits[1]["body"])?["ack"],"cancel");
        assert_eq!(rpc.mass_cancel(&permits[2]["body"])?["ack"],"mass_cancel");
        rpc.close();
    }
    let destination=origin.strip_prefix("http://").unwrap();
    let policy = || FixtureTransport(destination.to_owned());
    let client=aux_rs::PolymarketClient::new().with_transport(policy());
    let sports=serde_json::json!({"service":"sports"});
    let via_policy=client.subscribe(&sports,Some("wss://policy.invalid/poly-ws/sports"))?;
    drop(client);
    assert_eq!(via_policy.next_event()?.unwrap()["score"],"1-0");
    via_policy.close(); via_policy.close();
    let denied=aux_rs::PolymarketClient::new().with_transport(policy()).subscribe(&sports,Some("wss://rejected.invalid/poly-ws/sports"));
    assert!(matches!(denied,Err(aux_rs::Error::AuthenticationRequired)));
    let quotes=aux_rs::ChainlinkCandlestickClient::new().with_transport(policy()).with_origin("https://policy.invalid").with_api_key("test-key")
        .stream_prices(&serde_json::json!({"symbol":["ETHUSD","BTCUSD"]}))?;
    assert_eq!(quotes.next_event()?.unwrap()["heartbeat"],1748525528);
    quotes.close();
    let nonce=aux_rs::LighterClient::new().with_transport(policy()).with_origin("https://policy.invalid").with_signer(300,37,3,&format!("01{}","00".repeat(39))).resync_nonce()?;
    assert_eq!(nonce,42);
    let lighter=aux_rs::LighterClient::new().with_origin(&origin).with_signer(300,37,3,&format!("01{}","00".repeat(39)));
    assert_eq!(lighter.active_orders(&serde_json::json!({"market_id":4096}))?["orders"], serde_json::json!([]));
    assert_eq!(lighter.next_nonce(&serde_json::json!({"account_index":37,"api_key_index":3}))?["nonce"],42);
    let metric = |key: &str| aux_rs::MetricClient::new().with_origin(&origin).with_api_key(key);
    let client = metric("fixture");
    let empty = serde_json::json!({});
    assert_eq!(client.chains(&empty)?, metric("last").chains(&empty)?);
    assert!(matches!(metric("fail").chains(&empty), Err(aux_rs::Error::ExchangeResponse(d)) if d.status == 429 && d.venue == "metric" && d.body == "fixture rate limit"));
    for row in serde_json::from_str::<Vec<serde_json::Value>>(include_str!("../data-http.json")).unwrap() {
        if row["venue"] != "metric" || row["verb"] != "POST" { continue; }
        let params = &row["request"];
        let response = match row["operation"].as_str().unwrap() {
            "pamm_quote" => client.pamm_quote(params)?,
            "route" => client.route(params)?,
            "batch_bid_ask" => client.batch_bid_ask(params)?,
            op => panic!("unexpected operation {op}"),
        };
        assert_eq!(response,row["response"]);
    }
    let payload=lighter.prepare_transaction("cancel_order",&serde_json::json!({"Index":577305177233555425u64,"MarketIndex":4099}),
        &serde_json::json!({"expired_at":1800000600000u64}),42)?;
    assert_eq!(payload["digest"],"2c27717bcde837652d24a207ffb7bdc6b57b95c4b9d47226c9290a273f90f04badcb595107dd5b2f");
    let address="0xabcdef0000000000000000000000000000000001";
    let key="07".repeat(32);
    let rows:Vec<serde_json::Value>=serde_json::from_str(include_str!("../data-http.json")).unwrap();
    for row in rows.iter().filter(|r| r["venue"]=="arcus" && r["private"]==true) {
        let op=row["operation"].as_str().unwrap();
        let p=&row["request"];
        let proof=aux_rs::ArcusClient::signing_payload(address,2,op,p)?;
        let sig=aux_rs::auth::arcus_sign(&key,proof["payload"].as_str().unwrap())?;
        let local=aux_rs::ArcusClient::new().with_origin(&origin).with_signer(address,2,&key);
        let external=aux_rs::ArcusClient::new().with_origin(&origin).with_order_signature(address,2,proof["timestamp"].as_u64().unwrap(),sig["public_key"].as_str().unwrap(),sig["signature"].as_str().unwrap());
        for client in [local,external] {
            let reply=if op=="place_order"{client.place_order(p)?}else{client.cancel_order(p)?};
            assert_eq!(reply,row["response"]);
        }
    }
    let wsbase=origin.replacen("http:", "ws:", 1);
    let hd=aux_rs::HyperdashClient::new().with_api_key("hyperdash-key").open_stream(Some(&format!("{wsbase}/hd/stream")),&serde_json::json!({}))?;
    assert_eq!(hd.next_event()?.unwrap()["op"],"ready");
    hd.subscribe(&serde_json::json!({"channel":"trades","coin":"BTC"}))?;
    let batch=hd.next_event()?.unwrap();
    assert_eq!(batch["batch"],9007199254740993u64);
    assert!(hd.ack(&batch["sub"],&serde_json::json!("9007199254740993")).is_err());
    hd.ack(&batch["sub"],&batch["batch"])?;
    assert_eq!(hd.next_event()?.unwrap()["credits_charged"],2);
    hd.unsubscribe(&batch["sub"])?;
    assert_eq!(hd.next_event()?.unwrap()["detail"],"[redacted]");
    assert_eq!(hd.next_event()?.unwrap()["op"],"unsubscribed");
    hd.close();
    let shared_reader=|key:&str| aux_rs::ChainlinkDataStreamsClient::new().with_credentials(key,"secret").subscribe_reports(&serde_json::json!({"feedIDs":["ab".repeat(32)]}),Some(&format!("{wsbase}/shared/rust/")));
    let step=|n:u8| aux_rs::MetricClient::new().with_origin(format!("{origin}/shared/rust")).with_api_key(n.to_string()).chains(&serde_json::json!({}));
    let (one,other,two)=(shared_reader("one")?,shared_reader("one")?,shared_reader("two")?);
    assert_eq!(step(1)?,serde_json::json!({"one":1,"two":1}));
    assert_eq!(one.next_event()?,other.next_event()?);
    assert_eq!(two.next_event()?.unwrap()["report"]["fullReport"],"two");
    one.close();step(2)?;assert_eq!(other.next_event()?.unwrap()["report"]["seq"],2);
    other.close();two.close();
    let reports = aux_rs::ChainlinkDataStreamsClient::new().with_credentials("fixture","secret").subscribe_reports(
        &serde_json::json!({"feedIDs":[format!("0x{}","01".repeat(32)),"ab".repeat(32)]}),Some(&format!("{wsbase}/reports/")))?;
    assert_eq!(reports.next_event()?.unwrap()["report"]["sequence"],9007199254740993u64);
    assert!(reports.next_event().is_err());
    reports.close();
    let book = aux_rs::ArcusClient::new().subscribe(&serde_json::json!({"channel":"l2OrderbookUpdates","id":"BTC","nLevels":10}),Some(&format!("{wsbase}/arcus/ws")))?;
    assert_eq!(book.next_event()?.unwrap()["contents"],serde_json::json!({"bids":[],"asks":[]}));
    assert_eq!(book.next_event()?.unwrap()["contents"]["seq"],9007199254740993u64);
    book.close();
    let perps_ws=format!("{wsbase}/perps-ws");
    let events=lighter.subscribe_account_orders(Some(&format!("{perps_ws}/lighter/account_all_orders")))?;
    assert_eq!(events.next_event()?.unwrap()["orders"]["4"][0]["order_index"],9007199254740993u64); events.close();
    let sub=aux_rs::HyperliquidClient::new().subscribe(&serde_json::json!({"type":"l2Book","coin":"ETH"}),Some(&format!("{perps_ws}/hyperliquid/l2Book")))?;
    assert_eq!(sub.next_event()?.unwrap()["data"]["levels"][0][0]["px"],"3000.00"); sub.close();
    let reconnect=aux_rs::PythProClient::new().with_api_key("fixture").subscribe(&serde_json::json!({"channel":"real_time","symbols":["BTC/USD"],"formats":["evm"],"subscriptionId":42}),Some(&[format!("{wsbase}/pro/reconnect-rust")]))?;
    assert_eq!(reconnect.next_event()?.unwrap()["price"],9007199254740993u64);
    assert_eq!(reconnect.next_event()?.unwrap()["price"],10);
    reconnect.close();
    let action=serde_json::json!({"type":"approveAgent","agentAddress":"0x0000000000000000000000000000000000000005","agentName":"desk-test","nonce":1677777606040u64});
    assert_eq!(aux_rs::HyperliquidClient::user_signing_payload(&action,"Testnet",1)?,"0x526f10736a11efe66db2a9b57a48a963e08835b7c37c62b3daaf498948fd6961");
    let roots=aux_rs::PythProClient::new().with_api_key("fixture").stream_merkle_roots(Some(&[format!("{wsbase}/ignored")]))?;
    assert_eq!(roots.next_event()?.unwrap()["root"],"abc"); roots.close();
    let market_vectors:serde_json::Value=serde_json::from_str(include_str!("../hyperliquid-signing.json")).unwrap();
    let vector=market_vectors["l1"][0].clone();
    let order=serde_json::json!({"orders":vector["action"]["orders"],"options":{"nonce":vector["nonce"]}});
    let external=aux_rs::HyperliquidClient::new().with_origin(&origin).with_external_signer("0x14791697260e4c9a71f18484c9f997b308e59325","Mainnet",None,move|digest|{
        assert_eq!(digest,vector["digest"].as_str().unwrap());Ok(vector["signature"].clone())
    });
    assert_eq!(external.order(&order)?["status"],"ok");
    let reply=aux_rs::HyperliquidClient::new().with_origin(&origin).with_signer(
        "0x0123456789012345678901234567890123456789012345678901234567890123","Mainnet",None).order(
        &serde_json::json!({"orders":[{"s":"0.0147","a":4,"t":{"limit":{"tif":"Ioc"}},"r":false,"p":"1670.1","b":true}],"options":{"nonce":1677777606040u64}}))?;
    assert_eq!(reply["response"]["data"]["statuses"][1]["error"],"fixture rejection");
    let replay = aux_rs::TardisClient::new().with_origin(&origin).with_api_key("fixture").replay(
        &serde_json::json!({"exchange":"bit/mex","from":"2024-02-29T23:59:00Z","to":"2024-03-01T00:02:00Z",
        "filters":[{"channel":"trade","symbols":["BTC/USD"]}]}))?;
    let minutes = replay.read_batch(5)?;
    assert_eq!(minutes.len(),3);
    assert_eq!(minutes[0]["message"]["n"],9007199254740993u64);
    assert_eq!(minutes[1]["type"],"disconnect");
    assert_eq!(minutes[2]["localTimestamp"],"1702");
    replay.close();
    let machine = aux_rs::TardisMachineClient::new().with_origin(format!("{origin}/machine")).ws_replay_normalized(
        &serde_json::json!({"exchange":"bit/mex","symbols":["BTC/USD"]}))?;
    machine.send(&serde_json::json!({"ping":true}))?;
    assert_eq!(machine.read_batch(2)?[0]["seq"],9007199254740993u64);
    machine.close();
    let ws = origin.replacen("http:", "ws:", 1);
    let market = aux_rs::PolymarketClient::new().subscribe(
        &serde_json::json!({"service":"sports"}), Some(&format!("{ws}/poly-ws/sports")))?;
    assert_eq!(market.next_event()?.unwrap()["score"], "1-0");
    assert!(market.next_event()?.is_none());
    market.close();
    let seed="000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
    let rows = |source: &str| serde_json::from_str::<Vec<serde_json::Value>>(source).unwrap();
    for row in rows(include_str!("../monaco-ws.json")) {
        let kind = row["params"]["type"].as_str().unwrap();
        let url = format!("{wsbase}/monaco-ws/{kind}");
        let client = aux_rs::MonacoClient::new().with_session(&origin, seed, 4102444800, "legacy")
            .with_session_ws_origin(&url);
        let stream = client.subscribe(&row["params"], Some(&url))?;
        let event = stream.next_event()?.unwrap();
        assert_eq!(event["channel"], row["channel"].as_str().unwrap().replace('*', "ETH"));
        assert_eq!(event["data"]["book_seq"], 9007199254740993u64);
        if matches!(kind, "depth" | "positions") {
            assert!(matches!(stream.next_event(), Err(aux_rs::Error::StreamGap)));
        }
        if kind == "positions" {
            assert_eq!(stream.next_event()?.unwrap()["code"], "SNAPSHOT_UNAVAILABLE");
        }
        stream.close();
    }
    for row in rows(include_str!("../perps-ws.json")) {
        let venue = row["venue"].as_str().unwrap();
        let url = format!("{perps_ws}/{venue}/{}", row["name"].as_str().unwrap());
        let stream = if venue == "hyperliquid" {
            aux_rs::HyperliquidClient::new().subscribe(&row["params"], Some(&url))?
        } else { lighter.subscribe(&row["params"], Some(&url))? };
        assert_eq!(stream.next_event()?.unwrap(), row["event"]);
        assert!(stream.next_event()?.is_none());
    }
    let address = format!("0x{}", "21".repeat(20));
    for row in rows(include_str!("../polymarket-ws.json")) {
        let service = row["service"].as_str().unwrap();
        let url = format!("{wsbase}/poly-ws/{service}");
        let options = serde_json::json!({"service": service, "params":row["params"]});
        let stream = if service == "perps" {
            aux_rs::PolymarketPerpsClient::new().with_perps_auth(aux_rs::PerpsAuth {
                proxy: Some(address.clone()), secret: Some("fixture-secret".into()), ..Default::default()
            }).subscribe(&options, Some(&url))?
        } else {
            aux_rs::PolymarketClient::new().with_clob_credentials(aux_rs::ClobCredentials {
                address: address.clone(), key:"fixture".into(), secret:"c2VjcmV0".into(), passphrase:"fixture-pass".into()
            }).subscribe(&options, Some(&url))?
        };
        assert_eq!(stream.next_event()?.unwrap(), row["event"]);
        assert!(stream.next_event()?.is_none());
    }
    let options=serde_json::json!({"client_id":"login","address":format!("0x{}","21".repeat(20)),"seed":seed,"chain_id":1329});
    let logged=aux_rs::MonacoClient::new().with_origin(&origin).login(&options,|message| {
        assert_eq!(message,"Authorize 03a107bff3ce10be1d70dd18e74bc09967e4d6309ba50d5f1ddc8664125531b8"); Ok("0xabcd".into())
    })?;
    let logged=logged.with_session_ws_origin(&format!("{wsbase}/monaco-ws/hold"));
    let active=logged.subscribe(&serde_json::json!({"type":"balances"}),None)?;
    assert_eq!(active.next_event()?.unwrap()["data"]["book_seq"],9007199254740993u64);
    let waiting=std::thread::spawn(move ||active.next_event());
    let clone=logged.clone();
    assert_eq!(logged.refresh_session(&serde_json::json!({}))?["expiresAt"],4102444801u64);
    clone.revoke_session(&serde_json::json!({}))?;
    assert!(matches!(waiting.join().unwrap(),Err(aux_rs::Error::AuthenticationRequired)));
    assert!(matches!(logged.balances(&serde_json::json!({})),Err(aux_rs::Error::AuthenticationRequired)));
    let monaco = aux_rs::MonacoClient::new().with_session(&origin,
        "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f", 4102444800, "rfc9421");
    let lost = monaco.clone().with_transport(|_: OutboundRequest| Err(aux_rs::Error::Network));
    assert!(matches!(lost.cancel_all(&serde_json::json!({})), Err(aux_rs::Error::PendingOutcome)));
    let balances = monaco.balances(&serde_json::json!({"pageSize": 350, "pageToken": "a/b ?"}))?;
    assert_eq!(balances, serde_json::json!({"data": [{"available": "1.0000000000000000001"}]}));
    let url = format!("{}/trade", origin.replacen("http:", "ws:", 1));
    let trading = aux_rs::QfexClient::new().with_credentials("fixture", "secret")
        .trading_session(&serde_json::json!({"timeout_ms":180}), Some(&url))?;
    trading.subscribe_account()?;
    let order = serde_json::json!({"symbol":"BTC","side":"BUY","order_type":"LIMIT",
        "order_time_in_force":"GTC","quantity":0.5,"price":123,"client_order_id":"cid"});
    assert_eq!(trading.add_order(&order)?["order_id"], "abc");
    assert_eq!(trading.next_event()?.unwrap()["total"],9007199254740993u64);
    let cancel = serde_json::json!({"symbol":"BTC","order_id":"abc","cancel_order_id_type":"order_id"});
    assert!(matches!(trading.cancel_order(&cancel),Err(aux_rs::Error::PendingOutcome)));
    assert!(matches!(trading.add_order(&order),Err(aux_rs::Error::PendingOutcome)));
    assert_eq!(trading.next_event()?.unwrap()["order_response"]["status"],"canceled");
    assert_eq!(trading.user_orders(&serde_json::json!({"limit":100,"offset":0}))?,serde_json::json!({"orders":[],"twaps":[]}));
    trading.close();
    trading.close();
    let auth: serde_json::Value = serde_json::from_str(include_str!("../auth.json")).unwrap();
    let key = auth["databento"]["key"].as_str().unwrap();
    let history = aux_rs::HistoricalClient::new().with_origin(&origin).with_api_key(key);
    let params = serde_json::json!({"symbols":["ETH","BTC"],"encoding":"csv","compression":"zstd"});
    let raw = history.timeseries(&params)?;
    let mut payload = Vec::new();
    while let Some(chunk) = raw.next_chunk()? { payload.extend(chunk); }
    assert_eq!(payload, include_bytes!("../reference-lines.zst").as_slice());
    let url = format!("{origin}/files/batch?sig=a%2Fb");
    let dest = std::path::Path::new("batch.dbn.zst");
    assert_eq!(history.download_to(&url, dest)?, payload.len() as u64);
    assert_eq!(std::fs::read(dest).unwrap(), payload);
    assert!(history.download_to(&url, dest).is_err());
    std::fs::remove_file(dest).unwrap();
    let events = history.get_range(&params)?;
    assert_eq!(events.read_batch(1)?[0]["metadata"]["dataset"],"GLBX.MDP3");
    events.close();
    let reference = aux_rs::ReferenceClient::new().with_origin(&origin).with_api_key(key);
    let rows = reference.security_master_get_last(&params)?.read_batch(3)?;
    assert_eq!(rows[0]["id"],9007199254740993u64);
    let gateway = std::env::var("AUX_DBN_GATEWAY").unwrap();
    let impatient = aux_rs::databento::LiveClient::new(auth["databento"]["key"].as_str().unwrap()).with_timeouts(400, 100)?;
    assert!(matches!(impatient.connect(&serde_json::json!({"dataset":"IDLE","gateway":gateway})), Err(aux_rs::Error::Network)));
    let live = aux_rs::databento::LiveClient::new(auth["databento"]["key"].as_str().unwrap())
        .connect(&serde_json::json!({"dataset":"DBN","gateway":gateway}))?;
    assert_eq!(live.session_id, "fixture-session");
    live.subscribe(&serde_json::json!({"schema":"mbo","symbols":["ETH"]}))?;
    live.start_session()?;
    assert_eq!(live.read_batch(1)?[0]["metadata"], live.metadata()?);
    assert_eq!(live.read_batch(24)?.len(), 23);
    live.close();
    let rows: Vec<serde_json::Value> = serde_json::from_str(include_str!("../text-http.json")).unwrap();
    for row in rows {
        let response = aux_rs::http::execute_with_key(
            row["venue"].as_str().unwrap(), row["operation"].as_str().unwrap(), &row["request"],
            Some(&origin), Some("test-key")
        )?;
        assert_eq!(response, row["expected"]);
    }
    let stream = aux_rs::ChainlinkCandlestickClient::new().with_origin(&origin).with_api_key("test-key")
        .stream_prices(&serde_json::json!({"symbol":["ETHUSD","BTCUSD"]}))?;
    assert_eq!(stream.next_event()?.unwrap()["heartbeat"], 1748525528);
    assert_eq!(stream.next_event()?.unwrap()["p"].to_string().to_lowercase(), "1.0000000000000000001e+21");
    assert!(stream.next_event()?.is_none());
    stream.close();

    let evm: serde_json::Value = serde_json::from_str(include_str!("../evm-signing.json")).unwrap();
    let chain = aux_rs::EvmChainClient::new(1, format!("{origin}/read"), format!("{origin}/write"));
    assert_eq!(chain.submit_raw(evm[0]["signed"].as_str().unwrap())?, evm[0]["hash"]);
    let tx = &evm[0];
    assert_eq!(chain.send_transaction_with_signer(&tx["prepared"],tx["address"].as_str().unwrap(),
        |digest| aux_rs::evm::sign_digest(tx["key"].as_str().unwrap(),digest))?,tx["hash"]);
    let sol: serde_json::Value = serde_json::from_str(include_str!("../pascal-solana.json")).unwrap();
    let pascal = aux_rs::PascalChainClient::new(&format!("{origin}/solana"), sol["mint"].as_str().unwrap(), 6)?;
    assert_eq!(pascal.block_height()?, 987654321);
    Ok(())
}
