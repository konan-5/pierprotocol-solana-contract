use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount, Transfer, CloseAccount},
};

#[derive(Accounts)]
pub struct CancelBookCtx<'info> {
    #[account(mut)]
    creator: Signer<'info>,

    offered_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = creator_ata_offered.owner == creator.key(),
        constraint = creator_ata_offered.mint == offered_mint.key(),
    )]
    creator_ata_offered: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [BOOK_SEED.as_bytes(), &book.id.to_le_bytes()],
        bump = book.book_bump,
        constraint = book.creator == creator.key(),
        constraint = book.offered_mint == offered_mint.key(),
        constraint = book.escrow == escrow.key(),
        constraint = book.state == BookState::Created as u8 @ ErrorCode::InvalidBookState,
    )]
    book: Account<'info, Book>,

    #[account(
        mut,
        seeds = [ESCROW_SEED.as_bytes(), &book.id.to_le_bytes()],
        bump = book.escrow_bump,
        constraint = escrow.owner == book.key(),
        constraint = escrow.mint == offered_mint.key()
    )]
    escrow: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
}

pub fn cancel_book_handler(
    ctx: Context<CancelBookCtx>
) -> Result<()> {
    let book = &mut ctx.accounts.book;
    book.state = BookState::Cancelled as u8;

    let book_id_bytes = book.id.to_le_bytes();

    let bump_vector = &[book.book_bump][..];
    let inner = vec![
        BOOK_SEED.as_bytes(),
        &book_id_bytes,
        bump_vector.as_ref(),
    ];

    let outer = vec![inner.as_slice()];

    let transfer_instruction = Transfer{
        from: ctx.accounts.escrow.to_account_info(),
        to: ctx.accounts.creator_ata_offered.to_account_info(),
        authority: book.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        outer.as_slice(),
    );

    anchor_spl::token::transfer(cpi_ctx, book.offered_amount)?;

    let should_close = {
        ctx.accounts.escrow.reload()?;
        ctx.accounts.escrow.amount == 0
    };

    if should_close {
        let ca = CloseAccount {
            account: ctx.accounts.escrow.to_account_info(),
            destination: ctx.accounts.creator.to_account_info(),
            authority: book.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            ca,
            outer.as_slice(),
        );
        anchor_spl::token::close_account(cpi_ctx)?;
    }

    Ok(())
}
