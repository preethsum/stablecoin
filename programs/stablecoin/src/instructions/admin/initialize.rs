use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{
    error::ErrorCode, StablecoinState, STABLECOIN_MINT_SEED, STABLECOIN_STATE_SEED,
    STABLECOIN_VAULT_SEED,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
            init,
            payer = admin,
            space = 8 + StablecoinState::INIT_SPACE,
            seeds = [STABLECOIN_STATE_SEED.as_bytes()],
            bump,
        )]
    pub stablecoin_state: Account<'info, StablecoinState>,
    #[account(
            init,
            payer = admin,
            mint::decimals = 9,
            mint::authority = stablecoin_mint,
            mint::token_program = token_program,
            seeds = [STABLECOIN_MINT_SEED.as_bytes()],
            bump,
        )]
    pub stablecoin_mint: Account<'info, Mint>,
    #[account(
            seeds = [STABLECOIN_VAULT_SEED.as_bytes()],
            bump,
        )]
    pub stablecoin_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn process_initialize(
    ctx: Context<Initialize>,
    liquidation_threshold: u64,
    liquidation_bonus: u64,
) -> Result<()> {
    msg!(
        "Initializing stablecoin account at: {:?}",
        ctx.accounts.stablecoin_state.key()
    );
    // liquidation_threshold should be between 0 and 100
    // liquidation_bonus should be between 0 and 100
    require!(
        liquidation_threshold > 0 && liquidation_threshold <= 100,
        ErrorCode::InvalidLiquidationThreshold
    );
    require!(
        liquidation_bonus > 0 && liquidation_bonus <= 100,
        ErrorCode::InvalidLiquidationBonus
    );
    ctx.accounts.stablecoin_state.set_inner(StablecoinState {
        admin: ctx.accounts.admin.key(),
        paused: false,
        liquidation_threshold,
        liquidation_bonus,
        vault_bump: ctx.bumps.stablecoin_vault,
        mint_bump: ctx.bumps.stablecoin_mint,
        bump: ctx.bumps.stablecoin_state,
    });
    Ok(())
}
