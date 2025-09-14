use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{check_health_factor, mint_stablecoin_to_user, transfer_sol_to_vault, StablecoinState, UserAccount};

#[derive(Accounts)]
pub struct DepositAndMintStablecoin<'info> {
    
    #[account(mut)]
    pub minter: Signer<'info>,

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
        payer = minter,
        space = 8 + UserAccount::INIT_SPACE,
        seeds = [b"user", minter.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init_if_needed,
        payer = minter,
        associated_token::mint = stablecoin_mint,
        associated_token::authority = minter,
    
    )]
    pub user_mint_ata: Account<'info, TokenAccount>,

    pub price_update: Account<'info, PriceUpdateV2>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}


pub fn process_deposit_collateral_and_mint_stablecoin(ctx: Context<DepositAndMintStablecoin>,sol_amount: u64, amount: u64) -> Result<()> {
    let user_account = &mut ctx.accounts.user_account;

    user_account.deposited_sol += sol_amount;
    user_account.minted_stablecoins += amount;


    if !user_account.is_initialized {   
       user_account.user = ctx.accounts.minter.key();
       user_account.is_initialized = true;
       user_account.bump = ctx.bumps.user_account
    }

    // check health factor
    check_health_factor(user_account, &ctx.accounts.stablecoin_state, &ctx.accounts.price_update)?;

    // transfer sol to vault
    transfer_sol_to_vault(&ctx.accounts.minter, &ctx.accounts.stablecoin_vault, &ctx.accounts.system_program, sol_amount)?;

    // mint stablecoin to user
mint_stablecoin_to_user(&ctx.accounts.stablecoin_mint, &ctx.accounts.user_mint_ata, &ctx.accounts.stablecoin_mint, &ctx.accounts.token_program,ctx.accounts.stablecoin_state.mint_bump ,amount)?;

    Ok(())
}