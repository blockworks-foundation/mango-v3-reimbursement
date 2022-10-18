use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::state::{Group, ReimbursementAccount};

#[derive(Accounts)]
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

    pub mango_account_owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_create_reimbursement_account(
    _ctx: Context<CreateReimbursementAccount>,
) -> Result<()> {
    Ok(())
}
