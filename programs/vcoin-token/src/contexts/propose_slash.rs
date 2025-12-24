use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

use crate::constants::{VCOIN_CONFIG_SEED, SLASH_REQUEST_SEED};
use crate::errors::VCoinError;
use crate::state::{VCoinConfig, SlashRequest};

/// Context for proposing a slash (H-01 Security Fix)
/// Only the permanent delegate can propose slashes
#[derive(Accounts)]
#[instruction(target: Pubkey, request_id: u64)]
pub struct ProposeSlash<'info> {
    #[account(
        seeds = [VCOIN_CONFIG_SEED],
        bump = config.bump,
        constraint = config.permanent_delegate == authority.key() @ VCoinError::Unauthorized
    )]
    pub config: Account<'info, VCoinConfig>,
    
    #[account(
        init,
        payer = authority,
        space = SlashRequest::LEN,
        seeds = [SLASH_REQUEST_SEED, target.as_ref(), &request_id.to_le_bytes()],
        bump
    )]
    pub slash_request: Account<'info, SlashRequest>,
    
    /// The target token account to slash (for validation)
    #[account(
        constraint = target_account.mint == config.mint @ VCoinError::InvalidMint
    )]
    pub target_account: InterfaceAccount<'info, TokenAccount>,
    
    /// The permanent delegate proposing the slash
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

