use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Book is in invalid state")]
    InvalidBookState,
}