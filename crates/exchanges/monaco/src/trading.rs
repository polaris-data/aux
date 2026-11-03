use crate::*;
use aux_rs_core::{Error as ClientError, HttpMethod, HttpTransport};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

fn decimal(value: &str) -> Result<(), Error> {
    aux_rs_core::positive_decimal(value).map_err(Error::InvalidRequest)
}
fn cmp_decimal(a: &str, b: &str) -> std::cmp::Ordering {
    let parts = |s: &str| {
        let (whole, fraction) = s.split_once('.').unwrap_or((s, ""));
        (
            whole.trim_start_matches('0').to_owned(),
            fraction.trim_end_matches('0').to_owned(),
        )
    };
    let (ai, af) = parts(a);
    let (bi, bf) = parts(b);
    ai.len()
        .cmp(&bi.len())
        .then_with(|| ai.cmp(&bi))
        .then_with(|| {
            let n = af.len().max(bf.len());
            af.bytes()
                .chain(std::iter::repeat(b'0'))
                .take(n)
                .cmp(bf.bytes().chain(std::iter::repeat(b'0')).take(n))
        })
}
fn identifier(s: &str) -> Result<(), Error> {
    if s.is_empty()
        || s.len() > 64
        || !s
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
    {
        return Err(Error::InvalidRequest(
            "expected a 1-64 character native correlation/idempotency key",
        ));
    }
    Ok(())
}
fn shared_fields(v: &Value) -> Result<(), Error> {
    for key in ["marginAccountId", "riskBucketId"] {
        if let Some(s) = v.get(key).and_then(Value::as_str) {
            uuid(s)?;
        }
    }
    if let Some(s) = v.get("idempotencyKey").and_then(Value::as_str) {
        identifier(s)?;
    }
    if let Some(s) = v.get("clientOrderId").and_then(Value::as_str)
        && !s.trim().is_empty()
    {
        identifier(s.trim())?;
    }
    for (key, choices) in [
        ("marginMode", &["ISOLATED", "CROSS"][..]),
        (
            "selfTradePreventionMode",
            &["CANCEL_MAKER", "CANCEL_TAKER", "CANCEL_BOTH", "SKIP"][..],
        ),
    ] {
        if let Some(s) = v.get(key).and_then(Value::as_str) {
            one_of(s, choices)?;
        }
    }
    for key in ["leverage", "riskBucketCollateral"] {
        if let Some(s) = v.get(key).and_then(Value::as_str) {
            decimal(s)?;
        }
    }
    for key in ["takeProfit", "stopLoss", "trailingStop"] {
        if let Some(object) = v.get(key).and_then(Value::as_object) {
            for (k, v) in object {
                if [
                    "triggerPrice",
                    "limitPrice",
                    "quantity",
                    "trailingDelta",
                    "activationPrice",
                ]
                .contains(&k.as_str())
                    && let Some(s) = v.as_str()
                {
                    decimal(s)?;
                }
            }
        }
    }
    Ok(())
}
fn increments(
    market: &TradingPairData,
    price: Option<&str>,
    quantity: Option<&str>,
) -> Result<(), Error> {
    if let Some(p) = price {
        decimal(p)?;
        let step = market
            .tick_size
            .as_deref()
            .ok_or(Error::InvalidRequest("market tickSize required"))?;
        aux_rs_core::decimal_units(p, step).map_err(Error::InvalidRequest)?;
    }
    if let Some(q) = quantity {
        decimal(q)?;
        let step = market
            .quantity_step_size
            .as_deref()
            .ok_or(Error::InvalidRequest("market quantityStepSize required"))?;
        aux_rs_core::decimal_units(q, step).map_err(Error::InvalidRequest)?;
        if let Some(min) = &market.min_order_size {
            decimal(min)?;
            if cmp_decimal(q, min).is_lt() {
                return Err(Error::InvalidRequest("quantity below market minimum"));
            }
        }
        if let Some(max) = &market.max_order_size {
            decimal(max)?;
            if cmp_decimal(q, max).is_gt() {
                return Err(Error::InvalidRequest("quantity above market maximum"));
            }
        }
    }
    Ok(())
}
impl CreateOrderRequest {
    pub fn limit(
        pair: impl Into<String>,
        side: impl Into<String>,
        price: impl Into<String>,
        quantity: impl Into<String>,
    ) -> Self {
        Self {
            trading_pair_id: pair.into(),
            order_type: "LIMIT".into(),
            side: side.into(),
            price: Some(price.into()),
            quantity: quantity.into(),
            ..Default::default()
        }
    }
    pub fn market(
        pair: impl Into<String>,
        side: impl Into<String>,
        quantity: impl Into<String>,
    ) -> Self {
        Self {
            trading_pair_id: pair.into(),
            order_type: "MARKET".into(),
            side: side.into(),
            quantity: quantity.into(),
            ..Default::default()
        }
    }
    pub fn validate(&self, market: &TradingPairData) -> Result<(), Error> {
        uuid(&self.trading_pair_id)?;
        if market.id.as_deref() != Some(&self.trading_pair_id) {
            return Err(Error::InvalidRequest("order/market pair mismatch"));
        }
        one_of(&self.order_type, &["LIMIT", "MARKET"])?;
        one_of(&self.side, &["BUY", "SELL"])?;
        if self.order_type == "LIMIT" && self.price.is_none() {
            return Err(Error::InvalidRequest("LIMIT requires price"));
        }
        if self.order_type == "MARKET"
            && (self.price.is_some()
                || self.time_in_force.is_some()
                || self.post_only == Some(true))
        {
            return Err(Error::InvalidRequest(
                "MARKET cannot carry price, timeInForce or postOnly",
            ));
        }
        if let Some(tif) = &self.time_in_force {
            one_of(tif, &["GTC", "IOC", "FOK"])?;
        }
        if self.post_only == Some(true) && self.time_in_force.as_deref().is_some_and(|s| s != "GTC")
        {
            return Err(Error::InvalidRequest("postOnly requires GTC"));
        }
        if self
            .slippage_tolerance_bps
            .is_some_and(|n| !(0..=10000).contains(&n))
        {
            return Err(Error::InvalidRequest(
                "slippageToleranceBps must be 0-10000",
            ));
        }
        let mode = self
            .trading_mode
            .as_deref()
            .or(market.market_type.as_deref())
            .ok_or(Error::InvalidRequest("marketType or tradingMode required"))?;
        one_of(mode, &["SPOT", "MARGIN"])?;
        if market.market_type.as_deref().is_some_and(|m| m != mode) {
            return Err(Error::InvalidRequest("trading mode does not match market"));
        }
        if mode == "MARGIN" && self.reduce_only != Some(true) && self.leverage.is_none() {
            return Err(Error::InvalidRequest("margin orders require leverage"));
        }
        if let Some(side) = &self.position_side
            && side != if self.side == "BUY" { "LONG" } else { "SHORT" }
        {
            return Err(Error::InvalidRequest("positionSide conflicts with side"));
        }
        if mode == "MARGIN" && self.reduce_only != Some(true) {
            if market.isolated_only == Some(true) && self.margin_mode.as_deref() == Some("CROSS") {
                return Err(Error::InvalidRequest(
                    "market only supports isolated margin",
                ));
            }
            if let Some(l) = &self.leverage {
                decimal(l)?;
                for (bound, min) in [(&market.min_leverage, true), (&market.max_leverage, false)] {
                    if let Some(b) = bound {
                        decimal(b)?;
                        if (min && cmp_decimal(l, b).is_lt()) || (!min && cmp_decimal(l, b).is_gt())
                        {
                            return Err(Error::InvalidRequest("leverage outside market bounds"));
                        }
                    }
                }
            }
        }
        increments(market, self.price.as_deref(), Some(&self.quantity))?;
        shared_fields(&serde_json::to_value(self)?)
    }
}
impl ReplaceOrderRequest {
    pub fn validate(&self, market: &TradingPairData) -> Result<(), Error> {
        increments(market, self.price.as_deref(), self.quantity.as_deref())?;
        shared_fields(&serde_json::to_value(self)?)
    }
}
fn batch_size(n: usize) -> Result<(), Error> {
    if (1..=100).contains(&n) {
        Ok(())
    } else {
        Err(Error::InvalidRequest("batch requires 1-100 items"))
    }
}
impl<T: HttpTransport, S: SessionSigner> MonacoClient<T, MonacoAuth<S>> {
    async fn write<R: DeserializeOwned>(
        &self,
        method: HttpMethod,
        path: &str,
        body: Option<Value>,
    ) -> Result<R, ClientError<Error>> {
        self.transport.ensure_open()?;
        self.auth
            .check(&self.exchange)
            .map_err(ClientError::Exchange)?;
        let mut wire = wire(&self.exchange, method, path, &[], body.as_ref())
            .map_err(ClientError::Exchange)?;
        self.auth
            .sign_request_at(&mut wire, unix_millis().map_err(ClientError::Exchange)?)
            .await
            .map_err(ClientError::Exchange)?;
        let response = self.transport.execute(wire).await?;
        if response.status >= 500 {
            return Err(ClientError::Exchange(Error::OutcomeUncertain {
                status: Some(response.status),
                body: serde_json::from_slice(&response.body).ok(),
            }));
        }
        let status = response.status;
        decode(response).map_err(|e| {
            ClientError::Exchange(match e {
                Error::Decode(_) => Error::OutcomeUncertain {
                    status: Some(status),
                    body: None,
                },
                other => other,
            })
        })
    }
    pub async fn place_order(
        &self,
        market: &TradingPairData,
        request: CreateOrderRequest,
    ) -> Result<CreateOrderResponse, ClientError<Error>> {
        request.validate(market).map_err(ClientError::Exchange)?;
        self.write(HttpMethod::Post, "/api/v1/orders", Some(json(request)?))
            .await
    }
    pub async fn replace_order(
        &self,
        market: &TradingPairData,
        order_id: &str,
        request: ReplaceOrderRequest,
    ) -> Result<ReplaceOrderResponse, ClientError<Error>> {
        uuid(order_id).map_err(ClientError::Exchange)?;
        request.validate(market).map_err(ClientError::Exchange)?;
        self.write(
            HttpMethod::Put,
            &format!("/api/v1/orders/{order_id}"),
            Some(json(request)?),
        )
        .await
    }
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<CancelOrderResponse, ClientError<Error>> {
        uuid(&request.order_id).map_err(ClientError::Exchange)?;
        if let Some(q) = &request.remaining_quantity_target
            && q != "0"
        {
            decimal(q).map_err(ClientError::Exchange)?;
        }
        self.write(
            HttpMethod::Post,
            "/api/v1/orders/cancel",
            Some(json(request)?),
        )
        .await
    }
    /// Supply metadata for each pair represented in the batch; input/result ordering is native.
    pub async fn batch_create(
        &self,
        markets: &[TradingPairData],
        request: BatchCreateOrdersRequest,
    ) -> Result<BatchCreateOrdersResponse, ClientError<Error>> {
        batch_size(request.orders.len()).map_err(ClientError::Exchange)?;
        for item in &request.orders {
            let market = markets
                .iter()
                .find(|m| m.id.as_deref() == Some(&item.trading_pair_id))
                .ok_or(ClientError::Exchange(Error::InvalidRequest(
                    "missing batch market metadata",
                )))?;
            let order: CreateOrderRequest = serde_json::from_value(json(item)?)
                .map_err(Error::from)
                .map_err(ClientError::Exchange)?;
            order.validate(market).map_err(ClientError::Exchange)?;
        }
        self.write(
            HttpMethod::Post,
            "/api/v1/orders/batch-create",
            Some(json(request)?),
        )
        .await
    }
    /// Metadata is positional: one market for each original order being replaced.
    pub async fn batch_replace(
        &self,
        markets: &[TradingPairData],
        request: BatchReplaceOrdersRequest,
    ) -> Result<BatchReplaceOrdersResponse, ClientError<Error>> {
        batch_size(request.orders.len()).map_err(ClientError::Exchange)?;
        if markets.len() != request.orders.len() {
            return Err(ClientError::Exchange(Error::InvalidRequest(
                "one market per replacement required",
            )));
        }
        for (market, item) in markets.iter().zip(&request.orders) {
            uuid(&item.order_id).map_err(ClientError::Exchange)?;
            increments(market, item.price.as_deref(), item.quantity.as_deref())
                .map_err(ClientError::Exchange)?;
            shared_fields(&json(item)?).map_err(ClientError::Exchange)?;
        }
        self.write(
            HttpMethod::Post,
            "/api/v1/orders/batch-replace",
            Some(json(request)?),
        )
        .await
    }
    pub async fn batch_cancel(
        &self,
        request: BatchCancelOrdersRequest,
    ) -> Result<BatchCancelOrdersResponse, ClientError<Error>> {
        batch_size(request.order_ids.len()).map_err(ClientError::Exchange)?;
        for id in &request.order_ids {
            uuid(id).map_err(ClientError::Exchange)?;
        }
        self.write(
            HttpMethod::Post,
            "/api/v1/orders/batch-cancel",
            Some(json(request)?),
        )
        .await
    }
    pub async fn cancel_all(
        &self,
        trading_pair_id: Option<&str>,
    ) -> Result<BatchCancelAllResponse, ClientError<Error>> {
        let path = if let Some(id) = trading_pair_id {
            uuid(id).map_err(ClientError::Exchange)?;
            format!("/api/v1/orders/batch-cancel-all/{id}")
        } else {
            "/api/v1/orders/batch-cancel-all".into()
        };
        self.write(HttpMethod::Post, &path, None).await
    }
}
fn json(v: impl Serialize) -> Result<Value, ClientError<Error>> {
    serde_json::to_value(v)
        .map_err(Error::from)
        .map_err(ClientError::Exchange)
}
