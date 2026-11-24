use aux_zig::{
    chainlink,
    contracts::{self, Venue},
};
use serde_json::{Value, json};

fn rows(text: &str) -> Vec<Value> {
    serde_json::from_str(text).unwrap()
}

// The Solidity fixtures express integers as decimal strings so other runtimes
// can read them exactly. Accept that representation when comparing ABI output.
fn compare(actual: &Value, expected: &Value) {
    match (actual, expected) {
        (Value::Number(a), Value::String(b)) => assert_eq!(a.to_string(), *b),
        (Value::Array(a), Value::Array(b)) => {
            assert_eq!(a.len(), b.len());
            for (a, b) in a.iter().zip(b) {
                compare(a, b);
            }
        }
        _ => assert_eq!(actual, expected),
    }
}

#[test]
fn metric_contract_vectors() {
    for row in rows(include_str!("../../../tests/metric-abi.json")) {
        let c = row["contract"].as_str().unwrap();
        let f = row["signature"].as_str().unwrap();
        match row["kind"].as_str().unwrap() {
            "function" => {
                let data =
                    contracts::encode_contract_call(Venue::Metric, c, f, &row["args"]).unwrap();
                assert_eq!(json!(data), row["input"], "{c} {f}");
                compare(
                    &contracts::decode_contract_input(Venue::Metric, c, f, &data).unwrap(),
                    &row["args"],
                );
                compare(
                    &contracts::decode_contract_output(
                        Venue::Metric,
                        c,
                        f,
                        row["output"].as_str().unwrap(),
                    )
                    .unwrap(),
                    &row["outputArgs"],
                );
            }
            "error" => {
                let v = contracts::decode_metric_error(c, row["input"].as_str().unwrap()).unwrap();
                assert_eq!(v["name"], f.split_once('(').unwrap().0);
                compare(&v["arguments"], &row["args"]);
            }
            "event" => {
                let topics: Vec<String> = serde_json::from_value(row["topics"].clone()).unwrap();
                let v =
                    contracts::decode_metric_event(c, f, &topics, row["data"].as_str().unwrap())
                        .unwrap();
                assert_eq!(v["name"], f.split_once('(').unwrap().0);
                compare(&v["indexed"], &row["indexed"]);
                compare(&v["body"], &row["body"]);
            }
            other => panic!("unrecognized fixture {other}"),
        }
    }
}

#[test]
fn polymarket_contract_vectors() {
    for row in rows(include_str!("../../../tests/polymarket-abi.json")) {
        let c = row["interface"].as_str().unwrap();
        let f = row["function"].as_str().unwrap();
        let data = contracts::encode_contract_call(Venue::Polymarket, c, f, &row["args"]).unwrap();
        assert_eq!(json!(data), row["calldata"], "{c} {f}");
        compare(
            &contracts::decode_contract_input(Venue::Polymarket, c, f, &data).unwrap(),
            &row["args"],
        );
    }
}

#[test]
fn chainlink_reports() {
    for row in rows(include_str!("../../../tests/chainlink-reports.json")) {
        let hex = row["full_report"].as_str().unwrap();
        let report = chainlink::decode_report(hex).unwrap();
        assert_eq!(report["full_report"], row["full_report"]);
        assert_eq!(report["report_body"], row["report_body"]);
        assert_eq!(report["version"], row["version"]);
        if row["version"] == 42 {
            assert_eq!(report["kind"], "unknown");
        } else {
            assert_eq!(report["kind"], "known");
            assert_eq!(
                report["fields"][row["key_field"].as_str().unwrap()],
                row["key_value"]
            );
        }
        assert!(chainlink::decode_report(&hex[..hex.len() - 2]).is_err());
        assert!(chainlink::decode_report(&(hex.to_owned() + "00")).is_err());
    }
}

#[test]
fn exchange_http_contracts() {
    use aux_zig::http;
    for row in rows(include_str!("../../../tests/http.json"))
        .into_iter()
        .chain(rows(include_str!("../../../tests/prediction-http.json")))
    {
        let venue = row["venue"].as_str().unwrap();
        let name = row["operation"].as_str().unwrap();
        let request = http::prepare(venue, name, &row["request"])
            .unwrap_or_else(|err| panic!("{venue}.{name}: {err}"));
        assert_eq!(request.method, row["verb"], "{venue}.{name}");
        assert_eq!(request.path, row["route"], "{venue}.{name}");
        assert_eq!(request.body, row["body"], "{venue}.{name}");
        assert_eq!(request.authenticated, row["private"], "{venue}.{name}");
        if let Some(headers) = row["headers"].as_object() {
            for (key, value) in headers {
                assert_eq!(request.headers[key], *value, "{venue}.{name}");
            }
        }
        let wire = if venue == "derive" {
            json!({"jsonrpc":"2.0","id":1,"result":row["response"]})
        } else {
            row["response"].clone()
        };
        assert_eq!(
            http::decode(venue, 200, &wire.to_string()).unwrap(),
            row["response"],
            "{venue}.{name}"
        );
    }
}

