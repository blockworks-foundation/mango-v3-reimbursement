use anchor_lang::{__private::bytemuck, prelude::*};
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::state::{Group, ReimbursementAccount, Table};

#[derive(Accounts)]
#[instruction(token_index: usize)]
pub struct Reimburse<'info> {
    #[account (
        constraint = group.load()?.reimbursement_started == 1
    )]
    pub group: AccountLoader<'info, Group>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,
    pub mango_account_owner: Signer<'info>,

    // #[account(mut)]
    // pub claim_mint_token_account: Account<'info, TokenAccount>,
    // #[account(mut)]
    // pub claim_mint: Account<'info, Mint>,
    //
    //
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
    let (pda_address, _) = Pubkey::find_program_address(
        &[
            b"ReimbursementAccount".as_ref(),
            ctx.accounts.group.key().as_ref(),
            ctx.accounts.mango_account_owner.key().as_ref(),
        ],
        &crate::id(),
    );
    require_eq!(pda_address, ctx.accounts.reimbursement_account.key());

    let group = ctx.accounts.group.load()?;
    require_eq!(group.mints[token_index], ctx.accounts.mint.key());
    require_eq!(ctx.accounts.vault.key(), group.vaults[token_index]);
    // require_eq!(
    //     ctx.accounts.claim_mint.key(),
    //     group.claim_mints[token_index]
    // );

    let table_ai = &ctx.accounts.table.to_account_info();
    let data = table_ai.try_borrow_data()?;
    let table = bytemuck::from_bytes::<Table>(&data[40..]);
    require_eq!(
        table.rows[index_into_table].owner,
        ctx.accounts.mango_account_owner.key()
    );

    let mut reimbursement_account = ctx.accounts.reimbursement_account.load_mut()?;
    require_eq!(reimbursement_account.done & (1 << token_index), 0);

    require_eq!(
        ctx.accounts.token_account.owner,
        ctx.accounts.mango_account_owner.key()
    );
    let signer_seeds = [
        b"Group".as_ref(),
        &group.group_num.to_le_bytes(),
        &[group.bump],
    ];
    token::transfer(
        {
            let accounts = token::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.group.to_account_info(),
            };
            CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts)
                .with_signer(&[&signer_seeds])
        },
        table.rows[index_into_table].balances[token_index],
    )?;
    // reimbursement_account.done |= 1 << token_index;

    // if transfer_claim {
    //     require_eq!(
    //         ctx.accounts.claim_mint_token_account.owner,
    //         group.claim_transfer_destination
    //     );
    //     token::mint_to(
    //         {
    //             let accounts = token::MintTo {
    //                 mint: ctx.accounts.claim_mint.to_account_info(),
    //                 to: ctx.accounts.claim_mint_token_account.to_account_info(),
    //                 authority: ctx.accounts.group.to_account_info(),
    //             };
    //             CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts)
    //                 .with_signer(&[seeds])
    //         },
    //         table.rows[index_into_table].balances[token_index],
    //     )?;
    //     reimbursement_account.claim_transferred |= 1 << token_index;
    // }

    Ok(())
}
