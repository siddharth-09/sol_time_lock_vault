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
        #[account(
        mut,
        seeds = [b"treasury_wallet",treasury.key().as_ref()],
        bump
    )]
    pub treasury_wallet : SystemAccount<'info>,
    pub system_program : Program<'info,System>
}

impl <'info>WithdrawTreasury<'info>{
    pub fn withdraw_and_close(&mut self,bumps: &WithdrawTreasuryBumps)->Result<()>{
        let treasury_key = self.treasury.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"treasury_wallet",
            treasury_key.as_ref(),
            &[bumps.treasury_wallet],
        ]];
        let cpi = CpiContext::new_with_signer(
        self.system_program.to_account_info(),
        Transfer{
            from : self.treasury_wallet.to_account_info(),
            to : self.user.to_account_info()
        },
        signer_seeds
        );
        let lamports = self.treasury_wallet.get_lamports();
        transfer(cpi,lamports)?;
        Ok(())
    }
}