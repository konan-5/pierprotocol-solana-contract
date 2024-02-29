use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct CollectFeeCtx<'info> {
    #[account(mut)]
    collector: Signer<'info>,

    offered_mint: Account<'info, Mint>,

    #[account(
        seeds = [FEE_SEED.as_bytes()],
        bump = fee.bump
    )]
    fee: Account<'info, Fee>,

    #[account(
        mut,
        constraint=fee_ata.owner == fee.wallet,
        constraint=fee_ata.mint == offered_mint.key(),
    )]
    fee_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [BOOK_SEED.as_bytes(), &book.id.to_le_bytes()],
        bump = book.book_bump,
        constraint = book.state == BookState::Closed as u8 @ ErrorCode::InvalidBookState,
    )]
    book: Account<'info, Book>,

    #[account(
        mut,
        seeds = [ESCROW_SEED.as_bytes(), &book.id.to_le_bytes()],
        bump = book.escrow_bump,
        constraint = escrow.owner == book.key(),
        constraint = escrow.mint == offered_mint.key(),
    )]
    escrow: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
}

pub fn collect_fee_handler(
    ctx: Context<CollectFeeCtx>
) -> Result<()> {
    let escrow_account = &ctx.accounts.escrow;
    let fee_ata_account = &ctx.accounts.fee_ata;

    let transfer_instruction = Transfer {
        from: escrow_account.to_account_info(),
        to: fee_ata_account.to_account_info(),
        authority: escrow_account.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        transfer_instruction
    );
    
    // Transfer all tokens from escrow to fee_ata
    anchor_spl::token::transfer(
        cpi_ctx,
        escrow_account.amount,
    )?;

    Ok(())
}