#[test]
fn native_http_and_caller_transport() {
    use aux_zig::{Error, http};
    use std::{
        io::{BufRead, BufReader, Write},
        net::TcpListener,
        thread,
    };
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = format!("http://{}", server.local_addr().unwrap());
    let task = thread::spawn(move || {
        let (mut socket, _) = server.accept().unwrap();
        let mut reader = BufReader::new(socket.try_clone().unwrap());
        let mut lines = Vec::new();
        loop {
            let mut line = String::new();
            assert!(reader.read_line(&mut line).unwrap() > 0);
            if line == "\r\n" {
                break;
            }
            lines.push(line);
        }
        assert_eq!(
            lines[0],
            "GET /api/v1/info/assets?asset=a%20b&asset=c%2Fd HTTP/1.1\r\n"
        );
        let body = r#"{"data":123456789012345678901234567890.123456789}"#;
        write!(
            socket,
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        )
        .unwrap();
    });
    let result = http::execute(
        "extended",
        "get_assets",
        &json!({"asset":["a b","c/d"]}),
        Some(&origin),
    )
    .unwrap();
    assert_eq!(
        result["data"].to_string(),
        "123456789012345678901234567890.123456789"
    );
    task.join().unwrap();
    assert!(matches!(
        http::execute(
            "derive",
            "private_get_all_portfolios",
            &json!({}),
            Some(&origin)
        ),
        Err(Error::AuthenticationRequired)
    ));
    assert!(matches!(
        http::decode("derive", 200, r#"{"error":{"code":42}}"#),
        Err(Error::ExchangeResponse(ref detail)) if detail.status == 200 && detail.body["error"]["code"] == 42
    ));
    assert!(http::prepare("extended", "get_asset_price", &json!({"asset":".."})).is_err());
    assert!(http::prepare("phoenix", "unknown", &json!({})).is_err());
    assert!(http::decode("extended", 200, "{").is_err());
}

#[test]
fn polymarket_contract_output() {
    let data = format!("0x{:064x}", 9007199254740993_u64);
    let value =
        contracts::decode_contract_output(Venue::Polymarket, "IERC20", "allowance", &data).unwrap();
    assert_eq!(value, json!([9007199254740993_u64]));
    assert!(
        contracts::decode_contract_output(Venue::Polymarket, "IERC20", "allowance", "0x00")
            .is_err()
    );
}

// Compare form query pairs independent of their order. Decimals remain raw JSON
// tokens in the Birdeye vectors until they cross the SDK boundary.
fn unescape(value: &str) -> String {
    let input = value.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < input.len() {
        match input[i] {
            b'%' => {
                out.push(u8::from_str_radix(&value[i + 1..i + 3], 16).unwrap());
                i += 3;
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    String::from_utf8(out).unwrap()
}

#[test]
fn data_http_contracts() {
    for row in rows(include_str!("../../../tests/data-http.json")) {
        let venue = row["venue"].as_str().unwrap();
        let name = row["operation"].as_str().unwrap();
        let params = row["request_json"]
            .as_str()
            .map(|text| serde_json::from_str(text).unwrap())
            .unwrap_or_else(|| row["request"].clone());
        let req = aux_zig::http::prepare(venue, name, &params)
            .unwrap_or_else(|err| panic!("{venue}.{name}: {err}"));
        let (path, query) = req.path.split_once('?').unwrap_or((&req.path, ""));
        assert_eq!(path, row["path"], "{venue}.{name}");
        let mut pairs = json!({});
        for item in query.split('&').filter(|s| !s.is_empty()) {
            let (k, v) = item.split_once('=').unwrap();
            let key = unescape(k);
            if row["query"][&key].is_array() {
                if pairs[&key].is_null() {
                    pairs[&key] = json!([]);
                }
                pairs[&key].as_array_mut().unwrap().push(json!(unescape(v)));
            } else {
                assert!(pairs.get(&key).is_none());
                pairs[key] = json!(unescape(v));
            }
        }
        assert_eq!(pairs, row["query"], "{venue}.{name}");
        assert_eq!(req.method, row["verb"]);
        assert_eq!(req.authenticated, row["private"]);
        let body = row["body_json"]
            .as_str()
            .map(|s| serde_json::from_str(s).unwrap())
            .unwrap_or_else(|| row["body"].clone());
        assert_eq!(req.body, body, "{venue}.{name}");
        for (k, v) in row["headers"].as_object().unwrap() {
            assert_eq!(req.headers[k], *v, "{venue}.{name}");
        }
        let text = row["response_json"]
            .as_str()
            .map(str::to_string)
            .unwrap_or_else(|| row["response"].to_string());
        let status = row["status"].as_u64().unwrap_or(200) as u16;
        if !(200..300).contains(&status) {
            assert!(matches!(
                aux_zig::http::decode(venue, status, &text),
                Err(aux_zig::Error::ExchangeResponse(ref detail)) if detail.status == status && detail.venue == venue
            ));
            continue;
        }
        assert_eq!(
            aux_zig::http::decode(venue, status, &text)
                .unwrap_or_else(|err| panic!("{venue}.{name}: {err}")),
            serde_json::from_str::<Value>(&text).unwrap()
        );
    }
}

#[test]
fn independent_auth_vectors() {
    use aux_zig::auth::*;
    let data: Value = serde_json::from_str(include_str!("../../../tests/auth.json")).unwrap();
    let f = &data["qfex"];
    assert_eq!(
        qfex_signature(
            f["secret"].as_str().unwrap(),
            f["nonce"].as_str().unwrap(),
            f["timestamp"].as_u64().unwrap()
        )
        .unwrap(),
        f["hmac"]
    );
    assert!(qfex_signature("test", "!", 0).is_err());
    let f = &data["databento"];
    assert_eq!(
        databento_cram(f["key"].as_str().unwrap(), f["challenge"].as_str().unwrap()).unwrap(),
        f["response"]
    );
    let f = &data["arcus"];
    assert_eq!(
        arcus_sign(f["seed"].as_str().unwrap(), f["payload"].as_str().unwrap()).unwrap(),
        json!({"public_key":f["public_key"], "signature":f["signature"]})
    );
    for f in data["monaco"].as_array().unwrap() {
        let result = monaco_sign(
            f["privateKey"].as_str().unwrap(),
            f["method"].as_str().unwrap(),
            f["target"].as_str().unwrap(),
            f["body"].as_str().unwrap(),
            f["created"].as_u64().unwrap(),
        )
        .unwrap();
        assert_eq!(
            result,
            json!({"Content-Digest":f["contentDigest"],"Signature-Input":f["signatureInput"],"Signature":f["signature"]})
        );
    }
}

#[test]
fn signed_transactions() {
    for row in rows(include_str!("../../../tests/evm-signing.json")) {
        let signed =
            aux_zig::evm::sign_transaction(row["key"].as_str().unwrap(), &row["prepared"]).unwrap();
        assert_eq!(
            signed,
            json!({"address":row["address"],"signed":row["signed"],"hash":row["hash"]})
        );
    }
}

#[test]
fn sse_split_records_and_cleanup() {
    let chunks = b"\xef\xbb\xbf:beat\r\ndata: first\r\ndata: second\r\n\r\ndata: last\n\n";
    for offset in 0..=chunks.len() {
        let mut stream = aux_zig::hermes::EventDecoder::new(64).unwrap();
        let mut events = stream.feed(&chunks[..offset]).unwrap();
        events.extend(stream.feed(&chunks[offset..]).unwrap());
        assert_eq!(events, ["first\nsecond", "last"]);
        stream.finish().unwrap();
        stream.close();
        assert!(stream.feed(b"").is_err());
    }
}

#[test]
fn hermes_stream_network_and_cancel() {
    use std::{
        io::{BufRead, BufReader, Read, Write},
        net::TcpListener,
        sync::mpsc,
        thread,
        time::Duration,
    };
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let origin = format!("http://{}", server.local_addr().unwrap());
    let id = "42".repeat(32);
    let expected = format!("GET /v2/updates/price/stream?ids%5B%5D={id} HTTP/1.1\r\n");
    let responder = thread::spawn(move || {
        let (mut sock, _) = server.accept().unwrap();
        sock.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let mut reader = BufReader::new(sock.try_clone().unwrap());
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        assert_eq!(line, expected);
        loop {
            line.clear();
            reader.read_line(&mut line).unwrap();
            if line == "\r\n" {
                break;
            }
        }
        sock.write_all(
            b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\n\r\ndata: {\"price\":",
        )
        .unwrap();
        sock.write_all(b"12345678901234567891.01}\r\n\r\n").unwrap();
        assert_eq!(sock.read(&mut [0]).unwrap(), 0);
    });
    let stream = aux_zig::PythHermesClient::new()
        .with_origin(origin)
        .stream_price_updates(&json!({"ids":[id]}))
        .unwrap();
    assert_eq!(
        stream.next_event().unwrap().unwrap()["price"].to_string(),
        "12345678901234567891.01"
    );
    let reader = stream.clone();
    let (send, recv) = mpsc::channel();
    thread::spawn(move || {
        send.send(reader.next_event()).unwrap();
    });
    stream.close();
    stream.close();
    assert!(
        recv.recv_timeout(Duration::from_secs(2))
            .unwrap()
            .unwrap()
            .is_none()
    );
    responder.join().unwrap();
}

#[test]
fn dbn_v3_records() {
    let bytes = include_bytes!("../../../tests/schema-records.dbn");
    let mut offset = 8 + u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    for row in rows(include_str!("../../../tests/schema-records.json")) {
        let size = usize::from(bytes[offset]) * 4;
        let raw = &bytes[offset..offset + size];
        assert_eq!(
            aux_zig::databento::decode_record(raw, false).unwrap(),
            row["record"],
            "{}",
            row["schema"]
        );
        assert!(aux_zig::databento::decode_record(&raw[..size - 1], false).is_err());
        offset += size;
    }
    assert_eq!(offset, bytes.len());
    let bytes = include_bytes!("../../../tests/ts-out.dbn");
    let offset = 8 + u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
    assert_eq!(
        aux_zig::databento::decode_record(&bytes[offset..], true).unwrap()["ts_out"],
        "1704067200000000999"
    );
}

#[test]
fn paradex_sbe_templates_and_truncation() {
    let fixture: Value =
        serde_json::from_str(include_str!("../../../tests/paradex-sbe.json")).unwrap();
    for f in fixture["frames"].as_array().unwrap() {
        let hex = f["hex"].as_str().unwrap();
        let bytes: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
            .collect();
        let frame = aux_zig::paradex::decode_sbe(&bytes).unwrap();
        assert_eq!(frame["data"], f["native"], "{}", f["name"]);
        assert_eq!(frame["template"], f["template"].to_string());
        for end in 0..bytes.len() {
            assert!(aux_zig::paradex::decode_sbe(&bytes[..end]).is_err());
        }
        for index in [2, 4, 6] {
            let mut unsupported = bytes.clone();
            unsupported[index] = 99;
            assert!(aux_zig::paradex::decode_sbe(&unsupported).is_err());
        }
        let mut extended = bytes.clone();
        let block = u16::from_le_bytes(bytes[..2].try_into().unwrap());
        extended.splice(8 + usize::from(block)..8 + usize::from(block), [0; 8]);
        extended[..2].copy_from_slice(&(block + 8).to_le_bytes());
        assert_eq!(
            aux_zig::paradex::decode_sbe(&extended).unwrap()["data"],
            f["native"]
        );
    }
}

#[test]
fn named_clients_use_application_transport_for_tls() {
    use aux_zig::{
        BirdeyeClient, PythHermesClient,
        http::{OutboundRequest, TransportResponse},
    };
    let client = BirdeyeClient::new()
        .with_api_key("fixture")
        .with_origin("https://internal.example/test")
        .with_transport(|req: OutboundRequest| {
            assert_eq!(
                req.url,
                "https://internal.example/test/defi/price?address=Test"
            );
            assert_eq!(req.method, "GET");
            assert_eq!(req.headers["X-API-KEY"], "fixture");
            assert!(req.body.is_none());
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: r#"{"success":true,"data":12345678901234567890.125}"#.into(),
            })
        });
    assert_eq!(
        client.price(&json!({"address":"Test"})).unwrap()["data"].to_string(),
        "12345678901234567890.125"
    );
    let hermes = PythHermesClient::new().with_transport(|_: OutboundRequest| {
        Ok(TransportResponse {
            status: 200,
            headers: vec![("retry-after".into(), "9".into())],
            body: r#"{"parsed":[]}"#.into(),
        })
    });
    assert_eq!(
        hermes
            .latest_price_updates(&json!({"ids":["ab".repeat(32)]}))
            .unwrap(),
        json!({"status":200,"headers":[["retry-after","9"]],"data":{"parsed":[]}})
    );
    assert!(
        hermes
            .stream_price_updates(&json!({"ids":["ab".repeat(32)]}))
            .is_err()
    );
}

#[test]
fn exchange_authentication_dispatch() {
    use aux_zig::http::{OutboundRequest, TransportResponse};
    let success = || TransportResponse {
        status: 200,
        headers: vec![],
        body: "{}".into(),
    };
    let f: Value = serde_json::from_str(include_str!("../../../tests/auth.json")).unwrap();
    let f = &f["kalshi"];
    let seed = f["seed"].as_str().unwrap();
    assert_eq!(
        aux_zig::auth::kalshi_headers(
            "fixture-k",
            seed,
            "GET",
            f["path"].as_str().unwrap(),
            f["timestamp"].as_u64().unwrap()
        )
        .unwrap()["KALSHI-ACCESS-SIGNATURE"],
        f["signature"]
    );
    let key = seed.to_string();
    aux_zig::KalshiClient::new()
        .with_credentials("fixture-k", seed)
        .with_transport(move |r: OutboundRequest| {
            assert_eq!(
                r.url,
                "https://external-api.kalshi.com/trade-api/v2/portfolio/balance?subaccount=1"
            );
            let time = r.headers["KALSHI-ACCESS-TIMESTAMP"].parse().unwrap();
            let headers = aux_zig::auth::kalshi_headers(
                "fixture-k",
                &key,
                "GET",
                "/trade-api/v2/portfolio/balance",
                time,
            )?;
            assert_eq!(
                r.headers["KALSHI-ACCESS-SIGNATURE"],
                headers["KALSHI-ACCESS-SIGNATURE"]
            );
            Ok(success())
        })
        .get_balance(&json!({"subaccount":1}))
        .unwrap();
    aux_zig::KalshiPerpsClient::new()
        .with_api_key("clearing-token")
        .with_transport(move |r: OutboundRequest| {
            assert_eq!(r.headers["Authorization"], "Bearer clearing-token");
            Ok(success())
        })
        .clearing_list_margin_fcm_api_keys(&json!({}))
        .unwrap();
    let check_key = move |r: OutboundRequest| {
        assert_eq!(r.headers["X-Api-Key"], "fixture-k");
        Ok(success())
    };
    aux_zig::ExtendedClient::new()
        .with_api_key("fixture-k")
        .with_transport(check_key)
        .get_accounts(&json!({}))
        .unwrap();
    let fixture: Value = serde_json::from_str(include_str!("../../../tests/http.json")).unwrap();
    let row = fixture
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["venue"] == "phoenix" && r["operation"] == "activate_referral")
        .unwrap();
    aux_zig::PhoenixClient::new()
        .with_api_key("fixture-k")
        .with_transport(check_key)
        .activate_referral(&row["request"])
        .unwrap();
}

#[test]
fn qfex_websocket_reconnects_and_replays_subscription() {
    use std::{
        io::{BufRead, BufReader},
        process::{Command, Stdio},
    };
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/ws_server.py");
    let mut peer = Command::new("python3")
        .arg(path)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut endpoint = String::new();
    BufReader::new(peer.stdout.take().unwrap())
        .read_line(&mut endpoint)
        .unwrap();
    let stream = aux_zig::QfexClient::new()
        .subscribe(
            &json!({"channel":"trade","symbol":"BTC"}),
            Some(endpoint.trim()),
        )
        .unwrap();
    assert_eq!(
        stream.next_event().unwrap().unwrap()["seq"].as_u64(),
        Some(9999999999999999999)
    );
    assert_eq!(stream.next_event().unwrap().unwrap()["seq"], 2);
    assert!(stream.next_event().unwrap().is_none());
    stream.close();
    stream.close();
    assert!(peer.wait().unwrap().success());
}

#[test]
fn phoenix_websocket_channels_and_snapshot_recovery() {
    use std::{
        io::{BufRead, BufReader},
        process::{Command, Stdio},
    };
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/ws_server.py");
    let mut peer = Command::new("python3")
        .args([path, "phoenix"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut endpoint = String::new();
    BufReader::new(peer.stdout.take().unwrap())
        .read_line(&mut endpoint)
        .unwrap();
    let client = aux_zig::PhoenixClient::new().with_api_key("fixture-key");
    let rows: std::collections::BTreeMap<String, Value> =
        serde_json::from_str(include_str!("../../../tests/phoenix-ws.json")).unwrap();
    for (name, row) in rows {
        let stream = client
            .subscribe(&row["specification"], Some(endpoint.trim()))
            .unwrap();
        assert_eq!(stream.next_event().unwrap().unwrap(), row["frame"]);
        if name == "exchange" {
            assert_eq!(
                stream.next_event().unwrap().unwrap()["decodedPayload"]["data"],
                json!([1, 2, 3])
            );
        }
        assert!(stream.next_event().unwrap().is_none());
        stream.close();
    }
    assert!(peer.wait().unwrap().success());
}

#[test]
fn derive_matching_signatures() {
    use aux_zig::derive::{Action, login, sign_action};
    let f: Value =
        serde_json::from_str(include_str!("../../../tests/derive-signing.json")).unwrap();
    let key = f["key"].as_str().unwrap();
    let owner = f["owner"].as_str().unwrap();
    let time = f["login"]["timestamp"].as_str().unwrap();
    let proof = login(key, owner, time.parse().unwrap()).unwrap();
    assert_eq!(proof.signature, f["login"]["signature"]);
    assert_eq!(
        (proof.timestamp.as_str(), proof.wallet.as_str()),
        (time, owner)
    );
    for row in f["actions"].as_array().unwrap() {
        for vector in row["vectors"].as_array().unwrap() {
            let action: Action = serde_json::from_value(json!({"owner":owner,"expiry":f["expiry"],"nonce":f["nonce"],"subaccount_id":f["subaccount_id"],"chain_id":vector["chain_id"],"data":row["data"]})).unwrap();
            let signed = sign_action(key, &action).unwrap();
            assert_eq!(signed.module, row["module"]);
            assert_eq!(signed.encoded, row["encoded"]);
            assert_eq!(signed.signature, vector["signature"]);
            assert_eq!(signed.digest, vector["digest"]);
            assert!(
                signed
                    .signer
                    .eq_ignore_ascii_case(f["signer"].as_str().unwrap())
            );
        }
    }
}
#[test]
fn derive_public_and_authenticated_subscriptions() {
    use std::{
        io::{BufRead, BufReader},
        process::{Command, Stdio},
    };
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/ws_server.py");
    let mut peer = Command::new("python3")
        .args([path, "derive"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut endpoint = String::new();
    BufReader::new(peer.stdout.take().unwrap())
        .read_line(&mut endpoint)
        .unwrap();
    let client = aux_zig::DeriveClient::new().with_credentials(
        "0x2222222222222222222222222222222222222222",
        "0x0000000000000000000000000000000000000000000000000000000000000001",
    );
    let rows: std::collections::BTreeMap<String, Value> =
        serde_json::from_str(include_str!("../../../tests/derive-ws.json")).unwrap();
    for row in rows.values() {
        let stream = client
            .subscribe(&row["specification"], Some(endpoint.trim()))
            .unwrap();
        assert_eq!(
            stream.next_event().unwrap().unwrap()["params"],
            json!({"channel":row["channel"],"data":row["payload"]})
        );
        assert!(stream.next_event().unwrap().is_none());
        stream.close();
    }
    assert!(peer.wait().unwrap().success());
}

#[test]
fn paradex_json_and_binary_subscriptions() {
    use std::{
        io::{BufRead, BufReader},
        process::{Command, Stdio},
    };
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/ws_server.py");
    let mut peer = Command::new("python3")
        .args([path, "paradex"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut endpoint = String::new();
    BufReader::new(peer.stdout.take().unwrap())
        .read_line(&mut endpoint)
        .unwrap();
    let client = aux_zig::ParadexClient::new().with_account_bearer(
        "0x001234",
        "test.eyJleHAiOjQxMDI0NDQ4MDB9.test",
        true,
    );
    let frames: Value =
        serde_json::from_str(include_str!("../../../tests/paradex-sbe.json")).unwrap();
    let rows: std::collections::BTreeMap<String, Value> =
        serde_json::from_str(include_str!("../../../tests/paradex-ws.json")).unwrap();
    for (kind, row) in &rows {
        let stream = client
            .subscribe(&row["specification"], Some(endpoint.trim()))
            .unwrap();
        let data = if row["template"].is_null() {
            &row["payload"]
        } else {
            &frames["frames"]
                .as_array()
                .unwrap()
                .iter()
                .find(|f| f["template"] == row["template"])
                .unwrap()["native"]
        };
        assert_eq!(
            stream.next_event().unwrap().unwrap()["params"],
            json!({"channel":row["channel"],"data":data})
        );
        if kind == "account" {
            for control in ["heartbeat", "subscribed"] {
                assert_eq!(stream.next_event().unwrap().unwrap()["method"], control);
            }
        }
        assert!(stream.next_event().unwrap().is_none());
        stream.close();
    }
    assert!(peer.wait().unwrap().success());
}

#[test]
fn evm_reads_use_the_supplied_transport_and_check_replies() {
    use aux_zig::{
        EvmReader,
        http::{OutboundRequest, TransportResponse},
    };
    let fixtures = rows(include_str!("../../../tests/evm-rpc.json"));
    for row in fixtures {
        let fixture = row.clone();
        let reader = EvmReader::new("https://rpc.example/path?token=test").with_transport(
            move |r: OutboundRequest| {
                assert_eq!(r.method, "POST");
                assert_eq!(r.url, "https://rpc.example/path?token=test");
                let request: Value = serde_json::from_str(&r.body.unwrap()).unwrap();
                assert_eq!(request["params"], fixture["args"]);
                assert_eq!(request["method"], fixture["method"]);
                Ok(TransportResponse {
                    status: 200,
                    headers: vec![],
                    body: json!({"jsonrpc":"2.0","id":request["id"],"result":fixture["reply"]})
                        .to_string(),
                })
            },
        );
        let p = &row["params"];
        let field = |name: &str| p[name].as_str().unwrap();
        let value = match row["operation"].as_str().unwrap() {
            "chain_id" => json!(reader.chain_id().unwrap()),
            "code" => json!(reader.code(field("address"), field("block")).unwrap()),
            "eth_call" => json!(
                reader
                    .eth_call(field("to"), field("data"), field("block"), None, "0x0")
                    .unwrap()
            ),
            "logs" => json!(reader.logs(&p["filter"]).unwrap()),
            "transaction_count" => json!(reader.transaction_count(field("address")).unwrap()),
            "receipt" => json!(reader.receipt(field("hash")).unwrap()),
            "transaction" => json!(reader.transaction(field("hash")).unwrap()),
            "estimate_gas" => json!(reader.estimate_gas(&p["request"]).unwrap()),
            "block_number" => json!(reader.block_number().unwrap()),
            "block" => json!(reader.block(field("block")).unwrap()),
            "native_balance" => json!(
                reader
                    .native_balance(field("address"), field("block"))
                    .unwrap()
            ),
            "gas_price" => json!(reader.gas_price().unwrap()),
            _ => unreachable!(),
        };
        assert_eq!(value, row["expected"]);
    }
    for (id, version, result) in [
        (json!(0), "2.0", "0x1"),
        (Value::Null, "1.0", "0x1"),
        (Value::Null, "2.0", "0x01"),
    ] {
        let reader = EvmReader::new("https://rpc.example").with_transport(move |r:OutboundRequest| {
            let req:Value=serde_json::from_str(&r.body.unwrap()).unwrap();
            Ok(TransportResponse{status:200,headers:vec![],body:json!({"jsonrpc":version,"id":if id.is_null(){&req["id"]}else{&id},"result":result}).to_string()})
        });
        assert!(reader.chain_id().is_err());
    }
}

#[test]
fn derive_orders_use_caller_transport_for_metadata_and_submission() {
    use aux_zig::{
        DeriveClient,
        http::{OutboundRequest, TransportResponse},
    };
    let orders: Value =
        serde_json::from_str(include_str!("../../../tests/derive-orders.json")).unwrap();
    let f: Value =
        serde_json::from_str(include_str!("../../../tests/derive-signing.json")).unwrap();
    let metadata = orders["metadata"].clone();
    let fixture = orders["orders"]["order"].clone();
    let client = DeriveClient::new()
        .with_credentials(f["owner"].as_str().unwrap(), f["key"].as_str().unwrap())
        .with_origin("https://custom.example")
        .with_signing_chain_id(1)
        .with_transport(move |r: OutboundRequest| {
            let body: Value = serde_json::from_str(r.body.as_deref().unwrap()).unwrap();
            let result = if r.url.ends_with("/public/get_instrument") {
                assert!(!r.headers.contains_key("X-DeriveSignature"));
                metadata[body["instrument_name"].as_str().unwrap()].clone()
            } else {
                assert_eq!(r.url, "https://custom.example/v3/private/order");
                assert_eq!(body["signature"], fixture["signature"]);
                assert_eq!(body["nonce"], fixture["params"]["nonce"]);
                assert!(r.headers.contains_key("X-DeriveWallet"));
                json!({"accepted":true})
            };
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: json!({"result":result}).to_string(),
            })
        });
    assert_eq!(
        client
            .private_order(&orders["orders"]["order"]["params"])
            .unwrap(),
        json!({"accepted":true})
    );
}

#[test]
fn clob_read_authenticates_before_calling_host_transport() {
    use aux_zig::{
        ClobCredentials, PolymarketClient,
        http::{OutboundRequest, TransportResponse},
    };
    let c = PolymarketClient::new()
        .with_clob_credentials(ClobCredentials {
            address: "0x7e5f4552091a69125d5dfcb7b8c2659029395bdf".into(),
            key: "test".into(),
            passphrase: "pass".into(),
            secret: "cHVibGljLXRlc3Qtc2VjcmV0".into(),
        })
        .with_transport(|r: OutboundRequest| {
            assert_eq!(
                r.headers["POLY_ADDRESS"],
                "0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf"
            );
            assert_eq!(r.headers["POLY_API_KEY"], "test");
            assert_eq!(r.headers["POLY_PASSPHRASE"], "pass");
            assert!(r.headers["POLY_SIGNATURE"].ends_with('='));
            assert!(r.url.ends_with("/data/orders?market=fixture"));
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: "[]".into(),
            })
        });
    assert_eq!(
        c.clob_get_orders(&json!({"market":"fixture"})).unwrap(),
        json!([])
    );
}

#[test]
fn derive_write_rejects_wrong_network_and_ambiguous_metadata_before_submission() {
    use aux_zig::{
        DeriveClient,
        http::{OutboundRequest, TransportResponse},
    };
    let f: Value =
        serde_json::from_str(include_str!("../../../tests/derive-signing.json")).unwrap();
    let creds = DeriveClient::new()
        .with_credentials(f["owner"].as_str().unwrap(), f["key"].as_str().unwrap());
    let orders: Value =
        serde_json::from_str(include_str!("../../../tests/derive-orders.json")).unwrap();
    for origin in ["https://api.derive.xyz", "https://custom.example"] {
        let client = creds
            .clone()
            .with_origin(origin)
            .with_transport(|_: OutboundRequest| {
                panic!("incorrect signing configuration reached transport")
            });
        let client = if origin == "https://api.derive.xyz" {
            client.with_signing_chain_id(11155111)
        } else {
            client
        };
        assert!(
            client
                .private_order(&orders["orders"]["order"]["params"])
                .is_err()
        );
    }
    let currencies = orders["metadata"]["currencies"][0].clone();
    let reader = creds.with_transport(move |r: OutboundRequest| {
        assert!(r.url.ends_with("/public/get_all_currencies"));
        Ok(TransportResponse {
            status: 200,
            headers: vec![],
            body: json!({"result":[currencies,currencies]}).to_string(),
        })
    });
    assert!(
        reader
            .private_withdraw(&orders["orders"]["withdraw"]["params"])
            .is_err()
    );
}

#[test]
fn derive_generated_nonces_are_unique_across_concurrent_clients() {
    use aux_zig::{
        DeriveClient,
        http::{OutboundRequest, TransportResponse},
    };
    let f: Value =
        serde_json::from_str(include_str!("../../../tests/derive-signing.json")).unwrap();
    let client = DeriveClient::new()
        .with_credentials(f["owner"].as_str().unwrap(), f["key"].as_str().unwrap())
        .with_transport(|r: OutboundRequest| {
            let body: Value = serde_json::from_str(&r.body.unwrap()).unwrap();
            assert!(body["signature_expiry_sec"].as_u64().is_some());
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: json!({"result":body["nonce"]}).to_string(),
            })
        });
    let workers: Vec<_> = (0..8)
        .map(|_| {
            let client = client.clone();
            std::thread::spawn(move || {
                client
                    .private_delete_subaccount(&json!({"subaccount_id":9}))
                    .unwrap()
            })
        })
        .collect();
    let values: std::collections::HashSet<String> = workers
        .into_iter()
        .map(|w| w.join().unwrap().as_str().unwrap().to_owned())
        .collect();
    assert_eq!(values.len(), 8);
}

#[test]
fn paradex_scopes_reach_the_host_transport_only_when_authorized() {
    use aux_zig::{
        Error, ParadexClient,
        http::{OutboundRequest, TransportResponse},
    };
    let jwt = "test.eyJleHAiOjQxMDI0NDQ4MDB9.test";
    let client = ParadexClient::new()
        .with_bearer_token(jwt, true)
        .with_transport(move |r: OutboundRequest| {
            assert!(r.url.ends_with("/v1/account"));
            assert_eq!(r.headers["Authorization"], format!("Bearer {jwt}"));
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: "{}".into(),
            })
        });
    assert_eq!(client.get_account(&json!({})).unwrap(), json!({}));
    assert!(matches!(
        client.orders_cancel_all(&json!({})),
        Err(Error::AuthenticationRequired)
    ));
}

