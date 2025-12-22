use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};
use crate::constants::*;
use crate::state::GaslessConfig;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = GaslessConfig::LEN,
        seeds = [GASLESS_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, GaslessConfig>,
    
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// Fee vault for VCoin fees
    #[account(
        init,
        payer = authority,
        seeds = [FEE_VAULT_SEED],
        bump,
        token::mint = vcoin_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

