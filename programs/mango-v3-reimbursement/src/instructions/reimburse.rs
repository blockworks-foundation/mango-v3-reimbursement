use anchor_lang::{__private::bytemuck, prelude::*};
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::state::{Group, ReimbursementAccount, Table};

#[derive(Accounts)]
#[instruction(token_index: usize)]
pub struct Reimburse<'info> {
    #[account (
        constraint = group.load()?.has_reimbursement_started()
    )]
    pub group: AccountLoader<'info, Group>,

    #[account(
        mut,
        constraint = group.load()?.vaults[token_index] == vault.key()
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        constraint = group.load()?.mints[token_index] == mint.key()
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = token_account.owner == mango_account_owner.key()
    )]
    pub token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        // TODO: enable after testing is done
        // constraint = !reimbursement_account.load()?.reimbursed(token_index),
        // constraint = !reimbursement_account.load()?.claim_transferred(token_index),        
    )]
    pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,
    pub mango_account_owner: Signer<'info>,

    #[account(mut)]
    pub claim_mint_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = group.load()?.claim_mints[token_index] == claim_mint.key()
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
    let group = ctx.accounts.group.load()?;

    // Verify entry in reimbursement table
    let table_ai = &ctx.accounts.table;
    let data = table_ai.try_borrow_data()?;
    let table: &Table = bytemuck::from_bytes::<Table>(&data[40..]);
    require_eq!(
        table.rows[index_into_table].owner,
        ctx.accounts.mango_account_owner.key()
    );

    // Verify reimbursement_account
    let mut reimbursement_account = ctx.accounts.reimbursement_account.load_mut()?;
    let (pda_address, _) = Pubkey::find_program_address(
        &[
            b"ReimbursementAccount".as_ref(),
            ctx.accounts.group.key().as_ref(),
            ctx.accounts.mango_account_owner.key().as_ref(),
        ],
        &crate::id(),
    );
    require_eq!(pda_address, ctx.accounts.reimbursement_account.key());

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
        table.rows[index_into_table].balances[token_index],
    )?;
    reimbursement_account.mark_reimbursed(token_index);

    if transfer_claim {
        require_eq!(
            ctx.accounts.claim_mint_token_account.owner,
            group.claim_transfer_destination
        );
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
            table.rows[index_into_table].balances[token_index],
        )?;
        reimbursement_account.mark_claim_transferred(token_index);
    }

    Ok(())
}
