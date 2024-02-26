use anchor_lang::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum SwapState {
    Created = 1,
    Closed = 2,
    Cancelled = 3,
}

pub const BOOK_SEED: &str = "book";
pub const BOOK_SIZE: usize = 8 + (4 * 32) + (2 * 8) + (3 * 1);

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
}

#[account]
pub struct Fee {
    pub wallet: Pubkey,
    pub owner: Pubkey,
}

#[account]
pub struct Config {
    pub last_offered_id: u64,
}
