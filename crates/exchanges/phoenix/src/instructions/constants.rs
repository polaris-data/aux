//! Phoenix program constants and addresses.

#[cfg(not(target_os = "solana"))]
use std::sync::LazyLock;

use sha2::{Digest, Sha256};
use solana_pubkey::Pubkey;

use super::discriminants::EmberInstruction;

use super::error::PhoenixIxError;

/// The Phoenix program ID (mainnet).
pub const PROD_PHOENIX_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("EtrnLzgbS7nMMy5fbD42kXiUzGg8XQzJ972Xtk1cjWih");

/// The Phoenix program ID (beta).
pub const BETA_PHOENIX_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("phDEVv4w6BcfkLrLNeXr8HhhgQxnxziVGXpGPcaadMf");

/// The Phoenix log authority address (mainnet).
pub const PROD_PHOENIX_LOG_AUTHORITY: Pubkey =
    solana_pubkey::pubkey!("GdxfTLSsdSY37G6fZoYtdGDSfgFnbT2EmRpuePZxWShS");

/// The Phoenix log authority address (beta).
pub const BETA_PHOENIX_LOG_AUTHORITY: Pubkey =
    solana_pubkey::pubkey!("8Q1zeC7qAPUhJ2ncHAW8N1TGcpMVDsSxdSBPLaE8G2ab");

/// The Phoenix global configuration address (mainnet).
pub const PROD_PHOENIX_GLOBAL_CONFIGURATION: Pubkey =
    solana_pubkey::pubkey!("2zskx2iyCvb6Stg7RBZkt1f6MrF4dpYtMG3yMvKwqtUZ");

/// The Phoenix global configuration address (beta).
pub const BETA_PHOENIX_GLOBAL_CONFIGURATION: Pubkey =
    solana_pubkey::pubkey!("3CkM38UaZW6nyTJku4ABE5jjS5AComQErrkd55LGTfxa");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(all(), derive(serde::Serialize, serde::Deserialize))]
pub struct PhoenixInstructionAddresses {
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub program_id: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub log_authority: Pubkey,
    #[cfg_attr(all(), serde(with = "crate::instructions::serde_helpers::pubkey"))]
    pub global_configuration: Pubkey,
}

pub const PROD_PHOENIX_INSTRUCTION_ADDRESSES: PhoenixInstructionAddresses =
    PhoenixInstructionAddresses {
        program_id: PROD_PHOENIX_PROGRAM_ID,
        log_authority: PROD_PHOENIX_LOG_AUTHORITY,
        global_configuration: PROD_PHOENIX_GLOBAL_CONFIGURATION,
    };

pub const BETA_PHOENIX_INSTRUCTION_ADDRESSES: PhoenixInstructionAddresses =
    PhoenixInstructionAddresses {
        program_id: BETA_PHOENIX_PROGRAM_ID,
        log_authority: BETA_PHOENIX_LOG_AUTHORITY,
        global_configuration: BETA_PHOENIX_GLOBAL_CONFIGURATION,
    };

/// Active Phoenix instruction addresses for the current process.
///
/// This is resolved once from `PHOENIX_ENV`. Set `PHOENIX_ENV=beta` before
/// first use to target the beta deployment.
#[cfg(not(target_os = "solana"))]
pub static PHOENIX_INSTRUCTION_ADDRESSES: LazyLock<PhoenixInstructionAddresses> =
    LazyLock::new(|| PROD_PHOENIX_INSTRUCTION_ADDRESSES);

#[cfg(target_os = "solana")]
pub static PHOENIX_INSTRUCTION_ADDRESSES: &PhoenixInstructionAddresses =
    &PROD_PHOENIX_INSTRUCTION_ADDRESSES;

/// Active Phoenix program ID for the current process.
///
/// This is resolved once from `PHOENIX_ENV`. Set `PHOENIX_ENV=beta` before
/// first use to target the beta deployment.
#[cfg(not(target_os = "solana"))]
pub static PHOENIX_PROGRAM_ID: LazyLock<Pubkey> =
    LazyLock::new(|| phoenix_instruction_addresses().program_id);

#[cfg(target_os = "solana")]
pub static PHOENIX_PROGRAM_ID: &Pubkey = &PROD_PHOENIX_PROGRAM_ID;

/// Active Phoenix log authority for the current process.
#[cfg(not(target_os = "solana"))]
pub static PHOENIX_LOG_AUTHORITY: LazyLock<Pubkey> =
    LazyLock::new(|| phoenix_instruction_addresses().log_authority);

