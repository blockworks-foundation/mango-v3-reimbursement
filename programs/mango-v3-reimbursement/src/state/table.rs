use std::mem::size_of;

use anchor_lang::{__private::bytemuck, prelude::*};
use static_assertions::const_assert_eq;

#[account(zero_copy)]
pub struct Table {
    pub rows: [Row; 1],
}
const_assert_eq!(size_of::<Table>(), (32 + 8 * 16) * 1);
const_assert_eq!(size_of::<Table>() % 8, 0);

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize)]
#[repr(C)]
pub struct Row {
    pub owner: Pubkey,
    pub balances: [u64; 16],
}
const_assert_eq!(size_of::<Row>(), 32 + 8 * 16);
const_assert_eq!(size_of::<Row>() % 8, 0);
