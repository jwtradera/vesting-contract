use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Error: You need to wait at least lockup period.")]
    LockTimeInvalid,

    #[msg("Error: You need to wait more for get reward.")]
    RewardTimeInvalid,

    #[msg("Error: Already claimed reward for release schdeule.")]
    AlreadyClaimed,

    #[msg("Error: Sorry but reward vault not enough.")]
    VaultShortage,

    #[msg("Error: Your balance is not enough.")]
    BalanceShortage,
}