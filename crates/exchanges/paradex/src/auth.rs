use crate::*;
use aux_rs_core::{Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpTransport};
use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use serde_json::{Value, json};
use std::{
    future::Future,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};
use zeroize::Zeroizing;
pub(crate) fn now_ms() -> Result<u64, Error> {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::InvalidRequest("clock precedes Unix epoch"))?
            .as_millis(),
    )
    .map_err(|_| Error::InvalidRequest("clock overflow"))
}
struct Token {
    value: Zeroizing<String>,
    expires: u64,
    read_only: bool,
}
/// Clones share credentials. Renewal is always explicitly requested by the caller.
#[derive(Clone, Default)]
pub struct TokenState(Arc<Mutex<Option<Token>>>);
impl std::fmt::Debug for TokenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenState").finish_non_exhaustive()
    }
}
impl TokenState {
    pub(crate) fn rpc_credentials(&self) -> Result<(Zeroizing<String>, bool), Error> {
        let guard = self.0.lock().unwrap_or_else(|e| e.into_inner());
        let token = guard
            .as_ref()
            .ok_or(Error::Authentication("no bearer token attached"))?;
        if token.expires <= now_ms()? / 1000 {
            return Err(Error::Authentication(
                "JWT expired; call authenticate explicitly",
            ));
        }
        Ok((Zeroizing::new(token.value.to_string()), !token.read_only))
    }
    pub(crate) fn scope(&self) -> String {
        format!("{:p}", Arc::as_ptr(&self.0))
    }
    /// JWT decoding discovers expiry only; the exchange validates authenticity.
    pub fn set(&self, value: Zeroizing<String>, read_only: bool) -> Result<(), Error> {
        if value.contains(['\r', '\n']) {
            return Err(Error::Authentication("invalid bearer token"));
        }
        let body = value
            .split('.')
            .nth(1)
            .ok_or(Error::Authentication("invalid JWT"))?;
        let decoded = URL_SAFE_NO_PAD
            .decode(body)
            .map_err(|_| Error::Authentication("invalid JWT"))?;
        let claims: Value = serde_json::from_slice(&decoded)
            .map_err(|_| Error::Authentication("invalid JWT claims"))?;
        let expires = claims["exp"]
            .as_u64()
            .ok_or(Error::Authentication("JWT has no expiry"))?;
        if expires <= now_ms()? / 1000 {
            return Err(Error::Authentication("JWT has expired"));
        }
        *self.0.lock().unwrap_or_else(|e| e.into_inner()) = Some(Token {
            value,
            expires,
            read_only,
        });
        Ok(())
    }
    pub fn clear(&self) {
        *self.0.lock().unwrap_or_else(|e| e.into_inner()) = None;
    }
    pub fn bearer(&self, write: bool) -> Result<Zeroizing<String>, Error> {
        let guard = self.0.lock().unwrap_or_else(|e| e.into_inner());
        let token = guard.as_ref().ok_or(Error::Authentication(
            "authenticate or attach a bearer token first",
        ))?;
        if token.expires <= now_ms()? / 1000 {
            return Err(Error::Authentication(
                "JWT expired; call authenticate explicitly",
            ));
        }
        if write && token.read_only {
            return Err(Error::Authentication(
                "read-only token cannot mutate account state",
            ));
        }
        Ok(Zeroizing::new(token.value.to_string()))
    }
}
pub trait Authentication: Send + Sync {
    fn account(&self) -> &str;
    fn token(&self) -> &TokenState;
    fn prepare(
        &self,
        e: &Paradex,
        operation: &Operation,
        request: &mut HttpRequest,
    ) -> impl Future<Output = Result<(), Error>> + Send;
}
#[derive(Clone)]
pub struct ParadexAuth<S = NoSigner> {
    account: String,
    signer: S,
    chain_id: String,
    evm_context: (u64, String, String),
    subkey: Option<String>,
    tokens: TokenState,
}
impl<S> std::fmt::Debug for ParadexAuth<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParadexAuth").finish_non_exhaustive()
    }
}
impl ParadexAuth<NoSigner> {
    pub fn bearer(
        e: &Paradex,
        account: &str,
        token: Zeroizing<String>,
        read_only: bool,
    ) -> Result<Self, Error> {
        let out = Self::new(e, account, NoSigner, None)?;
        out.tokens.set(token, read_only)?;
        Ok(out)
    }
}
impl<S: Signer> ParadexAuth<S> {
    /// Prepare native SIWE text and verify the local/external EVM signature.
    /// Caller supplies issuance, expiry and nonce for an inspectable payload.
    pub async fn sign_siwe(
        &self,
        e: &Paradex,
        statement: &str,
        nonce: &str,
        issued_at: &str,
        expiration: Option<&str>,
    ) -> Result<Value, Error> {
        self.check(e)?;
        if self.signer.kind() != SignerKind::Evm {
            return Err(Error::Signing("SIWE requires an EVM signer"));
        }
        let message = siwe_message(
            e,
            &self.signer.public_key(),
            statement,
            nonce,
            issued_at,
            expiration,
        )?;
        let payload = SigningPayload {
            digest: personal_hash(message.as_bytes()),
            scheme: SigningScheme::EvmPersonal,
            typed_data: None,
            personal_message: Some(message.clone()),
        };
        let signature = self.signer.sign(&payload).await?;
        verify_signature(&self.signer, &payload, &signature)?;
        Ok(
            json!({"message":message,"message_base64":STANDARD.encode(message.as_bytes()),"signature":signature[0]}),
        )
    }
    pub async fn block_signature(
        &self,
        e: &Paradex,
        nonce: &str,
        expiration: u64,
        trades: &Value,
        parent: Option<&str>,
    ) -> Result<ResponsesBlockTradeSignature, Error> {
        let data = block_typed_data(e, nonce, expiration, trades, parent)?;
        let signature = self.sign_typed_data(e, data).await?;
        Ok(ResponsesBlockTradeSignature {
            nonce: nonce.into(),
            signature_data: serde_json::to_string(&signature)?,
            signature_expiration: expiration.into(),
            signature_timestamp: now_ms()?.into(),
            signature_type: "STARKNET".into(),
            signer_account: self.account.clone(),
            signer_public_key: Some(self.signer.public_key()),
            extra: Default::default(),
        })
    }
    pub fn with_signer<B: Signer>(&self, signer: B) -> Result<ParadexAuth<B>, Error> {
        if signer.kind() != self.signer.kind() || signer.public_key() != self.signer.public_key() {
            return Err(Error::Authentication("replacement signer identity differs"));
        }
        Ok(ParadexAuth {
            account: self.account.clone(),
            signer,
            chain_id: self.chain_id.clone(),
            evm_context: self.evm_context.clone(),
            subkey: self.subkey.clone(),
            tokens: self.tokens.clone(),
        })
    }
    pub fn new(
        e: &Paradex,
        account: &str,
        signer: S,
        subkey: Option<String>,
    ) -> Result<Self, Error> {
        let account = format!("{:#x}", felt(account)?);
        if let Some(key) = &subkey {
            felt(key)?;
            if signer.kind() != SignerKind::Starknet || felt(key)? != felt(&signer.public_key())? {
                return Err(Error::Authentication("subkey does not match signer"));
            }
        }
        Ok(Self {
            account,
            signer,
            chain_id: e.chain_id.clone(),
            evm_context: (e.evm_chain_id, e.siwe_domain.clone(), e.siwe_uri.clone()),
            subkey,
            tokens: TokenState::default(),
        })
    }
    pub fn signer(&self) -> &S {
        &self.signer
    }
    pub fn tokens(&self) -> &TokenState {
        &self.tokens
    }
    pub async fn sign_typed_data(&self, e: &Paradex, data: Value) -> Result<Vec<String>, Error> {
        self.check(e)?;
        let domain = data
            .get("domain")
            .ok_or(Error::Signing("missing signing domain"))?;
        let chain = domain
            .get("chainId")
            .and_then(Value::as_str)
            .ok_or(Error::Signing("missing signing chain"))?;
        if felt(chain).or_else(|_| short_string(chain))? != short_string(&e.chain_id)? {
            return Err(Error::Signing("typed data network mismatch"));
        }
        let payload = typed_payload(&self.account, data)?;
        let sig = self.signer.sign(&payload).await?;
        verify_signature(&self.signer, &payload, &sig)?;
        Ok(sig)
    }
    fn check(&self, e: &Paradex) -> Result<(), Error> {
        if self.chain_id != e.chain_id
            || self.evm_context != (e.evm_chain_id, e.siwe_domain.clone(), e.siwe_uri.clone())
        {
            return Err(Error::Authentication("signing network mismatch"));
        }
        Ok(())
    }
    pub async fn authentication_request(&self, e: &Paradex) -> Result<HttpRequest, Error> {
        self.check(e)?;
        let timestamp = now_ms()? / 1000;
        let expiry = timestamp + 300;
        let mut headers = vec![
            ("Content-Type".into(), "application/json".into()),
            ("PARADEX-STARKNET-ACCOUNT".into(), self.account.clone()),
        ];
        let path = if self.signer.kind() == SignerKind::Evm {
            let mut nonce = [0; 16];
            getrandom::getrandom(&mut nonce)
                .map_err(|_| Error::Signing("random nonce unavailable"))?;
            let issued = time::OffsetDateTime::from_unix_timestamp(timestamp as i64)
                .map_err(|_| Error::InvalidRequest("invalid time"))?
                .format(&time::format_description::well_known::Rfc3339)
                .map_err(|_| Error::InvalidRequest("invalid time"))?;
            let expiration = time::OffsetDateTime::from_unix_timestamp(expiry as i64)
                .map_err(|_| Error::InvalidRequest("invalid time"))?
                .format(&time::format_description::well_known::Rfc3339)
                .map_err(|_| Error::InvalidRequest("invalid time"))?;
            let message = siwe_message(
                e,
                &self.signer.public_key(),
                "Paradex Auth",
                &aux_rs_core::hex::encode(nonce),
                &issued,
                Some(&expiration),
            )?;
            let payload = SigningPayload {
                digest: personal_hash(message.as_bytes()),
                scheme: SigningScheme::EvmPersonal,
                typed_data: None,
                personal_message: Some(message.clone()),
            };
            let sig = self.signer.sign(&payload).await?;
            verify_signature(&self.signer, &payload, &sig)?;
            headers.push(("PARADEX-EVM-SIGNATURE".into(), sig[0].clone()));
            headers.push(("PARADEX-SIWE-MESSAGE".into(), STANDARD.encode(message)));
            "/v2/auth".into()
        } else {
            let sig = self
                .sign_typed_data(e, auth_typed_data(e, timestamp, expiry)?)
                .await?;
            headers.extend([
                (
                    "PARADEX-STARKNET-SIGNATURE".into(),
                    serde_json::to_string(&sig)?,
                ),
                ("PARADEX-TIMESTAMP".into(), timestamp.to_string()),
                ("PARADEX-SIGNATURE-EXPIRATION".into(), expiry.to_string()),
            ]);
            self.subkey
                .as_ref()
                .map(|key| format!("/v1/auth/{key}"))
                .unwrap_or("/v1/auth".into())
        };
        Ok(HttpRequest {
            method: HttpMethod::Post,
            url: format!("{}{path}", e.http_url()),
            headers,
            body: vec![],
        })
    }
    pub(crate) async fn sign_order(
        &self,
        e: &Paradex,
        body: &mut Value,
        modify: Option<&str>,
    ) -> Result<(), Error> {
        if body
            .get("signature")
            .and_then(Value::as_str)
            .is_some_and(|s| !s.is_empty())
        {
            return Ok(());
        }
        if body["signature_timestamp"].as_u64().is_none_or(|n| n == 0) {
            body["signature_timestamp"] = json!(now_ms()?);
        }
        let sig = self
            .sign_typed_data(e, order_typed_data(e, body, modify)?)
            .await?;
        body["signature"] = json!(serde_json::to_string(&sig)?);
        Ok(())
    }
}
impl<S: Signer> Authentication for ParadexAuth<S> {
    fn account(&self) -> &str {
        &self.account
    }
    fn token(&self) -> &TokenState {
        &self.tokens
    }
    async fn prepare(
        &self,
        e: &Paradex,
        operation: &Operation,
        request: &mut HttpRequest,
    ) -> Result<(), Error> {
        self.check(e)?;
        let token = self.tokens.bearer(!operation.safe_read)?;
        let mut body: Value = if request.body.is_empty() {
            Value::Null
        } else {
            serde_json::from_slice(&request.body)?
        };
        match (operation.method, operation.path) {
            (HttpMethod::Post, "/orders") => self.sign_order(e, &mut body, None).await?,
            (HttpMethod::Put, "/orders/{order_id}") => {
                let url = url::Url::parse(&request.url)
                    .map_err(|_| Error::InvalidRequest("invalid order URL"))?;
                let id = url
                    .path_segments()
                    .and_then(|mut s| s.next_back())
                    .ok_or(Error::InvalidRequest("missing order id"))?;
                self.sign_order(e, &mut body, Some(id)).await?;
            }
            (HttpMethod::Post, "/orders/batch") => {
                let orders = body
                    .as_array_mut()
                    .ok_or(Error::InvalidRequest("missing batch orders"))?;
                for order in orders {
                    self.sign_order(e, order, None).await?;
                }
            }
            (HttpMethod::Post, "/block-trades") => {
                if body["signatures"].get(&self.account).is_none() {
                    let nonce = body["nonce"]
                        .as_str()
                        .ok_or(Error::InvalidRequest("missing block nonce"))?;
                    let expiration = body["block_expiration"]
                        .as_u64()
                        .ok_or(Error::InvalidRequest("missing block expiration"))?;
                    let sig = self
                        .block_signature(e, nonce, expiration, &body["trades"], None)
                        .await?;
                    if body["signatures"].is_null() {
                        body["signatures"] = json!({});
                    }
                    body["signatures"][&self.account] = serde_json::to_value(sig)?;
                }
            }
            (HttpMethod::Post, "/block-trades/{block_trade_id}/offers") => {
                if body["signature"]["signature_data"]
                    .as_str()
                    .is_none_or(str::is_empty)
                {
                    let nonce = body["nonce"]
                        .as_str()
                        .ok_or(Error::InvalidRequest("missing offer nonce"))?;
                    let url = url::Url::parse(&request.url)
                        .map_err(|_| Error::InvalidRequest("invalid offer URL"))?;
                    let id = url
                        .path_segments()
                        .and_then(|mut s| s.nth(2))
                        .ok_or(Error::InvalidRequest("missing parent block id"))?;
                    let sig = self
                        .block_signature(e, nonce, now_ms()? + 300_000, &body["trades"], Some(id))
                        .await?;
                    body["signature"] = serde_json::to_value(sig)?;
                }
            }
            _ => {}
        }
        if !body.is_null() {
            request.body = serde_json::to_vec(&body)?;
        }
        request
            .headers
            .push(("Authorization".into(), format!("Bearer {}", &*token)));
        Ok(())
    }
}
impl<T: HttpTransport, S: Signer> ParadexClient<T, ParadexAuth<S>> {
    /// Explicit renewal; no order method or reconnect silently authenticates.
    pub async fn authenticate(&self) -> Result<(), ClientError<Error>> {
        self.transport.ensure_open()?;
        let wire = self
            .auth
            .authentication_request(&self.exchange)
            .await
            .map_err(ClientError::Exchange)?;
        let response: Value = crate::client::decode(self.transport.execute(wire).await?)
            .map_err(ClientError::Exchange)?;
        let token =
            response["jwt_token"]
                .as_str()
                .ok_or(ClientError::Exchange(Error::Authentication(
                    "missing JWT in auth response",
                )))?;
        self.auth
            .tokens
            .set(Zeroizing::new(token.into()), false)
            .map_err(ClientError::Exchange)
    }
}
/// SIWE text is inspectable before an external wallet signs it.
pub fn siwe_message(
    e: &Paradex,
    address: &str,
    statement: &str,
    nonce: &str,
    issued_at: &str,
    expiration: Option<&str>,
) -> Result<String, Error> {
    let address = checksum_address(address)?;
    if !nonce.bytes().all(|b| b.is_ascii_alphanumeric())
        || nonce.len() < 8
        || [statement, issued_at, expiration.unwrap_or("")]
            .iter()
            .any(|s| s.contains(['\r', '\n']))
    {
        return Err(Error::InvalidRequest("invalid SIWE fields"));
    }
    let mut message = format!(
        "{} wants you to sign in with your Ethereum account:\n{address}\n\n{statement}\n\nURI: {}\nVersion: 1\nChain ID: {}\nNonce: {nonce}\nIssued At: {issued_at}",
        e.siwe_domain, e.siwe_uri, e.evm_chain_id
    );
    if let Some(expiration) = expiration {
        message.push_str(&format!("\nExpiration Time: {expiration}"));
    }
    Ok(message)
}
fn checksum_address(value: &str) -> Result<String, Error> {
    use sha3::{Digest, Keccak256};
    let mut bytes = [0; 20];
    aux_rs_core::hex::decode_to_slice(value.strip_prefix("0x").unwrap_or(value), &mut bytes)
        .map_err(|_| Error::InvalidRequest("invalid EVM address"))?;
    let lower = aux_rs_core::hex::encode(bytes);
    let hash = Keccak256::digest(lower.as_bytes());
    let mut out = String::from("0x");
    for (i, b) in lower.bytes().enumerate() {
        let nibble = if i % 2 == 0 {
            hash[i / 2] >> 4
        } else {
            hash[i / 2] & 15
        };
        out.push(if nibble >= 8 {
            (b as char).to_ascii_uppercase()
        } else {
            b as char
        });
    }
    Ok(out)
}
/// Address derivation needs deployment class hashes, supplied explicitly from system/config.
pub fn evm_account_address(evm_address: &str, class_hash: &str) -> Result<String, Error> {
    use starknet_core::{types::Felt, utils::get_contract_address};
    let addr = felt(&checksum_address(evm_address)?)?;
    Ok(format!(
        "{:#x}",
        get_contract_address(
            addr,
            felt(class_hash)?,
            &[Felt::from(3u8), addr, Felt::ONE],
            Felt::ZERO
        )
    ))
}
