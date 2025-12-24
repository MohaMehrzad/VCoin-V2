use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::constants::HOOK_CONFIG_SEED;
use crate::state::HookConfig;

/// H-01 Security Fix: Context for initializing extra account metas
#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// The extra account meta list account
    /// CHECK: Validated by the transfer hook interface
    #[account(mut)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// Hook config account - needed to derive PDA for extra metas
    #[account(
        seeds = [HOOK_CONFIG_SEED],
        bump = hook_config.bump
    )]
    pub hook_config: Account<'info, HookConfig>,
    
    pub system_program: Program<'info, System>,
}

