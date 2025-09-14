pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6FE21fTwmD8DSMsG99jU7G5rAt56u4QjrDwHrKYsYZsi");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        liquidation_threshold: u64,
        liquidation_bonus: u64,
    ) -> Result<()> {
        process_initialize(ctx, liquidation_threshold, liquidation_bonus)
    }

    pub fn toggle_pause(ctx: Context<TogglePause>) -> Result<()> {
        process_toggle_pause(ctx)
    }
    pub fn deposit_collateral_and_mint_stablecoin(
        ctx: Context<DepositAndMintStablecoin>,
        sol_amount: u64,
        amount: u64,
    ) -> Result<()> {
        process_deposit_collateral_and_mint_stablecoin(ctx, sol_amount, amount)
    }
    pub fn redeem_collateral_and_burn_stablecoin(
        ctx: Context<RedeemCollateralAndBurnStablecoin>,
        sol_amount: u64,
        amount: u64,
    ) -> Result<()> {
        process_redeem_collateral_and_burn_stablecoin(ctx, sol_amount, amount)
    }
    pub fn liquidate(ctx: Context<Liquidate>, sol_amount: u64, amount: u64) -> Result<()> {
        process_liquidate(ctx, sol_amount, amount)
    }
}
