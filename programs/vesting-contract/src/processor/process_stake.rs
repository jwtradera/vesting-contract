use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

use crate::{constants::*, errors::*, states::*};

pub fn process_stake(
    ctx: Context<Stake>,
    amount: u64,
    lock_months: u8,
    reward_months: u8,
) -> Result<()> {
    // Check user balance first
    require!(
        ctx.accounts.user_vault.amount >= amount,
        CustomError::BalanceShortage
    );

    // Transfer reward amount from user to vault's account
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_vault.to_account_info(),
        to: ctx.accounts.reward_vault.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    // Store stake information
    let stake_state = &mut ctx.accounts.stake_state;
    stake_state.authority = ctx.accounts.authority.key();
    stake_state.amount = amount;
    stake_state.lock_months = lock_months;
    stake_state.reward_months = reward_months;
    stake_state.locked_at = Clock::get().unwrap().unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Stake<'info> {
    #[account(
        seeds = [GLOBAL_STATE_TAG],
        bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        init,
        seeds = [LOCK_STATE_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<StakeState>() + 8
    )]
    pub stake_state: Box<Account<'info, StakeState>>,

    #[account(
        mut,
        seeds = [REWARD_VAULT_TAG],
        bump,
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
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
