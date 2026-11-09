use aux_rs_core::{HttpRequest, HttpResponse, HttpTransport, TransportError};
use aux_rs_phoenix::{instructions::types::Instruction, *};
use serde_json::{Value, json};
use solana_pubkey::Pubkey;
use std::sync::{Arc, Mutex};
#[derive(Clone, Default)]
struct RpcFixture {
    requests: Arc<Mutex<Vec<Value>>>,
    signature: Arc<Mutex<String>>,
}
impl HttpTransport for RpcFixture {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, TransportError> {
        let request: Value = serde_json::from_slice(&request.body).unwrap();
        let result = match request["method"].as_str().unwrap() {
            "getLatestBlockhash" => {
                json!({"context":{"slot":100},"value":{"blockhash":Pubkey::new_from_array([4;32]).to_string(),"lastValidBlockHeight":500}})
            }
            "getBlockHeight" => json!(100),
            "simulateTransaction" => {
                json!({"value":{"err":null,"logs":["fixture"],"unitsConsumed":1}})
            }
            "sendTransaction" => json!(*self.signature.lock().unwrap()),
            "getSignatureStatuses" => {
                json!({"value":[{"slot":101,"err":null,"confirmationStatus":"confirmed"}]})
            }
            "getTransaction" => json!({"slot":101,"meta":{"err":null}}),
            _ => panic!("unexpected RPC method"),
        };
        self.requests.lock().unwrap().push(request);
        Ok(HttpResponse {
            status: 200,
            headers: vec![],
            body: serde_json::to_vec(&json!({"jsonrpc":"2.0","id":1,"result":result})).unwrap(),
        })
    }
}
#[tokio::test]
async fn rpc_preparation_simulation_and_reconciliation() {
    let transport = RpcFixture::default();
    let client = PhoenixChainClient::with_transport("http://localhost", transport.clone()).unwrap();
    let signer = LocalSigner::from_hex(zeroize::Zeroizing::new("01".repeat(32))).unwrap();
    let fee_payer = bs58::encode(signer.public_key()).into_string();
    let ixs = [Instruction {
        program_id: Pubkey::new_from_array([3; 32]),
        accounts: vec![],
        data: vec![1],
    }];
    let (first, second) = tokio::join!(
        client.prepare(&ixs, &fee_payer),
        client.prepare(&ixs, &fee_payer)
    );
    let prepared = first.unwrap();
    assert_eq!(prepared.message_base64, second.unwrap().message_base64);
    assert_eq!(prepared.last_valid_block_height, 500);
    let signatures = prepared.sign(&signer, Default::default()).await.unwrap();
    let signed = prepared.assemble(&signatures).unwrap();
    *transport.signature.lock().unwrap() = signed.signature.clone();
    assert!(client.simulate(&signed).await.unwrap()["value"]["err"].is_null());
    assert_eq!(
        client.submit(signed.clone()).await.unwrap(),
        signed.signature
    );
    assert_eq!(
        client.signature_status(&signed.signature).await.unwrap()["value"][0]["confirmationStatus"],
        "confirmed"
    );
    assert_eq!(
        client.transaction(&signed.signature).await.unwrap()["slot"],
        101
    );
    assert!(client.pending().is_some());
    client.clear_reconciled_submission();
    assert!(client.pending().is_none());
    let mut expired = signed;
    expired.last_valid_block_height = 99;
    assert!(client.submit(expired).await.is_err());
    assert!(client.pending().is_none());
    let sent = transport.requests.lock().unwrap();
    let writes = sent
        .iter()
        .filter(|r| r["method"] == "sendTransaction")
        .collect::<Vec<_>>();
    assert_eq!(writes.len(), 1);
    assert_eq!(writes[0]["params"][1]["maxRetries"], 0);
}