#[test]
fn polymarket_perps_selects_owner_or_proxy_before_host_transport() {
    use aux_zig::{
        Error, PerpsAuth, PolymarketPerpsClient,
        http::{OutboundRequest, TransportResponse},
    };
    let golden: Value =
        serde_json::from_str(include_str!("../../../tests/polymarket-perps-signing.json")).unwrap();
    let rows = rows(include_str!("../../../tests/prediction-http.json"));
    let mut order = rows
        .iter()
        .find(|r| r["operation"] == "perps.create_orders")
        .unwrap()["request"]
        .clone();
    order["body"].as_object_mut().unwrap().remove("sig");
    let sig = golden["signatures"]["perps.create_orders"].clone();
    let auth = PerpsAuth {
        proxy_key: Some(golden["key"].as_str().unwrap().into()),
        ..Default::default()
    };
    let client = PolymarketPerpsClient::new()
        .with_perps_auth(auth.clone())
        .with_transport(move |r: OutboundRequest| {
            let body: Value = serde_json::from_str(r.body.as_deref().unwrap()).unwrap();
            assert_eq!(body["sig"], sig);
            assert!(!r.headers.contains_key("POLYMARKET-SECRET"));
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: "true".into(),
            })
        });
    assert_eq!(client.create_orders(&order).unwrap(), json!(true));
    let mut withdrawal = rows
        .iter()
        .find(|r| r["operation"] == "perps.withdraw")
        .unwrap()["request"]
        .clone();
    withdrawal["body"].as_object_mut().unwrap().remove("sig");
    assert!(matches!(
        client.withdraw(&withdrawal),
        Err(Error::AuthenticationRequired)
    ));
    assert!(client.withdraw(&order).is_err());
    let mismatch = PerpsAuth {
        proxy: Some(format!("0x{}", "00".repeat(20))),
        secret: Some("fixture".into()),
        ..auth
    };
    assert!(matches!(
        client.with_perps_auth(mismatch).create_orders(&order),
        Err(Error::AuthenticationRequired)
    ));
}

