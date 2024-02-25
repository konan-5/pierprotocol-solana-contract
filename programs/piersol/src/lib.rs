use anchor_lang::prelude::*;

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
