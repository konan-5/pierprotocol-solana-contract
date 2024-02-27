use {
    crate::state::*,
    anchor_lang::prelude::*,
};


#[derive(Accounts)]
pub struct UpdateFeeCtx<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        has_one = creator,
        seeds=[FEE_SEED.as_bytes()],
        bump=fee.bump,
    )]
    pub fee: Account<'info, Fee>
}

pub fn update_fee_handler(ctx: Context<UpdateFeeCtx>, fee_wallet: Pubkey) -> Result<()> {
    let fee = &mut ctx.accounts.fee;
    fee.wallet = fee_wallet;
    Ok(())
}
