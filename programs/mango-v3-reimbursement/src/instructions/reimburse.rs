use crate::state::{Group, ReimbursementAccount, Row};
use crate::Error;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, spl_token};
use solana_program::instruction::Instruction;

pub const REIMBURSE_OPCODE: u64 = 0x6f91dd5910a34ca5;


#[derive(Accounts)]
#[instruction(token_index: usize)]
pub struct Reimburse<'info> {
    #[account (
        constraint = group.load()?.has_reimbursement_started() @ Error::ReimbursementNotStarted,
        has_one = table
    )]
    pub group: AccountLoader<'info, Group>,

    #[account(
        mut,
        address = group.load()?.vaults[token_index]
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = token_account.owner == mango_account_owner.key() @ Error::TokenAccountNotOwnedByMangoAccountOwner
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"ReimbursementAccount".as_ref(), group.key().as_ref(), mango_account_owner.key().as_ref()],
        bump,
        constraint = group.load()?.is_testing() || !reimbursement_account.load()?.reimbursed(token_index) @ Error::AlreadyReimbursed,
    )]
    pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,
    /// CHECK: address is part of the ReimbursementAccount PDA
    pub mango_account_owner: UncheckedAccount<'info>,

    #[account (
        constraint = signer.key() == mango_account_owner.key() || signer.key() == group.load()?.authority @ Error::BadSigner
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

/// Disclaimer:
/// Please make sure you and your users (of integrating programs) read and accept
/// the following waiver when reclaiming their funds using below instruction:
///
///  By executing this instruction and accepting the tokens, I hereby
///  irrevocably sell, convey, transfer and assign to Mango Labs,
///  LLC all of my right, title and interest in, to and under all
///  claims arising out of or related to the loss of my tokens in
///  the October 2022 incident, including, without limitation, all
///  of my causes of action or other rights with respect to such
///  claims, all rights to receive any amounts or property or other
///  distribution in respect of or in connection with such claims,
///  and any and all proceeds of any of the foregoing (including
///  proceeds of proceeds). I further irrevocably and
///  unconditionally release all claims I may have against Mango
///  Labs, LLC, the Mango Decentralized Autonomous Entity, its core
///  contributors, and any of their agents, affiliates, officers,
///  employees, or principals related to this matter. This release
///  constitutes an express, informed, knowing and voluntary waiver
///  and relinquishment to the fullest extent permitted by law.
pub fn handle_reimburse<'key, 'accounts, 'remaining, 'info>(
    ctx: Context<'key, 'accounts, 'remaining, 'info, Reimburse<'info>>,
    token_index: usize,
    index_into_table: usize,
    transfer_claim: bool,
) -> Result<()> {
    require!(token_index < 16usize, Error::SomeError);
    require!(transfer_claim, Error::MustTransferClaim);

    let group = ctx.accounts.group.load()?;

    // More checks on table
    let table_ai = &ctx.accounts.table;
    let data = table_ai.try_borrow_data()?;
    if !group.is_testing() {
        require_keys_eq!(Pubkey::new(&data[5..37]), group.authority);
    }

    // Verify entry in reimbursement table
    let row = Row::load(&data, index_into_table)?;
    require_keys_eq!(
        row.owner,
        ctx.accounts.mango_account_owner.key(),
        Error::TableRowHasWrongOwner
    );

    let amount = row.balances[token_index];

    let mut reimbursement_account = ctx.accounts.reimbursement_account.load_mut()?;

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
        amount,
    )?;
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
            amount,
        )?;
        reimbursement_account.mark_claim_transferred(token_index);
    }

    Ok(())
}

pub fn reimburse(
    program_id: &Pubkey,
    group_pk: &Pubkey,
    vault_pk: &Pubkey,
    token_account_pk: &Pubkey,
    reimbursement_account_pk: &Pubkey,
    mango_account_owner_pk: &Pubkey,
    signer_pk: &Pubkey,
    claim_mint_token_account_pk: &Pubkey,
    claim_mint_pk: &Pubkey,
    table_pk: &Pubkey,
    token_index: usize,
    index_into_table: usize,
    transfer_claim: bool,
) -> Result<Instruction> {
    let accounts = vec![
        AccountMeta::new(*group_pk, false),
        AccountMeta::new(*vault_pk, false),
        AccountMeta::new(*token_account_pk, false),
        AccountMeta::new(*reimbursement_account_pk, false),
        AccountMeta::new_readonly(*mango_account_owner_pk, false),
        AccountMeta::new(*signer_pk, true),
        AccountMeta::new(*claim_mint_token_account_pk, false),
        AccountMeta::new(*claim_mint_pk, false),
        AccountMeta::new_readonly(*table_pk, false),
        AccountMeta::new_readonly(spl_token::ID, false),
        AccountMeta::new_readonly(solana_program::system_program::ID, false),
        AccountMeta::new_readonly(solana_program::sysvar::rent::ID, false)
    ];

    let mut ix_data = Vec::<u8>::new();
    ix_data.extend(REIMBURSE_OPCODE.to_be_bytes().to_vec());
    ix_data.extend_from_slice(&token_index.to_le_bytes());
    ix_data.extend_from_slice(&index_into_table.to_le_bytes());
    ix_data.extend_from_slice(&[transfer_claim as u8]);

    Ok(Instruction { program_id: *program_id, accounts, data: ix_data })
}
