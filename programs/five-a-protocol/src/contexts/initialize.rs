use anchor_lang::prelude::*;

use crate::constants::FIVE_A_CONFIG_SEED;
use crate::state::FiveAConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = FiveAConfig::LEN,
        seeds = [FIVE_A_CONFIG_SEED],
        bump
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    /// CHECK: Vouch stake vault
    pub vouch_vault: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

