use {
    crate::state::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount},
};


#[derive(Accounts)]
pub struct InitializeBookPdaCtx<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        init,
        space = BOOK_SIZE,
        payer = creator,
        seeds = [BOOK_SEED.as_bytes(), creator.key().as_ref()],
        bump
    )]
    book: Account<'info, Book>,

    system_program: Program<'info, System>,
}

pub fn initialize_book_handler(ctx: Context<InitializeBookPdaCtx>) -> Result<()> {
    let book = &mut ctx.accounts.book;
    book.book_bump = *ctx.bumps.get(BOOK_SEED).unwrap();
    Ok(())
}
