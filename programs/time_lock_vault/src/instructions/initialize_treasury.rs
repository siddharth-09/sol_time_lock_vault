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
        seeds = [b"treasury",user.key().as_ref()],
        bump
    )]
    pub treasury : Account<'info,Treasury>,
    pub system_program : Program<'info,System>
}

impl<'info>InitializeTreasury<'info>{
    pub fn initialize(&mut self)->Result<()>{
        self.treasury.total_penalties = 0;
        self.treasury.authority = self.user.key();
        Ok(())
    }
}
