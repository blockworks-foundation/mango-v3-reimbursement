#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod instructions {
    pub use create_group::*;
    pub use create_reimbursement_account::*;
    pub use create_vault::*;
    pub use edit_group::*;
    pub use reimburse::*;
    pub use start_reimbursement::*;
    mod create_group {
        use anchor_lang::prelude::*;
        use crate::state::Group;
        use crate::Error;
        # [instruction (group_num : u32)]
        pub struct CreateGroup<'info> {
            # [account (init , seeds = [b"Group" . as_ref () , & group_num . to_le_bytes ()] , bump , payer = payer , space = 8 + std :: mem :: size_of :: < Group > () ,)]
            pub group: AccountLoader<'info, Group>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub authority: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for CreateGroup<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut ix_data = ix_data;
                struct __Args {
                    group_num: u32,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    u32: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
                        borsh::BorshSerialize::serialize(&self.group_num, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    u32: borsh::BorshDeserialize,
                {
                    fn deserialize(
                        buf: &mut &[u8],
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
                        Ok(Self {
                            group_num: borsh::BorshDeserialize::deserialize(buf)?,
                        })
                    }
                }
                let __Args { group_num } = __Args::deserialize(&mut ix_data)
                    .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
                if accounts.is_empty() {
                    return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                }
                let group = &accounts[0];
                *accounts = &accounts[1..];
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("payer"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("authority"))?;
                let system_program: anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[b"Group".as_ref(), &group_num.to_le_bytes()],
                    program_id,
                );
                __bumps.insert("group".to_string(), __bump);
                let group = {
                    let actual_field = group.to_account_info();
                    let actual_owner = actual_field.owner;
                    let space = 8 + std::mem::size_of::<Group>();
                    let pa: anchor_lang::accounts::account_loader::AccountLoader<Group> = if !false
                        || actual_owner == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = payer.to_account_info();
                        let __current_lamports = group.lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: group.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context.with_signer(&[&[
                                    b"Group".as_ref(),
                                    &group_num.to_le_bytes(),
                                    &[__bump][..],
                                ][..]]),
                                lamports,
                                space as u64,
                                program_id,
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: group.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: group.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context.with_signer(&[&[
                                    b"Group".as_ref(),
                                    &group_num.to_le_bytes(),
                                    &[__bump][..],
                                ][..]]),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: group.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context.with_signer(&[&[
                                    b"Group".as_ref(),
                                    &group_num.to_le_bytes(),
                                    &[__bump][..],
                                ][..]]),
                                program_id,
                            )?;
                        }
                        anchor_lang::accounts::account_loader::AccountLoader::try_from_unchecked(
                            program_id, &group,
                        )
                        .map_err(|e| e.with_account_name("group"))?
                    } else {
                        anchor_lang::accounts::account_loader::AccountLoader::try_from(&group)
                            .map_err(|e| e.with_account_name("group"))?
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("group")
                            .with_values((space, actual_field.data_len())));
                        }
                        if actual_owner != program_id {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("group")
                            .with_pubkeys((*actual_owner, *program_id)));
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("group"));
                            }
                        }
                    }
                    pa
                };
                if group.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("group")
                    .with_pubkeys((group.key(), __pda_address)));
                }
                if !group.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("group"));
                }
                if !__anchor_rent.is_exempt(
                    group.to_account_info().lamports(),
                    group.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("group"));
                }
                if !payer.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("payer"));
                }
                Ok(CreateGroup {
                    group,
                    payer,
                    authority,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CreateGroup<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.group.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CreateGroup<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.group.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for CreateGroup<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.group, program_id)
                    .map_err(|e| e.with_account_name("group"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_create_group {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`CreateGroup`].
            pub struct CreateGroup {
                pub group: anchor_lang::solana_program::pubkey::Pubkey,
                pub payer: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for CreateGroup
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.group, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for CreateGroup {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.group, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.payer, true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.authority,
                            true,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_create_group {
            use super::*;
            /// Generated CPI struct of the accounts for [`CreateGroup`].
            pub struct CreateGroup<'info> {
                pub group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for CreateGroup<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.group),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.payer),
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.authority),
                            true,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for CreateGroup<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.group));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.payer));
                    account_infos
                        .push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos
                }
            }
        }
        pub fn handle_create_group(
            ctx: Context<CreateGroup>,
            group_num: u32,
            table: Pubkey,
            claim_transfer_destination: Pubkey,
            testing: u8,
        ) -> Result<()> {
            ::solana_program::log::sol_log(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &[
                        "Creating group ",
                        " with table ",
                        " and claim_transfer_destination ",
                    ],
                    &[
                        ::core::fmt::ArgumentV1::new_debug(&group_num),
                        ::core::fmt::ArgumentV1::new_debug(&table),
                        ::core::fmt::ArgumentV1::new_debug(&claim_transfer_destination),
                    ],
                ));
                res
            });
            let mut group = ctx.accounts.group.load_init()?;
            group.group_num = group_num;
            group.table = table;
            group.claim_transfer_destination = claim_transfer_destination;
            group.authority = ctx.accounts.authority.key();
            group.bump = *ctx.bumps.get("group").ok_or(Error::SomeError)?;
            group.testing = testing;
            Ok(())
        }
    }
    mod create_reimbursement_account {
        use anchor_lang::prelude::*;
        use crate::state::{Group, ReimbursementAccount};
        pub struct CreateReimbursementAccount<'info> {
            pub group: AccountLoader<'info, Group>,
            # [account (init , seeds = [b"ReimbursementAccount" . as_ref () , group . key () . as_ref () , mango_account_owner . key () . as_ref ()] , bump , payer = payer , space = 8 + std :: mem :: size_of :: < ReimbursementAccount > () ,)]
            pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,
            pub mango_account_owner: UncheckedAccount<'info>,
            # [account (constraint = signer . key () == mango_account_owner . key () || signer . key () == group . load () ?. authority)]
            pub signer: Signer<'info>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub system_program: Program<'info, System>,
            pub rent: Sysvar<'info, Rent>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for CreateReimbursementAccount<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let group: anchor_lang::accounts::account_loader::AccountLoader<Group> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("group"))?;
                if accounts.is_empty() {
                    return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                }
                let reimbursement_account = &accounts[0];
                *accounts = &accounts[1..];
                let mango_account_owner: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("mango_account_owner"))?;
                let signer: Signer = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("signer"))?;
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("payer"))?;
                let system_program: anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let rent: Sysvar<Rent> = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"ReimbursementAccount".as_ref(),
                        group.key().as_ref(),
                        mango_account_owner.key().as_ref(),
                    ],
                    program_id,
                );
                __bumps.insert("reimbursement_account".to_string(), __bump);
                let reimbursement_account = {
                    let actual_field = reimbursement_account.to_account_info();
                    let actual_owner = actual_field.owner;
                    let space = 8 + std::mem::size_of::<ReimbursementAccount>();
                    let pa: anchor_lang::accounts::account_loader::AccountLoader<
                        ReimbursementAccount,
                    > = if !false
                        || actual_owner == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = payer.to_account_info();
                        let __current_lamports = reimbursement_account.lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: reimbursement_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context.with_signer(&[&[
                                    b"ReimbursementAccount".as_ref(),
                                    group.key().as_ref(),
                                    mango_account_owner.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                lamports,
                                space as u64,
                                program_id,
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: reimbursement_account.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: reimbursement_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context.with_signer(&[&[
                                    b"ReimbursementAccount".as_ref(),
                                    group.key().as_ref(),
                                    mango_account_owner.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: reimbursement_account.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context.with_signer(&[&[
                                    b"ReimbursementAccount".as_ref(),
                                    group.key().as_ref(),
                                    mango_account_owner.key().as_ref(),
                                    &[__bump][..],
                                ][..]]),
                                program_id,
                            )?;
                        }
                        anchor_lang::accounts::account_loader::AccountLoader::try_from_unchecked(
                            program_id,
                            &reimbursement_account,
                        )
                        .map_err(|e| e.with_account_name("reimbursement_account"))?
                    } else {
                        anchor_lang::accounts::account_loader::AccountLoader::try_from(
                            &reimbursement_account,
                        )
                        .map_err(|e| e.with_account_name("reimbursement_account"))?
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("reimbursement_account")
                            .with_values((space, actual_field.data_len())));
                        }
                        if actual_owner != program_id {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("reimbursement_account")
                            .with_pubkeys((*actual_owner, *program_id)));
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("reimbursement_account"));
                            }
                        }
                    }
                    pa
                };
                if reimbursement_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("reimbursement_account")
                    .with_pubkeys((reimbursement_account.key(), __pda_address)));
                }
                if !reimbursement_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("reimbursement_account"));
                }
                if !__anchor_rent.is_exempt(
                    reimbursement_account.to_account_info().lamports(),
                    reimbursement_account.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("reimbursement_account"));
                }
                if !(signer.key() == mango_account_owner.key()
                    || signer.key() == group.load()?.authority)
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("signer"));
                }
                if !payer.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("payer"));
                }
                Ok(CreateReimbursementAccount {
                    group,
                    reimbursement_account,
                    mango_account_owner,
                    signer,
                    payer,
                    system_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CreateReimbursementAccount<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.group.to_account_infos());
                account_infos.extend(self.reimbursement_account.to_account_infos());
                account_infos.extend(self.mango_account_owner.to_account_infos());
                account_infos.extend(self.signer.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CreateReimbursementAccount<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.group.to_account_metas(None));
                account_metas.extend(self.reimbursement_account.to_account_metas(None));
                account_metas.extend(self.mango_account_owner.to_account_metas(None));
                account_metas.extend(self.signer.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for CreateReimbursementAccount<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.reimbursement_account, program_id)
                    .map_err(|e| e.with_account_name("reimbursement_account"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_create_reimbursement_account {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`CreateReimbursementAccount`].
            pub struct CreateReimbursementAccount {
                pub group: anchor_lang::solana_program::pubkey::Pubkey,
                pub reimbursement_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub mango_account_owner: anchor_lang::solana_program::pubkey::Pubkey,
                pub signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub payer: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for CreateReimbursementAccount
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.group, writer)?;
                    borsh::BorshSerialize::serialize(&self.reimbursement_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.mango_account_owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for CreateReimbursementAccount {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.group, false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reimbursement_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.mango_account_owner,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.signer,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.payer, true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent, false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_create_reimbursement_account {
            use super::*;
            /// Generated CPI struct of the accounts for [`CreateReimbursementAccount`].
            pub struct CreateReimbursementAccount<'info> {
                pub group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reimbursement_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mango_account_owner:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for CreateReimbursementAccount<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.group),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reimbursement_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.mango_account_owner),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.signer),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.payer),
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for CreateReimbursementAccount<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.group));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.reimbursement_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.mango_account_owner,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.signer));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.payer));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                    account_infos
                }
            }
        }
        pub fn handle_create_reimbursement_account(
            _ctx: Context<CreateReimbursementAccount>,
        ) -> Result<()> {
            Ok(())
        }
    }
    mod create_vault {
        use crate::Error;
        use anchor_lang::prelude::*;
        use anchor_spl::{
            associated_token::AssociatedToken,
            token::{Mint, Token, TokenAccount},
        };
        use crate::state::Group;
        # [instruction (token_index : usize)]
        pub struct CreateVault<'info> {
            # [account (mut , has_one = authority , constraint =! group . load () ?. has_reimbursement_started ())]
            pub group: AccountLoader<'info, Group>,
            pub authority: Signer<'info>,
            # [account (init , associated_token :: mint = mint , payer = payer , associated_token :: authority = group ,)]
            pub vault: Account<'info, TokenAccount>,
            # [account (init , seeds = [b"Mint" . as_ref () , group . key () . as_ref () , & token_index . to_le_bytes ()] , bump , mint :: authority = group , mint :: decimals = mint . decimals , payer = payer)]
            pub claim_mint: Account<'info, Mint>,
            pub mint: Account<'info, Mint>,
            #[account(mut)]
            pub payer: Signer<'info>,
            pub token_program: Program<'info, Token>,
            pub associated_token_program: Program<'info, AssociatedToken>,
            pub system_program: Program<'info, System>,
            pub rent: Sysvar<'info, Rent>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for CreateVault<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut ix_data = ix_data;
                struct __Args {
                    token_index: usize,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    usize: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
                        borsh::BorshSerialize::serialize(&self.token_index, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    usize: borsh::BorshDeserialize,
                {
                    fn deserialize(
                        buf: &mut &[u8],
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
                        Ok(Self {
                            token_index: borsh::BorshDeserialize::deserialize(buf)?,
                        })
                    }
                }
                let __Args { token_index } = __Args::deserialize(&mut ix_data)
                    .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
                let group: anchor_lang::accounts::account_loader::AccountLoader<Group> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("group"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("authority"))?;
                if accounts.is_empty() {
                    return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                }
                let vault = &accounts[0];
                *accounts = &accounts[1..];
                if accounts.is_empty() {
                    return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                }
                let claim_mint = &accounts[0];
                *accounts = &accounts[1..];
                let mint: anchor_lang::accounts::account::Account<Mint> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("mint"))?;
                let payer: Signer = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("payer"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let associated_token_program: anchor_lang::accounts::program::Program<
                    AssociatedToken,
                > = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("associated_token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let rent: Sysvar<Rent> = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
                let __anchor_rent = Rent::get()?;
                let vault: anchor_lang::accounts::account::Account<TokenAccount> = {
                    if !false
                        || AsRef::<AccountInfo>::as_ref(&vault).owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = payer.to_account_info();
                        let cpi_program = associated_token_program.to_account_info();
                        let cpi_accounts = anchor_spl::associated_token::Create {
                            payer: payer.to_account_info(),
                            associated_token: vault.to_account_info(),
                            authority: group.to_account_info(),
                            mint: mint.to_account_info(),
                            system_program: system_program.to_account_info(),
                            token_program: token_program.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx =
                            anchor_lang::context::CpiContext::new(cpi_program, cpi_accounts);
                        anchor_spl::associated_token::create(cpi_ctx)?;
                    }
                    let pa: anchor_lang::accounts::account::Account<TokenAccount> =
                        anchor_lang::accounts::account::Account::try_from_unchecked(&vault)
                            .map_err(|e| e.with_account_name("vault"))?;
                    if false {
                        if pa.mint != mint.key() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenMint,
                            )
                            .with_account_name("vault")
                            .with_pubkeys((pa.mint, mint.key())));
                        }
                        if pa.owner != group.key() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                            )
                            .with_account_name("vault")
                            .with_pubkeys((pa.owner, group.key())));
                        }
                        if pa.key()
                            != anchor_spl::associated_token::get_associated_token_address(
                                &group.key(),
                                &mint.key(),
                            )
                        {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::AccountNotAssociatedTokenAccount,
                            )
                            .with_account_name("vault"));
                        }
                    }
                    pa
                };
                if !vault.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault"));
                }
                if !__anchor_rent.is_exempt(
                    vault.to_account_info().lamports(),
                    vault.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("vault"));
                }
                let __anchor_rent = Rent::get()?;
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"Mint".as_ref(),
                        group.key().as_ref(),
                        &token_index.to_le_bytes(),
                    ],
                    program_id,
                );
                __bumps.insert("claim_mint".to_string(), __bump);
                let claim_mint: anchor_lang::accounts::account::Account<Mint> = {
                    if !false
                        || AsRef::<AccountInfo>::as_ref(&claim_mint).owner
                            == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = payer.to_account_info();
                        let __current_lamports = claim_mint.lamports();
                        if __current_lamports == 0 {
                            let lamports =
                                __anchor_rent.minimum_balance(anchor_spl::token::Mint::LEN);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: claim_mint.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context.with_signer(&[&[
                                    b"Mint".as_ref(),
                                    group.key().as_ref(),
                                    &token_index.to_le_bytes(),
                                    &[__bump][..],
                                ][..]]),
                                lamports,
                                anchor_spl::token::Mint::LEN as u64,
                                &token_program.key(),
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(anchor_spl::token::Mint::LEN)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: claim_mint.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: claim_mint.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context.with_signer(&[&[
                                    b"Mint".as_ref(),
                                    group.key().as_ref(),
                                    &token_index.to_le_bytes(),
                                    &[__bump][..],
                                ][..]]),
                                anchor_spl::token::Mint::LEN as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: claim_mint.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context.with_signer(&[&[
                                    b"Mint".as_ref(),
                                    group.key().as_ref(),
                                    &token_index.to_le_bytes(),
                                    &[__bump][..],
                                ][..]]),
                                &token_program.key(),
                            )?;
                        }
                        let cpi_program = token_program.to_account_info();
                        let accounts = anchor_spl::token::InitializeMint {
                            mint: claim_mint.to_account_info(),
                            rent: rent.to_account_info(),
                        };
                        let cpi_ctx = anchor_lang::context::CpiContext::new(cpi_program, accounts);
                        anchor_spl::token::initialize_mint(
                            cpi_ctx,
                            mint.decimals,
                            &group.key(),
                            Option::<&anchor_lang::prelude::Pubkey>::None,
                        )?;
                    }
                    let pa: anchor_lang::accounts::account::Account<Mint> =
                        anchor_lang::accounts::account::Account::try_from_unchecked(&claim_mint)
                            .map_err(|e| e.with_account_name("claim_mint"))?;
                    if false {
                        if pa.mint_authority
                            != anchor_lang::solana_program::program_option::COption::Some(
                                group.key(),
                            )
                        {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMintMintAuthority,
                            )
                            .with_account_name("claim_mint"));
                        }
                        if pa
                            .freeze_authority
                            .as_ref()
                            .map(|fa| {
                                Option::<&anchor_lang::prelude::Pubkey>::None
                                    .as_ref()
                                    .map(|expected_fa| fa != *expected_fa)
                                    .unwrap_or(true)
                            })
                            .unwrap_or(Option::<&anchor_lang::prelude::Pubkey>::None.is_some())
                        {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMintFreezeAuthority,
                            )
                            .with_account_name("claim_mint"));
                        }
                        if pa.decimals != mint.decimals {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintMintDecimals,
                            )
                            .with_account_name("claim_mint")
                            .with_values((pa.decimals, mint.decimals)));
                        }
                    }
                    pa
                };
                if claim_mint.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("claim_mint")
                    .with_pubkeys((claim_mint.key(), __pda_address)));
                }
                if !claim_mint.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("claim_mint"));
                }
                if !__anchor_rent.is_exempt(
                    claim_mint.to_account_info().lamports(),
                    claim_mint.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("claim_mint"));
                }
                if !group.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("group"));
                }
                {
                    let my_key = group.load()?.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("group")
                        .with_pubkeys((my_key, target_key)));
                    }
                }
                if !(!group.load()?.has_reimbursement_started()) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("group"));
                }
                if !payer.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("payer"));
                }
                Ok(CreateVault {
                    group,
                    authority,
                    vault,
                    claim_mint,
                    mint,
                    payer,
                    token_program,
                    associated_token_program,
                    system_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for CreateVault<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.group.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos.extend(self.vault.to_account_infos());
                account_infos.extend(self.claim_mint.to_account_infos());
                account_infos.extend(self.mint.to_account_infos());
                account_infos.extend(self.payer.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.associated_token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for CreateVault<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.group.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas.extend(self.vault.to_account_metas(None));
                account_metas.extend(self.claim_mint.to_account_metas(None));
                account_metas.extend(self.mint.to_account_metas(None));
                account_metas.extend(self.payer.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.associated_token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for CreateVault<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.group, program_id)
                    .map_err(|e| e.with_account_name("group"))?;
                anchor_lang::AccountsExit::exit(&self.vault, program_id)
                    .map_err(|e| e.with_account_name("vault"))?;
                anchor_lang::AccountsExit::exit(&self.claim_mint, program_id)
                    .map_err(|e| e.with_account_name("claim_mint"))?;
                anchor_lang::AccountsExit::exit(&self.payer, program_id)
                    .map_err(|e| e.with_account_name("payer"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_create_vault {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`CreateVault`].
            pub struct CreateVault {
                pub group: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub claim_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub payer: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub associated_token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for CreateVault
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.group, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.claim_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.payer, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.associated_token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for CreateVault {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.group, false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.authority,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.claim_mint,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.mint, false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.payer, true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.associated_token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent, false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_create_vault {
            use super::*;
            /// Generated CPI struct of the accounts for [`CreateVault`].
            pub struct CreateVault<'info> {
                pub group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub claim_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub payer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub associated_token_program:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for CreateVault<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.group),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.authority),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.claim_mint),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.mint),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.payer),
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.associated_token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for CreateVault<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.group));
                    account_infos
                        .push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.claim_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.mint));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.payer));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.associated_token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                    account_infos
                }
            }
        }
        pub fn handle_create_vault(ctx: Context<CreateVault>, token_index: usize) -> Result<()> {
            if !(token_index < 16usize) {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : Error :: SomeError . name () , error_code_number : Error :: SomeError . into () , error_msg : Error :: SomeError . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/mango-v3-reimbursement/src/instructions/create_vault.rs" , line : 52u32 , })) , compared_values : None , })) ;
            };
            let mut group = ctx.accounts.group.load_mut()?;
            if group.vaults[token_index] != Pubkey::default() {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : anchor_lang :: error :: ErrorCode :: RequireEqViolated . name () , error_code_number : anchor_lang :: error :: ErrorCode :: RequireEqViolated . into () , error_msg : anchor_lang :: error :: ErrorCode :: RequireEqViolated . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/mango-v3-reimbursement/src/instructions/create_vault.rs" , line : 55u32 , })) , compared_values : None , }) . with_values ((group . vaults [token_index] , Pubkey :: default ()))) ;
            };
            if group.claim_mints[token_index] != Pubkey::default() {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : anchor_lang :: error :: ErrorCode :: RequireEqViolated . name () , error_code_number : anchor_lang :: error :: ErrorCode :: RequireEqViolated . into () , error_msg : anchor_lang :: error :: ErrorCode :: RequireEqViolated . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/mango-v3-reimbursement/src/instructions/create_vault.rs" , line : 56u32 , })) , compared_values : None , }) . with_values ((group . claim_mints [token_index] , Pubkey :: default ()))) ;
            };
            if group.mints[token_index] != Pubkey::default() {
                return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : anchor_lang :: error :: ErrorCode :: RequireEqViolated . name () , error_code_number : anchor_lang :: error :: ErrorCode :: RequireEqViolated . into () , error_msg : anchor_lang :: error :: ErrorCode :: RequireEqViolated . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/mango-v3-reimbursement/src/instructions/create_vault.rs" , line : 57u32 , })) , compared_values : None , }) . with_values ((group . mints [token_index] , Pubkey :: default ()))) ;
            };
            group.vaults[token_index] = ctx.accounts.vault.key();
            group.claim_mints[token_index] = ctx.accounts.claim_mint.key();
            group.mints[token_index] = ctx.accounts.mint.key();
            Ok(())
        }
    }
    mod edit_group {
        use anchor_lang::prelude::*;
        use crate::state::Group;
        pub struct EditGroup<'info> {
            # [account (mut , has_one = authority , constraint = group . load () ?. is_testing () ,)]
            pub group: AccountLoader<'info, Group>,
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for EditGroup<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let group: anchor_lang::accounts::account_loader::AccountLoader<Group> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("group"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("authority"))?;
                if !group.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("group"));
                }
                {
                    let my_key = group.load()?.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("group")
                        .with_pubkeys((my_key, target_key)));
                    }
                }
                if !(group.load()?.is_testing()) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("group"));
                }
                Ok(EditGroup { group, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for EditGroup<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.group.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for EditGroup<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.group.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for EditGroup<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.group, program_id)
                    .map_err(|e| e.with_account_name("group"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_edit_group {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`EditGroup`].
            pub struct EditGroup {
                pub group: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for EditGroup
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.group, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for EditGroup {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.group, false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.authority,
                            true,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_edit_group {
            use super::*;
            /// Generated CPI struct of the accounts for [`EditGroup`].
            pub struct EditGroup<'info> {
                pub group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for EditGroup<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.group),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.authority),
                            true,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for EditGroup<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.group));
                    account_infos
                        .push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                    account_infos
                }
            }
        }
        pub fn handle_edit_group(ctx: Context<EditGroup>, table: Pubkey) -> Result<()> {
            let mut group = ctx.accounts.group.load_mut()?;
            group.table = table;
            Ok(())
        }
    }
    mod reimburse {
        use crate::state::{Group, ReimbursementAccount, Table};
        use crate::Error;
        use anchor_lang::{__private::bytemuck, prelude::*};
        use anchor_spl::token::{self, Mint, Token, TokenAccount};
        # [instruction (token_index : usize)]
        pub struct Reimburse<'info> {
            # [account (constraint = group . load () ?. has_reimbursement_started ())]
            pub group: AccountLoader<'info, Group>,
            # [account (mut , address = group . load () ?. vaults [token_index])]
            pub vault: Account<'info, TokenAccount>,
            # [account (mut , constraint = token_account . owner == mango_account_owner . key ())]
            pub token_account: Box<Account<'info, TokenAccount>>,
            # [account (mut , seeds = [b"ReimbursementAccount" . as_ref () , group . key () . as_ref () , mango_account_owner . key () . as_ref ()] , bump , constraint = group . load () ?. is_testing () ||! reimbursement_account . load () ?. reimbursed (token_index) , constraint = group . load () ?. is_testing () ||! reimbursement_account . load () ?. claim_transferred (token_index) ,)]
            pub reimbursement_account: AccountLoader<'info, ReimbursementAccount>,
            pub mango_account_owner: UncheckedAccount<'info>,
            # [account (constraint = signer . key () == mango_account_owner . key () || signer . key () == group . load () ?. authority)]
            pub signer: Signer<'info>,
            # [account (mut , associated_token :: mint = claim_mint , associated_token :: authority = group . load () ?. claim_transfer_destination ,)]
            pub claim_mint_token_account: Box<Account<'info, TokenAccount>>,
            # [account (mut , address = group . load () ?. claim_mints [token_index])]
            pub claim_mint: Box<Account<'info, Mint>>,
            /// CHECK:
            pub table: UncheckedAccount<'info>,
            pub token_program: Program<'info, Token>,
            pub system_program: Program<'info, System>,
            pub rent: Sysvar<'info, Rent>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for Reimburse<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let mut ix_data = ix_data;
                struct __Args {
                    token_index: usize,
                }
                impl borsh::ser::BorshSerialize for __Args
                where
                    usize: borsh::ser::BorshSerialize,
                {
                    fn serialize<W: borsh::maybestd::io::Write>(
                        &self,
                        writer: &mut W,
                    ) -> ::core::result::Result<(), borsh::maybestd::io::Error>
                    {
                        borsh::BorshSerialize::serialize(&self.token_index, writer)?;
                        Ok(())
                    }
                }
                impl borsh::de::BorshDeserialize for __Args
                where
                    usize: borsh::BorshDeserialize,
                {
                    fn deserialize(
                        buf: &mut &[u8],
                    ) -> ::core::result::Result<Self, borsh::maybestd::io::Error>
                    {
                        Ok(Self {
                            token_index: borsh::BorshDeserialize::deserialize(buf)?,
                        })
                    }
                }
                let __Args { token_index } = __Args::deserialize(&mut ix_data)
                    .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
                let group: anchor_lang::accounts::account_loader::AccountLoader<Group> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("group"))?;
                let vault: anchor_lang::accounts::account::Account<TokenAccount> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("vault"))?;
                let token_account: Box<anchor_lang::accounts::account::Account<TokenAccount>> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_account"))?;
                let reimbursement_account: anchor_lang::accounts::account_loader::AccountLoader<
                    ReimbursementAccount,
                > = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("reimbursement_account"))?;
                let mango_account_owner: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("mango_account_owner"))?;
                let signer: Signer = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("signer"))?;
                let claim_mint_token_account: Box<
                    anchor_lang::accounts::account::Account<TokenAccount>,
                > = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("claim_mint_token_account"))?;
                let claim_mint: Box<anchor_lang::accounts::account::Account<Mint>> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("claim_mint"))?;
                let table: UncheckedAccount = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("table"))?;
                let token_program: anchor_lang::accounts::program::Program<Token> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("token_program"))?;
                let system_program: anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("system_program"))?;
                let rent: Sysvar<Rent> = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("rent"))?;
                if !(group.load()?.has_reimbursement_started()) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("group"));
                }
                if !vault.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("vault"));
                }
                {
                    let actual = vault.key();
                    let expected = group.load()?.vaults[token_index];
                    if actual != expected {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAddress,
                        )
                        .with_account_name("vault")
                        .with_pubkeys((actual, expected)));
                    }
                }
                if !token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("token_account"));
                }
                if !(token_account.owner == mango_account_owner.key()) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("token_account"));
                }
                let (__pda_address, __bump) = Pubkey::find_program_address(
                    &[
                        b"ReimbursementAccount".as_ref(),
                        group.key().as_ref(),
                        mango_account_owner.key().as_ref(),
                    ],
                    &program_id,
                );
                __bumps.insert("reimbursement_account".to_string(), __bump);
                if reimbursement_account.key() != __pda_address {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSeeds,
                    )
                    .with_account_name("reimbursement_account")
                    .with_pubkeys((reimbursement_account.key(), __pda_address)));
                }
                if !reimbursement_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("reimbursement_account"));
                }
                if !(group.load()?.is_testing()
                    || !reimbursement_account.load()?.reimbursed(token_index))
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("reimbursement_account"));
                }
                if !(group.load()?.is_testing()
                    || !reimbursement_account.load()?.claim_transferred(token_index))
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("reimbursement_account"));
                }
                if !(signer.key() == mango_account_owner.key()
                    || signer.key() == group.load()?.authority)
                {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("signer"));
                }
                {
                    let my_owner = claim_mint_token_account.owner;
                    let wallet_address = group.load()?.claim_transfer_destination.key();
                    if my_owner != wallet_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintTokenOwner,
                        )
                        .with_account_name("claim_mint_token_account")
                        .with_pubkeys((my_owner, wallet_address)));
                    }
                    let __associated_token_address =
                        anchor_spl::associated_token::get_associated_token_address(
                            &wallet_address,
                            &claim_mint.key(),
                        );
                    let my_key = claim_mint_token_account.key();
                    if my_key != __associated_token_address {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAssociated,
                        )
                        .with_account_name("claim_mint_token_account")
                        .with_pubkeys((my_key, __associated_token_address)));
                    }
                }
                if !claim_mint_token_account.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("claim_mint_token_account"));
                }
                if !claim_mint.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("claim_mint"));
                }
                {
                    let actual = claim_mint.key();
                    let expected = group.load()?.claim_mints[token_index];
                    if actual != expected {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintAddress,
                        )
                        .with_account_name("claim_mint")
                        .with_pubkeys((actual, expected)));
                    }
                }
                Ok(Reimburse {
                    group,
                    vault,
                    token_account,
                    reimbursement_account,
                    mango_account_owner,
                    signer,
                    claim_mint_token_account,
                    claim_mint,
                    table,
                    token_program,
                    system_program,
                    rent,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Reimburse<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.group.to_account_infos());
                account_infos.extend(self.vault.to_account_infos());
                account_infos.extend(self.token_account.to_account_infos());
                account_infos.extend(self.reimbursement_account.to_account_infos());
                account_infos.extend(self.mango_account_owner.to_account_infos());
                account_infos.extend(self.signer.to_account_infos());
                account_infos.extend(self.claim_mint_token_account.to_account_infos());
                account_infos.extend(self.claim_mint.to_account_infos());
                account_infos.extend(self.table.to_account_infos());
                account_infos.extend(self.token_program.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos.extend(self.rent.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Reimburse<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.group.to_account_metas(None));
                account_metas.extend(self.vault.to_account_metas(None));
                account_metas.extend(self.token_account.to_account_metas(None));
                account_metas.extend(self.reimbursement_account.to_account_metas(None));
                account_metas.extend(self.mango_account_owner.to_account_metas(None));
                account_metas.extend(self.signer.to_account_metas(None));
                account_metas.extend(self.claim_mint_token_account.to_account_metas(None));
                account_metas.extend(self.claim_mint.to_account_metas(None));
                account_metas.extend(self.table.to_account_metas(None));
                account_metas.extend(self.token_program.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas.extend(self.rent.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Reimburse<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.vault, program_id)
                    .map_err(|e| e.with_account_name("vault"))?;
                anchor_lang::AccountsExit::exit(&self.token_account, program_id)
                    .map_err(|e| e.with_account_name("token_account"))?;
                anchor_lang::AccountsExit::exit(&self.reimbursement_account, program_id)
                    .map_err(|e| e.with_account_name("reimbursement_account"))?;
                anchor_lang::AccountsExit::exit(&self.claim_mint_token_account, program_id)
                    .map_err(|e| e.with_account_name("claim_mint_token_account"))?;
                anchor_lang::AccountsExit::exit(&self.claim_mint, program_id)
                    .map_err(|e| e.with_account_name("claim_mint"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_reimburse {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`Reimburse`].
            pub struct Reimburse {
                pub group: anchor_lang::solana_program::pubkey::Pubkey,
                pub vault: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub reimbursement_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub mango_account_owner: anchor_lang::solana_program::pubkey::Pubkey,
                pub signer: anchor_lang::solana_program::pubkey::Pubkey,
                pub claim_mint_token_account: anchor_lang::solana_program::pubkey::Pubkey,
                pub claim_mint: anchor_lang::solana_program::pubkey::Pubkey,
                pub table: anchor_lang::solana_program::pubkey::Pubkey,
                pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
                pub rent: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for Reimburse
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.group, writer)?;
                    borsh::BorshSerialize::serialize(&self.vault, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.reimbursement_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.mango_account_owner, writer)?;
                    borsh::BorshSerialize::serialize(&self.signer, writer)?;
                    borsh::BorshSerialize::serialize(&self.claim_mint_token_account, writer)?;
                    borsh::BorshSerialize::serialize(&self.claim_mint, writer)?;
                    borsh::BorshSerialize::serialize(&self.table, writer)?;
                    borsh::BorshSerialize::serialize(&self.token_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    borsh::BorshSerialize::serialize(&self.rent, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Reimburse {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.group, false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.vault, false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.reimbursement_account,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.mango_account_owner,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.signer,
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.claim_mint_token_account,
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.claim_mint,
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.table, false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.token_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.rent, false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_reimburse {
            use super::*;
            /// Generated CPI struct of the accounts for [`Reimburse`].
            pub struct Reimburse<'info> {
                pub group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub vault: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_account: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub reimbursement_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub mango_account_owner:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub signer: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub claim_mint_token_account:
                    anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub claim_mint: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub table: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub token_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub rent: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Reimburse<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.group),
                            false,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.vault),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.reimbursement_account),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.mango_account_owner),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.signer),
                            true,
                        ),
                    );
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.claim_mint_token_account),
                        false,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.claim_mint),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.table),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.token_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.rent),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Reimburse<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.group));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.vault));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.reimbursement_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.mango_account_owner,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.signer));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.claim_mint_token_account,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.claim_mint,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.table));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.token_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.rent));
                    account_infos
                }
            }
        }
        pub fn handle_reimburse<'key, 'accounts, 'remaining, 'info>(
            ctx: Context<'key, 'accounts, 'remaining, 'info, Reimburse<'info>>,
            index_into_table: usize,
            token_index: usize,
            transfer_claim: bool,
        ) -> Result<()> {
            if !(token_index < 16usize) {
                return Err(anchor_lang::error::Error::from(
                    anchor_lang::error::AnchorError {
                        error_name: Error::SomeError.name(),
                        error_code_number: Error::SomeError.into(),
                        error_msg: Error::SomeError.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename:
                                    "programs/mango-v3-reimbursement/src/instructions/reimburse.rs",
                                line: 66u32,
                            },
                        )),
                        compared_values: None,
                    },
                ));
            };
            let group = ctx.accounts.group.load()?;
            let table_ai = &ctx.accounts.table;
            let data = table_ai.try_borrow_data()?;
            let table: &Table = bytemuck::from_bytes::<Table>(&data[40..]);
            if table.rows[index_into_table].owner != ctx.accounts.mango_account_owner.key() {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: anchor_lang::error::ErrorCode::RequireKeysEqViolated.name(),
                        error_code_number: anchor_lang::error::ErrorCode::RequireKeysEqViolated
                            .into(),
                        error_msg: anchor_lang::error::ErrorCode::RequireKeysEqViolated.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename:
                                    "programs/mango-v3-reimbursement/src/instructions/reimburse.rs",
                                line: 74u32,
                            },
                        )),
                        compared_values: None,
                    })
                    .with_pubkeys((
                        table.rows[index_into_table].owner,
                        ctx.accounts.mango_account_owner.key(),
                    )),
                );
            };
            token::transfer(
                {
                    let accounts = token::Transfer {
                        from: ctx.accounts.vault.to_account_info(),
                        to: ctx.accounts.token_account.to_account_info(),
                        authority: ctx.accounts.group.to_account_info(),
                    };
                    CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts)
                        .with_signer(&[&[
                            b"Group".as_ref(),
                            &group.group_num.to_le_bytes(),
                            &[group.bump],
                        ]])
                },
                table.rows[index_into_table].balances[token_index],
            )?;
            let mut reimbursement_account = ctx.accounts.reimbursement_account.load_mut()?;
            reimbursement_account.mark_reimbursed(token_index);
            if transfer_claim {
                token::mint_to(
                    {
                        let accounts = token::MintTo {
                            mint: ctx.accounts.claim_mint.to_account_info(),
                            to: ctx.accounts.claim_mint_token_account.to_account_info(),
                            authority: ctx.accounts.group.to_account_info(),
                        };
                        CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts)
                            .with_signer(&[&[
                                b"Group".as_ref(),
                                &group.group_num.to_le_bytes(),
                                &[group.bump],
                            ]])
                    },
                    table.rows[index_into_table].balances[token_index],
                )?;
                reimbursement_account.mark_claim_transferred(token_index);
            }
            Ok(())
        }
    }
    mod start_reimbursement {
        use anchor_lang::prelude::*;
        use crate::state::Group;
        pub struct StartReimbursement<'info> {
            # [account (mut , has_one = authority , constraint =! group . load () ?. has_reimbursement_started ())]
            pub group: AccountLoader<'info, Group>,
            pub authority: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for StartReimbursement<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
                __reallocs: &mut std::collections::BTreeSet<
                    anchor_lang::solana_program::pubkey::Pubkey,
                >,
            ) -> anchor_lang::Result<Self> {
                let group: anchor_lang::accounts::account_loader::AccountLoader<Group> =
                    anchor_lang::Accounts::try_accounts(
                        program_id, accounts, ix_data, __bumps, __reallocs,
                    )
                    .map_err(|e| e.with_account_name("group"))?;
                let authority: Signer = anchor_lang::Accounts::try_accounts(
                    program_id, accounts, ix_data, __bumps, __reallocs,
                )
                .map_err(|e| e.with_account_name("authority"))?;
                if !group.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("group"));
                }
                {
                    let my_key = group.load()?.authority;
                    let target_key = authority.key();
                    if my_key != target_key {
                        return Err(anchor_lang::error::Error::from(
                            anchor_lang::error::ErrorCode::ConstraintHasOne,
                        )
                        .with_account_name("group")
                        .with_pubkeys((my_key, target_key)));
                    }
                }
                if !(!group.load()?.has_reimbursement_started()) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRaw,
                    )
                    .with_account_name("group"));
                }
                Ok(StartReimbursement { group, authority })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for StartReimbursement<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.group.to_account_infos());
                account_infos.extend(self.authority.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for StartReimbursement<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.group.to_account_metas(None));
                account_metas.extend(self.authority.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for StartReimbursement<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.group, program_id)
                    .map_err(|e| e.with_account_name("group"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_start_reimbursement {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`StartReimbursement`].
            pub struct StartReimbursement {
                pub group: anchor_lang::solana_program::pubkey::Pubkey,
                pub authority: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for StartReimbursement
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.group, writer)?;
                    borsh::BorshSerialize::serialize(&self.authority, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for StartReimbursement {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.group, false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.authority,
                            true,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_start_reimbursement {
            use super::*;
            /// Generated CPI struct of the accounts for [`StartReimbursement`].
            pub struct StartReimbursement<'info> {
                pub group: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub authority: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for StartReimbursement<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.group),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.authority),
                            true,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for StartReimbursement<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.group));
                    account_infos
                        .push(anchor_lang::ToAccountInfo::to_account_info(&self.authority));
                    account_infos
                }
            }
        }
        pub fn handle_start_reimbursement(ctx: Context<StartReimbursement>) -> Result<()> {
            let mut group = ctx.accounts.group.load_mut()?;
            group.reimbursement_started = 1;
            Ok(())
        }
    }
}
pub mod state {
    pub use group::*;
    pub use reimbursement_account::*;
    pub use table::*;
    mod group {
        use std::mem::size_of;
        pub use anchor_lang::prelude::*;
        use static_assertions::const_assert_eq;
        #[repr(C)]
        pub struct Group {
            pub group_num: u32,
            pub table: Pubkey,
            pub claim_transfer_destination: Pubkey,
            pub authority: Pubkey,
            pub vaults: [Pubkey; 16],
            pub claim_mints: [Pubkey; 16],
            pub mints: [Pubkey; 16],
            pub reimbursement_started: u8,
            pub bump: u8,
            pub testing: u8,
            pub padding: [u8; 1],
        }
        #[automatically_derived]
        impl Group {}
        #[automatically_derived]
        impl ::core::marker::Copy for Group {}
        #[automatically_derived]
        impl ::core::clone::Clone for Group {
            #[inline]
            fn clone(&self) -> Group {
                let _: ::core::clone::AssertParamIsClone<u32>;
                let _: ::core::clone::AssertParamIsClone<Pubkey>;
                let _: ::core::clone::AssertParamIsClone<[Pubkey; 16]>;
                let _: ::core::clone::AssertParamIsClone<[Pubkey; 16]>;
                let _: ::core::clone::AssertParamIsClone<[Pubkey; 16]>;
                let _: ::core::clone::AssertParamIsClone<u8>;
                let _: ::core::clone::AssertParamIsClone<[u8; 1]>;
                *self
            }
        }
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Pod for Group {}
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Zeroable for Group {}
        #[automatically_derived]
        impl anchor_lang::ZeroCopy for Group {}
        #[automatically_derived]
        impl anchor_lang::Discriminator for Group {
            fn discriminator() -> [u8; 8] {
                [209, 249, 208, 63, 182, 89, 186, 254]
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for Group {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [209, 249, 208, 63, 182, 89, 186, 254].len() {
                    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
                }
                let given_disc = &buf[..8];
                if &[209, 249, 208, 63, 182, 89, 186, 254] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number:
                                anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/mango-v3-reimbursement/src/state/group.rs",
                                    line: 6u32,
                                },
                            )),
                            compared_values: None,
                        })
                        .with_account_name("Group"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let data: &[u8] = &buf[8..];
                let account = anchor_lang::__private::bytemuck::from_bytes(data);
                Ok(*account)
            }
        }
        #[automatically_derived]
        impl anchor_lang::Owner for Group {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool =
                size_of::<Group>() == 4 + 32 + 32 + 32 + 32 * 16 + 32 * 16 + 32 * 16 + 4;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = size_of::<Group>() % 8 == 0;
            ASSERT
        } as usize] = [];
        impl Group {
            pub fn has_reimbursement_started(&self) -> bool {
                self.reimbursement_started == 1
            }
            pub fn is_testing(&self) -> bool {
                self.testing == 1
            }
        }
    }
    mod reimbursement_account {
        use std::mem::size_of;
        use anchor_lang::prelude::*;
        use static_assertions::const_assert_eq;
        #[repr(C)]
        pub struct ReimbursementAccount {
            pub reimbursed: u16,
            pub claim_transferred: u16,
            pub padding: [u8; 4],
        }
        #[automatically_derived]
        impl ReimbursementAccount {}
        #[automatically_derived]
        impl ::core::marker::Copy for ReimbursementAccount {}
        #[automatically_derived]
        impl ::core::clone::Clone for ReimbursementAccount {
            #[inline]
            fn clone(&self) -> ReimbursementAccount {
                let _: ::core::clone::AssertParamIsClone<u16>;
                let _: ::core::clone::AssertParamIsClone<[u8; 4]>;
                *self
            }
        }
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Pod for ReimbursementAccount {}
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Zeroable for ReimbursementAccount {}
        #[automatically_derived]
        impl anchor_lang::ZeroCopy for ReimbursementAccount {}
        #[automatically_derived]
        impl anchor_lang::Discriminator for ReimbursementAccount {
            fn discriminator() -> [u8; 8] {
                [155, 71, 126, 130, 2, 189, 251, 164]
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for ReimbursementAccount {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [155, 71, 126, 130, 2, 189, 251, 164].len() {
                    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
                }
                let given_disc = &buf[..8];
                if &[155, 71, 126, 130, 2, 189, 251, 164] != given_disc {
                    return Err (anchor_lang :: error :: Error :: from (anchor_lang :: error :: AnchorError { error_name : anchor_lang :: error :: ErrorCode :: AccountDiscriminatorMismatch . name () , error_code_number : anchor_lang :: error :: ErrorCode :: AccountDiscriminatorMismatch . into () , error_msg : anchor_lang :: error :: ErrorCode :: AccountDiscriminatorMismatch . to_string () , error_origin : Some (anchor_lang :: error :: ErrorOrigin :: Source (anchor_lang :: error :: Source { filename : "programs/mango-v3-reimbursement/src/state/reimbursement_account.rs" , line : 6u32 , })) , compared_values : None , }) . with_account_name ("ReimbursementAccount")) ;
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let data: &[u8] = &buf[8..];
                let account = anchor_lang::__private::bytemuck::from_bytes(data);
                Ok(*account)
            }
        }
        #[automatically_derived]
        impl anchor_lang::Owner for ReimbursementAccount {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = size_of::<ReimbursementAccount>() == 8;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = size_of::<ReimbursementAccount>() % 8 == 0;
            ASSERT
        } as usize] = [];
        impl ReimbursementAccount {
            pub fn reimbursed(&self, token_index: usize) -> bool {
                self.reimbursed & (1 << token_index) == 1
            }
            pub fn mark_reimbursed(&mut self, token_index: usize) {
                self.reimbursed |= 1 << token_index
            }
            pub fn claim_transferred(&self, token_index: usize) -> bool {
                self.reimbursed & (1 << token_index) == 1
            }
            pub fn mark_claim_transferred(&mut self, token_index: usize) {
                self.claim_transferred |= 1 << token_index
            }
        }
    }
    mod table {
        use std::mem::size_of;
        use anchor_lang::prelude::*;
        use static_assertions::const_assert_eq;
        #[repr(C)]
        pub struct Table {
            pub rows: [Row; 1],
        }
        #[automatically_derived]
        impl Table {}
        #[automatically_derived]
        impl ::core::marker::Copy for Table {}
        #[automatically_derived]
        impl ::core::clone::Clone for Table {
            #[inline]
            fn clone(&self) -> Table {
                let _: ::core::clone::AssertParamIsClone<[Row; 1]>;
                *self
            }
        }
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Pod for Table {}
        #[automatically_derived]
        unsafe impl anchor_lang::__private::bytemuck::Zeroable for Table {}
        #[automatically_derived]
        impl anchor_lang::ZeroCopy for Table {}
        #[automatically_derived]
        impl anchor_lang::Discriminator for Table {
            fn discriminator() -> [u8; 8] {
                [34, 100, 138, 97, 236, 129, 230, 112]
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for Table {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [34, 100, 138, 97, 236, 129, 230, 112].len() {
                    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
                }
                let given_disc = &buf[..8];
                if &[34, 100, 138, 97, 236, 129, 230, 112] != given_disc {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .name(),
                            error_code_number:
                                anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into(),
                            error_msg: anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                                .to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/mango-v3-reimbursement/src/state/table.rs",
                                    line: 6u32,
                                },
                            )),
                            compared_values: None,
                        })
                        .with_account_name("Table"),
                    );
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let data: &[u8] = &buf[8..];
                let account = anchor_lang::__private::bytemuck::from_bytes(data);
                Ok(*account)
            }
        }
        #[automatically_derived]
        impl anchor_lang::Owner for Table {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = size_of::<Table>() == (32 + 8 * 16) * 1;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = size_of::<Table>() % 8 == 0;
            ASSERT
        } as usize] = [];
        #[repr(C)]
        pub struct Row {
            pub owner: Pubkey,
            pub balances: [u64; 16],
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Row {}
        #[automatically_derived]
        impl ::core::clone::Clone for Row {
            #[inline]
            fn clone(&self) -> Row {
                let _: ::core::clone::AssertParamIsClone<Pubkey>;
                let _: ::core::clone::AssertParamIsClone<[u64; 16]>;
                *self
            }
        }
        impl borsh::ser::BorshSerialize for Row
        where
            Pubkey: borsh::ser::BorshSerialize,
            [u64; 16]: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.owner, writer)?;
                borsh::BorshSerialize::serialize(&self.balances, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Row
        where
            Pubkey: borsh::BorshDeserialize,
            [u64; 16]: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    owner: borsh::BorshDeserialize::deserialize(buf)?,
                    balances: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = size_of::<Row>() == 32 + 8 * 16;
            ASSERT
        } as usize] = [];
        #[allow(unknown_lints, eq_op)]
        const _: [(); 0 - !{
            const ASSERT: bool = size_of::<Row>() % 8 == 0;
            ASSERT
        } as usize] = [];
    }
}
use instructions::*;
use anchor_lang::prelude::*;
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        11u8, 72u8, 201u8, 171u8, 89u8, 13u8, 242u8, 112u8, 84u8, 187u8, 73u8, 230u8, 110u8, 25u8,
        157u8, 60u8, 67u8, 49u8, 60u8, 166u8, 237u8, 41u8, 25u8, 68u8, 129u8, 234u8, 205u8, 40u8,
        45u8, 160u8, 77u8, 244u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use self::mango_v3_reimbursement::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data).map_err(|e| {
        e.log();
        e.into()
    })
}
fn try_entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct MangoV3Reimbursement;
    #[automatically_derived]
    impl ::core::clone::Clone for MangoV3Reimbursement {
        #[inline]
        fn clone(&self) -> MangoV3Reimbursement {
            MangoV3Reimbursement
        }
    }
    impl anchor_lang::Id for MangoV3Reimbursement {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>:<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [79, 60, 158, 134, 61, 199, 56, 248] => {
            __private::__global::create_group(program_id, accounts, ix_data)
        }
        [229, 115, 171, 187, 69, 180, 216, 94] => {
            __private::__global::edit_group(program_id, accounts, ix_data)
        }
        [29, 237, 247, 208, 193, 82, 54, 135] => {
            __private::__global::create_vault(program_id, accounts, ix_data)
        }
        [111, 145, 221, 89, 16, 163, 76, 165] => {
            __private::__global::create_reimbursement_account(program_id, accounts, ix_data)
        }
        [186, 158, 55, 251, 88, 92, 120, 15] => {
            __private::__global::start_reimbursement(program_id, accounts, ix_data)
        }
        [160, 92, 125, 187, 32, 179, 114, 88] => {
            __private::__global::reimburse(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Close => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlCloseAccount::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_close_account(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut reallocs = std::collections::BTreeSet::new();
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                        &mut reallocs,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_close_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCloseAccount,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCloseAccount");
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn create_group(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CreateGroup");
            let ix = instruction::CreateGroup::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateGroup {
                group_num,
                table,
                claim_transfer_destination,
            } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = CreateGroup::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mango_v3_reimbursement::create_group(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                group_num,
                table,
                claim_transfer_destination,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn edit_group(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: EditGroup");
            let ix = instruction::EditGroup::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::EditGroup { table } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = EditGroup::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mango_v3_reimbursement::edit_group(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                table,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn create_vault(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CreateVault");
            let ix = instruction::CreateVault::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateVault { token_index } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = CreateVault::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mango_v3_reimbursement::create_vault(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                token_index,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn create_reimbursement_account(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: CreateReimbursementAccount");
            let ix = instruction::CreateReimbursementAccount::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::CreateReimbursementAccount = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = CreateReimbursementAccount::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mango_v3_reimbursement::create_reimbursement_account(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn start_reimbursement(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: StartReimbursement");
            let ix = instruction::StartReimbursement::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::StartReimbursement = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = StartReimbursement::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result =
                mango_v3_reimbursement::start_reimbursement(anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ))?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn reimburse(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Reimburse");
            let ix = instruction::Reimburse::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Reimburse {
                index_into_table,
                token_index,
                transfer_claim,
            } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut __reallocs = std::collections::BTreeSet::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = Reimburse::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
                &mut __reallocs,
            )?;
            let result = mango_v3_reimbursement::reimburse(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                index_into_table,
                token_index,
                transfer_claim,
            )?;
            accounts.exit(program_id)
        }
    }
}
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
    pub fn edit_group(ctx: Context<EditGroup>, table: Pubkey) -> Result<()> {
        handle_edit_group(ctx, table)
    }
    pub fn create_vault(ctx: Context<CreateVault>, token_index: usize) -> Result<()> {
        handle_create_vault(ctx, token_index)
    }
    pub fn create_reimbursement_account(ctx: Context<CreateReimbursementAccount>) -> Result<()> {
        handle_create_reimbursement_account(ctx)
    }
    pub fn start_reimbursement(ctx: Context<StartReimbursement>) -> Result<()> {
        handle_start_reimbursement(ctx)
    }
    pub fn reimburse<'key, 'accounts, 'remaining, 'info>(
        ctx: Context<'key, 'accounts, 'remaining, 'info, Reimburse<'info>>,
        index_into_table: usize,
        token_index: usize,
        transfer_claim: bool,
    ) -> Result<()> {
        handle_reimburse(ctx, index_into_table, token_index, transfer_claim)
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct CreateGroup {
        pub group_num: u32,
        pub table: Pubkey,
        pub claim_transfer_destination: Pubkey,
    }
    impl borsh::ser::BorshSerialize for CreateGroup
    where
        u32: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.group_num, writer)?;
            borsh::BorshSerialize::serialize(&self.table, writer)?;
            borsh::BorshSerialize::serialize(&self.claim_transfer_destination, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateGroup
    where
        u32: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                group_num: borsh::BorshDeserialize::deserialize(buf)?,
                table: borsh::BorshDeserialize::deserialize(buf)?,
                claim_transfer_destination: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::Discriminator for CreateGroup {
        fn discriminator() -> [u8; 8] {
            [79, 60, 158, 134, 61, 199, 56, 248]
        }
    }
    impl anchor_lang::InstructionData for CreateGroup {
        fn data(&self) -> Vec<u8> {
            let mut d = [79, 60, 158, 134, 61, 199, 56, 248].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct EditGroup {
        pub table: Pubkey,
    }
    impl borsh::ser::BorshSerialize for EditGroup
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.table, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for EditGroup
    where
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                table: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::Discriminator for EditGroup {
        fn discriminator() -> [u8; 8] {
            [229, 115, 171, 187, 69, 180, 216, 94]
        }
    }
    impl anchor_lang::InstructionData for EditGroup {
        fn data(&self) -> Vec<u8> {
            let mut d = [229, 115, 171, 187, 69, 180, 216, 94].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CreateVault {
        pub token_index: usize,
    }
    impl borsh::ser::BorshSerialize for CreateVault
    where
        usize: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.token_index, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateVault
    where
        usize: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                token_index: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::Discriminator for CreateVault {
        fn discriminator() -> [u8; 8] {
            [29, 237, 247, 208, 193, 82, 54, 135]
        }
    }
    impl anchor_lang::InstructionData for CreateVault {
        fn data(&self) -> Vec<u8> {
            let mut d = [29, 237, 247, 208, 193, 82, 54, 135].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct CreateReimbursementAccount;
    impl borsh::ser::BorshSerialize for CreateReimbursementAccount {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for CreateReimbursementAccount {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for CreateReimbursementAccount {
        fn discriminator() -> [u8; 8] {
            [111, 145, 221, 89, 16, 163, 76, 165]
        }
    }
    impl anchor_lang::InstructionData for CreateReimbursementAccount {
        fn data(&self) -> Vec<u8> {
            let mut d = [111, 145, 221, 89, 16, 163, 76, 165].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct StartReimbursement;
    impl borsh::ser::BorshSerialize for StartReimbursement {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for StartReimbursement {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {})
        }
    }
    impl anchor_lang::Discriminator for StartReimbursement {
        fn discriminator() -> [u8; 8] {
            [186, 158, 55, 251, 88, 92, 120, 15]
        }
    }
    impl anchor_lang::InstructionData for StartReimbursement {
        fn data(&self) -> Vec<u8> {
            let mut d = [186, 158, 55, 251, 88, 92, 120, 15].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Reimburse {
        pub index_into_table: usize,
        pub token_index: usize,
        pub transfer_claim: bool,
    }
    impl borsh::ser::BorshSerialize for Reimburse
    where
        usize: borsh::ser::BorshSerialize,
        usize: borsh::ser::BorshSerialize,
        bool: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.index_into_table, writer)?;
            borsh::BorshSerialize::serialize(&self.token_index, writer)?;
            borsh::BorshSerialize::serialize(&self.transfer_claim, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Reimburse
    where
        usize: borsh::BorshDeserialize,
        usize: borsh::BorshDeserialize,
        bool: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                index_into_table: borsh::BorshDeserialize::deserialize(buf)?,
                token_index: borsh::BorshDeserialize::deserialize(buf)?,
                transfer_claim: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::Discriminator for Reimburse {
        fn discriminator() -> [u8; 8] {
            [160, 92, 125, 187, 32, 179, 114, 88]
        }
    }
    impl anchor_lang::InstructionData for Reimburse {
        fn data(&self) -> Vec<u8> {
            let mut d = [160, 92, 125, 187, 32, 179, 114, 88].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_reimburse::*;
    pub use crate::__client_accounts_create_group::*;
    pub use crate::__client_accounts_create_vault::*;
    pub use crate::__client_accounts_edit_group::*;
    pub use crate::__client_accounts_create_reimbursement_account::*;
    pub use crate::__client_accounts_start_reimbursement::*;
}
#[repr(u32)]
pub enum Error {
    SomeError,
}
#[automatically_derived]
impl ::core::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "SomeError")
    }
}
#[automatically_derived]
impl ::core::clone::Clone for Error {
    #[inline]
    fn clone(&self) -> Error {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Error {}
impl Error {
    /// Gets the name of this [#enum_name].
    pub fn name(&self) -> String {
        match self {
            Error::SomeError => "SomeError".to_string(),
        }
    }
}
impl From<Error> for u32 {
    fn from(e: Error) -> u32 {
        e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
    }
}
impl From<Error> for anchor_lang::error::Error {
    fn from(error_code: Error) -> anchor_lang::error::Error {
        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
            error_name: error_code.name(),
            error_code_number: error_code.into(),
            error_msg: error_code.to_string(),
            error_origin: None,
            compared_values: None,
        })
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Error::SomeError => <Self as std::fmt::Debug>::fmt(self, fmt),
        }
    }
}
