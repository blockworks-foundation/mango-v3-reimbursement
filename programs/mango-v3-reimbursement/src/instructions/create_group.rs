use anchor_lang::prelude::*;

use crate::state::Group;
use crate::Error;

#[derive(Accounts)]
#[instruction(group_num: u32)]
pub struct CreateGroup<'info> {
    #[account(
        init,
        seeds = [b"Group".as_ref(), &group_num.to_le_bytes()],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<Group>(),
    )]
    pub group: AccountLoader<'info, Group>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handle_create_group(
    ctx: Context<CreateGroup>,
    group_num: u32,
    table: Pubkey,
    claim_transfer_destination: Pubkey,
) -> Result<()> {
    let mut group = ctx.accounts.group.load_init()?;
    group.group_num = group_num;
    group.table = table;
    group.claim_transfer_destination = claim_transfer_destination;
    group.authority = ctx.accounts.authority.key();
    group.bump = *ctx.bumps.get("group").ok_or(Error::SomeError)?;
    Ok(())
}
