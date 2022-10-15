use std::mem::size_of;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use solana_program::pubkey;
use static_assertions::const_assert_eq;

declare_id!("m3roABq4Ta3sGyFRLdY4LH1KN16zBtg586gJ3UxoBzb");

////
// entrypoint
////

#[program]
pub mod mango_v3_reimbursement {
    use super::*;

    pub fn create_group(
        ctx: Context<CreateGroup>,
        group_num: u32,
        table: Pubkey,
        claim_transfer_destination_atas_owner: Pubkey,
    ) -> Result<()> {
        handle_create_group(ctx, group_num, table, claim_transfer_destination_atas_owner)
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

////
// Initialiaze
////

#[derive(Accounts)]
#[instruction(group_num: u32)]
pub struct CreateGroup<'info> {
    #[account(
        init,
        seeds = [b"Group".as_ref(), payer.key().as_ref(), &group_num.to_le_bytes()],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<Group>(),
    )]
    pub group: AccountLoader<'info, Group>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

fn handle_create_group(
    ctx: Context<CreateGroup>,
    group_num: u32,
    table: Pubkey,
    claim_transfer_destination_atas_owner: Pubkey,
) -> Result<()> {
    let mut group = ctx.accounts.group.load_mut()?;
    group.group_num = group_num;
    group.table = table;
    group.claim_transfer_destination_atas_owner = claim_transfer_destination_atas_owner;
    group.authority = ctx.accounts.authority.key();
    Ok(())
}

#[account(zero_copy)]
pub struct Group {
    pub group_num: u32,
    pub table: Pubkey,
    pub claim_transfer_destination_atas_owner: Pubkey,
    pub authority: Pubkey,
    pub vaults: [Pubkey; 16],
    pub claim_mints: [Pubkey; 16],
    pub mints: [Pubkey; 16],
    pub reimbursement_started: u8,
    pub padding: [u8; 3],
}
const_assert_eq!(
    size_of::<Group>(),
    4 + 32 + 32 + 32 + 32 * 16 + 32 * 16 + 32 * 16 + 4
);
const_assert_eq!(size_of::<Group>() % 8, 0);

macro_rules! seeds {
    ( $group:expr ) => {
        &[b"Group".as_ref()]
    };
}

#[account(zero_copy)]
pub struct Table {
    rows: [Row; 32000],
}
const_assert_eq!(size_of::<Table>(), (32 + 8 * 16) * 32000);
const_assert_eq!(size_of::<Table>() % 8, 0);

#[account(zero_copy)]
pub struct Row {
    pub owner: Pubkey,
    pub balances: [u64; 16],
}
const_assert_eq!(size_of::<Row>(), 32 + 8 * 16);
const_assert_eq!(size_of::<Row>() % 8, 0);

////
// Create vaults
////

#[derive(Accounts)]
#[instruction(token_index: usize, mint_decimals: u8)]
pub struct CreateVault<'info> {
    #[account(mut)]
    pub group: AccountLoader<'info, Group>,

    #[account(
        init,
        seeds = [b"Vault".as_ref(), group.key().as_ref(), &token_index.to_le_bytes()],
        bump,
        token::authority = group,
        token::mint = mint,
        payer = payer
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [b"Mint".as_ref(), group.key().as_ref(), &token_index.to_le_bytes()],
        bump,
        mint::authority = group,
        mint::decimals = mint_decimals,
        payer = payer
    )]
    pub claim_mint: Account<'info, Mint>,

    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

fn handle_create_vault(
    ctx: Context<CreateVault>,
    token_index: usize,
    mint_decimals: u8,
) -> Result<()> {
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

////
// Create Reimbusment account
////

#[derive(Accounts)]
#[instruction(token_index: usize)]
pub struct CreateReimbursementAccount<'info> {
    pub group: AccountLoader<'info, Group>,

    #[account(
        init,
        seeds = [b"ReimbursementAccount".as_ref(), group.key().as_ref(), mango_account_owner.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<ReimbursementAccount>(),
    )]
    pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,

    #[account()]
    /// CHECK
    pub mango_account_owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

