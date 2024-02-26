use {anchor_lang::prelude::*, instructions::*};

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("BFyTZKQiRPnk8AkBEfD1mhm5ZGuq46GWXg5wbFAYPzPx");

#[program]
pub mod piersol {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfigPdaCtx>) -> Result<()> {
        initialize_pdas::initialize_config_handler(ctx)
    }

    pub fn initialize_book(ctx: Context<InitializeBookPdaCtx>) -> Result<()> {
        initialize_pdas::initialize_book_handler(ctx)
    }

    pub fn initialize_escrow(ctx: Context<InitializeEscrowPdaCtx>) -> Result<()> {
        initialize_pdas:: initialize_escrow_handler(ctx)
    }
}
