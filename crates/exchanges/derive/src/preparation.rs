use crate::*;
use aux_rs_core::{Error as ClientError, HttpTransport, WebSocketConnection};
use serde_json::{Value, json};
pub(crate) trait NativeRead: Send {
    fn read(
        &mut self,
        path: &str,
        params: Value,
    ) -> impl std::future::Future<Output = Result<Value, ClientError<Error>>> + Send;
}
impl<T: HttpTransport, A: Sync> NativeRead for &DeriveClient<T, A> {
    async fn read(&mut self, path: &str, params: Value) -> Result<Value, ClientError<Error>> {
        self.execute_public(path, params).await
    }
}
impl<C: WebSocketConnection, A: Send + Sync> NativeRead for RpcSession<C, A> {
    async fn read(&mut self, path: &str, params: Value) -> Result<Value, ClientError<Error>> {
        self.call(path.trim_start_matches('/'), params).await
    }
}
fn text(body: &Value, key: &str) -> Result<String, Error> {
    body[key]
        .as_str()
        .map(str::to_owned)
        .ok_or(Error::InvalidRequest("missing signed decimal or address"))
}
fn integer(body: &Value, key: &str) -> Result<String, Error> {
    let v = body
        .get(key)
        .ok_or(Error::InvalidRequest("missing signed integer"))?;
    let s = v
        .as_str()
        .map(str::to_owned)
        .unwrap_or_else(|| v.to_string());
    uint_word(&s)?;
    Ok(s)
}
fn u32_field(body: &Value, key: &str) -> Result<u32, Error> {
    body[key]
        .as_u64()
        .and_then(|n| u32::try_from(n).ok())
        .ok_or(Error::InvalidRequest("signed field must fit uint32"))
}
async fn asset(
    reader: &mut impl NativeRead,
    name: &str,
) -> Result<(String, u8), ClientError<Error>> {
    if name.eq_ignore_ascii_case("0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee") {
        return Ok((name.into(), 18));
    }
    let currencies = reader.read("/public/get_all_currencies", json!({})).await?;
    let result = (|| -> Result<(String, u8), Error> {
        let mut found = Vec::new();
        for currency in currencies
            .as_array()
            .ok_or(Error::InvalidRequest("invalid currency metadata"))?
        {
            for spot in currency["spot"]
                .as_array()
                .ok_or(Error::InvalidRequest("invalid spot metadata"))?
            {
                let decimals = spot["erc20"]["decimals"].as_u64().unwrap_or_default();
                if decimals == 0 && spot["erc20"]["underlying_erc20"].is_null() {
                    continue;
                }
                let addr = spot["address"]
                    .as_str()
                    .ok_or(Error::InvalidRequest("missing spot asset address"))?;
                if currency["currency"]
                    .as_str()
                    .is_some_and(|c| c.eq_ignore_ascii_case(name))
                    || addr.eq_ignore_ascii_case(name)
                {
                    found.push((
                        addr.into(),
                        u8::try_from(decimals)
                            .map_err(|_| Error::InvalidRequest("invalid underlying decimals"))?,
                    ));
                }
            }
        }
        if found.len() != 1 {
            return Err(Error::InvalidRequest(
                "unknown or ambiguous spot asset; use a deposit-enabled protocol address",
            ));
        }
        Ok(found.remove(0))
    })();
    result.map_err(ClientError::Exchange)
}
pub(crate) async fn prepare_native(
    reader: &mut impl NativeRead,
    exchange: &Derive,
    auth: &impl Authentication,
    path: &str,
    body: &mut Value,
) -> Result<(), ClientError<Error>> {
    if path == "/private/transfer_positions" {
        for (key, route) in [
            ("maker_params", "/private/send_quote"),
            ("taker_params", "/private/execute_quote"),
        ] {
            let child = body
                .get_mut(key)
                .ok_or(ClientError::Exchange(Error::InvalidRequest(
                    "missing position transfer side",
                )))?;
            prepare_one(reader, exchange, auth, route, child).await?;
        }
        return Ok(());
    }
    prepare_one(reader, exchange, auth, path, body).await
}
async fn prepare_one(
    reader: &mut impl NativeRead,
    exchange: &Derive,
    auth: &impl Authentication,
    path: &str,
    body: &mut Value,
) -> Result<(), ClientError<Error>> {
    if !body
        .get("signature")
        .is_some_and(|s| s.as_str().is_none_or(str::is_empty))
    {
        return Ok(());
    }
    let native = (|| -> Result<Option<ModuleData>, Error> {
        Ok(match path {
            "/private/delete_subaccount" => Some(ModuleData::DeleteSubaccount),
            "/private/liquidate" => Some(ModuleData::Liquidation {
                liquidate_subaccount_id: integer(body, "liquidate_subaccount_id")?,
                percent_of_acc: text(body, "percent_of_acc")?,
                price_limit: text(body, "price_limit")?,
            }),
            "/private/set_session_key" => {
                let scope_names = [
                    "admin",
                    "withdraw",
                    "trade:all",
                    "trade:orderbook:all",
                    "trade:orderbook:spot",
                    "trade:orderbook:perp",
                    "trade:orderbook:option",
                    "trade:rfq:all",
                    "trade:rfq:spot",
                    "trade:rfq:perp",
                    "trade:rfq:option",
                    "transfer:all",
                    "transfer:existing_subaccount",
                    "transfer:new_subaccount",
                    "transfer:different_owner_subaccount",
                    "set_session_key",
                    "liquidate",
                    "vault:all",
                    "vault:curator_create",
                    "vault:curator_mint_and_burn",
                    "vault:user_deposit",
                    "vault:user_withdraw",
                    "vault:user_cancel",
                ];
                let scopes = body["protocol_scopes"]
                    .as_array()
                    .ok_or(Error::InvalidRequest("missing protocol scopes"))?
                    .iter()
                    .map(|s| {
                        scope_names
                            .iter()
                            .position(|name| s == name)
                            .map(|n| n as u8)
                            .ok_or(Error::InvalidRequest("unknown protocol scope"))
                    })
                    .collect::<Result<_, _>>()?;
                let subaccount_ids = body["subaccount_ids"]
                    .as_array()
                    .map(|xs| xs.iter().map(|v| v.to_string()).collect())
                    .unwrap_or_default();
                Some(ModuleData::SessionKey {
                    session_key: text(body, "public_session_key")?,
                    expiry_sec: body["expiry_sec"]
                        .as_u64()
                        .ok_or(Error::InvalidRequest("invalid session expiry"))?,
                    scopes,
                    subaccount_ids,
                })
            }
            "/private/update_whitelisted_recipients" => Some(ModuleData::WhitelistedRecipients {
                add: serde_json::from_value(body["add"].clone())?,
                remove: serde_json::from_value(body["remove"].clone())?,
            }),
            "/private/create_vault" => Some(ModuleData::VaultCreate {
                manager_id: integer(body, "manager_id")?,
                deposit_spot_asset: text(body, "deposit_spot_asset")?,
                initial_deposit: text(body, "initial_deposit")?,
                management_fee_bps: u32_field(body, "management_fee_bps")?,
                performance_fee_bps: u32_field(body, "performance_fee_bps")?,
                max_slippage_bps: u32_field(body, "max_slippage_bps")?,
                cooldown_sec: u32_field(body, "cooldown_sec")?,
                max_fee_usd: text(body, "max_fee_usd")?,
                initial_share_price_usd: text(body, "initial_share_price_usd")?,
                benchmark_asset: body["benchmark_asset"].as_str().map(str::to_owned),
            }),
            "/private/request_vault_deposit" => Some(ModuleData::VaultDeposit {
                vault_subaccount_id: integer(body, "vault_subaccount_id")?,
                deposit_spot_asset: text(body, "deposit_spot_asset")?,
                amount: text(body, "amount")?,
            }),
            "/private/request_vault_withdraw" => Some(ModuleData::VaultWithdraw {
                vault_subaccount_id: integer(body, "vault_subaccount_id")?,
                shares_to_burn: text(body, "shares_to_burn")?,
            }),
            "/private/cancel_all_vault_requests" => Some(ModuleData::VaultCancel {
                vault_subaccount_id: integer(body, "vault_subaccount_id")?,
            }),
            "/private/mint_vault_shares" => Some(ModuleData::VaultMint {
                share_price: text(body, "share_price")?,
                deposit_hash: text(body, "deposit_hash")?,
            }),
            "/private/burn_vault_shares" => Some(ModuleData::VaultBurn {
                share_price: text(body, "share_price")?,
                withdraw_hash: text(body, "withdraw_hash")?,
            }),
            "/private/force_burn" => Some(ModuleData::VaultForceBurn {
                holder: text(body, "holder")?,
            }),
            _ => None,
        })
    })()
    .map_err(ClientError::Exchange)?;
    let data =
        if let Some(data) = native {
            data
        } else {
            match path {
                "/private/order" | "/private/replace" | "/private/order_quote" => {
                    let name = text(body, "instrument_name").map_err(ClientError::Exchange)?;
                    let instrument = reader
                        .read("/public/get_instrument", json!({"instrument_name":name}))
                        .await?;
                    ModuleData::Trade {
                        asset: text(&instrument, "base_asset_address")
                            .map_err(ClientError::Exchange)?,
                        sub_id: text(&instrument, "base_asset_sub_id")
                            .map_err(ClientError::Exchange)?,
                        limit_price: text(body, "limit_price").map_err(ClientError::Exchange)?,
                        amount: text(body, "amount").map_err(ClientError::Exchange)?,
                        max_fee: text(body, "max_fee").map_err(ClientError::Exchange)?,
                        recipient_id: integer(body, "subaccount_id")
                            .map_err(ClientError::Exchange)?,
                        is_bid: match body["direction"].as_str() {
                            Some("buy") => true,
                            Some("sell") => false,
                            _ => {
                                return Err(ClientError::Exchange(Error::InvalidRequest(
                                    "invalid direction",
                                )));
                            }
                        },
                    }
                }
                "/private/transfer_spot" | "/private/transfer_spot_external" => {
                    let (address, _) = asset(
                        reader,
                        &text(body, "asset_name").map_err(ClientError::Exchange)?,
                    )
                    .await?;
                    ModuleData::Transfer {
                        to_subaccount_id: integer(body, "to_subaccount_id")
                            .map_err(ClientError::Exchange)?,
                        new_subaccount_manager: integer(body, "new_subaccount_manager")
                            .map_err(ClientError::Exchange)?,
                        asset: address,
                        sub_id: integer(body, "sub_id").map_err(ClientError::Exchange)?,
                        amount: text(body, "amount").map_err(ClientError::Exchange)?,
                        max_fee_usd: text(body, "max_fee_usd").map_err(ClientError::Exchange)?,
                        recipient: if path.ends_with("external") {
                            Some(text(body, "recipient_address").map_err(ClientError::Exchange)?)
                        } else {
                            None
                        },
                    }
                }
                "/private/withdraw" => {
                    let (address, decimals) = asset(
                        reader,
                        &text(body, "asset_name").map_err(ClientError::Exchange)?,
                    )
                    .await?;
                    ModuleData::Withdrawal {
                        asset: address,
                        max_fee_usd: text(body, "max_fee_usd").map_err(ClientError::Exchange)?,
                        recipient: body["recipient"].as_str().unwrap_or(auth.wallet()).into(),
                        amount: text(body, "amount_in_underlying")
                            .map_err(ClientError::Exchange)?,
                        decimals,
                        force_batch: body["force_batch"].as_bool().unwrap_or(false),
                    }
                }
                "/private/send_quote" | "/private/replace_quote" | "/private/execute_quote" => {
                    let mut legs = Vec::new();
                    for leg in body["legs"].as_array().ok_or(ClientError::Exchange(
                        Error::InvalidRequest("missing quote legs"),
                    ))? {
                        let name = text(leg, "instrument_name").map_err(ClientError::Exchange)?;
                        let instrument = reader
                            .read("/public/get_instrument", json!({"instrument_name":name}))
                            .await?;
                        legs.push(SigningLeg {
                            instrument_name: name,
                            asset_address: text(&instrument, "base_asset_address")
                                .map_err(ClientError::Exchange)?,
                            sub_id: text(&instrument, "base_asset_sub_id")
                                .map_err(ClientError::Exchange)?,
                            price: text(leg, "price").map_err(ClientError::Exchange)?,
                            amount: text(leg, "amount").map_err(ClientError::Exchange)?,
                            direction: text(leg, "direction").map_err(ClientError::Exchange)?,
                        });
                    }
                    let direction = text(body, "direction").map_err(ClientError::Exchange)?;
                    let max_fee = text(body, "max_fee").map_err(ClientError::Exchange)?;
                    if path == "/private/execute_quote" {
                        ModuleData::RfqExecute {
                            max_fee,
                            direction,
                            legs,
                        }
                    } else {
                        ModuleData::RfqQuote {
                            max_fee,
                            direction,
                            legs,
                        }
                    }
                }
                _ => {
                    return Err(ClientError::Exchange(Error::Signing(
                        "this action requires explicit module preparation with sign_request",
                    )));
                }
            }
        };
    sign_request(exchange, auth, body, &data)
        .await
        .map_err(ClientError::Exchange)
}
