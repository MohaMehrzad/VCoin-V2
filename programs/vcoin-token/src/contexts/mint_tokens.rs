use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::constants::*;
use crate::state::VCoinConfig;

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(seeds = [VCOIN_CONFIG_SEED], bump = config.bump, has_one = mint)]
    pub config: Box<Account<'info, VCoinConfig>>,

    #[account(mut)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut, token::mint = mint, token::token_program = token_program)]
    pub destination: Box<InterfaceAccount<'info, TokenAccount>>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}
