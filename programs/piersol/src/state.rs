use anchor_lang::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum BookState {
    Created = 1,
    Closed = 2,
    Cancelled = 3,
}

pub const BOOK_SEED: &str = "book";
pub const ESCROW_SEED: &str = "escrow";
pub const CONFIG_SEED: &str = "config";
pub const FEE_SEED: &str = "fee";
pub const FRIEND_SEED: &str = "friend";
pub const FEE_SIZE: usize = 8 + 2 * 32 + 1;
pub const BOOK_SIZE: usize = 8 + (4 * 32) + (2 * 8) + (3 * 1) + 8;
pub const CONFIG_SIZE: usize = 8 + 32 + 8 + 1;
pub const FRIEND_SIZE: usize = 8 + 2 * 1;

#[account]
pub struct Book {
    pub creator: Pubkey,
    pub offered_mint: Pubkey,
    pub desired_mint: Pubkey,
    pub escrow: Pubkey,
    pub offered_amount: u64,
    pub desired_amount: u64,
    pub state: u8,
    pub book_bump: u8,
    pub escrow_bump: u8,
    pub id: u64,
}

#[account]
pub struct Fee {
    pub wallet: Pubkey,
    pub bump: u8,
}

#[account]
pub struct Config {
    pub creator: Pubkey,
    pub last_offered_id: u64,
    pub bump: u8,
}

#[account]
pub struct Friend {
    pub decrease_fee_rate: u8,
    pub bump: u8,
}
