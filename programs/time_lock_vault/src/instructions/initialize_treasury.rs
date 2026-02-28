use anchor_lang::prelude::*;

use crate::{Treasury};

#[derive(Accounts)]
pub struct InitializeTreasury<'info>{
    #[account(mut)]
    pub user : Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + Treasury::INIT_SPACE,
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

impl<'info>InitializeTreasury<'info>{
    pub fn initialize(&mut self)->Result<()>{
        self.treasury.total_penalties = 0;
        self.treasury.authority = self.user.key();
        Ok(())
    }
}
