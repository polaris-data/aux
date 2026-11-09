//! Explicit Solana transaction preparation and submission. No implicit funding.
use crate::{Error, Signer, instructions::types::Instruction, public_key, verify_signature};
use aux_rs_core::{
    DefaultTransport, Error as ClientError, HttpMethod, HttpRequest, HttpResponse, HttpTransport,
};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Serialize, Deserialize)]
pub struct PreparedTransaction {
    pub message_base64: String,
    pub required_signers: Vec<String>,
    pub recent_blockhash: String,
    pub last_valid_block_height: u64,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub transaction_base64: String,
    pub signature: String,
    pub last_valid_block_height: u64,
}
impl PreparedTransaction {
    pub fn new(
        instructions: &[Instruction],
        fee_payer: &str,
        blockhash: &str,
        last_valid_block_height: u64,
    ) -> Result<Self, Error> {
        use solana_message::{AccountMeta, Address, Hash, Instruction as Ix, Message};
        let mut accounts = std::collections::BTreeSet::new();
        accounts.insert(public_key(fee_payer)?);
        let ixs = instructions
            .iter()
            .map(|ix| {
                accounts.insert(ix.program_id.to_bytes());
                for a in &ix.accounts {
                    accounts.insert(a.pubkey.to_bytes());
                }
                Ix {
                    program_id: Address::new_from_array(ix.program_id.to_bytes()),
                    accounts: ix
                        .accounts
                        .iter()
                        .map(|a| AccountMeta {
                            pubkey: Address::new_from_array(a.pubkey.to_bytes()),
                            is_signer: a.is_signer,
                            is_writable: a.is_writable,
                        })
                        .collect(),
                    data: ix.data.clone(),
                }
            })
            .collect::<Vec<_>>();
        if accounts.len() > 256 || instructions.is_empty() {
            return Err(Error::InvalidRequest(
                "transaction requires 1..256 static accounts and nonempty instructions",
            ));
        }
        let message = Message::new_with_blockhash(
            &ixs,
            Some(&Address::new_from_array(public_key(fee_payer)?)),
            &Hash::new_from_array(public_key(blockhash)?),
        );
        let bytes = message.serialize();
        let count = usize::from(message.header.num_required_signatures);
        if count > 127 || 1 + count * 64 + bytes.len() > 1232 {
            return Err(Error::InvalidRequest(
                "Solana transaction exceeds packet limit",
            ));
        }
        Ok(Self {
            message_base64: STANDARD.encode(bytes),
            required_signers: message.account_keys[..count]
                .iter()
                .map(ToString::to_string)
                .collect(),
            recent_blockhash: blockhash.into(),
            last_valid_block_height,
        })
    }
    /// Compile a v0 message with caller-reviewed address lookup table contents.
    pub fn new_v0(
        instructions: &[Instruction],
        fee_payer: &str,
        blockhash: &str,
        last_valid_block_height: u64,
        tables: &[LookupTable],
    ) -> Result<Self, Error> {
        use solana_message::{
            AccountMeta, Address, AddressLookupTableAccount, Hash, Instruction as Ix,
            VersionedMessage, v0,
        };
        if instructions.is_empty() {
            return Err(Error::InvalidRequest("nonempty instructions required"));
        }
        let ixs = instructions
            .iter()
            .map(|ix| Ix {
                program_id: Address::new_from_array(ix.program_id.to_bytes()),
                accounts: ix
                    .accounts
                    .iter()
                    .map(|a| AccountMeta {
                        pubkey: Address::new_from_array(a.pubkey.to_bytes()),
                        is_signer: a.is_signer,
                        is_writable: a.is_writable,
                    })
                    .collect(),
                data: ix.data.clone(),
            })
            .collect::<Vec<_>>();
        let tables = tables
            .iter()
            .map(|table| {
                Ok(AddressLookupTableAccount {
                    key: Address::new_from_array(public_key(&table.key)?),
                    addresses: table
                        .addresses
                        .iter()
                        .map(|a| public_key(a).map(Address::new_from_array))
                        .collect::<Result<Vec<_>, Error>>()?,
                })
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let message = v0::Message::try_compile(
            &Address::new_from_array(public_key(fee_payer)?),
            &ixs,
            &tables,
            Hash::new_from_array(public_key(blockhash)?),
        )
        .map_err(|_| Error::InvalidRequest("invalid v0 message or lookup table"))?;
        let message = VersionedMessage::V0(message);
        message
            .sanitize()
            .map_err(|_| Error::InvalidRequest("invalid v0 message"))?;
        let count = usize::from(message.header().num_required_signatures);
        let bytes = message.serialize();
        if count == 0 || count > 127 || 1 + count * 64 + bytes.len() > 1232 {
            return Err(Error::InvalidRequest(
                "Solana transaction exceeds packet limit",
            ));
        }
        Ok(Self {
            message_base64: STANDARD.encode(bytes),
            required_signers: message.static_account_keys()[..count]
                .iter()
                .map(ToString::to_string)
                .collect(),
            recent_blockhash: blockhash.into(),
            last_valid_block_height,
        })
    }
    fn message(&self) -> Result<Vec<u8>, Error> {
        if self.message_base64.len() > 1700 {
            return Err(Error::InvalidRequest("message exceeds packet limit"));
        }
        let bytes = STANDARD
            .decode(&self.message_base64)
            .map_err(|_| Error::InvalidRequest("invalid transaction message"))?;
        let m: solana_message::VersionedMessage = bincode::deserialize(&bytes)
            .map_err(|_| Error::InvalidRequest("invalid Solana message"))?;
        m.sanitize()
            .map_err(|_| Error::InvalidRequest("invalid compiled Solana message"))?;
        if m.serialize() != bytes {
            return Err(Error::InvalidRequest("noncanonical Solana message"));
        }
        let count = usize::from(m.header().num_required_signatures);
        if count == 0
            || count > m.static_account_keys().len()
            || m.static_account_keys()[..count]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                != self.required_signers
            || m.recent_blockhash().to_string() != self.recent_blockhash
        {
            return Err(Error::InvalidRequest(
                "transaction metadata differs from message",
            ));
        }
        Ok(bytes)
    }
    pub fn validate_server_transaction(&self, transaction_base64: &str) -> Result<(), Error> {
        if transaction_base64.len() > 1700 {
            return Err(Error::InvalidRequest(
                "server transaction exceeds packet limit",
            ));
        }
        let bytes = STANDARD
            .decode(transaction_base64)
            .map_err(|_| Error::InvalidRequest("invalid server transaction"))?;
        let offset = 1 + self.required_signers.len() * 64;
        if bytes.first().copied() != Some(self.required_signers.len() as u8)
            || bytes.get(offset..) != Some(self.message()?.as_slice())
        {
            return Err(Error::Signing(
                "server transaction differs from reviewed transaction",
            ));
        }
        Ok(())
    }
    pub async fn sign<S: Signer>(
        &self,
        signer: &S,
        mut signatures: BTreeMap<String, String>,
    ) -> Result<BTreeMap<String, String>, Error> {
        let message = self.message()?;
        let key = bs58::encode(signer.public_key()).into_string();
        if !self.required_signers.contains(&key) {
            return Err(Error::Signing("signer is not required by this message"));
        }
        let sig = signer.sign(&message).await?;
        verify_signature(&signer.public_key(), &message, &sig)?;
        signatures.insert(key, bs58::encode(sig).into_string());
        Ok(signatures)
    }
    pub fn assemble(
        &self,
        signatures: &BTreeMap<String, String>,
    ) -> Result<SignedTransaction, Error> {
        let message = self.message()?;
        let mut bytes = vec![self.required_signers.len() as u8];
        for key in &self.required_signers {
            let sig: [u8; 64] = bs58::decode(
                signatures
                    .get(key)
                    .ok_or(Error::Signing("missing transaction signature"))?,
            )
            .into_vec()
            .map_err(|_| Error::Signing("invalid signature"))?
            .try_into()
            .map_err(|_| Error::Signing("64-byte signature required"))?;
            verify_signature(&public_key(key)?, &message, &sig)?;
            bytes.extend(sig)
        }
        bytes.extend(message);
        if bytes.len() > 1232 {
            return Err(Error::InvalidRequest(
                "transaction exceeds Solana packet limit",
            ));
        }
        Ok(SignedTransaction {
            transaction_base64: STANDARD.encode(bytes),
            signature: signatures[&self.required_signers[0]].clone(),
            last_valid_block_height: self.last_valid_block_height,
        })
    }
}
#[derive(Clone)]
pub struct PhoenixChainClient<T = DefaultTransport> {
    rpc_url: String,
    transport: T,
    pending: Arc<Mutex<Option<SignedTransaction>>>,
}
impl<T> PhoenixChainClient<T> {
    pub fn with_transport(rpc_url: &str, transport: T) -> Result<Self, Error> {
        let mut endpoint = url::Url::parse(rpc_url)
            .map_err(|_| Error::InvalidRequest("invalid Solana RPC endpoint"))?;
        endpoint.set_query(None);
        crate::validate_url(endpoint.as_str(), false)?;
        Ok(Self {
            rpc_url: rpc_url.into(),
            transport,
            pending: Arc::default(),
        })
    }
    pub fn transport(&self) -> &T {
        &self.transport
    }
    pub fn pending(&self) -> Option<SignedTransaction> {
        self.pending
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }
    /// Only call after external reconciliation; does not imply rejection.
    pub fn clear_reconciled_submission(&self) {
        self.pending
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .take();
    }
}
#[cfg(any(feature = "http", feature = "ws"))]
impl PhoenixChainClient<aux_rs_core::StandardTransport> {
    pub fn new(rpc_url: &str) -> Result<Self, ClientError<Error>> {
        Self::with_transport(
            rpc_url,
            aux_rs_core::StandardTransport::new(Default::default())?,
        )
        .map_err(ClientError::Exchange)
    }
}
impl<T: HttpTransport> PhoenixChainClient<T> {
    async fn rpc(
        &self,
        method: &str,
        params: Value,
        safe: bool,
    ) -> Result<Value, ClientError<Error>> {
        self.transport.ensure_open()?;
        let request = HttpRequest {
            method: HttpMethod::Post,
            url: self.rpc_url.clone(),
            headers: vec![("Content-Type".into(), "application/json".into())],
            body: serde_json::to_vec(
                &json!({"jsonrpc":"2.0","id":1,"method":method,"params":params}),
            )
            .map_err(Error::from)
            .map_err(ClientError::Exchange)?,
        };
        let response: HttpResponse = if safe {
            self.transport.execute_read(request).await?
        } else {
            self.transport.execute(request).await?
        };
        let result: Value = crate::client::decode(response).map_err(ClientError::Exchange)?;
        result
            .get("result")
            .cloned()
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "missing Solana RPC result",
            )))
    }
    pub async fn latest_blockhash(&self) -> Result<Value, ClientError<Error>> {
        self.rpc(
            "getLatestBlockhash",
            json!([{"commitment":"confirmed"}]),
            true,
        )
        .await
    }
    pub async fn block_height(&self) -> Result<u64, ClientError<Error>> {
        self.rpc("getBlockHeight", json!([{"commitment":"confirmed"}]), true)
            .await?
            .as_u64()
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "invalid RPC block height",
            )))
    }
    pub async fn prepare(
        &self,
        instructions: &[Instruction],
        fee_payer: &str,
    ) -> Result<PreparedTransaction, ClientError<Error>> {
        let block = self.latest_blockhash().await?;
        let hash = block["value"]["blockhash"]
            .as_str()
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "missing recent blockhash",
            )))?;
        let height =
            block["value"]["lastValidBlockHeight"]
                .as_u64()
                .ok_or(ClientError::Exchange(Error::InvalidRequest(
                    "missing blockhash expiry",
                )))?;
        PreparedTransaction::new(instructions, fee_payer, hash, height)
            .map_err(ClientError::Exchange)
    }
    pub async fn simulate(&self, signed: &SignedTransaction) -> Result<Value, ClientError<Error>> {
        self.rpc("simulateTransaction",json!([signed.transaction_base64,{"encoding":"base64","sigVerify":true,"commitment":"confirmed"}]),true).await
    }
    pub async fn submit(&self, signed: SignedTransaction) -> Result<String, ClientError<Error>> {
        signed.validate().map_err(ClientError::Exchange)?;
        if self.block_height().await? > signed.last_valid_block_height {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "recent blockhash expired",
            )));
        }
        {
            let mut pending = self.pending.lock().unwrap_or_else(|e| e.into_inner());
            if pending.is_some() {
                return Err(ClientError::Exchange(Error::PendingOutcome));
            }
            *pending = Some(signed.clone());
        }
        let result=self.rpc("sendTransaction",json!([signed.transaction_base64,{"encoding":"base64","maxRetries":0,"skipPreflight":false}]),false).await;
        if let Err(ClientError::Transport(ref e)) = result
            && e.outcome == aux_rs_core::RequestOutcome::NotSent
        {
            self.clear_reconciled_submission();
        }
        let signature = result?
            .as_str()
            .map(str::to_owned)
            .ok_or(ClientError::Exchange(Error::InvalidRequest(
                "invalid RPC signature",
            )))?;
        if signature != signed.signature {
            return Err(ClientError::Exchange(Error::Signing(
                "RPC returned a different transaction signature",
            )));
        }
        Ok(signature)
    }
    pub async fn signature_status(&self, signature: &str) -> Result<Value, ClientError<Error>> {
        self.rpc(
            "getSignatureStatuses",
            json!([[signature],{"searchTransactionHistory":true}]),
            true,
        )
        .await
    }
    pub async fn transaction(&self, signature: &str) -> Result<Value, ClientError<Error>> {
        self.rpc("getTransaction",json!([signature,{"encoding":"json","commitment":"confirmed","maxSupportedTransactionVersion":0}]),true).await
    }
}

