use std::mem::size_of;

use anchor_lang::{__private::bytemuck, prelude::*};
use static_assertions::const_assert_eq;

pub const ROW_HEADER_SIZE: usize = 40;

#[derive(Debug, Copy, Clone, AnchorSerialize, AnchorDeserialize)]
#[repr(C)]
pub struct Row {
    pub owner: Pubkey,
    pub balances: [u64; 16],
}
const_assert_eq!(size_of::<Row>(), 32 + 8 * 16);
const_assert_eq!(size_of::<Row>() % 8, 0);

unsafe impl bytemuck::Pod for Row {}
unsafe impl bytemuck::Zeroable for Row {}
