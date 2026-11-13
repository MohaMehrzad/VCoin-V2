use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::constants::*;
use crate::errors::SscreError;
use crate::state::{ClaimStatus, Epoch, PoolConfig};

/// Upgrade authority only (audit SSC-08 / X-1); the VCoin mint is pinned.
#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = PoolConfig::LEN, seeds = [POOL_CONFIG_SEED], bump)]
    pub config: Box<Account<'info, PoolConfig>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = authority,
        seeds = [RESERVE_VAULT_SEED],
        bump,
        token::mint = vcoin_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub reserve_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(token::mint = vcoin_mint, token::token_program = token_program)]
    pub fee_destination: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ SscreError::InvalidProgramData)]
    pub program: Program<'info, crate::program::SscreProtocol>,
    pub program_data: Account<'info, ProgramData>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PoolConfig>>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PoolConfig>>,
    pub new_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProposeFee<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = config.bump, has_one = vcoin_mint)]
    pub config: Box<Account<'info, PoolConfig>>,
    pub authority: Signer<'info>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(token::mint = vcoin_mint, token::token_program = token_program)]
    pub fee_destination: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
}

/// Permissionless once the timelock has passed.
#[derive(Accounts)]
pub struct ApplyFee<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PoolConfig>>,
}

/// Permissionless crank: funds the next epoch from the reserve on schedule.
#[derive(Accounts)]
pub struct StartEpoch<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = config.bump, has_one = reserve_vault, has_one = vcoin_mint)]
    pub config: Box<Account<'info, PoolConfig>>,
    #[account(mut)]
    pub reserve_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = payer,
        space = Epoch::LEN,
        seeds = [EPOCH_SEED, &config.epoch_count.to_le_bytes()],
        bump,
    )]
    pub epoch: Box<Account<'info, Epoch>>,
    #[account(
        init,
        payer = payer,
        seeds = [EPOCH_VAULT_SEED, &config.epoch_count.to_le_bytes()],
        bump,
        token::mint = vcoin_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub epoch_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteRoot<'info> {
    #[account(seeds = [POOL_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PoolConfig>>,
    #[account(mut, seeds = [EPOCH_SEED, &epoch.index.to_le_bytes()], bump = epoch.bump)]
    pub epoch: Box<Account<'info, Epoch>>,
    pub oracle: Signer<'info>,
}

#[derive(Accounts)]
pub struct VetoRoot<'info> {
    #[account(seeds = [POOL_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PoolConfig>>,
    #[account(mut, seeds = [EPOCH_SEED, &epoch.index.to_le_bytes()], bump = epoch.bump)]
    pub epoch: Box<Account<'info, Epoch>>,
    pub authority: Signer<'info>,
}

/// Permissionless once the dispute delay has passed.
#[derive(Accounts)]
pub struct ActivateRoot<'info> {
    #[account(seeds = [POOL_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PoolConfig>>,
    #[account(mut, seeds = [EPOCH_SEED, &epoch.index.to_le_bytes()], bump = epoch.bump)]
    pub epoch: Box<Account<'info, Epoch>>,
}

/// The claimant signs; `payer` (e.g. a relayer) funds the claim-status rent (SSC-07).
#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(seeds = [POOL_CONFIG_SEED], bump = config.bump, has_one = vcoin_mint, has_one = fee_destination)]
    pub config: Box<Account<'info, PoolConfig>>,
    #[account(mut, seeds = [EPOCH_SEED, &epoch.index.to_le_bytes()], bump = epoch.bump, has_one = vault)]
    pub epoch: Box<Account<'info, Epoch>>,
    #[account(mut)]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    pub claimant: Signer<'info>,
    #[account(mut, token::mint = vcoin_mint, token::authority = claimant, token::token_program = token_program)]
    pub claimant_token: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub fee_destination: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init,
        payer = payer,
        space = ClaimStatus::LEN,
        seeds = [CLAIM_SEED, epoch.key().as_ref(), claimant.key().as_ref()],
        bump,
    )]
    pub claim_status: Box<Account<'info, ClaimStatus>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/// Permissionless after the deadline: unclaimed funds return to the reserve.
#[derive(Accounts)]
pub struct Clawback<'info> {
    #[account(seeds = [POOL_CONFIG_SEED], bump = config.bump, has_one = reserve_vault, has_one = vcoin_mint)]
    pub config: Box<Account<'info, PoolConfig>>,
    #[account(
        mut,
        seeds = [EPOCH_SEED, &epoch.index.to_le_bytes()],
        bump = epoch.bump,
        has_one = vault,
        has_one = rent_payer,
    )]
    pub epoch: Box<Account<'info, Epoch>>,
    #[account(mut)]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub reserve_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    /// CHECK: receives the epoch vault's rent; bound by `has_one`.
    #[account(mut)]
    pub rent_payer: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct CloseClaimStatus<'info> {
    #[account(seeds = [EPOCH_SEED, &epoch.index.to_le_bytes()], bump = epoch.bump)]
    pub epoch: Box<Account<'info, Epoch>>,
    #[account(
        mut,
        seeds = [CLAIM_SEED, epoch.key().as_ref(), claim_status.claimant.as_ref()],
        bump = claim_status.bump,
        has_one = epoch,
        has_one = rent_payer,
        close = rent_payer,
    )]
    pub claim_status: Box<Account<'info, ClaimStatus>>,
    /// CHECK: receives the status rent; bound by `has_one`.
    #[account(mut)]
    pub rent_payer: UncheckedAccount<'info>,
}