#[cfg(target_os = "solana")]
pub static PHOENIX_LOG_AUTHORITY: &Pubkey = &PROD_PHOENIX_LOG_AUTHORITY;

/// Active Phoenix global configuration for the current process.
#[cfg(not(target_os = "solana"))]
pub static PHOENIX_GLOBAL_CONFIGURATION: LazyLock<Pubkey> =
    LazyLock::new(|| phoenix_instruction_addresses().global_configuration);

#[cfg(target_os = "solana")]
pub static PHOENIX_GLOBAL_CONFIGURATION: &Pubkey = &PROD_PHOENIX_GLOBAL_CONFIGURATION;

/// Resolve Phoenix instruction addresses for an explicit environment value.
///
/// `beta` selects the beta deployment. Everything else defaults to production.
pub fn resolve_phoenix_instruction_addresses_for_env(
    phoenix_env: Option<&str>,
) -> PhoenixInstructionAddresses {
    match phoenix_env
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("beta") => BETA_PHOENIX_INSTRUCTION_ADDRESSES,
        _ => PROD_PHOENIX_INSTRUCTION_ADDRESSES,
    }
}

/// Resolve Phoenix instruction addresses once from `PHOENIX_ENV`.
pub fn phoenix_instruction_addresses() -> PhoenixInstructionAddresses {
    *PHOENIX_INSTRUCTION_ADDRESSES
}

/// The active Phoenix program ID for the current process.
pub fn phoenix_program_id() -> Pubkey {
    *PHOENIX_PROGRAM_ID
}

/// The active Phoenix log authority for the current process.
pub fn phoenix_log_authority() -> Pubkey {
    *PHOENIX_LOG_AUTHORITY
}

/// The active Phoenix global configuration for the current process.
pub fn phoenix_global_configuration() -> Pubkey {
    *PHOENIX_GLOBAL_CONFIGURATION
}

/// The Ember program ID (for USDC -> Phoenix token conversion).
pub const EMBER_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("EMBERpYNE6ehWmXymZZS2skiFmCa9V5dp14e1iduM5qy");

/// USDC mint address (mainnet).
///
/// Prefer [`usdc_mint`] when the process may target the beta deployment.
pub const USDC_MINT: Pubkey =
    solana_pubkey::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

/// USDC mint address (beta).
pub const BETA_USDC_MINT: Pubkey =
    solana_pubkey::pubkey!("DPTSTVhvfzQhY8pAJL5EeqfZxf9aNKTmHErfG4R3Z1SE");

/// Active USDC mint for the current process.
///
/// This is resolved once from `PHOENIX_ENV`. Set `PHOENIX_ENV=beta` before
/// first use to target the beta deployment.
#[cfg(not(target_os = "solana"))]
static ACTIVE_USDC_MINT: LazyLock<Pubkey> =
    LazyLock::new(|| resolve_usdc_mint_for_env(std::env::var("PHOENIX_ENV").ok().as_deref()));

#[cfg(target_os = "solana")]
static ACTIVE_USDC_MINT: &Pubkey = &USDC_MINT;

/// Resolve the USDC mint for an explicit environment value.
///
/// `beta` selects the beta deployment. Everything else defaults to production.
pub fn resolve_usdc_mint_for_env(phoenix_env: Option<&str>) -> Pubkey {
    match phoenix_env
        .map(str::trim)
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("beta") => BETA_USDC_MINT,
        _ => USDC_MINT,
    }
}

/// The active USDC mint for the current process.
pub fn usdc_mint() -> Pubkey {
    *ACTIVE_USDC_MINT
}

/// SPL Token program ID.
pub const SPL_TOKEN_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// Associated Token program ID.
pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    solana_pubkey::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

/// System program ID.
pub const SYSTEM_PROGRAM_ID: Pubkey = solana_pubkey::pubkey!("11111111111111111111111111111111");

/// Compute the instruction discriminant using SHA-256.
/// Takes the first 8 bytes of SHA-256 hash of the input string.
pub fn compute_discriminant(input: &str) -> [u8; 8] {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    let mut discriminant = [0u8; 8];
    discriminant.copy_from_slice(&result[..8]);
    discriminant
}

/// Instruction discriminant for ember deposit.
pub fn ember_deposit_discriminant() -> [u8; 8] {
    EmberInstruction::Deposit.discriminant()
}

/// Instruction discriminant for ember withdraw.
pub fn ember_withdraw_discriminant() -> [u8; 8] {
    EmberInstruction::Withdraw.discriminant()
}

