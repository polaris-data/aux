export interface SigningLeg {
  instrument_name: string;
  asset_address: string;
  sub_id: string;
  price: string;
  amount: string;
  direction: string;
}
export interface Trade {
  type: "trade";
  asset: string;
  sub_id: string;
  limit_price: string;
  amount: string;
  max_fee: string;
  recipient_id: string;
  is_bid: boolean;
}
export interface Transfer {
  type: "transfer";
  to_subaccount_id: string;
  new_subaccount_manager: string;
  asset: string;
  sub_id: string;
  amount: string;
  max_fee_usd: string;
  recipient: string | null;
}
export interface Withdrawal {
  type: "withdrawal";
  asset: string;
  max_fee_usd: string;
  recipient: string;
  amount: string;
  decimals: number;
  force_batch: boolean;
}
export interface SessionKey {
  type: "session_key";
  session_key: string;
  expiry_sec: number | bigint;
  scopes: number[];
  subaccount_ids: string[];
}
export interface WhitelistedRecipients {
  type: "whitelisted_recipients";
  add: string[];
  remove: string[];
}
export interface RfqQuote {
  type: "rfq_quote";
  max_fee: string;
  direction: string;
  legs: SigningLeg[];
}
export interface RfqExecute {
  type: "rfq_execute";
  max_fee: string;
  direction: string;
  legs: SigningLeg[];
}
export interface VaultCreate {
  type: "vault_create";
  manager_id: string;
  deposit_spot_asset: string;
  initial_deposit: string;
  management_fee_bps: number;
  performance_fee_bps: number;
  max_slippage_bps: number;
  cooldown_sec: number;
  max_fee_usd: string;
  initial_share_price_usd: string;
  benchmark_asset: string | null;
}
export interface VaultDeposit {
  type: "vault_deposit";
  vault_subaccount_id: string;
  deposit_spot_asset: string;
  amount: string;
}
export interface VaultWithdraw {
  type: "vault_withdraw";
  vault_subaccount_id: string;
  shares_to_burn: string;
}
export interface VaultCancel {
  type: "vault_cancel";
  vault_subaccount_id: string;
}
export interface VaultMint {
  type: "vault_mint";
  share_price: string;
  deposit_hash: string;
}
export interface VaultBurn {
  type: "vault_burn";
  share_price: string;
  withdraw_hash: string;
}
export interface VaultForceBurn {
  type: "vault_force_burn";
  holder: string;
}
export interface DeleteSubaccount {
  type: "delete_subaccount";
}
export interface Liquidation {
  type: "liquidation";
  liquidate_subaccount_id: string;
  percent_of_acc: string;
  price_limit: string;
}
export interface ActionPayload {
  owner: string;
  signer: string;
  module: string;
  encoded_data: number[];
  subaccount_id: string;
  nonce: string;
  expiry: number | bigint;
}
export interface EncodedModule {
  type: "encoded";
  module: string;
  data: number[];
}
export type ActionData = EncodedModule | Trade | Transfer | Withdrawal | SessionKey | WhitelistedRecipients | RfqQuote | RfqExecute | VaultCreate | VaultDeposit | VaultWithdraw | VaultCancel | VaultMint | VaultBurn | VaultForceBurn | DeleteSubaccount | Liquidation;
export interface Action {
  subaccount_id: string;
  nonce: string;
  expiry: number | bigint;
  owner: string;
  chain_id: number | bigint;
  data: ActionData;
}
export interface SignedAction {
  module: string;
  encoded: string;
  digest: string;
  signature: string;
  signer: string;
}
export interface LoginProof {
  wallet: string;
  timestamp: string;
  signature: string;
}
