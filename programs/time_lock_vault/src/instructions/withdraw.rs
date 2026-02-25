use anchor_lang::prelude::*;

use crate::Vault;
use crate::Treasury;

#[derive(Accounts)]
pub struct Withdraw{

}

impl<'info>Withdraw{
    pub fn withdraw_and_close()->Result<()>{
        Ok(())
    }
}