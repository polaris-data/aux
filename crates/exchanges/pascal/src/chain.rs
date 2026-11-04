//! Explicit Solana collateral transfers. Matching-engine settlements stay on Pascal's API.
use crate::{Error, Signer, public_key};
use aux_rs_core::{DefaultTransport, Error as ClientError, HttpMethod, HttpRequest, HttpTransport};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::{Value, json};
use solana_pubkey::Pubkey;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};
const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ATA_PROGRAM: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub mint: String,
    pub decimals: u8,
}
impl SolanaConfig {
    pub fn new(
        rpc_url: impl Into<String>,
        mint: impl Into<String>,
        decimals: u8,
    ) -> Result<Self, Error> {
        let rpc_url = rpc_url.into();
        let mint = mint.into();
        let mut parsed = url::Url::parse(&rpc_url)
            .map_err(|_| Error::InvalidRequest("invalid Solana RPC endpoint"))?;
        parsed.set_query(None);
        crate::validate_url(parsed.as_str(), false)?;
        public_key(&mint)?;
        if decimals > 18 {
            return Err(Error::InvalidRequest("unsupported token decimals"));
        }
        Ok(Self {
            rpc_url,
            mint,
            decimals,
        })
    }
}
impl std::fmt::Debug for SolanaConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolanaConfig")
            .field("mint", &self.mint)
            .field("decimals", &self.decimals)
            .finish_non_exhaustive()
    }
}
pub fn associated_token_address(owner: &str, mint: &str) -> Result<String, Error> {
    let owner = public_key(owner)?;
    let mint = public_key(mint)?;
    let token = public_key(TOKEN_PROGRAM)?;
    let ata = Pubkey::new_from_array(public_key(ATA_PROGRAM)?);
    let (key, _) = Pubkey::find_program_address(&[&owner, &token, &mint], &ata);
    Ok(key.to_string())
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct PreparedDeposit {
    pub message: Vec<u8>,
    pub owner: String,
    pub source: String,
    pub recent_blockhash: String,
    pub last_valid_block_height: u64,
    pub amount: u64,
    pub destination: String,
    pub mint: String,
    pub decimals: u8,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SignedDeposit {
    pub signature: String,
    pub transaction_base64: String,
    pub last_valid_block_height: u64,
}
impl PreparedDeposit {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        owner: &str,
        source_token_account: &str,
        destination_token_account: &str,
        mint: &str,
        amount: u64,
        decimals: u8,
        blockhash: &str,
        last_valid_block_height: u64,
    ) -> Result<Self, Error> {
        if amount == 0 {
            return Err(Error::InvalidRequest("positive deposit amount required"));
        }
        let keys = [
            public_key(owner)?,
            public_key(source_token_account)?,
            public_key(destination_token_account)?,
            public_key(mint)?,
            public_key(TOKEN_PROGRAM)?,
        ];
        for (i, k) in keys.iter().enumerate() {
            if keys[..i].contains(k) {
                return Err(Error::InvalidRequest(
                    "deposit account identities must be distinct",
                ));
            }
        }
        let hash = public_key(blockhash)?;
        // Legacy message: owner is fee payer, source/destination writable; mint/program read-only.
        let mut message = vec![1, 0, 2, 5];
        for key in keys {
            message.extend(key);
        }
        message.extend(hash);
        message.extend([1, 4, 4, 1, 3, 2, 0, 10, 12]);
        message.extend(amount.to_le_bytes());
        message.push(decimals);
        Ok(Self {
            message,
            owner: owner.into(),
            source: source_token_account.into(),
            recent_blockhash: blockhash.into(),
            last_valid_block_height,
            amount,
            destination: destination_token_account.into(),
            mint: mint.into(),
            decimals,
        })
    }
    pub async fn sign<S: Signer>(&self, signer: &S) -> Result<SignedDeposit, Error> {
        let canonical = Self::new(
            &self.owner,
            &self.source,
            &self.destination,
            &self.mint,
            self.amount,
            self.decimals,
            &self.recent_blockhash,
            self.last_valid_block_height,
        )?;
        if canonical.message != self.message {
            return Err(Error::Signing(
                "deposit metadata differs from transaction message",
            ));
        }
        if signer.public_key() != public_key(&self.owner)? {
            return Err(Error::Signing("deposit signer differs from owner"));
        }
        let sig = signer.sign(&self.message).await?;
        ed25519_dalek::VerifyingKey::from_bytes(&signer.public_key())
            .map_err(|_| Error::Signing("invalid deposit signer"))?
            .verify_strict(&self.message, &ed25519_dalek::Signature::from_bytes(&sig))
            .map_err(|_| Error::Signing("invalid deposit signature"))?;
        let mut tx = vec![1];
        tx.extend(sig);
        tx.extend(&self.message);
        Ok(SignedDeposit {
            signature: bs58::encode(sig).into_string(),
            transaction_base64: STANDARD.encode(tx),
            last_valid_block_height: self.last_valid_block_height,
        })
    }
}
#[derive(Clone)]
pub struct PascalChainClient<T = DefaultTransport> {
    config: SolanaConfig,
    transport: T,
    id: Arc<AtomicU64>,
    pending: Arc<Mutex<std::collections::BTreeSet<String>>>,
}
impl<T> PascalChainClient<T> {
    pub fn with_transport(config: SolanaConfig, transport: T) -> Self {
        Self {
            config,
            transport,
            id: Arc::new(AtomicU64::new(1)),
            pending: Default::default(),
        }
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn config(&self) -> &SolanaConfig {
        &self.config
    }
    pub fn pending(&self) -> Vec<String> {
        self.pending
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .cloned()
            .collect()
    }
    /// Explicit caller attestation after confirming rejection, expiry or delayed credit externally.
    /// Never clears an unrelated pending transaction and never resubmits it.
    pub fn clear_reconciled_submission(&self, signature: &str) -> Result<(), Error> {
        if !self
            .pending
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .remove(signature)
        {
            return Err(Error::InvalidRequest(
                "signature is not pending on this client",
            ));
        }
        Ok(())
    }
}
impl<T: HttpTransport> PascalChainClient<T> {
    /// Return chain confirmation and native API history separately; inclusion is not deposit credit.
    pub async fn reconcile_deposit_credit<RT: HttpTransport, A>(
        &self,
        api: &crate::PascalClient<RT, A>,
        signature: &str,
        request: crate::read::TransferHistoryParams,
    ) -> Result<Value, ClientError<Error>> {
        let confirmation = self.reconcile(signature).await?;
        let transfers = api.transfer_history(request).await?;
        Ok(json!({"signature": signature, "confirmation": confirmation, "transfers": transfers}))
    }
    async fn rpc(
        &self,
        method: &str,
        params: Value,
        write: bool,
    ) -> Result<Value, ClientError<Error>> {
        self.transport.ensure_open()?;
        let id = self.id.fetch_add(1, Ordering::Relaxed);
        let req = HttpRequest {
            method: HttpMethod::Post,
            url: self.config.rpc_url.clone(),
            headers: vec![("content-type".into(), "application/json".into())],
            body: serde_json::to_vec(
                &json!({"jsonrpc":"2.0","id":id,"method":method,"params":params}),
            )
            .map_err(|e| ClientError::Exchange(e.into()))?,
        };
        let r = if write {
            self.transport.execute(req).await?
        } else {
            self.transport.execute_read(req).await?
        };
        let status = r.status;
        let v: Value = crate::decode_response(r).map_err(ClientError::Exchange)?;
        if v["id"] != id || v["jsonrpc"] != "2.0" {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "mismatched Solana RPC response",
            )));
        }
        if v.get("error").is_some() {
            return Err(ClientError::Exchange(Error::Api { status, body: v }));
        }
        v.get("result")
            .cloned()
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "missing Solana RPC result",
            )))
    }
    pub async fn account_info(&self, address: &str) -> Result<Value, ClientError<Error>> {
        public_key(address).map_err(ClientError::Exchange)?;
        self.rpc(
            "getAccountInfo",
            json!([address,{"encoding":"jsonParsed","commitment":"confirmed"}]),
            false,
        )
        .await
    }
    pub async fn token_balance(&self, address: &str) -> Result<Value, ClientError<Error>> {
        public_key(address).map_err(ClientError::Exchange)?;
        self.rpc(
            "getTokenAccountBalance",
            json!([address,{"commitment":"confirmed"}]),
            false,
        )
        .await
    }
    pub async fn block_height(&self) -> Result<u64, ClientError<Error>> {
        let v = self
            .rpc("getBlockHeight", json!([{"commitment":"confirmed"}]), false)
            .await?;
        v.as_u64()
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "invalid block height",
            )))
    }
    pub async fn signature_status(&self, signature: &str) -> Result<Value, ClientError<Error>> {
        self.rpc(
            "getSignatureStatuses",
            json!([[signature],{"searchTransactionHistory":true}]),
            false,
        )
        .await
    }
    pub async fn prepare_deposit(
        &self,
        owner: &str,
        source_token_account: &str,
        destination_token_account: &str,
        amount: u64,
    ) -> Result<PreparedDeposit, ClientError<Error>> {
        if !self.pending().is_empty() {
            return Err(ClientError::Exchange(Error::PendingOutcome));
        }
        for (address, is_source) in [
            (source_token_account, true),
            (destination_token_account, false),
        ] {
            let info = self.account_info(address).await?;
            let v = &info["value"];
            if v["owner"] != TOKEN_PROGRAM
                || v["data"]["parsed"]["info"]["mint"] != self.config.mint
                || is_source && v["data"]["parsed"]["info"]["owner"] != owner
            {
                return Err(ClientError::Exchange(Error::InvalidRequest(
                    "token account program, mint or owner mismatch",
                )));
            }
        }
        let mint = self.account_info(&self.config.mint).await?;
        if mint["value"]["owner"] != TOKEN_PROGRAM
            || mint["value"]["data"]["parsed"]["info"]["decimals"].as_u64()
                != Some(self.config.decimals.into())
        {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "token mint decimals mismatch",
            )));
        }
        let block = self
            .rpc(
                "getLatestBlockhash",
                json!([{"commitment":"confirmed"}]),
                false,
            )
            .await?;
        PreparedDeposit::new(
            owner,
            source_token_account,
            destination_token_account,
            &self.config.mint,
            amount,
            self.config.decimals,
            block["value"]["blockhash"]
                .as_str()
                .ok_or(ClientError::Exchange(Error::InvalidRequest(
                    "missing recent blockhash",
                )))?,
            block["value"]["lastValidBlockHeight"]
                .as_u64()
                .ok_or(ClientError::Exchange(Error::InvalidRequest(
                    "missing block expiry",
                )))?,
        )
        .map_err(ClientError::Exchange)
    }
    /// One attempt; never refresh/re-sign an uncertain transaction with another blockhash.
    pub async fn submit(&self, signed: &SignedDeposit) -> Result<String, ClientError<Error>> {
        let bytes = STANDARD.decode(&signed.transaction_base64).map_err(|_| {
            ClientError::Exchange(Error::InvalidRequest("invalid transaction base64"))
        })?;
        if bytes.len() < 101
            || bytes[0] != 1
            || bs58::encode(&bytes[1..65]).into_string() != signed.signature
        {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "transaction signature mismatch",
            )));
        }
        let owner: [u8; 32] = bytes[69..101]
            .try_into()
            .map_err(|_| ClientError::Exchange(Error::InvalidRequest("invalid deposit message")))?;
        let signature: [u8; 64] = bytes[1..65].try_into().map_err(|_| {
            ClientError::Exchange(Error::InvalidRequest("invalid signature length"))
        })?;
        ed25519_dalek::VerifyingKey::from_bytes(&owner)
            .map_err(|_| ClientError::Exchange(Error::Signing("invalid deposit owner")))?
            .verify_strict(
                &bytes[65..],
                &ed25519_dalek::Signature::from_bytes(&signature),
            )
            .map_err(|_| {
                ClientError::Exchange(Error::Signing("deposit signature does not match message"))
            })?;
        if self.block_height().await? > signed.last_valid_block_height {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "recent blockhash expired; reconcile before rebuilding",
            )));
        }
        {
            let mut pending = self.pending.lock().unwrap_or_else(|e| e.into_inner());
            if !pending.is_empty() {
                return Err(ClientError::Exchange(Error::PendingOutcome));
            }
            pending.insert(signed.signature.clone());
        }
        let result=self.rpc("sendTransaction",json!([signed.transaction_base64,{"encoding":"base64","skipPreflight":false,"preflightCommitment":"confirmed","maxRetries":0}]),true).await;
        if let Err(ClientError::Transport(e)) = &result
            && e.outcome == aux_rs_core::RequestOutcome::NotSent
        {
            self.pending
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(&signed.signature);
        }
        let v = result?;
        if v.as_str() != Some(&signed.signature) {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "RPC returned another transaction signature",
            )));
        }
        Ok(signed.signature.clone())
    }
    pub async fn reconcile(&self, signature: &str) -> Result<Value, ClientError<Error>> {
        let status = self.signature_status(signature).await?;
        let row = &status["value"][0];
        if !row["err"].is_null()
            || matches!(
                row["confirmationStatus"].as_str(),
                Some("confirmed" | "finalized")
            )
        {
            self.pending
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .remove(signature);
        }
        Ok(status)
    }
}
#[cfg(feature = "http")]
impl PascalChainClient<aux_rs_core::StandardTransport> {
    pub fn new(config: SolanaConfig) -> Result<Self, aux_rs_core::TransportError> {
        Self::with_config(config, aux_rs_core::TransportConfig::default())
    }
    pub fn with_config(
        config: SolanaConfig,
        transport: aux_rs_core::TransportConfig,
    ) -> Result<Self, aux_rs_core::TransportError> {
        Ok(Self::with_transport(
            config,
            aux_rs_core::StandardTransport::new(transport)?,
        ))
    }
}
