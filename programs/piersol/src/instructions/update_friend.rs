use {
    crate::{errors::ErrorCode, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::Mint,
};


#[derive(Accounts)]
pub struct UpdateFriendCtx<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    friend_mint: Account<'info, Mint>,

    #[account(
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump,
    )]
    config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [FRIEND_SEED.as_bytes(), friend_mint.key().as_ref()],
        bump = friend.bump,
        constraint = config.creator == creator.key(),
    )]
    pub friend: Account<'info, Friend>
}

pub fn update_friend_handler(ctx: Context<UpdateFriendCtx>, decrease_fee_rate: u8) -> Result<()> {
    // Validate that decrease_fee_rate is within the range 0 to 100
    if decrease_fee_rate > 100 {
        return Err(ErrorCode::FeeRateOutOfRange.into());
    }    
    let friend = &mut ctx.accounts.friend;
    friend.decrease_fee_rate = decrease_fee_rate;
    Ok(())
}
