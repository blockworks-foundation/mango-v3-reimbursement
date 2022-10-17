use std::mem::size_of;

use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;

#[account(zero_copy)]
pub struct ReimbursementAccount {
    pub done: u16,
    pub claim_transferred: u16,
    pub padding: [u8; 4],
}
const_assert_eq!(size_of::<ReimbursementAccount>(), 8);
const_assert_eq!(size_of::<ReimbursementAccount>() % 8, 0);
