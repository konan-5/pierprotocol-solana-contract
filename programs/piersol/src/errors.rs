use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Swap is in invalid state")]
    InvalidBookState,
}