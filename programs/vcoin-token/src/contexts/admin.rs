use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};

use crate::constants::*;
use crate::state::VCoinConfig;

/// Authority-only config changes (pause, propose/cancel authority transfer).
#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [VCOIN_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, VCoinConfig>>,
    pub authority: Signer<'info>,
}

/// Completes a pending authority transfer; signed by the new authority.
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [VCOIN_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, VCoinConfig>>,
    pub new_authority: Signer<'info>,
}

/// Updates a token-metadata field; the authority pays any extra rent.
#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
    #[account(seeds = [VCOIN_CONFIG_SEED], bump = config.bump, has_one = mint)]
    pub config: Box<Account<'info, VCoinConfig>>,
    #[account(mut)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
