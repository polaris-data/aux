"""Native Polymarket contract builders and explicit signing/position helpers."""
from . import _native
from ._polymarket_contracts import *
def market_asset_id(market, outcome):
    return _native.polymarket_helper("market_asset_id", {"market": market, "outcome": outcome})
def market_order_domain(market):
    return _native.polymarket_helper("market_order_domain", {"market": market})
def deployments():
    return _native.polymarket_helper("deployments", {})
def wallet_auth_typed_data(chain_id, address, timestamp, nonce=0):
    return _native.polymarket_helper("wallet_auth_typed_data", locals())
def perps_op_typed_data(chain_id, operation, salt, timestamp):
    return _native.polymarket_helper("perps_op_typed_data", locals())
def deposit_wallet_batch_typed_data(chain_id, wallet, nonce, deadline, calls):
    return _native.polymarket_helper("deposit_wallet_batch_typed_data", locals())
