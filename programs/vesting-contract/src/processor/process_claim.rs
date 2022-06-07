use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::{constants::*, errors::*, states::*};

pub fn process_claim(ctx: Context<Claim>) -> Result<()> {
    let stake_state = &mut ctx.accounts.stake_state;

    // Check locked period
    let now = Clock::get().unwrap().unix_timestamp;
    let lock_months_timestamp: i64 = (MONTH_TIMESTAMP * (stake_state.lock_months as u64)) as i64;
    msg!(
        "now = {}, staked_at = {}, claimed_at = {}",
        now,
        stake_state.locked_at,
        stake_state.claimed_at
    );
    require!(
        now > stake_state.locked_at + lock_months_timestamp,
        CustomError::LockTimeInvalid
    );

    // Check if already claimed
    let reward_months_timestamp: i64 =
        (MONTH_TIMESTAMP * (stake_state.reward_months as u64)) as i64;

    if stake_state.claimed_at > 0 {
        require!(
            now > stake_state.claimed_at + reward_months_timestamp,
            CustomError::AlreadyClaimed
        );
    } else {
        require!(
            now > stake_state.locked_at + reward_months_timestamp,
            CustomError::RewardTimeInvalid
        );
    }

    // Check vault amount first
    require!(
        ctx.accounts.reward_vault.amount >= stake_state.amount,
        CustomError::VaultShortage
    );

    // Find bump for vault PDA
    let (_found_key, bump) = Pubkey::find_program_address(&[REWARD_VAULT_TAG], &crate::ID);

    // Transfer reward amount from vault to user's account
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.reward_vault.to_account_info(),
        to: ctx.accounts.user_vault.to_account_info(),
        authority: ctx.accounts.reward_vault.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &[&[REWARD_VAULT_TAG, &[bump]]]),
        stake_state.amount,
    )?;

    stake_state.claimed_at = now;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Claim<'info> {
    #[account(
        seeds = [GLOBAL_STATE_TAG],
        bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        mut,
        seeds = [LOCK_STATE_TAG, authority.key().as_ref()],
        bump,
    )]
    pub stake_state: Box<Account<'info, StakeState>>,

    #[account(
        mut,
        seeds = [REWARD_VAULT_TAG],
        bump,
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = authority.key() == user_vault.owner,
        constraint = user_vault.mint == global_state.mint
    )]
    pub user_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
