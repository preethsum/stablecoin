use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub user: Pubkey,
    pub minted_stablecoins: u64,
    pub deposited_sol: u64,
    pub is_initialized: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct StablecoinState {
    pub admin: Pubkey,
    pub paused: bool,
    pub liquidation_threshold: u64,
    pub liquidation_bonus: u64,
    pub vault_bump: u8,
    pub mint_bump: u8,
    pub bump: u8,
}
