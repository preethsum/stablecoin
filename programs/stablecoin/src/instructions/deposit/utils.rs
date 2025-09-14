use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::STABLECOIN_MINT_SEED;

pub fn transfer_sol_to_vault<'info>(
    from: &Signer<'info>,
    to: &SystemAccount<'info>,
    system_program: &Program<'info, System>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = anchor_lang::system_program::Transfer {
        from: from.to_account_info(),
        to: to.to_account_info(),
    };
    let cpi_program = system_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    Ok(())
}

pub fn mint_stablecoin_to_user<'info>(
    mint: &Account<'info, Mint>,
    to: &Account<'info, TokenAccount>,
    authority: &Account<'info, Mint>,
    token_program: &Program<'info, Token>,
    bump: u8,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = anchor_spl::token::MintTo {
        mint: mint.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let seeds: &[&[&[u8]]] = &[&[STABLECOIN_MINT_SEED.as_ref(), &[bump]]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, seeds);
    anchor_spl::token::mint_to(cpi_ctx, amount)?;
    Ok(())
}
