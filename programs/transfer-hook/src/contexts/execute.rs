use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

use crate::constants::{HOOK_CONFIG_SEED, USER_ACTIVITY_SEED, PAIR_TRACKING_SEED};
use crate::errors::TransferHookError;
use crate::state::{HookConfig, UserActivity, PairTracking};

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(
        seeds = [HOOK_CONFIG_SEED],
        bump = hook_config.bump
    )]
    pub hook_config: Account<'info, HookConfig>,
    
    /// Source token account - MUST use VCoin mint
    #[account(constraint = sender.mint == hook_config.vcoin_mint @ TransferHookError::InvalidMint)]
    pub sender: InterfaceAccount<'info, TokenAccount>,
    
    /// Destination token account - MUST use VCoin mint
    #[account(constraint = receiver.mint == hook_config.vcoin_mint @ TransferHookError::InvalidMint)]
    pub receiver: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = UserActivity::LEN,
        seeds = [USER_ACTIVITY_SEED, sender.owner.as_ref()],
        bump
    )]
    pub sender_activity: Account<'info, UserActivity>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = UserActivity::LEN,
        seeds = [USER_ACTIVITY_SEED, receiver.owner.as_ref()],
        bump
    )]
    pub receiver_activity: Account<'info, UserActivity>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = PairTracking::LEN,
        seeds = [PAIR_TRACKING_SEED, sender.owner.as_ref(), receiver.owner.as_ref()],
        bump
    )]
    pub pair_tracking: Account<'info, PairTracking>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

