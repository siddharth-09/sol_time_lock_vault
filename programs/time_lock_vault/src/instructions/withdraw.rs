use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;
use anchor_lang::system_program::transfer;

use crate::Vault;
use crate::Treasury;

#[derive(Accounts)]
pub struct WithdrawVault<'info>{
    #[account(mut)]
    pub user : Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault",user.key().as_ref()],
        close =user,
        bump
    )]
    pub vault : Account<'info,Vault>,
    #[account(
        mut,
        seeds = [b"vault_wallet",user.key().as_ref()],
        bump
    )]
    pub vault_wallet : SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"treasury"],
        bump
    )]
    pub treasury : Account<'info,Treasury>,
        #[account(
        mut,
        seeds = [b"treasury_wallet",treasury.key().as_ref()],
        bump
    )]
    pub treasury_wallet : SystemAccount<'info>,
    pub system_program : Program<'info,System>
}

impl<'info>WithdrawVault<'info>{
    pub fn withdraw_and_close(&mut self,bumps: &WithdrawVaultBumps)->Result<()>{
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault_wallet",
            self.user.key.as_ref(),
            &[bumps.vault_wallet],
        ]];
        //when maturity is too much
        if self.vault.maturity_time > current_timestamp {
            //penalty of 10% to treasury account of stored sol in vault
            let penalty_amount = self.vault.amount / 10;
            //1) transfer penalty to treasury 
            let cpi_penalty_context = CpiContext::new_with_signer(self.system_program.to_account_info(),
            Transfer{
                from : self.vault_wallet.to_account_info(),
                to : self.treasury_wallet.to_account_info()
            },
            signer_seeds
            );
            transfer(cpi_penalty_context,penalty_amount)?;
            self.vault.amount = self.vault.amount - penalty_amount;
            self.treasury.total_penalties = self.treasury.total_penalties + penalty_amount;
            //2) now transfer the remaining amount to user
            let cpi_context = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer{
                    from : self.vault_wallet.to_account_info(),
                    to : self.user.to_account_info()
                },
                signer_seeds
            );
            transfer(cpi_context,self.vault.amount)?;
        }
        //when maturity is achieved or over
        if self.vault.maturity_time <= current_timestamp {
            //now transfer all amount from 
            let cpi_context = CpiContext::new_with_signer(self.system_program.to_account_info(),
            Transfer{
                from : self.vault_wallet.to_account_info(),
                to : self.user.to_account_info()
            },
            signer_seeds
            );
            transfer(cpi_context,self.vault.amount)?;
        }
        Ok(())
    }
}