use crate::*;
use alloy_primitives::{Address, keccak256};
use serde_json::{Value, json};
use std::sync::Arc;
use zeroize::Zeroizing;
fn pick(v: &Value, fields: &[&str]) -> Value {
    Value::Array(
        fields
            .iter()
            .filter_map(|k| v.get(*k).filter(|v| !v.is_null()).cloned())
            .collect(),
    )
}
/// Canonical MessagePack sequence for the published JSON operation envelope.
pub fn perps_signable_operation(op: &Value) -> Result<Value, Error> {
    if op.is_array() {
        return Ok(op.clone());
    }
    let kind = op["type"]
        .as_str()
        .ok_or(Error::InvalidRequest("missing perps operation type"))?;
    let a = &op["args"];
    let args = match kind {
        "createOrders" => {
            let rows = a
                .as_array()
                .ok_or(Error::InvalidRequest("order array required"))?;
            let mut result = Vec::new();
            for order in rows {
                let mut xs = Vec::new();
                for k in [
                    "iid",
                    "buy",
                    "p",
                    "qty",
                    "tif",
                    "po",
                    "ro",
                    "c",
                    "tr",
                    "gtd_expiry",
                    "builder",
                ] {
                    if let Some(v) = order.get(k).filter(|v| !v.is_null()) {
                        if k == "ro" && v == false {
                            continue;
                        }
                        xs.push(if k == "tr" {
                            pick(v, &["market", "trp", "tpsl", "trail_bps", "act"])
                        } else if k == "builder" {
                            pick(v, &["address", "fee_rate"])
                        } else {
                            v.clone()
                        });
                    } else if k == "po" {
                        xs.push(json!(false));
                    }
                }
                result.push(Value::Array(xs));
            }
            Value::Array(result)
        }
        "cancelOrders" | "cancelOrdersCOID" => a.clone(),
        "modifyOrders" | "modifyOrdersCOID" => Value::Array(
            a.as_array()
                .ok_or(Error::InvalidRequest("modify array required"))?
                .iter()
                .map(|v| {
                    pick(
                        v,
                        &[
                            if kind == "modifyOrders" {
                                "oid"
                            } else {
                                "coid"
                            },
                            "p",
                            "qty",
                        ],
                    )
                })
                .collect(),
        ),
        "createChase" => pick(
            a,
            &[
                "iid",
                "buy",
                "qty",
                "lim",
                "max_dist",
                "max_dist_bps",
                "po",
                "ro",
                "c",
            ],
        ),
        "cancelChase" => pick(a, &["chid"]),
        "createTwap" => pick(
            a,
            &[
                "iid", "buy", "qty", "dur", "ivl", "rnd", "slip_bps", "min_px", "max_px", "ro", "c",
            ],
        ),
        "controlTwap" => pick(a, &["twid", "act"]),
        "cancelTwap" => pick(a, &["twid"]),
        "cancelAll" => pick(a, &["iid"]),
        "autoCancel" => pick(a, &["time"]),
        "updateLeverage" => pick(a, &["iid", "lev", "cross"]),
        "updateLeverages" => Value::Array(
            a.as_array()
                .ok_or(Error::InvalidRequest("leverage array required"))?
                .iter()
                .map(|v| pick(v, &["iid", "lev", "cross"]))
                .collect(),
        ),
        "updateMargin" => pick(a, &["iid", "amt"]),
        "deleteProxy" => pick(a, &["proxy"]),
        "updateBuilder" => pick(a, &["name"]),
        "internalTransfer" => pick(a, &["account", "token", "amount", "to"]),
        "approveBuilder" => pick(a, &["builder", "max_fee_rate", "approval_version"]),
        _ => {
            return Err(Error::InvalidRequest(
                "unsupported perps operation for Op signing",
            ));
        }
    };
    let mut result = vec![json!(kind), args];
    if let Some(group) = op.get("grp").filter(|v| !v.is_null()) {
        result.push(group.clone());
    }
    Ok(Value::Array(result))
}
/// Encode signable arrays with native MessagePack integer widths. In particular,
/// serde_json's arbitrary-precision private Number representation is never serialized.
pub fn perps_operation_bytes(value: &Value) -> Result<Vec<u8>, Error> {
    fn encode(v: &Value, out: &mut Vec<u8>) -> Result<(), Error> {
        let failure = || Error::Signing("MessagePack encoding failed");
        match v {
            Value::Null => rmp::encode::write_nil(out).map_err(|_| failure())?,
            Value::Bool(b) => rmp::encode::write_bool(out, *b).map_err(|_| failure())?,
            Value::String(s) => rmp::encode::write_str(out, s).map_err(|_| failure())?,
            Value::Number(n) => {
                if let Some(n) = n.as_u64() {
                    rmp::encode::write_uint(out, n).map_err(|_| failure())?;
                } else if let Some(n) = n.as_i64() {
                    rmp::encode::write_sint(out, n).map_err(|_| failure())?;
                } else {
                    return Err(Error::InvalidRequest(
                        "perps signable numbers must fit native integers",
                    ));
                }
            }
            Value::Array(xs) => {
                rmp::encode::write_array_len(out, u32::try_from(xs.len()).map_err(|_| failure())?)
                    .map_err(|_| failure())?;
                for x in xs {
                    encode(x, out)?;
                }
            }
            Value::Object(_) => {
                return Err(Error::InvalidRequest("canonical signable arrays required"));
            }
        }
        Ok(())
    }
    let mut out = Vec::new();
    encode(value, &mut out)?;
    Ok(out)
}
pub fn perps_op_typed_data(
    chain: u64,
    op: &Value,
    salt: u64,
    timestamp: u64,
) -> Result<Value, Error> {
    let signable = perps_signable_operation(op)?;
    // Native signable data uses integer IDs/timestamps and decimal strings. Never encode a JSON decimal as binary float.
    fn validate(v: &Value) -> Result<(), Error> {
        match v {
            Value::Number(n) if n.as_u64().is_none() && n.as_i64().is_none() => Err(
                Error::InvalidRequest("perps signable numbers must be integers"),
            ),
            Value::Array(xs) => {
                for x in xs {
                    validate(x)?;
                }
                Ok(())
            }
            Value::Object(_) => Err(Error::InvalidRequest(
                "perps signable data must be canonical arrays",
            )),
            _ => Ok(()),
        }
    }
    validate(&signable)?;
    let bytes = perps_operation_bytes(&signable)?;
    Ok(
        json!({"domain":{"name":"Polymarket","version":"1","chainId":chain},"primaryType":"Op","types":{"EIP712Domain":[{"name":"name","type":"string"},{"name":"version","type":"string"},{"name":"chainId","type":"uint256"}],"Op":[{"name":"data","type":"bytes32"},{"name":"salt","type":"uint64"},{"name":"ts","type":"uint64"}]},"message":{"data":keccak256(bytes),"salt":salt,"ts":timestamp}}),
    )
}
#[derive(Clone)]
pub struct PerpsCredentials {
    pub proxy: Address,
    secret: Arc<Zeroizing<String>>,
    pub expires_at_ms: Option<u64>,
}
impl std::fmt::Debug for PerpsCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PerpsCredentials([redacted])")
    }
}
impl PerpsCredentials {
    pub fn new(
        proxy: Address,
        secret: Zeroizing<String>,
        expires_at_ms: Option<u64>,
    ) -> Result<Self, Error> {
        if secret.is_empty() || secret.contains(['\r', '\n']) {
            return Err(Error::Authentication("invalid perps secret"));
        }
        Ok(Self {
            proxy,
            secret: Arc::new(secret),
            expires_at_ms,
        })
    }
    pub fn headers(&self) -> Result<Vec<(String, String)>, Error> {
        if self
            .expires_at_ms
            .is_some_and(|t| crate::unix_millis().is_ok_and(|now| now >= t))
        {
            return Err(Error::Authentication("perps credentials expired"));
        }
        Ok(vec![
            ("POLYMARKET-PROXY".into(), self.proxy.to_string()),
            ("POLYMARKET-SECRET".into(), self.secret.as_str().to_owned()),
        ])
    }
    pub fn websocket_auth(&self) -> Result<Value, Error> {
        self.headers()?;
        Ok(json!({"proxy":self.proxy,"secret":self.secret.as_str()}))
    }
}
#[derive(Clone)]
pub struct PolymarketPerpsAuth<S = Arc<LocalEvmSigner>> {
    pub owner: Option<S>,
    pub proxy: Option<S>,
    pub credentials: Option<PerpsCredentials>,
    pub withdrawal_contract: Option<Address>,
}
impl<S> std::fmt::Debug for PolymarketPerpsAuth<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PolymarketPerpsAuth([redacted])")
    }
}
impl<S> Default for PolymarketPerpsAuth<S> {
    fn default() -> Self {
        Self {
            owner: None,
            proxy: None,
            credentials: None,
            withdrawal_contract: None,
        }
    }
}
impl<S: EvmSigner + Clone + 'static> PolymarketPerpsAuth<S> {
    pub async fn sign_body(&self, e: &Polymarket, body: &mut Value) -> Result<(), Error> {
        if body
            .get("sig")
            .and_then(Value::as_str)
            .is_some_and(|s| !s.is_empty())
        {
            return Ok(());
        }
        let kind = body["op"]["type"]
            .as_str()
            .ok_or(Error::InvalidRequest("missing perps operation"))?
            .to_owned();
        let owner_op = matches!(
            kind.as_str(),
            "createProxy" | "deleteProxy" | "withdraw" | "approveBuilder" | "updateBuilder"
        );
        let owner_op = owner_op || kind == "internalTransfer" && self.owner.is_some();
        let signer = if owner_op {
            self.owner.as_ref()
        } else {
            self.proxy.as_ref()
        }
        .ok_or(Error::Authentication(
            "required perps owner/proxy signer missing",
        ))?;
        if !owner_op
            && self
                .credentials
                .as_ref()
                .is_some_and(|c| c.proxy != signer.address())
        {
            return Err(Error::Authentication(
                "proxy signer differs from the credential identity",
            ));
        }
        if !owner_op && let Some(credentials) = &self.credentials {
            credentials.headers()?;
        }
        let salt = if let Some(v) = body.get("salt").filter(|v| !v.is_null()) {
            integer(v)?
        } else {
            let mut bytes = [0u8; 4];
            getrandom::getrandom(&mut bytes)
                .map_err(|_| Error::Signing("OS randomness unavailable"))?;
            u32::from_le_bytes(bytes).into()
        };
        let ts = if let Some(v) = body.get("ts").filter(|v| !v.is_null()) {
            integer(v)?
        } else if kind == "withdraw" {
            unix_seconds()?
        } else {
            unix_millis()?
        };
        let args = &body["op"]["args"];
        let mut td = if kind == "createProxy" {
            json!({"domain":{"name":"Polymarket","version":"1","chainId":e.chain_id},"primaryType":"CreateProxy","types":{"CreateProxy":[{"name":"addr","type":"address"},{"name":"exp","type":"uint64"},{"name":"salt","type":"uint64"},{"name":"ts","type":"uint64"}]},"message":{"addr":args["proxy"],"exp":args["expiry"],"salt":salt,"ts":ts}})
        } else if kind == "withdraw" {
            json!({"domain":{"name":"Polymarket","version":"1","chainId":e.chain_id,"verifyingContract":self.withdrawal_contract.ok_or(Error::InvalidRequest("perps withdrawal contract must be configured"))?},"primaryType":"Withdraw","types":{"Withdraw":[{"name":"account","type":"address"},{"name":"token","type":"address"},{"name":"amount","type":"uint256"},{"name":"fee","type":"uint256"},{"name":"to","type":"address"},{"name":"salt","type":"uint64"},{"name":"ts","type":"uint64"}]},"message":{"account":args["account"],"token":args["token"],"amount":args["amount"],"fee":0,"to":args["to"],"salt":salt,"ts":ts}})
        } else {
            perps_op_typed_data(e.chain_id, &body["op"], salt, ts)?
        };
        if td["types"].get("EIP712Domain").is_none() {
            let mut fields = vec![
                json!({"name":"name","type":"string"}),
                json!({"name":"version","type":"string"}),
                json!({"name":"chainId","type":"uint256"}),
            ];
            if kind == "withdraw" {
                fields.push(json!({"name":"verifyingContract","type":"address"}));
            }
            td["types"]["EIP712Domain"] = json!(fields);
        }
        let sig = sign_typed_data(signer, &td).await?;
        body["sig"] = json!(sig.to_string());
        body["salt"] = json!(salt);
        body["ts"] = json!(ts);
        Ok(())
    }
}
impl<S: EvmSigner + Clone + 'static> Authentication for PolymarketPerpsAuth<S> {
    fn identity(&self) -> String {
        format!(
            "perps:{}",
            self.credentials
                .as_ref()
                .map(|c| c.proxy.to_string())
                .unwrap_or_default()
        )
    }
    async fn prepare(
        &self,
        e: &Polymarket,
        _: &Operation,
        r: &mut aux_rs_core::HttpRequest,
    ) -> Result<(), Error> {
        if !r.body.is_empty() {
            let mut b: Value = serde_json::from_slice(&r.body)?;
            if b.get("op").is_some() {
                self.sign_body(e, &mut b).await?;
                r.body = serde_json::to_vec(&b)?;
                return Ok(());
            }
        }
        r.headers.extend(
            self.credentials
                .as_ref()
                .ok_or(Error::Authentication("perps proxy credentials required"))?
                .headers()?,
        );
        Ok(())
    }
}
