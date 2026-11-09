use aux_rs_phoenix::instructions::types::{AccountMeta, Instruction};
use aux_rs_phoenix::*;
use solana_pubkey::Pubkey;
use std::collections::BTreeMap;
#[tokio::test]
async fn signatures_are_bound_to_reviewed_message_and_all_required_signers() {
    let payer = LocalSigner::from_hex(zeroize::Zeroizing::new("01".repeat(32))).unwrap();
    let owner = LocalSigner::from_hex(zeroize::Zeroizing::new("02".repeat(32))).unwrap();
    let address = |s: &LocalSigner| bs58::encode(s.public_key()).into_string();
    let ix = Instruction {
        program_id: Pubkey::new_from_array([3; 32]),
        accounts: vec![AccountMeta::readonly_signer(Pubkey::new_from_array(
            owner.public_key(),
        ))],
        data: vec![1, 2, 3],
    };
    let prepared = PreparedTransaction::new(
        &[ix],
        &address(&payer),
        &Pubkey::new_from_array([4; 32]).to_string(),
        500,
    )
    .unwrap();
    let sigs = prepared.sign(&payer, BTreeMap::new()).await.unwrap();
    assert!(prepared.assemble(&sigs).is_err());
    let sigs = prepared.sign(&owner, sigs).await.unwrap();
    let signed = prepared.assemble(&sigs).unwrap();
    prepared
        .validate_server_transaction(&signed.transaction_base64)
        .unwrap();
    let mut changed = prepared.clone();
    changed.recent_blockhash = Pubkey::new_from_array([5; 32]).to_string();
    assert!(changed.sign(&payer, BTreeMap::new()).await.is_err());
    let wrong = LocalSigner::from_hex(zeroize::Zeroizing::new("06".repeat(32))).unwrap();
    assert!(prepared.sign(&wrong, BTreeMap::new()).await.is_err());
}
#[test]
fn compressed_snapshots_preserve_native_exact_numbers() {
    use base64::Engine;
    let payload = br#"{"number":0.123456789012345678901,"large":184467440737095516170}"#;
    let compressed = zstd::stream::encode_all(payload.as_slice(), 1).unwrap();
    let frame = serde_json::json!({"channel":"exchange","messageType":"encodedSnapshot","encoding":"base64+zstd","payload":base64::engine::general_purpose::STANDARD.encode(compressed)});
    let v = decode_event(&frame.to_string()).unwrap();
    assert_eq!(
        v["decodedPayload"]["number"].to_string(),
        "0.123456789012345678901"
    );
    assert_eq!(
        v["decodedPayload"]["large"].to_string(),
        "184467440737095516170"
    );
}
#[tokio::test]
async fn versioned_messages_validate_lookup_compilation_and_signature_identity() {
    let signer = LocalSigner::from_hex(zeroize::Zeroizing::new("01".repeat(32))).unwrap();
    let address = bs58::encode(signer.public_key()).into_string();
    let loaded = Pubkey::new_from_array([7; 32]);
    let ix = Instruction {
        program_id: Pubkey::new_from_array([3; 32]),
        accounts: vec![AccountMeta::writable(loaded)],
        data: vec![1],
    };
    let table = LookupTable {
        key: Pubkey::new_from_array([9; 32]).to_string(),
        addresses: vec![loaded.to_string()],
    };
    let prepared = PreparedTransaction::new_v0(
        &[ix],
        &address,
        &Pubkey::new_from_array([4; 32]).to_string(),
        500,
        &[table],
    )
    .unwrap();
    let signatures = prepared.sign(&signer, BTreeMap::new()).await.unwrap();
    let mut signed = prepared.assemble(&signatures).unwrap();
    signed.validate().unwrap();
    prepared
        .validate_server_transaction(&signed.transaction_base64)
        .unwrap();
    signed.signature = bs58::encode([0; 64]).into_string();
    assert!(signed.validate().is_err());
}
#[derive(Clone)]
struct RpcPeer {
    writes: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}
