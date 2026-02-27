use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault {
    pub depositor : Pubkey,
    pub amount : u64,
    pub deposit_time : i64,
    pub maturity_time : i64,
    pub claimed : bool,
    
}
