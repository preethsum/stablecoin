use anchor_lang::prelude::*;

use crate::STABLECOIN_STATE_SEED;

#[derive(Accounts)]
pub struct TogglePause<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        mut,
        seeds = [STABLECOIN_STATE_SEED.as_bytes()],
        bump,
        has_one = admin,
    )]
    pub stablecoin_state: Account<'info, crate::StablecoinState>,
}

pub fn process_toggle_pause(ctx: Context<TogglePause>) -> Result<()> {
    let stablecoin_state = &mut ctx.accounts.stablecoin_state;
    stablecoin_state.paused = !stablecoin_state.paused;
    Ok(())
}