#[test]
fn clob_wallet_l1_and_signed_order_batches_use_distinct_headers() {
    use aux_zig::{
        ClobCredentials, ClobWallet, OrderDomain, PolymarketClient,
        http::{OutboundRequest, TransportResponse},
    };
    let orders = rows(include_str!("../../../tests/polymarket-clob-orders.json"));
    let key = "07".repeat(32);
    let address = orders[0]["order"]["maker"].as_str().unwrap();
    let base = ClobWallet {
        key,
        address: None,
        nonce: u64::MAX,
        order_domain: None,
        session_signer: false,
    };
    let wallet = PolymarketClient::new()
        .with_clob_wallet(base.clone())
        .with_transport(|r: OutboundRequest| {
            assert_eq!(r.headers["POLY_NONCE"], u64::MAX.to_string());
            assert_eq!(r.headers["POLY_SIGNATURE"].len(), 132);
            assert_eq!(
                r.headers["POLY_ADDRESS"],
                "0x4a62316623ad457F02cDC5D997deD67a383EC569"
            );
            assert!(!r.headers.contains_key("POLY_API_KEY"));
            Ok(TransportResponse {
                status: 200,
                headers: vec![],
                body: "true".into(),
            })
        });
    wallet.clob_create_api_key(&json!({})).unwrap();
    wallet.clob_derive_api_key(&json!({})).unwrap();
    wallet.clob_get_api_keys(&json!({})).unwrap();
    for row in orders.iter() {
        let domain = OrderDomain {
            version: row["domain"]["version"].as_u64().unwrap() as u8,
            chain_id: 137,
            exchange: row["domain"]["exchange"].as_str().unwrap().into(),
        };
        let sig = row["signature"].clone();
        let client = PolymarketClient::new()
            .with_clob_wallet(ClobWallet {
                order_domain: Some(domain),
                ..base.clone()
            })
            .with_clob_credentials(ClobCredentials {
                address: address.into(),
                key: "test-key".into(),
                passphrase: "test-pass".into(),
                secret: "YQ==".into(),
            })
            .with_transport(move |r: OutboundRequest| {
                assert_eq!(r.headers["POLY_API_KEY"], "test-key");
                let body: Value = serde_json::from_str(r.body.as_deref().unwrap()).unwrap();
                let rows = body.as_array().cloned().unwrap_or_else(|| vec![body]);
                for entry in rows {
                    assert_eq!(entry["order"]["signature"], sig);
                }
                Ok(TransportResponse {
                    status: 200,
                    headers: vec![],
                    body: "true".into(),
                })
            });
        let entry = json!({"order":row["order"]});
        client.clob_post_order(&json!({"body":entry})).unwrap();
        client
            .clob_post_orders(&json!({"body":[entry,entry]}))
            .unwrap();
    }
}

