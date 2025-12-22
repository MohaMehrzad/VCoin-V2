use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;

use crate::constants::VCOIN_CONFIG_SEED;
use crate::state::VCoinConfig;

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = VCoinConfig::LEN,
        seeds = [VCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, VCoinConfig>,
    
    /// The VCoin mint (Token-2022)
    /// CHECK: Validated by Token-2022 program
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    
    /// Treasury token account
    /// CHECK: Will be validated during token operations
    pub treasury: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

