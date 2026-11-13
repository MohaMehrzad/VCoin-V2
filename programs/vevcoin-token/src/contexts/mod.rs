use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::constants::*;
use crate::errors::VeVCoinError;
use crate::state::VeVCoinConfig;

/// Creates the config and the NonTransferable + PermanentDelegate veVCoin mint.
/// Upgrade authority only (audit VEV-04 / X-1).
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = VeVCoinConfig::LEN, seeds = [VEVCOIN_CONFIG_SEED], bump)]
    pub config: Box<Account<'info, VeVCoinConfig>>,

    /// CHECK: address is the mint PDA; the account is created and initialized by the
    /// handler (Anchor 0.32 has no NonTransferable init constraint).
    #[account(mut, seeds = [VEVCOIN_MINT_SEED], bump)]
    pub mint: UncheckedAccount<'info>,

    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ VeVCoinError::InvalidProgramData)]
    pub program: Program<'info, crate::program::VevcoinToken>,
    pub program_data: Account<'info, ProgramData>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/// Mints receipts; signed by the staking registrar PDA via CPI.
#[derive(Accounts)]
pub struct MintVeVCoin<'info> {
    #[account(
        seeds = [VEVCOIN_CONFIG_SEED],
        bump = config.bump,
        has_one = mint @ VeVCoinError::InvalidMint,
        has_one = staking_authority @ VeVCoinError::Unauthorized,
    )]
    pub config: Box<Account<'info, VeVCoinConfig>>,
    #[account(mut)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut, token::mint = mint, token::token_program = token_program)]
    pub destination: Box<InterfaceAccount<'info, TokenAccount>>,
    pub staking_authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}

/// Burns receipts with the config PDA as permanent delegate; signed by the registrar.
#[derive(Accounts)]
pub struct BurnVeVCoin<'info> {
    #[account(
        seeds = [VEVCOIN_CONFIG_SEED],
        bump = config.bump,
        has_one = mint @ VeVCoinError::InvalidMint,
        has_one = staking_authority @ VeVCoinError::Unauthorized,
    )]
    pub config: Box<Account<'info, VeVCoinConfig>>,
    #[account(mut)]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut, token::mint = mint, token::token_program = token_program)]
    pub source: Box<InterfaceAccount<'info, TokenAccount>>,
    pub staking_authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [VEVCOIN_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, VeVCoinConfig>>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [VEVCOIN_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, VeVCoinConfig>>,
    pub new_authority: Signer<'info>,
}
