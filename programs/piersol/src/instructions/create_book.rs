use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct CreateBookCtx<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    #[account(
        mut,
        constraint=creator_ata_offered.mint == offered_mint.key(),
    )]
    creator_ata_offered: Account<'info, TokenAccount>,

    offered_mint: Account<'info, Mint>,

    desired_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [BOOK_SEED.as_bytes(), &config.last_offered_id.to_le_bytes()],
        bump = book.book_bump,
        constraint = book.escrow == escrow.key(),
        constraint = book.state != BookState::Created as u8 @ ErrorCode::InvalidBookState,
    )]
    book: Account<'info, Book>,

    #[account(
        mut,
        seeds = [ESCROW_SEED.as_bytes(), &config.last_offered_id.to_le_bytes()],
        bump = book.escrow_bump,
    )]
    escrow: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
}

pub fn create_book_handler(ctx: Context<CreateBookCtx>, offered_amount: u64, desired_amount: u64) -> Result<()> {
    let book = &mut ctx.accounts.book;
    book.creator = ctx.accounts.creator.key().clone();
    book.offered_mint = ctx.accounts.offered_mint.key();
    book.desired_mint = ctx.accounts.desired_mint.key();
    book.offered_amount = offered_amount;
    book.desired_amount = desired_amount;
    book.state = BookState::Created as u8;

    let transfer_instruction = Transfer{
        from: ctx.accounts.creator_ata_offered.to_account_info(),
        to: ctx.accounts.escrow.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );

    anchor_spl::token::transfer(cpi_ctx, offered_amount)?;

    Ok(())
}