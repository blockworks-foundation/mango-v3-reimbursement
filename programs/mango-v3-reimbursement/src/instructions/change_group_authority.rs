use anchor_lang::prelude::*;

use crate::state::Group;

#[derive(Accounts)]
pub struct ChangeGroupAuthority<'info> {
    #[account(
        mut,
        has_one = authority,
        constraint = group.load()?.is_testing(),
    )]
    pub group: AccountLoader<'info, Group>,

    pub authority: Signer<'info>,
}

pub fn handle_change_group_authority(
    ctx: Context<ChangeGroupAuthority>,
    new_authority: Pubkey,
) -> Result<()> {
    msg!("Changed group authority to {:?}", new_authority);
    let mut group = ctx.accounts.group.load_mut()?;
    group.authority = new_authority;
    Ok(())
}