#[test]
fn solana_transfer_checked_and_deposit_signature_match_the_solana_sdk() {
    use aux_zig::solana::*;
    let f: Value = serde_json::from_str(include_str!("../../../tests/pascal-solana.json")).unwrap();
    let mut params = f.clone();
    params["recent_blockhash"] = f["blockhash"].clone();
    params["last_valid_block_height"] = u64::MAX.into();
    let params: DepositParams = serde_json::from_value(params).unwrap();
    assert_eq!(
        associated_token_address(&params.owner, &params.mint).unwrap(),
        f["associated_token_address"]
    );
    let mut prepared = prepare_deposit(&params).unwrap();
    let hex: String = prepared
        .message
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(hex, f["message_hex"]);
    let signed = sign_deposit(&"07".repeat(32), &prepared).unwrap();
    assert_eq!(signed.signature, f["signature"]);
    assert_eq!(signed.transaction_base64, f["transaction_base64"]);
    assert_eq!(signed.last_valid_block_height, u64::MAX);
    prepared.params.amount += 1;
    assert!(sign_deposit(&"07".repeat(32), &prepared).is_err());
    assert!(
        prepare_deposit(&DepositParams {
            destination: params.source.clone(),
            ..params.clone()
        })
        .is_err()
    );
    assert!(associated_token_address(&format!("1{}", params.owner), &params.mint).is_err());
}

