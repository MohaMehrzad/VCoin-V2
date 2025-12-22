use anchor_lang::prelude::*;
use anchor_spl::token_2022::Token2022;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::constants::{VCOIN_CONFIG_SEED, SLASH_REQUEST_SEED};
use crate::errors::VCoinError;
use crate::state::{VCoinConfig, SlashRequest};

/// Context for executing an approved slash (H-01 Security Fix)
/// Requires timelock (48h) to have expired after governance approval
#[derive(Accounts)]
pub struct ExecuteSlash<'info> {
    #[account(
        seeds = [VCOIN_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, VCoinConfig>,
    
    #[account(
        mut,
        seeds = [SLASH_REQUEST_SEED, slash_request.target.as_ref(), &slash_request.created_at.to_le_bytes()],
        bump = slash_request.bump,
        constraint = slash_request.is_approved() @ VCoinError::SlashRequestNotApproved,
        constraint = !slash_request.is_executed() @ VCoinError::SlashRequestAlreadyExecuted
    )]
    pub slash_request: Account<'info, SlashRequest>,
    
    /// The VCoin mint
    #[account(
        mut,
        constraint = mint.key() == config.mint @ VCoinError::InvalidMint
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// Target account to slash tokens from
    #[account(
        mut,
        constraint = target_account.key() == slash_request.target @ VCoinError::InvalidMint,
        constraint = target_account.mint == config.mint @ VCoinError::InvalidMint
    )]
    pub target_account: InterfaceAccount<'info, TokenAccount>,
    
    /// The permanent delegate executing the slash
    #[account(
        constraint = executor.key() == config.permanent_delegate @ VCoinError::Unauthorized
    )]
    pub executor: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
}

