use {
    crate::state::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount},
};


#[derive(Accounts)]
pub struct InitializeConfigPdaCtx<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        init,
        space = CONFIG_SIZE,
        payer = creator,
        seeds = [CONFIG_SEED.as_bytes()],
        bump
    )]
    config: Account<'info, Config>,

    system_program: Program<'info, System>,
}

pub fn initialize_config_handler(ctx: Context<InitializeConfigPdaCtx>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.last_offered_id = 0;
    config.bump = *ctx.bumps.get(CONFIG_SEED).unwrap();
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeBookPdaCtx<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    config: Account<'info, Config>,

    #[account(
        init,
        space = BOOK_SIZE,
        payer = creator,
        seeds = [BOOK_SEED.as_bytes(), &config.last_offered_id.to_le_bytes()],
        bump
    )]
    book: Account<'info, Book>,

    system_program: Program<'info, System>,
}

pub fn initialize_book_handler(ctx: Context<InitializeBookPdaCtx>) -> Result<()> {
    let config = &mut ctx.accounts.config;

    let book = &mut ctx.accounts.book;
    book.book_bump = *ctx.bumps.get(BOOK_SEED).unwrap();
    book.id = config.last_offered_id;

    config.last_offered_id += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeEscrowPdaCtx<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    offered_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [BOOK_SEED.as_bytes(), &book.id.to_le_bytes()],
        bump = book.book_bump,
    )]
    book: Account<'info, Book>,

    #[account(
        init,
        payer = creator,
        seeds = [ESCROW_SEED.as_bytes(), &book.id.to_le_bytes()],
        bump,
        token::mint = offered_mint,
        token::authority = book,
    )]
    escrow: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

pub fn initialize_escrow_handler(ctx: Context<InitializeEscrowPdaCtx>) -> Result<()> {
    let book = &mut ctx.accounts.book;
    book.escrow = ctx.accounts.escrow.key().clone();
    book.escrow_bump = *ctx.bumps.get(ESCROW_SEED).unwrap();
    Ok(())
}
