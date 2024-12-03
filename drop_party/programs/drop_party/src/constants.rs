use anchor_lang::prelude::*;

pub const PUBKEY_L: usize = 32;
pub const U64_L: usize = 8;
pub const U16_L: usize = 2;
pub const BOOL_L: usize = 1;
pub const OPTION_L: usize = 1;
pub const U8_L: usize = 1;
pub const MAX_NAME_L: usize = 4 + 32; // 4 bytes for length, 32 bytes for string
pub const MINT_ID: Pubkey = Pubkey::new_from_array([
    0x2F, 0x67, 0x7A, 0x6D, 0x6C, 0x72, 0x50, 0x77, 0x44, 0x62, 0x61, 0x48, 0x41, 0x62, 0x44, 0x4A, 
    0x68, 0x71, 0x34, 0x79, 0x68, 0x48, 0x6B, 0x54, 0x54, 0x55, 0x79, 0x58, 0x63, 0x39, 0x55, 0x41,
]); // Pubkey: 2Gz6trPwDbaHAbDJhq4yhHkTTUyXc9UAkfpEjFuRK5Si (test token mint)
pub const MINT_DECIMALS: u8 = 9;