# Project Architecture

A stablecoin program, built on the Solana blockchain, designed to maintain a 1:1 peg with the US Dollar. The project leverages smart contracts to manage minting, burning, and transferring of the stablecoin, ensuring transparency and security.

## User Stories

<!-- As a [user persona], I want [an action], so that [a benefit]. -->

1. As a Admin, I want to initialize the stablecoin program, so that I can allow users to mint and burn stablecoins.
2. As an Admin, I want to pause/unpause the stablecoin operations, so that I can prevent any malicious activities during emergencies.
3. As a SOL holder, I want to mint stablecoins by depositing SOL, so that I can utilize my assets without selling them.
4. As a SOL holder/stablecoin holder, I want to deposit SOL into the stablecoin vault, so that I can mint stablecoin later/ not get liquidated.
5. As a user, I want to mint stablecoins against my deposited SOL, so that I can access liquidity without selling my SOL.
6. As a stablecoin holder, I want to burn my stablecoins to redeem SOL, so that I can convert my stablecoins back to SOL when needed.
7. As a stablecoin holder, I want to burn my stablecoins, so that I can reduce my debt and avoid liquidation.
8. As a stablecoin holder, I want to redeem my deposited SOL without burning stablecoins, so that I can withdraw my collateral when I have excess collateral.
9. As a user, I want stablecoin peg 1:1 with the US Dollar, so that I can liquidate under-collateralized positions.

## Data Model

<!-- What are the all the data that a program needs to be stored? -->

- admin address
- Liquidation threshold
- liquidation bonus
- paused state
- mint bump
- bump
<!-- - price feed address, (I think it is not needed to store the price feed address) -->

- user address
- user minted stablecoins
- user deposited Sol
- bump

## Design the Accounts

<!-- What are the all the accounts that a program needs to be stored? -->

- User specific accounts

  - user account

  ```rust
  // Seeds: [b"user-account", user_pubkey]
  struct UserAccount {
    pub user: Pubkey,
    pub minted_stablecoins: u64,
    pub deposited_sol: u64,
    pub bump: u8,
  }
  ```

- Global state account
  - stablecoin state account
  ```rust
  // seeds: [b"stablecoin"]
    struct StablecoinState {
        pub admin: Pubkey,
        pub paused: bool,
        pub liquidation_threshold: u64,
        pub liquidation_bonus: u64,
        pub mint_bump: u8,
        pub bump: u8,
    }
  ```
  - stablecoin mint account
  ```rust
    // seeds: [b"stablecoin-mint"]
  ```
  - stablecoin vault account
  ```rust
    // seeds: [b"stablecoin-vault"]
  ```
- Account Relationships

  ![Data model](<eraser/Data model.svg>)

## Plan of the Instruction Functions

- **Map User Stories to Instructions**
  - `initialize(liquidation_threshold, liquidation_bonus)`: Initialize the stablecoin program, setting up the global state account, mint account, and vault account.
  - `toggle_pause`: Allow the admin to pause or unpause the stablecoin operations
  - `deposit_collateral_and_mint_stablecoin(sol_amount, amount)`: Allow users to buy (amount) stablecoins by sending SOL to the vault
  - `redeem_collateral_and_burn_stablecoin(sol_amount, amount)`: Allow users to redeem their deposited SOL by burning (amount) stablecoins.
  - `liquidate`: Allow users to liquidate under-collateralized positions, ensuring the stability of the stablecoin peg.
- **Required Accounts**

  - `initialize`

    - Admin: signer, mutable
    - Stablecoin State Account: PDA, initialize,
    - Stablecoin Mint Account: PDA, initialize,
    - Stablecoin Vault Account: PDA, initialize,
    - System Program

  - `toggle_pause`: [Global State Account]
    - admin: signer, mutable
    - stablecoin state account: PDA, mutable
  - `deposit_collateral_and_mint_stablecoin`: [User Account, Vault Account]
    - minter: signer, mutable
    - stablecoin state account: PDA, read-only
    - stablecoin mint account: PDA, mutable'
    - stablecoin vault account: PDA, mutable
    - user account: PDA, initialize if not exists, mutable, init_if_needed
    - user mint ata: mutable, init_if_needed
    - price update account: PDA, read-only
    - token program
    - associated token program
    - system program
  - `redeem_collateral_and_burn_stablecoin`: [User Account, Vault Account]
    - minter: signer, mutable
    - stablecoin state account: PDA, read-only
    - stablecoin mint account: PDA, mutable'
    - stablecoin vault account: PDA, mutable
    - user account: PDA, initialize if not exists, mutable, init_if_needed
    - user mint ata: mutable, init_if_needed
    - token program
    - associated token program
    - system program
  - `liquidate`: [User Account, Vault Account]
    - liquidator: signer, mutable
    - stablecoin state account: PDA, read-only
    - stablecoin mint account: PDA, mutable'
    - stablecoin vault account: PDA, mutable
    - user account: PDA, mutable
    - user mint ata: mutable,
    - liquidator mint ata: mutable, init_if_needed
    - price update account: PDA, read-only
    - token program
    - associated token program
    - system program
