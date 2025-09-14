use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::{error::ErrorCode, StablecoinState, UserAccount};

pub fn get_sol_value<'info>(
    price_update: &Account<'info, PriceUpdateV2>,
    lamports: u64,
) -> Result<u64> {
    let maximum_age: u64 = 160;
    let feed_id: [u8; 32] =
        get_feed_id_from_hex("0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d")?;
    let price_data = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

    // 1 sol = 1000000000 lamports
    // 1 sol = (240 * 10^8) * 10^-8 usd
    //  => 1000000000 lamports = (240 * 10^8) * 10^-8 usd
    // 1 lamport = (240 * 10^8) * 10^-8 / 10 ^ 9
    require!(price_data.price > 0, ErrorCode::InvalidPriceFeed);
    let price = price_data.price * 10;
    msg!("Current SOL price: {}", price);
    let sol_value = ((lamports as u128).checked_mul(price as u128).unwrap())
        .checked_div(LAMPORTS_PER_SOL as u128)
        .unwrap();
    Ok(sol_value as u64) // 9 decimal places
}

pub fn check_health_factor<'info>(
    user_account: &Account<'info, UserAccount>,
    stablecoin_state: &Account<'info, StablecoinState>,
    price_update: &Account<'info, PriceUpdateV2>,
) -> Result<()> {
    let health_factor = get_health_factor(user_account, stablecoin_state, price_update);
    require!(health_factor >= 1, ErrorCode::InsufficientHealthFactor);
    Ok(())
}

pub fn get_health_factor<'info>(
    user_account: &Account<'info, UserAccount>,
    stablecoin_state: &Account<'info, StablecoinState>,
    price_update: &Account<'info, PriceUpdateV2>,
) -> u64 {
    let deposited_sol = user_account.deposited_sol;
    let liquidation_threshold = stablecoin_state.liquidation_threshold;

    let sol_value = get_sol_value(price_update, deposited_sol).unwrap();
    msg!("Sol value: {}", sol_value);

    let numerator = (sol_value * liquidation_threshold) / 100;
    let denominator = user_account.minted_stablecoins; // mint has 9 decimal places
    msg!("Numerator: {}", numerator);
    msg!("Denominator: {}", denominator);
    let health_factor = numerator.checked_div(denominator).unwrap();
    msg!("Health factor: {}", health_factor);
    health_factor
}
