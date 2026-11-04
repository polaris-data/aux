//! Native module payloads. Inputs are human-unit decimal strings except IDs.
use crate::*;
use serde::{Deserialize, Serialize};
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SigningLeg {
    pub instrument_name: String,
    pub asset_address: String,
    pub sub_id: String,
    pub price: String,
    pub amount: String,
    pub direction: String,
}
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
#[cfg_attr(feature = "typescript", ts(optional_fields = nullable))]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ModuleData {
    Trade {
        asset: String,
        sub_id: String,
        limit_price: String,
        amount: String,
        max_fee: String,
        recipient_id: String,
        is_bid: bool,
    },
    Transfer {
        to_subaccount_id: String,
        new_subaccount_manager: String,
        asset: String,
        sub_id: String,
        amount: String,
        max_fee_usd: String,
        recipient: Option<String>,
    },
    Withdrawal {
        asset: String,
        max_fee_usd: String,
        recipient: String,
        amount: String,
        decimals: u8,
        force_batch: bool,
    },
    SessionKey {
        session_key: String,
        expiry_sec: u64,
        scopes: Vec<u8>,
        subaccount_ids: Vec<String>,
    },
    WhitelistedRecipients {
        add: Vec<String>,
        remove: Vec<String>,
    },
    RfqQuote {
        max_fee: String,
        direction: String,
        legs: Vec<SigningLeg>,
    },
    RfqExecute {
        max_fee: String,
        direction: String,
        legs: Vec<SigningLeg>,
    },
    VaultCreate {
        manager_id: String,
        deposit_spot_asset: String,
        initial_deposit: String,
        management_fee_bps: u32,
        performance_fee_bps: u32,
        max_slippage_bps: u32,
        cooldown_sec: u32,
        max_fee_usd: String,
        initial_share_price_usd: String,
        benchmark_asset: Option<String>,
    },
    VaultDeposit {
        vault_subaccount_id: String,
        deposit_spot_asset: String,
        amount: String,
    },
    VaultWithdraw {
        vault_subaccount_id: String,
        shares_to_burn: String,
    },
    VaultCancel {
        vault_subaccount_id: String,
    },
    VaultMint {
        share_price: String,
        deposit_hash: String,
    },
    VaultBurn {
        share_price: String,
        withdraw_hash: String,
    },
    VaultForceBurn {
        holder: String,
    },
    DeleteSubaccount,
    Liquidation {
        liquidate_subaccount_id: String,
        percent_of_acc: String,
        price_limit: String,
    },
    /// Explicit module bytes for protocol actions with caller-owned preparation.
    Encoded {
        module: String,
        data: Vec<u8>,
    },
}
fn dec(value: &str, signed: bool, positive: bool) -> Result<AbiValue, Error> {
    if positive && value.starts_with('-') {
        return Err(Error::InvalidRequest("amount must be positive"));
    }
    if value.split_once('.').is_some_and(|(_, f)| f.len() > 12) {
        return Err(Error::InvalidRequest(
            "Derive accepts at most 12 decimal places",
        ));
    }
    let word = decimal_word(value, 18, signed)?;
    if positive && word == [0; 32] {
        return Err(Error::InvalidRequest("amount must be positive"));
    }
    Ok(AbiValue::Word(word))
}
fn hash_word(value: &str) -> Result<AbiValue, Error> {
    let mut word = [0; 32];
    aux_rs_core::hex::decode_to_slice(value.strip_prefix("0x").unwrap_or(value), &mut word)
        .map_err(|_| Error::InvalidRequest("expected bytes32 hash"))?;
    Ok(AbiValue::Word(word))
}
impl ModuleData {
    pub fn module(&self) -> &str {
        match self {
            Self::Trade { .. } => "0xB8D20c2B7a1Ad2EE33Bc50eF10876eD3035b5e7b",
            Self::Transfer {
                recipient: None, ..
            } => "0x01259207A40925b794C8ac320456F7F6c8FE2636",
            Self::Transfer {
                recipient: Some(_), ..
            } => "0x8F9B8f12ddA05FB1F0DDDDe8f5af8cECF54f8aC9",
            Self::Withdrawal { .. } => "0x9d0E8f5b25384C7310CB8C6aE32C8fbeb645d083",
            Self::SessionKey { .. } => "0xe330CF64ff6EbF41699aad344Cb21d78db1D2bb6",
            Self::WhitelistedRecipients { .. } => "0xB86D6DE1b76c9839e4BA860848CD98A1dABd6B54",
            Self::RfqQuote { .. } | Self::RfqExecute { .. } => {
                "0x9371352CCef6f5b36EfDFE90942fFE622Ab77F1D"
            }
            Self::DeleteSubaccount => "0x1057cF9E6725a0db70932a4691f595068D9e7B30",
            Self::Liquidation { .. } => "0x66d23e59DaEEF13904eFA2D4B8658aeD05f59a92",
            Self::Encoded { module, .. } => module,
            _ => "0x2885c174ebf5524aED9c721d60c12b1537685186",
        }
    }
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        use AbiValue as A;
        let words = match self {
            Self::Trade {
                asset,
                sub_id,
                limit_price,
                amount,
                max_fee,
                recipient_id,
                is_bid,
            } => vec![
                A::address(asset)?,
                A::uint(sub_id)?,
                dec(limit_price, true, false)?,
                dec(amount, true, true)?,
                dec(max_fee, false, false)?,
                A::uint(recipient_id)?,
                A::uint(u8::from(*is_bid))?,
            ],
            Self::Transfer {
                to_subaccount_id,
                new_subaccount_manager,
                asset,
                sub_id,
                amount,
                max_fee_usd,
                recipient,
            } => {
                let mut xs = vec![
                    A::uint(to_subaccount_id)?,
                    A::uint(new_subaccount_manager)?,
                    A::address(asset)?,
                    A::uint(sub_id)?,
                    dec(amount, false, true)?,
                    dec(max_fee_usd, false, false)?,
                ];
                if let Some(r) = recipient {
                    xs.push(A::address(r)?);
                }
                xs
            }
            Self::Withdrawal {
                asset,
                max_fee_usd,
                recipient,
                amount,
                decimals,
                force_batch,
            } => {
                let amount = decimal_word(amount, usize::from(*decimals), false)?;
                if amount == [0; 32] {
                    return Err(Error::InvalidRequest("withdrawal amount must be positive"));
                }
                vec![
                    A::address(asset)?,
                    dec(max_fee_usd, false, false)?,
                    A::address(recipient)?,
                    A::Word(amount),
                    A::uint(u8::from(*force_batch))?,
                ]
            }
            Self::SessionKey {
                session_key,
                expiry_sec,
                scopes,
                subaccount_ids,
            } => vec![
                A::address(session_key)?,
                A::uint(expiry_sec)?,
                A::Array(scopes.iter().map(A::uint).collect::<Result<_, _>>()?),
                A::Array(
                    subaccount_ids
                        .iter()
                        .map(A::uint)
                        .collect::<Result<_, _>>()?,
                ),
            ],
            Self::WhitelistedRecipients { add, remove } => vec![
                A::Array(
                    add.iter()
                        .map(|s| A::address(s))
                        .collect::<Result<_, _>>()?,
                ),
                A::Array(
                    remove
                        .iter()
                        .map(|s| A::address(s))
                        .collect::<Result<_, _>>()?,
                ),
            ],
            Self::RfqQuote {
                max_fee,
                direction,
                legs,
            }
            | Self::RfqExecute {
                max_fee,
                direction,
                legs,
            } => {
                if legs.is_empty() {
                    return Err(Error::InvalidRequest("RFQ must have at least one leg"));
                }
                let mut legs = legs.clone();
                legs.sort_by(|a, b| a.instrument_name.cmp(&b.instrument_name));
                if legs
                    .windows(2)
                    .any(|w| w[0].instrument_name == w[1].instrument_name)
                {
                    return Err(Error::InvalidRequest("duplicate RFQ instrument"));
                }
                let execute = matches!(self, Self::RfqExecute { .. });
                let direction = match direction.as_str() {
                    "buy" => false,
                    "sell" => true,
                    _ => return Err(Error::InvalidRequest("invalid RFQ direction")),
                };
                let legs = legs
                    .iter()
                    .map(|leg| {
                        let negative = match leg.direction.as_str() {
                            "buy" => false,
                            "sell" => true,
                            _ => return Err(Error::InvalidRequest("invalid RFQ leg direction")),
                        } ^ direction
                            ^ execute;
                        if leg.amount.starts_with('-') {
                            return Err(Error::InvalidRequest("RFQ sizes are unsigned"));
                        }
                        let amount = dec(&leg.amount, false, true)?;
                        let amount = if negative {
                            let AbiValue::Word(mut word) = amount else {
                                return Err(Error::InvalidRequest("invalid RFQ amount"));
                            };
                            if word[0] & 0x80 != 0 {
                                return Err(Error::InvalidRequest("RFQ amount exceeds int256"));
                            }
                            for b in &mut word {
                                *b = !*b;
                            }
                            for b in word.iter_mut().rev() {
                                let (v, c) = b.overflowing_add(1);
                                *b = v;
                                if !c {
                                    break;
                                }
                            }
                            A::Word(word)
                        } else {
                            amount
                        };
                        Ok(A::Tuple(vec![
                            A::address(&leg.asset_address)?,
                            A::uint(&leg.sub_id)?,
                            dec(&leg.price, false, false)?,
                            amount,
                        ]))
                    })
                    .collect::<Result<Vec<_>, Error>>()?;
                if execute {
                    vec![
                        A::Word(keccak(abi_encode(&[A::Array(legs)])?)),
                        dec(max_fee, false, false)?,
                    ]
                } else {
                    vec![A::Tuple(vec![dec(max_fee, false, false)?, A::Array(legs)])]
                }
            }
            Self::VaultCreate {
                manager_id,
                deposit_spot_asset,
                initial_deposit,
                management_fee_bps,
                performance_fee_bps,
                max_slippage_bps,
                cooldown_sec,
                max_fee_usd,
                initial_share_price_usd,
                benchmark_asset,
            } => vec![
                A::uint(0)?,
                A::uint(manager_id)?,
                A::address(deposit_spot_asset)?,
                dec(initial_deposit, false, false)?,
                A::uint(management_fee_bps)?,
                A::uint(performance_fee_bps)?,
                A::uint(max_slippage_bps)?,
                A::uint(cooldown_sec)?,
                dec(max_fee_usd, false, false)?,
                dec(initial_share_price_usd, false, false)?,
                A::address(
                    benchmark_asset
                        .as_deref()
                        .unwrap_or("0x0000000000000000000000000000000000000000"),
                )?,
                A::uint(u8::from(benchmark_asset.is_some()))?,
            ],
            Self::VaultDeposit {
                vault_subaccount_id,
                deposit_spot_asset,
                amount,
            } => vec![
                A::uint(1)?,
                A::uint(vault_subaccount_id)?,
                A::address(deposit_spot_asset)?,
                dec(amount, false, true)?,
            ],
            Self::VaultWithdraw {
                vault_subaccount_id,
                shares_to_burn,
            } => vec![
                A::uint(2)?,
                A::uint(vault_subaccount_id)?,
                dec(shares_to_burn, false, true)?,
            ],
            Self::VaultCancel {
                vault_subaccount_id,
            } => vec![A::uint(3)?, A::uint(vault_subaccount_id)?],
            Self::VaultMint {
                share_price,
                deposit_hash,
            } => vec![
                A::uint(4)?,
                dec(share_price, false, false)?,
                hash_word(deposit_hash)?,
            ],
            Self::VaultBurn {
                share_price,
                withdraw_hash,
            } => vec![
                A::uint(5)?,
                dec(share_price, false, false)?,
                hash_word(withdraw_hash)?,
            ],
            Self::VaultForceBurn { holder } => vec![A::uint(6)?, A::address(holder)?],
            Self::Liquidation {
                liquidate_subaccount_id,
                percent_of_acc,
                price_limit,
            } => {
                let fraction = decimal_word(percent_of_acc, 18, false)?;
                if fraction == [0; 32]
                    || fraction > decimal_word("1", 18, false)?
                    || percent_of_acc
                        .split_once('.')
                        .is_some_and(|(_, f)| f.trim_end_matches('0').len() > 2)
                {
                    return Err(Error::InvalidRequest(
                        "liquidation fraction must be 0.01 to 1 in whole percent increments",
                    ));
                }
                vec![
                    A::uint(liquidate_subaccount_id)?,
                    A::Word(fraction),
                    dec(price_limit, true, false)?,
                ]
            }
            Self::DeleteSubaccount => return Ok(vec![]),
            Self::Encoded { module, data } => {
                address(module)?;
                return Ok(data.clone());
            }
        };
        abi_encode(&words)
    }
}
/// Prepare the shared envelope without changing the native request's other fields.
pub async fn sign_request<A: Authentication>(
    exchange: &Derive,
    auth: &A,
    body: &mut serde_json::Value,
    data: &ModuleData,
) -> Result<(), Error> {
    use serde_json::{Value, json};
    let subaccount = body
        .get("subaccount_id")
        .map(|v| {
            v.as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| v.to_string())
        })
        .unwrap_or("0".into());
    uint_word(&subaccount)?;
    let nonce = match body.get("nonce") {
        Some(Value::String(s)) if !s.is_empty() => s.clone(),
        Some(Value::Number(n)) if n.as_u64().is_none_or(|n| n != 0) => n.to_string(),
        _ => auth.nonce()?,
    };
    uint_word(&nonce)?;
    let expiry = body["signature_expiry_sec"]
        .as_u64()
        .filter(|n| *n > 0)
        .unwrap_or(crate::auth::now_ms()? / 1000 + 300);
    let signer = format!("0x{}", aux_rs_core::hex::encode(auth.signer_address()));
    let action = ActionPayload {
        subaccount_id: subaccount,
        nonce: nonce.clone(),
        module: data.module().into(),
        encoded_data: data.encode()?,
        expiry,
        owner: auth.wallet().into(),
        signer: signer.clone(),
    };
    let signature = auth.sign_action(exchange, &action).await?;
    // Generated specs disagree on string-or-number nonces; preserve the request
    // schema's representation while retaining every digit.
    body["nonce"] = if body["nonce"].is_number() {
        Value::Number(
            nonce
                .parse()
                .map_err(|_| Error::InvalidRequest("invalid nonce"))?,
        )
    } else {
        json!(nonce)
    };
    body["signature_expiry_sec"] = json!(expiry);
    body["signer"] = json!(signer);
    body["signature"] = json!(signature);
    Ok(())
}
