use anchor_lang::prelude::*;
use solana_program::instruction::Instruction;

use crate::state::{Group, ReimbursementAccount};

pub const CREATE_REIMBURSEMENT_ACCOUNT_OPCODE: u64 = 0x6f91dd5910a34ca5;

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

pub fn create_reimbursement_account(
    program_id: &Pubkey,
    group_pk: &Pubkey,
    reimbursement_account_pk: &Pubkey,
    mango_account_owner_pk: &Pubkey,
    payer_pk: &Pubkey,
) -> Result<Instruction> {
    let accounts = vec![
        AccountMeta::new(*group_pk, false),
        AccountMeta::new(*reimbursement_account_pk, false),
        AccountMeta::new_readonly(*mango_account_owner_pk, false),
        AccountMeta::new(*payer_pk, true),
        AccountMeta::new_readonly(solana_program::system_program::ID, false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false)
    ];

    let mut ix_data = Vec::<u8>::new();
    ix_data.extend(CREATE_REIMBURSEMENT_ACCOUNT_OPCODE.to_be_bytes().to_vec());
    Ok(Instruction { program_id: *program_id, accounts, data: ix_data })
}
