use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount, Transfer, CloseAccount},
};

#[derive(Accounts)]
pub struct CloseBookCtx<'info> {
    /// CHECK: creator
    #[account(mut)]
    creator: AccountInfo<'info>,

    #[account(mut)]
    taker: Signer<'info>,

    offered_mint: Account<'info, Mint>,
    desired_mint: Account<'info, Mint>,

    #[account(
        seeds = [FEE_SEED.as_bytes()],
        bump = fee.bump
    )]
    fee: Account<'info, Fee>,

    #[account(
        seeds = [FRIEND_SEED.as_bytes(), offered_mint.key().as_ref()],
        bump = offered_friend.bump
    )]
    offered_friend: Account<'info, Friend>,

    #[account(
        mut,
        seeds = [FRIEND_SEED.as_bytes(), desired_mint.key().as_ref()],
        bump = desired_friend.bump
    )]
    desired_friend: Account<'info, Friend>,

    #[account(
        constraint=fee_ata.owner == fee.wallet,
    )]
    fee_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=creator_ata_offered.owner == creator.key(),
        constraint=creator_ata_offered.mint == offered_mint.key()
    )]
    creator_ata_offered: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=creator_ata_desired.owner == creator.key(),
        constraint=creator_ata_desired.mint == desired_mint.key()
    )]
    creator_ata_desired: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=taker_ata_offered.owner == taker.key(),
        constraint=taker_ata_offered.mint == offered_mint.key()
    )]
    taker_ata_offered: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=taker_ata_desired.owner == taker.key(),
        constraint=taker_ata_desired.mint == desired_mint.key()
    )]
    taker_ata_desired: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [BOOK_SEED.as_bytes(), &book.id.to_le_bytes()],
        bump = book.book_bump,
        constraint = book.creator == creator.key(),
        constraint = book.offered_mint == offered_mint.key(),
        constraint = book.desired_mint == desired_mint.key(),
        constraint = book.escrow == escrow.key(),
        constraint = book.state == BookState::Created as u8 @ ErrorCode::InvalidBookState,
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

pub fn close_book_handler(
    ctx: Context<CloseBookCtx>
) -> Result<()> {
    let offered_friend = &ctx.accounts.offered_friend;
    let desired_friend = &ctx.accounts.desired_friend;

    let fee_ata = &ctx.accounts.fee_ata;

    let book = &mut ctx.accounts.book;
    book.state = BookState::Closed as u8;
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
        to: ctx.accounts.taker_ata_offered.to_account_info(),
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
        let ca = CloseAccount{
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

    let fee_amount: u64 = book.desired_amount * 1 / 100;
    let fee_amount: u64 = fee_amount * (100 - desired_friend.decrease_fee_rate) as u64 / 100;
    let fee_amount: u64 = fee_amount * (100 - offered_friend.decrease_fee_rate) as u64 / 100;

    let transfer_instruction = Transfer{
        from: ctx.accounts.taker_ata_desired.to_account_info(),
        to: fee_ata.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction
    );

    anchor_spl::token::transfer(cpi_ctx, fee_amount)?;

    let transfer_instruction = Transfer{
        from: ctx.accounts.taker_ata_desired.to_account_info(),
        to: ctx.accounts.creator_ata_desired.to_account_info(),
        authority: ctx.accounts.taker.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction
    );
    
    anchor_spl::token::transfer(cpi_ctx, book.desired_amount - fee_amount)?;

    Ok(())
}