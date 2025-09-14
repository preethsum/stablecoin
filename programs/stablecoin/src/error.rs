use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid liquidation threshold, should be between 0 and 100")]
    InvalidLiquidationThreshold,
    #[msg("Invalid liquidation bonus, should be between 0 and 100")]
    InvalidLiquidationBonus,
    #[msg("Price should be greater than 0")]
    InvalidPriceFeed,
    #[msg("Insufficient health factor, please deposit more collateral or burn stablecoins")]
    InsufficientHealthFactor,
}
