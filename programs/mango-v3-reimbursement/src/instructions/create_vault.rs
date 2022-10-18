use crate::Error;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::Group;

#[derive(Accounts)]
#[instruction(token_index: usize)]
pub struct CreateVault<'info> {
    #[account (
        mut,
        has_one = authority,
        constraint = !group.load()?.has_reimbursement_started()
    )]
    pub group: AccountLoader<'info, Group>,

    pub authority: Signer<'info>,

    #[account(
        init,
        associated_token::mint = mint,
        payer = payer,
        associated_token::authority = group,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [b"Mint".as_ref(), group.key().as_ref(), &token_index.to_le_bytes()],
        bump,
        mint::authority = group,
        mint::decimals = mint.decimals,
        payer = payer
    )]
    pub claim_mint: Account<'info, Mint>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_create_vault(ctx: Context<CreateVault>, token_index: usize) -> Result<()> {
    require!(token_index < 16usize, Error::SomeError);

    let mut group = ctx.accounts.group.load_mut()?;
    require_eq!(group.vaults[token_index], Pubkey::default());
    require_eq!(group.claim_mints[token_index], Pubkey::default());
    require_eq!(group.mints[token_index], Pubkey::default());
    group.vaults[token_index] = ctx.accounts.vault.key();
    group.claim_mints[token_index] = ctx.accounts.claim_mint.key();
    group.mints[token_index] = ctx.accounts.mint.key();
    Ok(())
}
