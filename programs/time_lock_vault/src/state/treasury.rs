use anchor_lang::prelude::*;
//only for if withdrawal early a penalty amount is stored here

#[account]
#[derive(InitSpace)]
pub struct Treasury{
    pub total_penalties : u64,
    pub authority : Pubkey,
}