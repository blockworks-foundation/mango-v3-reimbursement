pub mod instructions;
pub mod state;

use instructions::*;

use anchor_lang::prelude::*;

declare_id!("m3roABq4Ta3sGyFRLdY4LH1KN16zBtg586gJ3UxoBzb");

#[program]
pub mod mango_v3_reimbursement {

    use super::*;

    pub fn create_group(
        ctx: Context<CreateGroup>,
        group_num: u32,
        table: Pubkey,
        claim_transfer_destination: Pubkey,
    ) -> Result<()> {
        handle_create_group(ctx, group_num, table, claim_transfer_destination)
    }

    pub fn edit_group(ctx: Context<EditGroup>, table: Pubkey) -> Result<()> {
        handle_edit_group(ctx, table)
    }

    pub fn create_vault(
        ctx: Context<CreateVault>,
        token_index: usize,
        mint_decimals: u8,
    ) -> Result<()> {
        handle_create_vault(ctx, token_index, mint_decimals)
    }

    pub fn create_reimbursement_account(ctx: Context<CreateReimbursementAccount>) -> Result<()> {
        handle_create_reimbursement_account(ctx)
    }

    pub fn start_reimbursement(ctx: Context<StartReimbursement>) -> Result<()> {
        handle_start_reimbursement(ctx)
    }

    pub fn reimburse<'key, 'accounts, 'remaining, 'info>(
        ctx: Context<'key, 'accounts, 'remaining, 'info, Reimburse<'info>>,
        index_into_table: usize,
        token_index: usize,
        transfer_claim: bool,
    ) -> Result<()> {
        handle_reimburse(ctx, index_into_table, token_index, transfer_claim)
    }
}

#[error_code]
pub enum Error {
    SomeError,
}
