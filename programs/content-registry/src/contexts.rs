use anchor_lang::prelude::*;
use gasless_protocol::{constants::SESSION_SEED, state::SessionToken};
use staking_protocol::constants::VOTER_SEED;

use crate::constants::*;
use crate::errors::ContentError;
use crate::state::*;

/// Upgrade authority only (audit CR-04).
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = RegistryConfig::LEN, seeds = [REGISTRY_CONFIG_SEED], bump)]
    pub config: Box<Account<'info, RegistryConfig>>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ ContentError::InvalidProgramData)]
    pub program: Program<'info, crate::program::ContentRegistry>,
    pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [REGISTRY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, RegistryConfig>>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [REGISTRY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, RegistryConfig>>,
    pub new_authority: Signer<'info>,
}

/// Permissionless once the timelock passed.
#[derive(Accounts)]
pub struct ApplyOracle<'info> {
    #[account(mut, seeds = [REGISTRY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, RegistryConfig>>,
}

/// Anyone may open a user's energy account; its tier comes from the user's staking deposits,
/// which must be passed.
#[derive(Accounts)]
pub struct InitUserEnergy<'info> {
    #[account(seeds = [REGISTRY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, RegistryConfig>>,
    /// CHECK: any wallet.
    pub user: UncheckedAccount<'info>,
    #[account(init, payer = payer, space = UserEnergy::LEN, seeds = [USER_ENERGY_SEED, user.key().as_ref()], bump)]
    pub user_energy: Box<Account<'info, UserEnergy>>,
    /// CHECK: the user's staking `Voter` PDA (address enforced here). Always required, so
    /// the tier can be neither understated by a third party nor left stale by the user; a
    /// wallet that never staked simply has no account there (tier 0).
    #[account(seeds = [VOTER_SEED, user.key().as_ref()], bump, seeds::program = staking_protocol::ID)]
    pub stake: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// `signer` is the author, or a gasless session key for the author (scope CONTENT).
#[derive(Accounts)]
#[instruction(tracking_id: [u8; 32])]
pub struct CreateContent<'info> {
    #[account(seeds = [REGISTRY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, RegistryConfig>>,
    /// CHECK: the author wallet; authorized through `signer` (and `session`).
    pub author: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    #[account(
        seeds = [SESSION_SEED, crate::ID.as_ref(), signer.key().as_ref(), author.key().as_ref()],
        bump = session.bump,
        seeds::program = gasless_protocol::ID,
    )]
    pub session: Option<Box<Account<'info, SessionToken>>>,
    #[account(mut, seeds = [USER_ENERGY_SEED, author.key().as_ref()], bump = user_energy.bump)]
    pub user_energy: Box<Account<'info, UserEnergy>>,
    /// CHECK: the author's staking `Voter` PDA (address enforced here). Always required, so
    /// the tier can be neither understated by a third party nor left stale by the author; a
    /// wallet that never staked simply has no account there (tier 0).
    #[account(seeds = [VOTER_SEED, author.key().as_ref()], bump, seeds::program = staking_protocol::ID)]
    pub stake: UncheckedAccount<'info>,
    #[account(
        init,
        payer = payer,
        space = ContentRecord::LEN,
        seeds = [CONTENT_RECORD_SEED, author.key().as_ref(), tracking_id.as_ref()],
        bump,
    )]
    pub content: Box<Account<'info, ContentRecord>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditContent<'info> {
    #[account(seeds = [REGISTRY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, RegistryConfig>>,
    /// CHECK: the author wallet; bound by `has_one` and authorized through `signer`.
    pub author: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    #[account(
        seeds = [SESSION_SEED, crate::ID.as_ref(), signer.key().as_ref(), author.key().as_ref()],
        bump = session.bump,
        seeds::program = gasless_protocol::ID,
    )]
    pub session: Option<Box<Account<'info, SessionToken>>>,
    #[account(mut, seeds = [USER_ENERGY_SEED, author.key().as_ref()], bump = user_energy.bump)]
    pub user_energy: Box<Account<'info, UserEnergy>>,
    /// CHECK: the author's staking `Voter` PDA (address enforced here). Always required, so
    /// the tier can be neither understated by a third party nor left stale by the author; a
    /// wallet that never staked simply has no account there (tier 0).
    #[account(seeds = [VOTER_SEED, author.key().as_ref()], bump, seeds::program = staking_protocol::ID)]
    pub stake: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, author.key().as_ref(), content.tracking_id.as_ref()],
        bump = content.bump,
        has_one = author,
    )]
    pub content: Box<Account<'info, ContentRecord>>,
}

/// Main wallet only; closes the record. The rent returns to whoever paid it (the author, or
/// the relayer that sponsored the post), so deleting sponsored posts earns nothing.
#[derive(Accounts)]
pub struct DeleteContent<'info> {
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, author.key().as_ref(), content.tracking_id.as_ref()],
        bump = content.bump,
        has_one = author,
        has_one = rent_payer,
        close = rent_payer,
    )]
    pub content: Box<Account<'info, ContentRecord>>,
    pub author: Signer<'info>,
    /// CHECK: bound by `has_one`; receives the record's rent.
    #[account(mut)]
    pub rent_payer: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateEngagement<'info> {
    #[account(seeds = [REGISTRY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, RegistryConfig>>,
    pub oracle: Signer<'info>,
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, content.author.as_ref(), content.tracking_id.as_ref()],
        bump = content.bump,
    )]
    pub content: Box<Account<'info, ContentRecord>>,
}

#[derive(Accounts)]
pub struct ClaimRefund<'info> {
    #[account(seeds = [REGISTRY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, RegistryConfig>>,
    pub author: Signer<'info>,
    #[account(mut, seeds = [USER_ENERGY_SEED, author.key().as_ref()], bump = user_energy.bump)]
    pub user_energy: Box<Account<'info, UserEnergy>>,
    /// CHECK: the author's staking `Voter` PDA (address enforced here). Always required, so
    /// the tier can be neither understated by a third party nor left stale by the author; a
    /// wallet that never staked simply has no account there (tier 0).
    #[account(seeds = [VOTER_SEED, author.key().as_ref()], bump, seeds::program = staking_protocol::ID)]
    pub stake: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, author.key().as_ref(), content.tracking_id.as_ref()],
        bump = content.bump,
        has_one = author,
    )]
    pub content: Box<Account<'info, ContentRecord>>,
}
