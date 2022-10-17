use anchor_lang::prelude::*;
use anchor_spl::token::Token;

use crate::state::Group;

#[derive(Accounts)]
pub struct EditGroup<'info> {
    #[account(mut)]
    pub group: AccountLoader<'info, Group>,

    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_edit_group(ctx: Context<EditGroup>, table: Pubkey) -> Result<()> {
    let mut group = ctx.accounts.group.load_mut()?;
    group.table = table;
    Ok(())
}
