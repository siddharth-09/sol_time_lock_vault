use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::Treasury;
#[derive(Accounts)]
pub struct WithdrawTreasury<'info>{
    #[account(mut)]
    pub user : Signer<'info>,
    #[account(
        mut,
        seeds = [b"treasury"],
        close = user,
        bump
    )]
    pub treasury : Account<'info,Treasury>,
    pub system_program : Program<'info,System>
}

impl <'info>WithdrawTreasury<'info>{
    pub fn withdraw_and_close(&mut self,bumps: &WithdrawTreasuryBumps)->Result<()>{
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            self.user.key.as_ref(),
            &[bumps.treasury],
        ]];
        let cpi = CpiContext::new_with_signer(
        self.system_program.to_account_info(),
        Transfer{
            from : self.treasury.to_account_info(),
            to : self.user.to_account_info()
        },
        signer_seeds
        );
        let lamports = self.treasury.get_lamports();
        transfer(cpi,lamports);
        Ok(())
    }
}