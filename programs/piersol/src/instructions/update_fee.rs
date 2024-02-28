use {
    crate::state::*,
    anchor_lang::prelude::*,
};


#[derive(Accounts)]
pub struct UpdateFeeCtx<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        seeds = [CONFIG_SEED.as_bytes()],
        bump = config.bump
    )]
    config: Account<'info, Config>,

    #[account(
        mut,
        seeds=[FEE_SEED.as_bytes()],
        bump=fee.bump,
        constraint = config.creator.key() == creator.key(),
    )]
    pub fee: Account<'info, Fee>
}

pub fn update_fee_handler(ctx: Context<UpdateFeeCtx>, fee_wallet: Pubkey) -> Result<()> {
    let fee = &mut ctx.accounts.fee;
    fee.wallet = fee_wallet;
    Ok(())
}
