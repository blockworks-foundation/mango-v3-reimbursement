pub mod instructions;
pub mod state;

use instructions::*;

use anchor_lang::prelude::*;

declare_id!("m3roABq4Ta3sGyFRLdY4LH1KN16zBtg586gJ3UxoBzb");

#[program]
pub mod mango_v3_reimbursement {

    use super::*;

    pub fn create_group(
        ctx: Context<CreateGroup>,
        group_num: u32,
        table: Pubkey,
        claim_transfer_destination: Pubkey,
    ) -> Result<()> {
        handle_create_group(ctx, group_num, table, claim_transfer_destination)
    }

    pub fn create_vault(
        ctx: Context<CreateVault>,
        token_index: usize,
        mint_decimals: u8,
    ) -> Result<()> {
        handle_create_vault(ctx, token_index, mint_decimals)
    }

    pub fn create_reimbursement_account(ctx: Context<CreateReimbursementAccount>) -> Result<()> {
        handle_create_reimbursement_account(ctx)
    }

    pub fn start_reimbursement(ctx: Context<StartReimbursement>) -> Result<()> {
        handle_start_reimbursement(ctx)
    }

    // pub fn reimburse<'key, 'accounts, 'remaining, 'info>(
    //     ctx: Context<'key, 'accounts, 'remaining, 'info, Reimburse<'info>>,
    //     index_into_table: usize,
    //     token_index: usize,
    //     transfer_claim: bool,
    // ) -> Result<()> {
    //     handle_reimburse(ctx, index_into_table, token_index, transfer_claim)
    // }
}

#[error_code]
pub enum Error {
    SomeError,
}

// ////
// // Reimburse
// ////

// #[derive(Accounts)]
// #[instruction(token_index: usize)]
// pub struct Reimburse<'info> {
//     #[account (
//         constraint = group.load()?.reimbursement_started == 1
//     )]
//     pub group: AccountLoader<'info, Group>,

//     #[account(mut)]
//     pub vault: Account<'info, TokenAccount>,

//     #[account(mut)]
//     pub token_account: Account<'info, TokenAccount>,
//     pub mint: Account<'info, Mint>,

//     #[account(mut)]
//     pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,
//     pub mango_account_owner: Signer<'info>,

//     #[account(mut)]
//     pub claim_mint_token_account: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub claim_mint: Account<'info, Mint>,

//     #[account(mut)]
//     pub payer: Signer<'info>,

//     pub table: AccountLoader<'info, Table>,

//     pub token_program: Program<'info, Token>,
//     pub system_program: Program<'info, System>,
//     pub rent: Sysvar<'info, Rent>,
// }

// fn handle_reimburse<'key, 'accounts, 'remaining, 'info>(
//     ctx: Context<'key, 'accounts, 'remaining, 'info, Reimburse<'info>>,
//     index_into_table: usize,
//     token_index: usize,
//     transfer_claim: bool,
// ) -> Result<()> {
//     let (pda_address, _) = Pubkey::find_program_address(
//         &[
//             b"MangoV3Reimbursement".as_ref(),
//             ctx.accounts.group.key().as_ref(),
//             ctx.accounts.mango_account_owner.key().as_ref(),
//             &token_index.to_le_bytes(),
//         ],
//         &crate::id(),
//     );
//     require_eq!(pda_address, ctx.accounts.reimbursement_account.key());

//     let group = ctx.accounts.group.load()?;
//     require_eq!(group.mints[token_index], ctx.accounts.mint.key());
//     require_eq!(ctx.accounts.vault.key(), group.vaults[token_index]);
//     require_eq!(
//         ctx.accounts.claim_mint.key(),
//         group.claim_mints[token_index]
//     );

//     let table = ctx.accounts.table.load()?;
//     require_eq!(
//         table.rows[index_into_table].owner,
//         ctx.accounts.mango_account_owner.key()
//     );

//     let mut reimbursement_account = ctx.accounts.reimbursement_account.load_mut()?;
//     require_eq!(reimbursement_account.done & (1 << token_index), 0);

//     let seeds = seeds!(group);
//     require_eq!(
//         ctx.accounts.token_account.owner,
//         ctx.accounts.mango_account_owner.key()
//     );
//     token::transfer(
//         {
//             let accounts = token::Transfer {
//                 from: ctx.accounts.vault.to_account_info(),
//                 to: ctx.accounts.token_account.to_account_info(),
//                 authority: ctx.accounts.group.to_account_info(),
//             };
//             CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts)
//                 .with_signer(&[seeds])
//         },
//         table.rows[index_into_table].balances[token_index],
//     )?;
//     reimbursement_account.done |= 1 << token_index;

//     if transfer_claim {
//         require_eq!(
//             ctx.accounts.claim_mint_token_account.owner,
//             group.claim_transfer_destination
//         );
//         token::mint_to(
//             {
//                 let accounts = token::MintTo {
//                     mint: ctx.accounts.claim_mint.to_account_info(),
//                     to: ctx.accounts.claim_mint_token_account.to_account_info(),
//                     authority: ctx.accounts.group.to_account_info(),
//                 };
//                 CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts)
//                     .with_signer(&[seeds])
//             },
//             table.rows[index_into_table].balances[token_index],
//         )?;
//         reimbursement_account.claim_transferred |= 1 << token_index;
//     }

//     Ok(())
// }
