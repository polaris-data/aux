use crate::{Error, Result, native::invoke};
use serde_json::{Value, json};

#[derive(Clone, Copy)]
pub enum Venue {
    Metric,
    Polymarket,
}

fn call(id: u32, args: Value) -> Result<Value> {
    let text = invoke(id, args.to_string().as_bytes())?;
    serde_json::from_str(&text).map_err(|_| Error::Decode("invalid ABI result"))
}

pub fn encode_contract_call(
    venue: Venue,
    contract: &str,
    function: &str,
    args: &Value,
) -> Result<String> {
    let op = match venue {
        Venue::Metric => 2,
        Venue::Polymarket => 7,
    };
    let result = call(
        op,
        json!({ "contract": contract, "function": function, "args": args }),
    )?;
    result
        .as_str()
        .map(str::to_owned)
        .ok_or(Error::Decode("expected hex calldata"))
}
pub fn decode_contract_input(
    venue: Venue,
    contract: &str,
    function: &str,
    data: &str,
) -> Result<Value> {
    let op = match venue {
        Venue::Metric => 3,
        Venue::Polymarket => 8,
    };
    call(
        op,
        json!({ "contract": contract, "function": function, "data": data }),
    )
}
pub fn decode_contract_output(
    venue: Venue,
    contract: &str,
    function: &str,
    data: &str,
) -> Result<Value> {
    let op = match venue {
        Venue::Metric => 4,
        Venue::Polymarket => 9,
    };
    call(
        op,
        json!({ "contract": contract, "function": function, "data": data }),
    )
}
pub fn decode_metric_error(contract: &str, data: &str) -> Result<Value> {
    call(5, json!({ "contract": contract, "data": data }))
}
pub fn decode_metric_event(
    contract: &str,
    event: &str,
    topics: &[String],
    data: &str,
) -> Result<Value> {
    call(
        6,
        json!({ "contract": contract, "event": event, "topics": topics, "data": data }),
    )
}
