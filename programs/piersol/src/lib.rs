use {anchor_lang::prelude::*, instructions::*};

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("CFYyNLTkbq8v9Rt76gSBSxL6oKwmuAan6B381PdQnoLU");

#[program]
pub mod piersol {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfigPdaCtx>) -> Result<()> {
        initialize_pdas::initialize_config_handler(ctx)
    }

    pub fn initialize_fee(ctx: Context<InitializeFeePdaCtx>, fee_wallet: Pubkey) -> Result<()> {
        initialize_pdas:: initialize_fee_handler(ctx, fee_wallet)
    }

    pub fn update_fee(ctx: Context<UpdateFeeCtx>, fee_wallet: Pubkey) -> Result<()> {
        update_fee:: update_fee_handler(ctx, fee_wallet)
    }

    pub fn initialize_book(ctx: Context<InitializeBookPdaCtx>) -> Result<()> {
        initialize_pdas::initialize_book_handler(ctx)
    }

    pub fn initialize_escrow(ctx: Context<InitializeEscrowPdaCtx>) -> Result<()> {
        initialize_pdas:: initialize_escrow_handler(ctx)
    }

    pub fn create_book(ctx: Context<CreateBookCtx>, offered_amount: u64, desired_amount: u64) -> Result<()> {
        create_book::create_book_handler(ctx, offered_amount, desired_amount)
    }

    pub fn close_book(ctx: Context<CloseBookCtx>) -> Result<()> {
        close_book::close_book_handler(ctx)
    }

    pub fn cancel_book(ctx: Context<CancelBookCtx>) -> Result<()> {
        cancel_book::cancel_book_handler(ctx)
    }
}
