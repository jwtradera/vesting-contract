use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod processor;
pub mod states;

use crate::processor::*;

declare_id!("Btf6rov5qyFDkrarrHb9r4Dpi1ruBQMdUJBREdZXYpXi");

#[program]
pub mod vesting_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        process_initialize(ctx)
    }
    pub fn stake(
        ctx: Context<Stake>,
        amount: u64,
        lock_months: u8,
        reward_months: u8,
    ) -> Result<()> {
        process_stake(ctx, amount, lock_months, reward_months)
    }
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        process_claim(ctx)
    }
}
