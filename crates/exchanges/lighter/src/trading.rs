use crate::LighterClient;
use crate::{
    AuthHandler, Error, ExtraFields, Lighter, Signer, SigningPayload, endpoints,
    signing::SigningPurpose,
};
use aux_rs_core::{Error as ClientError, Exchange, HttpMethod, HttpRequest, HttpTransport};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX48: i64 = (1i64 << 48) - 1;

/// Native integer units; obtain the market's supported decimals before scaling.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct CreateOrder {
    pub market_index: i64,
    pub client_order_index: i64,
    pub base_amount: i64,
    pub price: u32,
    pub is_ask: u8,
    #[serde(rename = "Type")]
    pub order_type: u8,
    pub time_in_force: u8,
    pub reduce_only: u8,
    pub trigger_price: u32,
    pub order_expiry: i64,
}
impl CreateOrder {
    pub fn validate(&self) -> Result<(), Error> {
        validate_market(self.market_index)?;
        if !(0..=MAX48).contains(&self.client_order_index)
            || !(0..=MAX48).contains(&self.base_amount)
            || (self.base_amount == 0 && self.reduce_only != 1)
            || self.price == 0
            || self.is_ask > 1
            || self.reduce_only > 1
            || self.time_in_force > 2
            || self.order_expiry < 0
        {
            return Err(Error::InvalidRequest(
                "invalid order quantity, price, flags, expiry, or client index",
            ));
        }
        let valid = match self.order_type {
            0 => {
                self.trigger_price == 0
                    && ((self.time_in_force == 0 && self.order_expiry == 0)
                        || (self.time_in_force != 0 && self.order_expiry != 0))
            }
            1 => self.time_in_force == 0 && self.order_expiry == 0 && self.trigger_price == 0,
            2 | 4 => self.time_in_force == 0 && self.trigger_price != 0 && self.order_expiry != 0,
            3 | 5 => self.trigger_price != 0 && self.order_expiry != 0,
            6 => self.time_in_force == 1 && self.trigger_price == 0 && self.order_expiry != 0,
            _ => false,
        };
        if !valid {
            return Err(Error::InvalidRequest(
                "incompatible order type, time-in-force, trigger, or expiry",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct CancelOrder {
    pub market_index: i64,
    pub index: i64,
}
impl CancelOrder {
    pub fn validate(&self) -> Result<(), Error> {
        validate_market(self.market_index)?;
        if !(1..=(1i64 << 60) - 1).contains(&self.index) {
            return Err(Error::InvalidRequest("invalid order or client order index"));
        }
        Ok(())
    }
}
pub(crate) fn validate_market(id: i64) -> Result<(), Error> {
    if !(0..=MAX48).contains(&id) || id == 255 {
        Err(Error::InvalidRequest("invalid market index"))
    } else {
        Ok(())
    }
}
pub(crate) fn validate_identity(account: i64, key: u8) -> Result<(), Error> {
    // -1 is a Go construction sentinel, never an account to query or trade.
    if !(0..MAX48).contains(&account) || key == 255 {
        Err(Error::InvalidRequest("invalid account or API-key index"))
    } else {
        Ok(())
    }
}

/// The first milestone supports ordinary create/cancel attributes, including
/// integrator fees and self-trade controls. Nonce skipping is deliberately
/// rejected because it changes the sequential submission contract.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TxAttributes(BTreeMap<u8, i64>);
impl TxAttributes {
    pub fn new(values: BTreeMap<u8, i64>) -> Result<Self, Error> {
        let result = Self(values);
        result.validate()?;
        Ok(result)
    }
    pub fn values(&self) -> &BTreeMap<u8, i64> {
        &self.0
    }
    pub fn validate(&self) -> Result<(), Error> {
        if self.0.len() > 4 {
            return Err(Error::InvalidRequest("at most four transaction attributes"));
        }
        for (&kind, &value) in &self.0 {
            let max = match kind {
                1 => MAX48 - 1,
                2 | 3 => 1_000_000,
                6 => 3,
                7 => 1,
                8 => MAX48,
                _ => {
                    return Err(Error::InvalidRequest(
                        "unsupported transaction attribute (including nonce skipping)",
                    ));
                }
            };
            if !(0..=max).contains(&value) {
                return Err(Error::InvalidRequest("attribute outside native range"));
            }
        }
        let get = |kind| *self.0.get(&kind).unwrap_or(&0);
        let fees = get(2) != 0 || get(3) != 0;
        if (fees && (get(1) == 0 || get(6) != 0 || get(7) != 0)) || (get(6) == 3 && get(7) == 1) {
            return Err(Error::InvalidRequest(
                "incompatible fee/integrator/self-trade attributes",
            ));
        }
        Ok(())
    }
    fn aggregate(&self, hash: [u8; 40]) -> [u8; 40] {
        if self.0.values().all(|v| *v == 0) {
            return hash;
        }
        let mut values = [0; 8];
        for (i, (&kind, &value)) in self.0.iter().filter(|(_, v)| **v != 0).enumerate() {
            values[2 * i] = kind as u64;
            values[2 * i + 1] = value as u64;
        }
        let attributes = crate::poseidon::hash(&values);
        let elements: Vec<u64> = hash
            .chunks_exact(8)
            .chain(attributes.chunks_exact(8))
            .map(|c| u64::from_le_bytes(c.try_into().expect("eight bytes")))
            .collect();
        crate::poseidon::hash(&elements)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Transaction {
    CreateOrder(CreateOrder),
    CancelOrder(CancelOrder),
}
impl Transaction {
    pub fn tx_type(&self) -> u8 {
        match self {
            Self::CreateOrder(_) => 14,
            Self::CancelOrder(_) => 15,
        }
    }
    pub fn validate(&self) -> Result<(), Error> {
        match self {
            Self::CreateOrder(v) => v.validate(),
            Self::CancelOrder(v) => v.validate(),
        }
    }
    pub fn prepare(
        self,
        chain_id: u32,
        account_index: i64,
        api_key_index: u8,
        nonce: i64,
        options: TxOptions,
    ) -> Result<PreparedTransaction, Error> {
        self.validate()?;
        validate_identity(account_index, api_key_index)?;
        options.attributes.validate()?;
        if nonce < 0 || !(0..=MAX48).contains(&options.expired_at) {
            return Err(Error::InvalidRequest("invalid transaction nonce or expiry"));
        }
        let mut values = vec![
            chain_id as u64,
            self.tx_type() as u64,
            nonce as u64,
            options.expired_at as u64,
            account_index as u64,
            api_key_index as u64,
        ];
        match &self {
            Self::CreateOrder(v) => values.extend([
                v.market_index as u64,
                v.client_order_index as u64,
                v.base_amount as u64,
                v.price as u64,
                v.is_ask as u64,
                v.order_type as u64,
                v.time_in_force as u64,
                v.reduce_only as u64,
                v.trigger_price as u64,
                v.order_expiry as u64,
            ]),
            Self::CancelOrder(v) => values.extend([v.market_index as u64, v.index as u64]),
        }
        let digest = options.attributes.aggregate(crate::poseidon::hash(&values));
        let payload = SigningPayload {
            digest,
            chain_id,
            account_index,
            api_key_index,
            purpose: SigningPurpose::Transaction {
                tx_type: self.tx_type(),
                nonce,
            },
        };
        Ok(PreparedTransaction {
            transaction: self,
            options,
            payload,
        })
    }
}

#[derive(Clone, Debug)]
pub struct TxOptions {
    /// Milliseconds since Unix epoch, unlike auth-token deadlines (seconds).
    pub expired_at: i64,
    pub attributes: TxAttributes,
}
impl TxOptions {
    pub fn new(expired_at: i64) -> Self {
        Self {
            expired_at,
            attributes: TxAttributes::default(),
        }
    }
    pub fn short_lived() -> Result<Self, Error> {
        Ok(Self::new(
            crate::auth::unix_millis()?
                .checked_add(600_000)
                .ok_or(Error::InvalidRequest("clock overflow"))?,
        ))
    }
}

#[derive(Clone, Debug)]
pub struct PreparedTransaction {
    transaction: Transaction,
    options: TxOptions,
    payload: SigningPayload,
}
impl PreparedTransaction {
    pub fn signing_payload(&self) -> &SigningPayload {
        &self.payload
    }
    pub async fn sign<S: Signer>(self, signer: &S) -> Result<SignedTransaction, Error> {
        let sig = signer.sign(&self.payload).await?;
        sig.verify(&self.payload, signer.public_key())?;
        let mut value = match &self.transaction {
            Transaction::CreateOrder(v) => serde_json::to_value(v),
            Transaction::CancelOrder(v) => serde_json::to_value(v),
        }
        .map_err(Error::Encode)?;
        let object = value.as_object_mut().expect("native request object");
        object.insert("AccountIndex".into(), self.payload.account_index.into());
        object.insert("ApiKeyIndex".into(), self.payload.api_key_index.into());
        object.insert(
            "Nonce".into(),
            match self.payload.purpose {
                SigningPurpose::Transaction { nonce, .. } => nonce.into(),
                _ => unreachable!(),
            },
        );
        object.insert("ExpiredAt".into(), self.options.expired_at.into());
        object.insert("Sig".into(), STANDARD.encode(sig.as_bytes()).into());
        object.insert(
            "L2TxAttributes".into(),
            if self.options.attributes.0.is_empty() {
                serde_json::Value::Null
            } else {
                serde_json::to_value(self.options.attributes).map_err(Error::Encode)?
            },
        );
        let tx_info = serde_json::to_string(&value).map_err(Error::Encode)?;
        Ok(SignedTransaction {
            tx_type: self.transaction.tx_type(),
            tx_info,
            payload: self.payload,
        })
    }
}
/// Signed fields are private and cannot be changed after authentication.
pub struct SignedTransaction {
    tx_type: u8,
    tx_info: String,
    payload: SigningPayload,
}
impl std::fmt::Debug for SignedTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignedTransaction")
            .field("tx_type", &self.tx_type)
            .finish_non_exhaustive()
    }
}
impl SignedTransaction {
    pub fn tx_type(&self) -> u8 {
        self.tx_type
    }
    pub fn tx_info(&self) -> &str {
        &self.tx_info
    }
    pub fn signing_payload(&self) -> &SigningPayload {
        &self.payload
    }
    pub(crate) fn request(&self, exchange: &Lighter) -> Result<HttpRequest, Error> {
        if exchange.chain_id() != self.payload.chain_id {
            return Err(Error::NetworkMismatch);
        }
        Ok(HttpRequest {
            method: HttpMethod::Post,
            url: format!("{}/api/v1/sendTx", exchange.http_url()),
            headers: vec![(
                "content-type".into(),
                "application/x-www-form-urlencoded".into(),
            )],
            body: format!(
                "tx_type={}&tx_info={}",
                self.tx_type,
                form_encode(&self.tx_info)
            )
            .into_bytes(),
        })
    }
}
pub(crate) fn form_encode(value: &str) -> String {
    let mut out = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            out.push(byte as char)
        } else {
            use std::fmt::Write;
            write!(out, "%{byte:02X}").expect("string write");
        }
    }
    out
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendTxResponse {
    pub code: i64,
    pub tx_hash: String,
    pub predicted_execution_time_ms: i64,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

impl<T: HttpTransport, A: AuthHandler> LighterClient<T, A> {
    pub async fn create_order(
        &self,
        order: CreateOrder,
        options: TxOptions,
    ) -> Result<SendTxResponse, ClientError<Error>> {
        submit(self, Transaction::CreateOrder(order), options).await
    }
    pub async fn cancel_order(
        &self,
        order: CancelOrder,
        options: TxOptions,
    ) -> Result<SendTxResponse, ClientError<Error>> {
        submit(self, Transaction::CancelOrder(order), options).await
    }
    /// Only after reconciling pending transactions. This is never called
    /// automatically after an ambiguous submission.
    pub async fn resync_nonce(&self) -> Result<i64, ClientError<Error>> {
        self.auth()
            .check_exchange(self.exchange())
            .map_err(ClientError::Exchange)?;
        let mut guard = self.auth().nonce_sequence().lock().await;
        let next = self
            .request(crate::NextNonceRequest {
                account_index: self.auth().account_index(),
                api_key_index: self.auth().api_key_index(),
            })
            .await?
            .nonce;
        if next < 0 {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "negative server nonce",
            )));
        }
        guard.set(next);
        Ok(next)
    }
}
async fn submit<T: HttpTransport, A: AuthHandler>(
    client: &LighterClient<T, A>,
    transaction: Transaction,
    options: TxOptions,
) -> Result<SendTxResponse, ClientError<Error>> {
    client.transport().ensure_open()?;
    let auth = client.auth();
    auth.check_exchange(client.exchange())
        .map_err(ClientError::Exchange)?;
    transaction.validate().map_err(ClientError::Exchange)?;
    options
        .attributes
        .validate()
        .map_err(ClientError::Exchange)?;
    let mut guard = auth.nonce_sequence().lock().await;
    client.transport().ensure_open()?;
    if guard.uncertain() {
        return Err(ClientError::Exchange(Error::NonceUncertain));
    }
    let nonce = match guard.next() {
        Some(n) => n,
        None => {
            let n = client
                .request(crate::NextNonceRequest {
                    account_index: auth.account_index(),
                    api_key_index: auth.api_key_index(),
                })
                .await?
                .nonce;
            guard.set(n);
            n
        }
    };
    let prepared = transaction
        .prepare(
            client.exchange().chain_id(),
            auth.account_index(),
            auth.api_key_index(),
            nonce,
            options,
        )
        .map_err(ClientError::Exchange)?;
    let signed = prepared
        .sign(auth.signer())
        .await
        .map_err(ClientError::Exchange)?;
    let wire = signed
        .request(client.exchange())
        .map_err(ClientError::Exchange)?;
    client.transport().ensure_open()?;
    // Set before polling transport. Cancellation, panic, rejected/unknown responses,
    // or decoding errors all retain quarantine. The lock spans the entire send.
    guard.mark_uncertain();
    let response = client.transport().execute(wire).await?;
    let decoded: SendTxResponse = endpoints::decode(response).map_err(ClientError::Exchange)?;
    if let Some(next) = nonce.checked_add(1) {
        guard.set(next);
    }
    Ok(decoded)
}
