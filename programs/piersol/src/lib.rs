use {anchor_lang::prelude::*, instructions::*};

pub mod instructions;
pub mod state;
pub mod errors;

declare_id!("BFyTZKQiRPnk8AkBEfD1mhm5ZGuq46GWXg5wbFAYPzPx");

#[program]
pub mod piersol {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
