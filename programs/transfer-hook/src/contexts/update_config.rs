use anchor_lang::prelude::*;

use crate::constants::HOOK_CONFIG_SEED;
use crate::errors::TransferHookError;
use crate::state::HookConfig;

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [HOOK_CONFIG_SEED],
        bump = hook_config.bump,
        has_one = authority @ TransferHookError::Unauthorized
    )]
    pub hook_config: Account<'info, HookConfig>,
    
    pub authority: Signer<'info>,
}

