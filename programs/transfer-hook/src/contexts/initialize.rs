use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::constants::HOOK_CONFIG_SEED;
use crate::state::HookConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = HookConfig::LEN,
        seeds = [HOOK_CONFIG_SEED],
        bump
    )]
    pub hook_config: Account<'info, HookConfig>,
    
    /// VCoin mint (Token-2022)
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

