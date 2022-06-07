use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub admin: Pubkey,
    pub mint: Pubkey,
}

#[account]
#[derive(Default)]
pub struct StakeState {
    pub authority: Pubkey,
    pub amount: u64,
    pub lock_months: u8,
    pub reward_months: u8,
    pub locked_at: i64,
    pub claimed_at: i64
}
