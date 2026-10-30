use aux_rs_core::hex;
use aux_rs_lighter::*;
use serde_json::Value;
use zeroize::Zeroizing;

fn array<const N: usize>(v: &Value) -> [u8; N] {
    hex::decode(v.as_str().unwrap())
        .unwrap()
        .try_into()
        .unwrap()
}
fn prepare(v: &Value) -> PreparedTransaction {
    let info = &v["tx_info"];
    let transaction = match v["kind"].as_str().unwrap() {
        "create" => Transaction::CreateOrder(serde_json::from_value(info.clone()).unwrap()),
        "cancel" => Transaction::CancelOrder(serde_json::from_value(info.clone()).unwrap()),
        _ => unreachable!(),
    };
    let attributes = if info["L2TxAttributes"].is_null() {
        TxAttributes::default()
    } else {
        serde_json::from_value(info["L2TxAttributes"].clone()).unwrap()
    };
    transaction
        .prepare(
            v["chain_id"].as_u64().unwrap() as u32,
            info["AccountIndex"].as_i64().unwrap(),
            info["ApiKeyIndex"].as_u64().unwrap() as u8,
            info["Nonce"].as_i64().unwrap(),
            TxOptions {
                expired_at: info["ExpiredAt"].as_i64().unwrap(),
                attributes,
            },
        )
        .unwrap()
}

// Replay official signatures through the production transaction encoder: local
// Schnorr signatures use fresh randomness, so they cannot be compared bytewise.
#[tokio::test]
async fn pinned_signature_wire_encodings_match_reference() {
    struct ReferenceSigner {
        public_key: PublicKey,
        signature: Signature,
        digest: [u8; 40],
    }
    impl Signer for ReferenceSigner {
        fn public_key(&self) -> PublicKey {
            self.public_key
        }
        async fn sign(&self, payload: &SigningPayload) -> Result<Signature, Error> {
            assert_eq!(payload.digest(), &self.digest);
            Ok(self.signature.clone())
        }
    }

    let fixtures: Value =
        serde_json::from_str(include_str!("fixtures/signing_vectors.json")).unwrap();
    let mut transactions = 0;
    for vector in fixtures["vectors"].as_array().unwrap() {
        if vector["kind"] == "auth" {
            continue;
        }
        let signer = ReferenceSigner {
            public_key: PublicKey::from_bytes(array(&vector["public_key"])).unwrap(),
            signature: Signature::from_bytes(array(&vector["signature"])).unwrap(),
            digest: array(&vector["digest"]),
        };
        let signed = prepare(vector).sign(&signer).await.unwrap();
        let actual: Value = serde_json::from_str(signed.tx_info()).unwrap();
        assert_eq!(actual, vector["tx_info"]);
        transactions += 1;
    }
    assert_eq!(transactions, 60);
}

#[tokio::test]
async fn pinned_reference_hashes_encodings_and_signatures() {
    let fixtures: Value =
        serde_json::from_str(include_str!("fixtures/signing_vectors.json")).unwrap();
    assert_eq!(
        fixtures["revision"],
        "ebae8edecfee9986b0a5fcde3cb9a06bd4af3a3a"
    );
    assert_eq!(fixtures["vectors"].as_array().unwrap().len(), 66);
    for vector in fixtures["vectors"].as_array().unwrap() {
        let signer =
            LocalSigner::from_hex(Zeroizing::new(vector["key"].as_str().unwrap().into())).unwrap();
        assert_eq!(
            *signer.public_key().as_bytes(),
            array::<40>(&vector["public_key"])
        );
        let payload = if vector["kind"] == "auth" {
            auth_signing_payload(
                vector["chain_id"].as_u64().unwrap() as u32,
                vector["account_index"].as_i64().unwrap(),
                vector["api_key_index"].as_u64().unwrap() as u8,
                vector["deadline"].as_i64().unwrap(),
            )
            .unwrap()
        } else {
            prepare(vector).signing_payload().clone()
        };
        assert_eq!(
            *payload.digest(),
            array::<40>(&vector["digest"]),
            "{vector}"
        );
        let signature = Signature::from_bytes(array::<80>(&vector["signature"])).unwrap();
        signature.verify(&payload, signer.public_key()).unwrap();
        let rust_sig = signer.sign(&payload).await.unwrap();
        rust_sig.verify(&payload, signer.public_key()).unwrap();
        if vector["kind"] != "auth" {
            let signed = prepare(vector).sign(&signer).await.unwrap();
            let mut actual: Value = serde_json::from_str(signed.tx_info()).unwrap();
            let mut expected = vector["tx_info"].clone();
            // Schnorr signing uses fresh randomness; compare all other fields.
            actual.as_object_mut().unwrap().remove("Sig");
            expected.as_object_mut().unwrap().remove("Sig");
            assert_eq!(actual, expected);
        } else {
            assert_eq!(
                vector["token"],
                format!(
                    "{}:{}",
                    vector["message"].as_str().unwrap(),
                    hex::encode(signature.as_bytes())
                )
            );
        }
    }
}

