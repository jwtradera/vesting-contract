use anchor_lang::prelude::*;

#[constant]
// pub const MONTH_TIMESTAMP: u64 = 60 * 60 * 24 * 30; // Assume 30 days per month
pub const MONTH_TIMESTAMP: u64 = 5; // Set test value as 5

#[constant]
pub const GLOBAL_STATE_SEED: &str = "VESTING_GLOBAL";
pub const GLOBAL_STATE_TAG: &[u8] = GLOBAL_STATE_SEED.as_bytes();

#[constant]
pub const REWARD_VAULT_SEED: &str = "REWARD_VAULT";
pub const REWARD_VAULT_TAG: &[u8] = REWARD_VAULT_SEED.as_bytes();

#[constant]
pub const LOCK_STATE_SEED: &str = "VESTING_LOCK";
pub const LOCK_STATE_TAG: &[u8] = LOCK_STATE_SEED.as_bytes();
