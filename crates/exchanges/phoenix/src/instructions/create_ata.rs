//! Create Associated Token Account instruction construction.

use solana_pubkey::Pubkey;

use crate::instructions::constants::{
    ASSOCIATED_TOKEN_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID, SYSTEM_PROGRAM_ID,
    get_associated_token_address,
};
use crate::instructions::error::PhoenixIxError;
use crate::instructions::types::{AccountMeta, Instruction};

/// Create an idempotent Associated Token Account instruction.
///
/// This instruction creates an ATA for the owner if it doesn't exist,
/// and does nothing if it already exists.
///
/// # Arguments
///
/// * `payer` - The account that will pay for the ATA creation
/// * `owner` - The owner of the ATA
/// * `mint` - The token mint
///
/// # Returns
///
/// A Solana instruction ready to be included in a transaction.
pub fn create_associated_token_account_idempotent_ix(
    payer: Pubkey,
    owner: Pubkey,
    mint: Pubkey,
) -> Result<Instruction, PhoenixIxError> {
    let ata = get_associated_token_address(&owner, &mint)?;

    let accounts = vec![
        AccountMeta::writable_signer(payer),
        AccountMeta::writable(ata),
        AccountMeta::readonly(owner),
        AccountMeta::readonly(mint),
        AccountMeta::readonly(SYSTEM_PROGRAM_ID),
        AccountMeta::readonly(SPL_TOKEN_PROGRAM_ID),
    ];

    // CreateIdempotent discriminant is 1
    let data = vec![1u8];

    Ok(Instruction {
        program_id: ASSOCIATED_TOKEN_PROGRAM_ID,
        accounts,
        data,
    })
}