#[test]
fn dbn_files_yield_complete_metadata_then_exact_records() {
    let rows = rows(include_str!("../../../tests/schema-records.json"));
    for file in [
        include_bytes!("../../../tests/schema-records.dbn").as_slice(),
        include_bytes!("../../../tests/schema-records.dbn.zst").as_slice(),
    ] {
        let mut events = aux_zig::databento::decode_file(file).unwrap();
        let meta = events.next().unwrap().unwrap();
        assert_eq!(meta["type"], "metadata");
        assert_eq!(meta["metadata"]["dataset"], "GLBX.MDP3");
        assert_eq!(meta["metadata"]["ts_out"], false);
        for r in &rows {
            assert_eq!(events.next().unwrap().unwrap(), r["record"]);
        }
        assert!(events.next().is_none());
    }
    for (source, golden) in [
        (
            include_bytes!("../../../tests/legacy-v1.dbn").as_slice(),
            include_str!("../../../tests/legacy-v1.jsonl"),
        ),
        (
            include_bytes!("../../../tests/legacy-v2.dbn").as_slice(),
            include_str!("../../../tests/legacy-v2.jsonl"),
        ),
    ] {
        let mut cursor = aux_zig::databento::decode_file(source).unwrap();
        for (index, line) in golden.lines().enumerate() {
            let event = cursor.next().unwrap().unwrap();
            let expected = serde_json::from_str::<Value>(line).unwrap();
            assert_eq!(
                if index == 0 {
                    &event["metadata"]
                } else {
                    &event
                },
                &expected
            );
        }
        assert!(cursor.next().is_none());
    }
    let truncated = include_bytes!("../../../tests/ts-out.dbn");
    let actual: Vec<_> = aux_zig::databento::decode_file(truncated)
        .unwrap()
        .collect();
    assert_eq!(actual.len(), 2);
    assert_eq!(actual[0].as_ref().unwrap()["metadata"]["ts_out"], true);
    assert!(actual[1].as_ref().unwrap()["ts_out"].is_string());
    for invalid in [&truncated[..1], &truncated[..truncated.len() - 1]] {
        let mut reader = aux_zig::databento::decode_file(invalid).unwrap();
        assert!(reader.any(|event| event.is_err()));
        assert!(reader.next().is_none());
    }
}

