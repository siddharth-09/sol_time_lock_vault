use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};

use crate::Vault;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct InitializeVault<'info>{
    #[account(mut)]
    pub user : Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + Vault::INIT_SPACE,
        seeds = [b"vault",user.key().as_ref()],
        bump
    )]
    pub vault : Account<'info,Vault>,
    #[account(
        mut,
        seeds = [b"vault_wallet",user.key().as_ref()],
        bump
    )]
    pub vault_wallet : SystemAccount<'info>,
    pub system_program : Program<'info,System>
}

impl<'info>InitializeVault<'info>{
    pub fn deposit(&mut self,amt:u64,duration:i64, bumps:&InitializeVaultBumps)->Result<()>{
        require!(amt > 0, ErrorCode::InvalidAmount);
        require!(duration > 0, ErrorCode::InvalidDuration);
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        self.vault.depositor = self.user.key();
        self.vault.amount = amt;
        self.vault.deposit_time = current_timestamp;
        self.vault.maturity_time = current_timestamp + duration;
        self.vault.claimed = false;

        let ix = system_instruction::transfer(
            &self.user.key(),
            &self.vault_wallet.key(),
            amt
        );

        program::invoke(
            &ix, &[
            self.user.to_account_info(),
            self.vault_wallet.to_account_info()
        ])?;

        msg!("Current Timestamp: {}", current_timestamp);
        Ok(())
    }
}
