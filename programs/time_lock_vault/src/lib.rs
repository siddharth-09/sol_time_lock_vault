pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BE2fwKWJx9QZeNEinHAGA8HdVkVmiVhLwED7qJVUNJJF");

#[program]
pub mod time_lock_vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>,amt:u64,duration:i64) -> Result<()> {
        ctx.accounts.deposit(amt,duration)?;
        Ok(())
    }
    pub fn initialize_treasury(ctx: Context<InitializeTreasury>)->Result<()>{
        //implement the create treasury account pda
        ctx.accounts.initialize()?;
        Ok(())
    }
    pub fn withdraw_and_close_vault(ctx: Context<WithdrawVault>)->Result<()>{
        //implement withdraw and close the vault
        ctx.accounts.withdraw_and_close(&ctx.bumps)?;
        Ok(())
    }

    pub fn withdraw_and_treasury(ctx: Context<WithdrawTreasury>)->Result<()>{
        //withdraw from treasury when needed
        ctx.accounts.withdraw_and_close(&ctx.bumps)?;
        Ok(())
    }
}