#[test]
fn stark_and_evm_signatures_match_both_networks() {
    use aux_zig::{ParadexClient, ParadexSigner};
    let fixture: Value =
        serde_json::from_str(include_str!("../../../tests/paradex-signing.json")).unwrap();
    for row in fixture["vectors"].as_array().unwrap() {
        for (key, expected) in [
            (
                ParadexSigner::Stark(fixture["key"].as_str().unwrap().into()),
                "signature",
            ),
            (
                ParadexSigner::Evm(fixture["evm_key"].as_str().unwrap().into()),
                "evm_signature",
            ),
        ] {
            let client = ParadexClient::new()
                .with_signer(
                    fixture["account"].as_str().unwrap(),
                    row["network"].as_str().unwrap(),
                    key,
                )
                .with_bearer_token("fixture", false);
            assert_eq!(
                client.sign_typed_data(&row["typed_data"]).unwrap(),
                json!({"digest":row["digest"],"signature":row[expected]})
            );
        }
    }
}

#[test]
fn guarded_wallet_writes_require_reconciliation_after_transport_failure() {
    use aux_zig::{
        Error, PascalClient, PascalPermit,
        http::{OutboundRequest, TransportResponse},
    };
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    let count = Arc::new(AtomicUsize::new(0));
    for r in rows(include_str!("../../../tests/pascal-permits.json")) {
        let c = count.clone();
        let client = PascalClient::new()
            .with_permit(PascalPermit {
                owner: r["auth"]["owner"].as_str().unwrap().into(),
                seed: r["auth"]["seed"].as_str().unwrap().into(),
                deployment_id: 3,
                recv_window_ms: 5000,
            })
            .with_transport(move |req: OutboundRequest| {
                let body: Value = serde_json::from_str(req.body.as_deref().unwrap()).unwrap();
                assert!(body["auth"]["signature"].as_str().unwrap().len() > 80);
                if c.fetch_add(1, Ordering::Relaxed) == 0 {
                    Err(Error::Network)
                } else {
                    Ok(TransportResponse {
                        status: 200,
                        headers: Vec::new(),
                        body: "{}".into(),
                    })
                }
            });
        assert_eq!(
            client
                .sign_permit_at(r["operation"].as_str().unwrap(), &r["body"], 1731536000000)
                .unwrap()["auth"]["signature"],
            r["signature"]
        );
        if r["operation"] == "write.withdraw_collateral" {
            let request = json!({"body":r["body"]});
            assert!(matches!(
                client.withdraw_collateral(&request),
                Err(Error::Network)
            ));
            assert!(matches!(
                client.clone().withdraw_collateral(&request),
                Err(Error::PendingOutcome)
            ));
            assert!(client.clone().wallet_outcome_pending().unwrap());
            client.reconcile_wallet().unwrap();
            assert!(!client.wallet_outcome_pending().unwrap());
            client.withdraw_collateral(&request).unwrap();
            assert_eq!(count.load(Ordering::Relaxed), 2);
        }
    }
}

