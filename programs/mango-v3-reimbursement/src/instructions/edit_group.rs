use anchor_lang::prelude::*;

use crate::state::Group;

#[derive(Accounts)]
pub struct EditGroup<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = !group.load()?.has_reimbursement_started()
    )]
    pub group: AccountLoader<'info, Group>,

    pub authority: Signer<'info>,
}

// TODO: remove once we go live, create_group also supports taking table,
// this is just a backup for testing without requiring a new group everytime
pub fn handle_edit_group(ctx: Context<EditGroup>, table: Pubkey) -> Result<()> {
    let mut group = ctx.accounts.group.load_mut()?;
    group.table = table;
    Ok(())
}