/// Derives the stop loss PDA for a given trader account and asset ID.
///
/// Seeds: ["stoploss", trader_account, &asset_id.to_le_bytes()]
pub fn get_stop_loss_address(
    trader_account: &Pubkey,
    asset_id: u64,
) -> Result<Pubkey, PhoenixIxError> {
    derive_program_address(
        &[
            b"stoploss",
            trader_account.as_ref(),
            &asset_id.to_le_bytes(),
        ],
        &phoenix_program_id(),
    )
}

/// Derives the conditional orders PDA for a given trader account.
///
/// Seeds: ["conditional_orders", trader_account]
pub fn get_conditional_orders_address(trader_account: &Pubkey) -> Result<Pubkey, PhoenixIxError> {
    derive_program_address(
        &[b"conditional_orders", trader_account.as_ref()],
        &phoenix_program_id(),
    )
}

/// Derives the escrow PDA for a given authority.
///
/// Seeds: ["escrow", authority]
pub fn get_escrow_address(authority: &Pubkey) -> Result<Pubkey, PhoenixIxError> {
    derive_program_address(&[b"escrow", authority.as_ref()], &phoenix_program_id())
}

/// Derives the permission PDA for an authority and delegated signer.
///
/// Seeds: ["permission", permission_authority, delegated_key]
pub fn get_permission_address(
    permission_authority: &Pubkey,
    delegated_key: &Pubkey,
) -> Result<Pubkey, PhoenixIxError> {
    derive_program_address(
        &[
            b"permission",
            permission_authority.as_ref(),
            delegated_key.as_ref(),
        ],
        &phoenix_program_id(),
    )
}

/// Derives the spline collection PDA for a given market (orderbook) address.
///
/// Seeds: ["spline", market_address]
pub fn get_spline_collection_address(market: &Pubkey) -> Result<Pubkey, PhoenixIxError> {
    derive_program_address(&[b"spline", market.as_ref()], &phoenix_program_id())
}

/// Derives the Ember state PDA.
///
/// Seeds: [phoenix_program_id, "state"] against Ember program
pub fn get_ember_state_address() -> Result<Pubkey, PhoenixIxError> {
    let program_id = *PHOENIX_PROGRAM_ID;
    derive_program_address(&[program_id.as_ref(), b"state"], &EMBER_PROGRAM_ID)
}

/// Derives the Ember vault PDA.
///
/// Seeds: [phoenix_program_id, "vault"] against Ember program
pub fn get_ember_vault_address() -> Result<Pubkey, PhoenixIxError> {
    let program_id = *PHOENIX_PROGRAM_ID;
    derive_program_address(&[program_id.as_ref(), b"vault"], &EMBER_PROGRAM_ID)
}

/// Derives the global vault PDA for a given mint.
///
/// Seeds: ["vault", mint] against Phoenix program
pub fn get_global_vault_address(mint: &Pubkey) -> Result<Pubkey, PhoenixIxError> {
    derive_program_address(&[b"vault", mint.as_ref()], &phoenix_program_id())
}

/// Derives the native SOL authority PDA, which custodies native SOL spot
/// collateral and signs the program's own withdrawals of it.
///
/// Seeds: ["native_sol"] against Phoenix program
pub fn get_native_sol_authority_address() -> Result<Pubkey, PhoenixIxError> {
    derive_program_address(&[b"native_sol"], &phoenix_program_id())
}

/// Derives the trader wallet PDA that owns a trader account's SPL spot
/// collateral custody ATAs and signs the program's own token movements out of
/// them.
///
/// Seeds: ["wallet", trader_account] against Phoenix program
pub fn get_trader_wallet_address(trader_account: &Pubkey) -> Result<Pubkey, PhoenixIxError> {
    derive_program_address(&[b"wallet", trader_account.as_ref()], &phoenix_program_id())
}

/// Derives the associated token address for an owner and mint.
///
/// This follows the standard SPL ATA derivation.
pub fn get_associated_token_address(
    owner: &Pubkey,
    mint: &Pubkey,
) -> Result<Pubkey, PhoenixIxError> {
    derive_program_address(
        &[owner.as_ref(), SPL_TOKEN_PROGRAM_ID.as_ref(), mint.as_ref()],
        &ASSOCIATED_TOKEN_PROGRAM_ID,
    )
}

fn derive_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> Result<Pubkey, PhoenixIxError> {
    let (pda, _bump) = Pubkey::find_program_address(seeds, program_id);
    Ok(pda)
}
