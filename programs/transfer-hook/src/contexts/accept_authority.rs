use anchor_lang::prelude::*;

use crate::constants::HOOK_CONFIG_SEED;
use crate::errors::TransferHookError;
use crate::state::HookConfig;

/// Context for accepting a pending authority transfer (H-02 security fix)
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [HOOK_CONFIG_SEED],
        bump = hook_config.bump,
        constraint = hook_config.pending_authority == new_authority.key() @ TransferHookError::NotPendingAuthority,
        constraint = hook_config.pending_authority != Pubkey::default() @ TransferHookError::NoPendingTransfer
    )]
    pub hook_config: Account<'info, HookConfig>,
    
    /// The new authority accepting the transfer - must sign
    pub new_authority: Signer<'info>,
}

