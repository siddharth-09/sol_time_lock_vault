use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Amount to Deposit")]
    InvalidAmount,
    #[msg("Duration must be greater than zero.")]
    InvalidDuration,
}
