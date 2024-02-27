use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Book is in invalid state")]
    InvalidBookState,
    
    #[msg("The fee rate must be between 0 and 100.")]
    FeeRateOutOfRange,
}