#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Serialize, Deserialize)]
pub struct LookupTable {
    pub key: String,
    pub addresses: Vec<String>,
}
impl SignedTransaction {
    pub fn validate(&self) -> Result<(), Error> {
        if self.transaction_base64.len() > 1700 {
            return Err(Error::InvalidRequest("transaction exceeds packet limit"));
        }
        let bytes = STANDARD
            .decode(&self.transaction_base64)
            .map_err(|_| Error::InvalidRequest("invalid transaction base64"))?;
        if bytes.len() > 1232 {
            return Err(Error::InvalidRequest("transaction exceeds packet limit"));
        }
        let count = usize::from(
            *bytes
                .first()
                .ok_or(Error::InvalidRequest("empty transaction"))?,
        );
        if count == 0 || count > 18 || bytes.len() <= 1 + count * 64 {
            return Err(Error::InvalidRequest("invalid signature count"));
        }
        let message = &bytes[1 + count * 64..];
        let decoded: solana_message::VersionedMessage = bincode::deserialize(message)
            .map_err(|_| Error::InvalidRequest("invalid Solana message"))?;
        decoded
            .sanitize()
            .map_err(|_| Error::InvalidRequest("invalid compiled Solana message"))?;
        if usize::from(decoded.header().num_required_signatures) != count
            || decoded.serialize() != message
        {
            return Err(Error::InvalidRequest(
                "signature count or canonical message mismatch",
            ));
        }
        for (i, key) in decoded.static_account_keys()[..count].iter().enumerate() {
            let signature: [u8; 64] = bytes[1 + i * 64..1 + (i + 1) * 64]
                .try_into()
                .map_err(|_| Error::Signing("invalid transaction signature"))?;
            verify_signature(&key.to_bytes(), message, &signature)?;
            if i == 0 && bs58::encode(signature).into_string() != self.signature {
                return Err(Error::Signing(
                    "transaction identity differs from signed message",
                ));
            }
        }
        Ok(())
    }
}
