use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::{constants::*, states::*};

pub fn process_initialize(ctx: Context<Initialize>) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    global_state.admin = ctx.accounts.authority.key();
    global_state.mint = ctx.accounts.mint.key();
    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [GLOBAL_STATE_TAG],
        bump,
        payer = authority,
        space = std::mem::size_of::<GlobalState>() + 8
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        token::mint = mint,
        token::authority = reward_vault,
        seeds = [REWARD_VAULT_TAG],
        bump,
        payer = authority,
    )]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
