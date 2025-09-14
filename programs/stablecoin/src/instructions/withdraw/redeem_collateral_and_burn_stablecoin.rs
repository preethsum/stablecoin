pub use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{
    burn_stablecoin_from_user, check_health_factor, transfer_sol_to_user, StablecoinState,
    UserAccount, STABLECOIN_VAULT_SEED,
};

#[derive(Accounts)]
pub struct RedeemCollateralAndBurnStablecoin<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"stablecoin"],
        bump = stablecoin_state.bump,
    )]
    pub stablecoin_state: Account<'info, StablecoinState>,

    #[account(
        mut,
        seeds = [b"stablecoin_mint"],
        bump = stablecoin_state.mint_bump,
    )]
    pub stablecoin_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"stablecoin_vault"],
        bump,
    )]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub stablecoin_vault: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserAccount::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = user,
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_redeem_collateral_and_burn_stablecoin(
    ctx: Context<RedeemCollateralAndBurnStablecoin>,
    sol_amount: u64,
    amount: u64,
) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    user_account.deposited_sol -= sol_amount;
    user_account.minted_stablecoins -= amount;

    check_health_factor(
        user_account,
        &ctx.accounts.stablecoin_state,
        &ctx.accounts.price_update,
    )?;

    transfer_sol_to_user(
        &ctx.accounts.stablecoin_vault,
        &ctx.accounts.user,
        &ctx.accounts.system_program,
        ctx.accounts.stablecoin_state.vault_bump,
        sol_amount,
    )?;

    burn_stablecoin_from_user(
        &ctx.accounts.stablecoin_mint,
        &ctx.accounts.user_mint_ata,
        &ctx.accounts.user,
        &ctx.accounts.token_program,
        ctx.accounts.stablecoin_state.mint_bump,
        amount,
    )?;

    Ok(())
}
