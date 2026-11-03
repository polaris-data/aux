use aux_rs_derive::*;
use serde_json::Value;
use zeroize::Zeroizing;
#[tokio::test]
async fn official_module_and_signature_vectors() {
    let vectors: Value = serde_json::from_str(include_str!("fixtures/signing.json")).unwrap();
    let signer =
        LocalSigner::from_hex(Zeroizing::new(vectors["key"].as_str().unwrap().into())).unwrap();
    assert_eq!(
        format!("0x{}", aux_rs_core::hex::encode(signer.address())).to_lowercase(),
        vectors["signer"].as_str().unwrap().to_lowercase()
    );
    let login = &vectors["login"];
    let payload = SigningPayload {
        digest: personal_hash(login["timestamp"].as_str().unwrap().as_bytes()),
        chain_id: 1,
        scheme: SigningScheme::Login,
    };
    assert_eq!(
        format!(
            "0x{}",
            aux_rs_core::hex::encode(signer.sign(&payload).await.unwrap())
        ),
        login["signature"]
    );
    for item in vectors["actions"].as_array().unwrap() {
        let data: ModuleData = serde_json::from_value(item["data"].clone()).unwrap();
        let encoded = data.encode().unwrap();
        assert_eq!(
            format!("0x{}", aux_rs_core::hex::encode(&encoded)),
            item["encoded"],
            "{}",
            item["data"]["type"]
        );
        assert_eq!(
            data.module().to_lowercase(),
            item["module"].as_str().unwrap().to_lowercase()
        );
        let action = ActionPayload {
            subaccount_id: vectors["subaccount_id"].as_str().unwrap().into(),
            nonce: vectors["nonce"].as_str().unwrap().into(),
            module: data.module().into(),
            encoded_data: encoded,
            expiry: vectors["expiry"].as_u64().unwrap(),
            owner: vectors["owner"].as_str().unwrap().into(),
            signer: vectors["signer"].as_str().unwrap().into(),
        };
        let mut previous = None;
        for vector in item["vectors"].as_array().unwrap() {
            let chain = vector["chain_id"].as_u64().unwrap();
            let digest = action.digest(chain, &Derive::mainnet().matching).unwrap();
            assert_eq!(
                format!("0x{}", aux_rs_core::hex::encode(digest)),
                vector["digest"]
            );
            assert_ne!(previous, Some(digest));
            previous = Some(digest);
            let sig = signer
                .sign(&SigningPayload {
                    digest,
                    chain_id: chain,
                    scheme: SigningScheme::Action,
                })
                .await
                .unwrap();
            assert_eq!(
                format!("0x{}", aux_rs_core::hex::encode(sig)),
                vector["signature"]
            );
            verify_signature(signer.address(), &digest, &sig).unwrap();
            let mut changed = digest;
            changed[0] ^= 1;
            assert!(verify_signature(signer.address(), &changed, &sig).is_err());
        }
    }
}
#[test]
fn exact_decimal_and_abi_validation() {
    assert!(decimal_word("1e-8", 18, false).is_err());
    assert!(decimal_word("-1", 18, false).is_err());
    assert!(uint_word(&"9".repeat(80)).is_err());
    assert!(decimal_word("1.000000001", 8, false).is_err());
    let data = ModuleData::VaultDeposit {
        vault_subaccount_id: "1".into(),
        deposit_spot_asset: "0x3333333333333333333333333333333333333333".into(),
        amount: "0.0000000000001".into(),
    };
    assert!(data.encode().is_err());
}
#[tokio::test]
async fn nonce_identity_coordination_and_owner_network_separation() {
    let key = || LocalSigner::from_hex(Zeroizing::new(format!("{:064x}", 1))).unwrap();
    let exchange = Derive::testnet();
    let owner = "0x2222222222222222222222222222222222222222";
    let a = DeriveAuth::new(&exchange, owner, key()).unwrap();
    let b = DeriveAuth::new(
        &exchange,
        "0x3333333333333333333333333333333333333333",
        key(),
    )
    .unwrap();
    let mut nonces = std::collections::BTreeSet::new();
    for _ in 0..100 {
        assert!(nonces.insert(a.nonce().unwrap()));
        assert!(nonces.insert(b.nonce().unwrap()));
    }
    let allocated = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|i| {
                let auth = if i % 2 == 0 { &a } else { &b };
                scope.spawn(move || (0..100).map(|_| auth.nonce().unwrap()).collect::<Vec<_>>())
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|w| w.join().unwrap())
            .collect::<Vec<_>>()
    });
    for nonce in allocated {
        assert!(nonces.insert(nonce));
    }
    assert!(a.login(&Derive::mainnet()).await.is_err());
    let action = ActionPayload {
        subaccount_id: "9".into(),
        nonce: a.nonce().unwrap(),
        module: ModuleData::DeleteSubaccount.module().into(),
        encoded_data: vec![],
        expiry: 1793710300,
        owner: b.wallet().into(),
        signer: format!("0x{}", aux_rs_core::hex::encode(a.signer_address())),
    };
    assert!(a.sign_action(&exchange, &action).await.is_err());
}