#[tokio::test]
async fn altered_context_payload_and_noncanonical_values_are_rejected() {
    let fixtures: Value =
        serde_json::from_str(include_str!("fixtures/signing_vectors.json")).unwrap();
    let vector = &fixtures["vectors"][0];
    let pubkey = PublicKey::from_bytes(array::<40>(&vector["public_key"])).unwrap();
    let signature = Signature::from_bytes(array::<80>(&vector["signature"])).unwrap();
    for field in [
        "AccountIndex",
        "ApiKeyIndex",
        "Nonce",
        "ExpiredAt",
        "BaseAmount",
        "Price",
        "IsAsk",
        "MarketIndex",
        "ClientOrderIndex",
    ] {
        let mut altered = vector.clone();
        let old = altered["tx_info"][field].as_i64().unwrap();
        altered["tx_info"][field] = (if field == "IsAsk" { 0 } else { old + 1 }).into();
        assert!(
            signature
                .verify(prepare(&altered).signing_payload(), pubkey)
                .is_err(),
            "{field}"
        );
    }
    let mut altered = vector.clone();
    altered["chain_id"] = 304.into();
    assert!(
        signature
            .verify(prepare(&altered).signing_payload(), pubkey)
            .is_err()
    );
    assert!(LocalSigner::from_bytes(Zeroizing::new([0; 40])).is_err());
    assert!(LocalSigner::from_bytes(Zeroizing::new([255; 40])).is_err());
    assert!(
        LocalSigner::from_bytes(Zeroizing::new(
            goldilocks_crypto::ScalarField::N.to_bytes_le()
        ))
        .is_err()
    );
    assert!(PublicKey::from_bytes([0; 40]).is_err());
    assert!(PublicKey::from_bytes([255; 40]).is_err());
    assert!(Signature::from_bytes([255; 80]).is_err());
    // Native auth tokens deliberately do not contain the chain ID in the hash.
    let auth = auth_signing_payload(300, 37, 3, 1800000000).unwrap();
    assert_eq!(
        auth.digest(),
        auth_signing_payload(304, 37, 3, 1800000000)
            .unwrap()
            .digest()
    );
    for p in [
        auth_signing_payload(300, 38, 3, 1800000000),
        auth_signing_payload(300, 37, 4, 1800000000),
        auth_signing_payload(300, 37, 3, 1800000001),
    ] {
        assert_ne!(auth.digest(), p.unwrap().digest());
    }
}

#[tokio::test]
async fn token_deadlines_and_debug_redaction() {
    let mut bytes = [0; 40];
    bytes[0] = 1;
    let signer = LocalSigner::from_bytes(Zeroizing::new(bytes)).unwrap();
    let auth = LighterAuth::new(signer, 300, 37, 3).unwrap();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    assert!(auth.token(now - 1).await.is_err());
    assert!(auth.token(now + 8 * 3600 + 60).await.is_err());
    let token = auth.token(now + 600).await.unwrap();
    let debug = format!("{token:?} {auth:?}");
    assert!(!debug.contains(token.as_str()));
    assert!(token.as_str().starts_with(&format!("{}:37:3:", now + 600)));
    let sig = Signature::from_bytes(
        hex::decode(token.as_str().rsplit(':').next().unwrap())
            .unwrap()
            .try_into()
            .unwrap(),
    )
    .unwrap();
    sig.verify(
        &auth_signing_payload(300, 37, 3, now + 600).unwrap(),
        auth.signer().public_key(),
    )
    .unwrap();
}
