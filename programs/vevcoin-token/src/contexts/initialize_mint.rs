use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;

use crate::constants::VEVCOIN_CONFIG_SEED;
use crate::state::VeVCoinConfig;

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = VeVCoinConfig::LEN,
        seeds = [VEVCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, VeVCoinConfig>,
    
    /// The veVCoin mint (Token-2022 with Non-Transferable extension)
    /// CHECK: Validated by Token-2022 program
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