fn handle_create_reimbursement_account(_ctx: Context<CreateReimbursementAccount>) -> Result<()> {
    Ok(())
}

////
// Start Reimbusment
////

#[derive(Accounts)]
pub struct StartReimbursement<'info> {
    #[account(
        mut,
        has_one = authority
    )]
    pub group: AccountLoader<'info, Group>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

fn handle_start_reimbursement(ctx: Context<StartReimbursement>) -> Result<()> {
    let mut group = ctx.accounts.group.load_mut()?;
    group.reimbursement_started = 1;
    Ok(())
}

#[account(zero_copy)]
pub struct ReimbursementAccount {
    pub done: u16,
    pub claim_transferred: u16,
    pub padding: [u8; 4],
}
const_assert_eq!(size_of::<ReimbursementAccount>(), 8);
const_assert_eq!(size_of::<ReimbursementAccount>() % 8, 0);

////
// Reimburse
////

#[derive(Accounts)]
#[instruction(token_index: usize)]
pub struct Reimburse<'info> {
    #[account (
        constraint = group.load()?.reimbursement_started == 1
    )]
    pub group: AccountLoader<'info, Group>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,
    pub mango_account_owner: Signer<'info>,

    #[account(mut)]
    pub claim_mint_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub claim_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub table: AccountLoader<'info, Table>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

fn handle_reimburse<'key, 'accounts, 'remaining, 'info>(
    ctx: Context<'key, 'accounts, 'remaining, 'info, Reimburse<'info>>,
    index_into_table: usize,
    token_index: usize,
    transfer_claim: bool,
) -> Result<()> {
    let (pda_address, _) = Pubkey::find_program_address(
        &[
            b"MangoV3Reimbursement".as_ref(),
            ctx.accounts.group.key().as_ref(),
            ctx.accounts.mango_account_owner.key().as_ref(),
            &token_index.to_le_bytes(),
        ],
        &crate::id(),
    );
    require_eq!(pda_address, ctx.accounts.reimbursement_account.key());

    let group = ctx.accounts.group.load()?;
    require_eq!(group.mints[token_index], ctx.accounts.mint.key());
    require_eq!(ctx.accounts.vault.key(), group.vaults[token_index]);
    require_eq!(
        ctx.accounts.claim_mint.key(),
        group.claim_mints[token_index]
    );

    let table = ctx.accounts.table.load()?;
    require_eq!(
        table.rows[index_into_table].owner,
        ctx.accounts.mango_account_owner.key()
    );

    let mut reimbursement_account = ctx.accounts.reimbursement_account.load_mut()?;
    require_eq!(reimbursement_account.done & (1 << token_index), 0);

    let seeds = seeds!(group);
    require_eq!(
        ctx.accounts.token_account.owner,
        ctx.accounts.mango_account_owner.key()
    );
    token::transfer(
        {
            let accounts = token::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.group.to_account_info(),
            };
            CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts)
                .with_signer(&[seeds])
        },
        table.rows[index_into_table].balances[token_index],
    )?;
    reimbursement_account.done |= 1 << token_index;

    if transfer_claim {
        require_eq!(
            ctx.accounts.claim_mint_token_account.owner,
            group.claim_transfer_destination_atas_owner
        );
        token::mint_to(
            {
                let accounts = token::MintTo {
                    mint: ctx.accounts.claim_mint.to_account_info(),
                    to: ctx.accounts.claim_mint_token_account.to_account_info(),
                    authority: ctx.accounts.group.to_account_info(),
                };
                CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts)
                    .with_signer(&[seeds])
            },
            table.rows[index_into_table].balances[token_index],
        )?;
        reimbursement_account.claim_transferred |= 1 << token_index;
    }

    Ok(())
}

#[account(zero_copy)]
pub struct MangoAccountReimbursementState {}
