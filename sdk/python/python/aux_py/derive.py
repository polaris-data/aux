"""Typed inputs and results of the Derive signing protocol."""
from typing import Literal, TypedDict, Union


class SigningLeg(TypedDict):
    instrument_name: str
    asset_address: str
    sub_id: str
    price: str
    amount: str
    direction: str


class Trade(TypedDict):
    type: Literal["trade"]
    asset: str
    sub_id: str
    limit_price: str
    amount: str
    max_fee: str
    recipient_id: str
    is_bid: bool


class Transfer(TypedDict):
    type: Literal["transfer"]
    to_subaccount_id: str
    new_subaccount_manager: str
    asset: str
    sub_id: str
    amount: str
    max_fee_usd: str
    recipient: str | None


class Withdrawal(TypedDict):
    type: Literal["withdrawal"]
    asset: str
    max_fee_usd: str
    recipient: str
    amount: str
    decimals: int
    force_batch: bool


class SessionKey(TypedDict):
    type: Literal["session_key"]
    session_key: str
    expiry_sec: int
    scopes: list[int]
    subaccount_ids: list[str]


class WhitelistedRecipients(TypedDict):
    type: Literal["whitelisted_recipients"]
    add: list[str]
    remove: list[str]


class RfqQuote(TypedDict):
    type: Literal["rfq_quote"]
    max_fee: str
    direction: str
    legs: list[SigningLeg]


class RfqExecute(TypedDict):
    type: Literal["rfq_execute"]
    max_fee: str
    direction: str
    legs: list[SigningLeg]


class VaultCreate(TypedDict):
    type: Literal["vault_create"]
    manager_id: str
    deposit_spot_asset: str
    initial_deposit: str
    management_fee_bps: int
    performance_fee_bps: int
    max_slippage_bps: int
    cooldown_sec: int
    max_fee_usd: str
    initial_share_price_usd: str
    benchmark_asset: str | None


class VaultDeposit(TypedDict):
    type: Literal["vault_deposit"]
    vault_subaccount_id: str
    deposit_spot_asset: str
    amount: str


class VaultWithdraw(TypedDict):
    type: Literal["vault_withdraw"]
    vault_subaccount_id: str
    shares_to_burn: str


class VaultCancel(TypedDict):
    type: Literal["vault_cancel"]
    vault_subaccount_id: str


class VaultMint(TypedDict):
    type: Literal["vault_mint"]
    share_price: str
    deposit_hash: str


class VaultBurn(TypedDict):
    type: Literal["vault_burn"]
    share_price: str
    withdraw_hash: str


class VaultForceBurn(TypedDict):
    type: Literal["vault_force_burn"]
    holder: str


class DeleteSubaccount(TypedDict):
    type: Literal["delete_subaccount"]


class Liquidation(TypedDict):
    type: Literal["liquidation"]
    liquidate_subaccount_id: str
    percent_of_acc: str
    price_limit: str


class EncodedModule(TypedDict):
    type: Literal["encoded"]
    module: str
    data: list[int]


class ActionPayload(TypedDict):
    owner: str
    signer: str
    module: str
    encoded_data: list[int]
    nonce: str
    expiry: int
    subaccount_id: str


ActionData = Union[EncodedModule, Trade, Transfer, Withdrawal, SessionKey, WhitelistedRecipients, RfqQuote, RfqExecute, VaultCreate, VaultDeposit, VaultWithdraw, VaultCancel, VaultMint, VaultBurn, VaultForceBurn, DeleteSubaccount, Liquidation]


class Action(TypedDict):
    subaccount_id: str
    nonce: str
    expiry: int
    owner: str
    chain_id: int
    data: ActionData


class SignedAction(TypedDict):
    module: str
    encoded: str
    digest: str
    signature: str
    signer: str


class LoginProof(TypedDict):
    wallet: str
    timestamp: str
    signature: str


