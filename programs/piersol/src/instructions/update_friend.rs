use {
    crate::state::*,
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

pub fn update_friend_handler(ctx: Context<UpdateFriendCtx>, fee_rate: u8) -> Result<()> {
    let friend = &mut ctx.accounts.friend;
    friend.fee_rate = fee_rate;
    Ok(())
}
