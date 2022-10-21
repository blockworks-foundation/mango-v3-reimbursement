use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;

use crate::state::{Group, ReimbursementAccount};

#[derive(Accounts)]
pub struct CreateReimbursementAccount<'info> {
    pub group: AccountLoader<'info, Group>,

    #[account(
        init_if_needed,
        seeds = [b"ReimbursementAccount".as_ref(), group.key().as_ref(), mango_account_owner.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<ReimbursementAccount>(),
    )]
    pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,

    /// CHECK: we want this be permissionless
    pub mango_account_owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handle_create_reimbursement_account(
    _ctx: Context<CreateReimbursementAccount>,
) -> Result<()> {
    Ok(())
}

// non-anchor style helper to make it easier for integrators
pub fn create_reimbursement_account_instruction(
    group: Pubkey,
    reimbursement_account: Pubkey,
    mango_account_owner: Pubkey,
    payer: Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: ToAccountMetas::to_account_metas(
            &crate::accounts::CreateReimbursementAccount {
                group,
                reimbursement_account,
                mango_account_owner,
                payer,
                system_program: solana_program::system_program::ID,
                rent: solana_program::sysvar::rent::ID,
            },
            None,
        ),
        data: anchor_lang::InstructionData::data(
            &crate::instruction::CreateReimbursementAccount {},
        ),
    }
}
