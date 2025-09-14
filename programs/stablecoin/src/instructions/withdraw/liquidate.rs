use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use pyth_solana_receiver_sdk::price_update::PriceUpdateV2;

use crate::{StablecoinState, UserAccount};

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

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
        mut,
        seeds = [b"user", user_account.user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = user_account.user.key(),
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,
    // - liquidator mint ata: mutable, init_if_needed
    #[account(
        init_if_needed,
        payer = liquidator,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = liquidator,

    )]
    pub liquidator_mint_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn process_liquidate(ctx: Context<Liquidate>, sol_amount: u64, amount: u64) -> Result<()> {
    Ok(())
}
