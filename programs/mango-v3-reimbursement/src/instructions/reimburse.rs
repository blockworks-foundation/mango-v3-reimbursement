use std::mem::size_of;

use crate::state::{Group, ReimbursementAccount, Row};
use crate::Error;
use anchor_lang::{__private::bytemuck, prelude::*};
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[derive(Accounts)]
#[instruction(token_index: usize)]
pub struct Reimburse<'info> {
    #[account (
        constraint = group.load()?.has_reimbursement_started()
    )]
    pub group: AccountLoader<'info, Group>,

    #[account(
        mut,
        address = group.load()?.vaults[token_index]
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = token_account.owner == mango_account_owner.key()
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"ReimbursementAccount".as_ref(), group.key().as_ref(), mango_account_owner.key().as_ref()],
        bump,
        constraint = group.load()?.is_testing() || !reimbursement_account.load()?.reimbursed(token_index),
        constraint = group.load()?.is_testing() || !reimbursement_account.load()?.claim_transferred(token_index)
    )]
    pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,
    pub mango_account_owner: UncheckedAccount<'info>,

    #[account (
        constraint = signer.key() == mango_account_owner.key() || signer.key() == group.load()?.authority
    )]
    pub signer: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = claim_mint,
        associated_token::authority = group.load()?.claim_transfer_destination,
    )]
    pub claim_mint_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        address = group.load()?.claim_mints[token_index]
    )]
    pub claim_mint: Box<Account<'info, Mint>>,

    /// CHECK:
    pub table: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_reimburse<'key, 'accounts, 'remaining, 'info>(
    ctx: Context<'key, 'accounts, 'remaining, 'info, Reimburse<'info>>,
    index_into_table: usize,
    token_index: usize,
    transfer_claim: bool,
) -> Result<()> {
    require!(token_index < 16usize, Error::SomeError);

    let group = ctx.accounts.group.load()?;

    // Verify entry in reimbursement table
    let table_ai = &ctx.accounts.table;
    let data = table_ai.try_borrow_data()?;
    require_eq!((data.len() - 40) % size_of::<Row>(), 0);
    let row: &Row = bytemuck::from_bytes::<Row>(&data[40 + index_into_table * size_of::<Row>()..]);
    require_keys_eq!(row.owner, ctx.accounts.mango_account_owner.key());

    token::transfer(
        {
            let accounts = token::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.group.to_account_info(),
            };
            CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts).with_signer(&[
                &[
                    b"Group".as_ref(),
                    &group.group_num.to_le_bytes(),
                    &[group.bump],
                ],
            ])
        },
        row.balances[token_index],
    )?;
    let mut reimbursement_account = ctx.accounts.reimbursement_account.load_mut()?;
    reimbursement_account.mark_reimbursed(token_index);

    if transfer_claim {
        token::mint_to(
            {
                let accounts = token::MintTo {
                    mint: ctx.accounts.claim_mint.to_account_info(),
                    to: ctx.accounts.claim_mint_token_account.to_account_info(),
                    authority: ctx.accounts.group.to_account_info(),
                };
                CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts).with_signer(
                    &[&[
                        b"Group".as_ref(),
                        &group.group_num.to_le_bytes(),
                        &[group.bump],
                    ]],
                )
            },
            row.balances[token_index],
        )?;
        reimbursement_account.mark_claim_transferred(token_index);
    }

    Ok(())
}