#[test]
fn hyperdash_text_formats_use_the_application_transport_and_preserve_bytes() {
    let c = aux_zig::HyperdashClient::new()
        .with_api_key("fixture")
        .with_transport(|r: aux_zig::http::OutboundRequest| {
            let text = match r.headers["accept"].as_str() {
                "text/csv" => "id,name\r\n1,\"a,b\"\r\n",
                "application/x-ndjson" => "{\"id\":1}\n",
                other => panic!("unexpected Accept {other}"),
            };
            Ok(aux_zig::http::TransportResponse {
                status: 200,
                headers: vec![("X-Request-Id".into(), "fixture-id".into())],
                body: text.into(),
            })
        });
    let p = json!({"address":"wallet"});
    for (result, text) in [
        (c.account_pnl90d_csv(&p), "id,name\r\n1,\"a,b\"\r\n"),
        (c.account_pnl90d_ndjson(&p), "{\"id\":1}\n"),
    ] {
        let response = result.unwrap();
        assert_eq!(response["data"], text);
        assert_eq!(response["metadata"]["request_id"], "[redacted]-id");
    }
}

#[test]
fn extended_path_and_rpc_subscriptions() {
    use std::{
        io::{BufRead, BufReader},
        process::{Command, Stdio},
    };
    let mut peer = Command::new("python3")
        .args([
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/ws_server.py"),
            "extended",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut endpoint = String::new();
    BufReader::new(peer.stdout.take().unwrap())
        .read_line(&mut endpoint)
        .unwrap();
    let base = format!("{}/v1", endpoint.trim().strip_suffix("/stream").unwrap());
    let rows: std::collections::BTreeMap<String, serde_json::Value> =
        serde_json::from_str(include_str!("../../../tests/extended-ws.json")).unwrap();
    for (name, row) in rows {
        let mut client = aux_zig::ExtendedClient::new();
        if row["private"] == true {
            client = client.with_api_key("fixture");
        }
        let stream = client
            .subscribe(&row["specification"], Some(&base))
            .unwrap();
        assert_eq!(stream.next_event().unwrap().unwrap(), row["frame"]);
        if name == "orderbooks_rpc" {
            assert_eq!(stream.next_event().unwrap().unwrap(), row["frame"]);
        }
        assert!(stream.next_event().unwrap().is_none());
    }
    assert!(peer.wait().unwrap().success());
}

#[test]
fn extended_settlement_and_conditional_order_vectors() {
    use aux_zig::extended::*;
    for row in rows(include_str!("../../../tests/extended-settlement.json")) {
        let args = |key| {
            settlement(
                row["kind"].as_str().unwrap(),
                row["chain_id"].as_str().unwrap(),
                row["public_key"].as_str().unwrap(),
                &row["message"],
                key,
            )
        };
        assert_eq!(args(Some("0x1")).unwrap(), row["expected"]);
        assert_eq!(args(None).unwrap()["hash"], row["expected"]["hash"]);
        assert!(args(Some("0x2")).is_err());
    }
    for row in rows(include_str!("../../../tests/extended-orders.json")) {
        let net = row["chain_id"].as_str().unwrap();
        let ctx = &row["context"];
        let mut body = row["body"].clone();
        assert_eq!(
            prepare_order(net, row["public_key"].as_str().unwrap(), &body, ctx).unwrap(),
            row["prepared"]
        );
        assert_eq!(sign_order(net, "0x1", &body, ctx).unwrap(), row["signed"]);
        body["qty"] = "0.000000001".into();
        assert!(sign_order(net, "0x1", &body, ctx).is_err());
    }
}
