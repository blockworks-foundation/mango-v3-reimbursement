use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::state::Group;

#[derive(Accounts)]
pub struct StartReimbursement<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = !group.load()?.has_reimbursement_started()
    )]
    pub group: AccountLoader<'info, Group>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// TODO: do we also want to have a end/freeze reimbursement?
pub fn handle_start_reimbursement(ctx: Context<StartReimbursement>) -> Result<()> {
    let mut group = ctx.accounts.group.load_mut()?;
    group.reimbursement_started = 1;
    Ok(())
}