impl aux_rs_core::HttpTransport for RpcPeer {
    async fn execute(
        &self,
        request: aux_rs_core::HttpRequest,
    ) -> Result<aux_rs_core::HttpResponse, aux_rs_core::TransportError> {
        let body: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        let result = match body["method"].as_str().unwrap() {
            "getBlockHeight" => serde_json::json!(1),
            "sendTransaction" => {
                self.writes
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                return std::future::pending().await;
            }
            "getSignatureStatuses" => serde_json::json!({"value":[null]}),
            _ => panic!("unexpected RPC method"),
        };
        Ok(aux_rs_core::HttpResponse {
            status: 200,
            headers: vec![],
            body: serde_json::to_vec(&serde_json::json!({"jsonrpc":"2.0","id":1,"result":result}))
                .unwrap(),
        })
    }
}
#[tokio::test]
async fn cancelled_submission_remains_pending_and_cannot_be_replayed() {
    let signer = LocalSigner::from_hex(zeroize::Zeroizing::new("01".repeat(32))).unwrap();
    let address = bs58::encode(signer.public_key()).into_string();
    let ix = Instruction {
        program_id: Pubkey::new_from_array([3; 32]),
        accounts: vec![],
        data: vec![1],
    };
    let prepared = PreparedTransaction::new(
        &[ix],
        &address,
        &Pubkey::new_from_array([4; 32]).to_string(),
        500,
    )
    .unwrap();
    let sigs = prepared.sign(&signer, BTreeMap::new()).await.unwrap();
    let signed = prepared.assemble(&sigs).unwrap();
    let writes = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let client = PhoenixChainClient::with_transport(
        "http://localhost",
        RpcPeer {
            writes: writes.clone(),
        },
    )
    .unwrap();
    assert!(
        tokio::time::timeout(
            std::time::Duration::from_millis(10),
            client.submit(signed.clone())
        )
        .await
        .is_err()
    );
    assert!(client.pending().is_some());
    assert!(matches!(
        client.submit(signed.clone()).await,
        Err(aux_rs_core::Error::Exchange(Error::PendingOutcome))
    ));
    assert_eq!(writes.load(std::sync::atomic::Ordering::SeqCst), 1);
    let _status = client.signature_status(&signed.signature).await.unwrap();
    assert!(client.pending().is_some());
    client.clear_reconciled_submission();
    assert!(client.pending().is_none());
}
struct ErrorStream {
    sent: bool,
    closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
}
impl aux_rs_core::WebSocketStream for ErrorStream {
    async fn next(&mut self) -> Option<Result<String, aux_rs_core::TransportError>> {
        if self.sent {
            return None;
        }
        self.sent = true;
        Some(Ok(r#"{"type":"subscriptionError","code":"invalid_subscription","message":"native rejection"}"#.into()))
    }
    async fn close(&mut self) -> Result<(), aux_rs_core::TransportError> {
        self.closed.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(())
    }
}
#[derive(Clone)]
struct ErrorPeer {
    closed: std::sync::Arc<std::sync::atomic::AtomicBool>,
}
impl aux_rs_core::WebSocketTransport for ErrorPeer {
    type Stream = ErrorStream;
    async fn subscribe<S: aux_rs_core::WebSocketSubscription>(
        &self,
        _url: &str,
        _subscription: S,
    ) -> Result<ErrorStream, aux_rs_core::TransportError> {
        Ok(ErrorStream {
            sent: false,
            closed: self.closed.clone(),
        })
    }
}
#[tokio::test]
async fn native_subscription_errors_preserve_code_and_close() {
    let closed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let client = PhoenixClient::with_transport(
        Phoenix::mainnet(),
        ErrorPeer {
            closed: closed.clone(),
        },
    );
    let mut stream = client
        .subscribe(Subscription(
            serde_json::json!({"channel":"orderbook","symbol":"SOL"}),
        ))
        .await
        .unwrap();
    let error = stream.next().await.unwrap();
    assert!(
        matches!(error,Err(aux_rs_core::Error::Exchange(Error::Api{body,..})) if body["code"]=="invalid_subscription")
    );
    assert!(closed.load(std::sync::atomic::Ordering::SeqCst));
}

#[test]
fn oversized_transaction_messages_are_rejected() {
    let ix = Instruction {
        program_id: Pubkey::new_from_array([3; 32]),
        accounts: vec![],
        data: vec![0; 1232],
    };
    let payer = Pubkey::new_from_array([1; 32]).to_string();
    let block = Pubkey::new_from_array([4; 32]).to_string();
    assert!(PreparedTransaction::new(std::slice::from_ref(&ix), &payer, &block, 500).is_err());
    assert!(PreparedTransaction::new_v0(&[ix], &payer, &block, 500, &[]).is_err());
}

#[test]
fn transaction_message_wire_vectors() {
    use base64::{Engine, engine::general_purpose::STANDARD};
    let signer = LocalSigner::from_hex(zeroize::Zeroizing::new("01".repeat(32))).unwrap();
    let payer = bs58::encode(signer.public_key()).into_string();
    let block = Pubkey::new_from_array([4; 32]).to_string();
    let mut ix = Instruction {
        program_id: Pubkey::new_from_array([3; 32]),
        accounts: vec![],
        data: vec![1],
    };
    let prepared = PreparedTransaction::new(&[ix.clone()], &payer, &block, 500).unwrap();
    // Solana wire layout: header; compact account list; blockhash; compiled ixs.
    let mut legacy = vec![1, 0, 1, 2];
    legacy.extend(signer.public_key());
    legacy.extend([3; 32]);
    legacy.extend([4; 32]);
    legacy.extend([1, 1, 0, 1, 1]);
    assert_eq!(STANDARD.decode(prepared.message_base64).unwrap(), legacy);
    let loaded = Pubkey::new_from_array([7; 32]);
    ix.accounts.push(AccountMeta::writable(loaded));
    let table = LookupTable {
        key: Pubkey::new_from_array([9; 32]).to_string(),
        addresses: vec![loaded.to_string()],
    };
    let prepared = PreparedTransaction::new_v0(&[ix], &payer, &block, 500, &[table]).unwrap();
    let mut v0 = vec![0x80, 1, 0, 1, 2];
    v0.extend(signer.public_key());
    v0.extend([3; 32]);
    v0.extend([4; 32]);
    v0.extend([1, 1, 1, 2, 1, 1, 1]);
    v0.extend([9; 32]);
    v0.extend([1, 0, 0]);
    assert_eq!(STANDARD.decode(prepared.message_base64).unwrap(), v0);
}
