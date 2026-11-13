use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::constants::*;
use crate::errors::IdentityError;
use crate::state::*;

/// Upgrade authority only (audit ID-06). USDC mint and treasury account are validated.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = IdentityConfig::LEN, seeds = [IDENTITY_CONFIG_SEED], bump)]
    pub config: Box<Account<'info, IdentityConfig>>,
    #[account(constraint = usdc_mint.decimals == USDC_DECIMALS @ IdentityError::InvalidUsdcMint)]
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(token::mint = usdc_mint)]
    pub treasury_token: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ IdentityError::InvalidProgramData)]
    pub program: Program<'info, crate::program::IdentityProtocol>,
    pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [IDENTITY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, IdentityConfig>>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [IDENTITY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, IdentityConfig>>,
    pub new_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProposePayment<'info> {
    #[account(mut, seeds = [IDENTITY_CONFIG_SEED], bump = config.bump, has_one = usdc_mint)]
    pub config: Box<Account<'info, IdentityConfig>>,
    pub authority: Signer<'info>,
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(token::mint = usdc_mint)]
    pub treasury_token: Box<InterfaceAccount<'info, TokenAccount>>,
}

/// Permissionless once the timelock passed.
#[derive(Accounts)]
pub struct ApplyPayment<'info> {
    #[account(mut, seeds = [IDENTITY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, IdentityConfig>>,
}

#[derive(Accounts)]
#[instruction(did_hash: [u8; 32], username: String)]
pub struct CreateIdentity<'info> {
    // The username is validated here and in the handler; the seed below is capped at 32
    // bytes so an over-long name fails validation instead of aborting address derivation.
    #[account(
        seeds = [IDENTITY_CONFIG_SEED],
        bump = config.bump,
        constraint = is_valid_username(&username) @ IdentityError::InvalidUsername,
    )]
    pub config: Box<Account<'info, IdentityConfig>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer = owner, space = Identity::LEN, seeds = [IDENTITY_SEED, owner.key().as_ref()], bump)]
    pub identity: Box<Account<'info, Identity>>,
    #[account(init, payer = owner, space = UsernameRecord::LEN, seeds = [USERNAME_SEED, &username.as_bytes()[..username.len().min(MAX_USERNAME_LEN)]], bump)]
    pub username_record: Box<Account<'info, UsernameRecord>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateIdentity<'info> {
    #[account(seeds = [IDENTITY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, IdentityConfig>>,
    #[account(mut, seeds = [IDENTITY_SEED, owner.key().as_ref()], bump = identity.bump, has_one = owner)]
    pub identity: Box<Account<'info, Identity>>,
    pub owner: Signer<'info>,
}

/// Closes the identity and frees the username (rent to the owner).
#[derive(Accounts)]
pub struct CloseIdentity<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_SEED, owner.key().as_ref()],
        bump = identity.bump,
        has_one = owner,
        close = owner,
    )]
    pub identity: Box<Account<'info, Identity>>,
    #[account(
        mut,
        seeds = [USERNAME_SEED, identity.username()],
        bump = username_record.bump,
        has_one = owner,
        close = owner,
    )]
    pub username_record: Box<Account<'info, UsernameRecord>>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

/// The owner consents and a trusted attester co-signs.
#[derive(Accounts)]
#[instruction(attestation_id: [u8; 32])]
pub struct LinkAttestation<'info> {
    #[account(seeds = [IDENTITY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, IdentityConfig>>,
    #[account(mut, seeds = [IDENTITY_SEED, owner.key().as_ref()], bump = identity.bump, has_one = owner)]
    pub identity: Box<Account<'info, Identity>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub attester: Signer<'info>,
    #[account(
        init_if_needed,
        payer = owner,
        space = AttestationLink::LEN,
        seeds = [ATTESTATION_SEED, attestation_id.as_ref()],
        bump,
    )]
    pub attestation_link: Box<Account<'info, AttestationLink>>,
    pub system_program: Program<'info, System>,
}

/// The issuing attester or the authority revokes; the identity loses the level if this was
/// its current attestation, and the link closes.
#[derive(Accounts)]
pub struct RevokeAttestation<'info> {
    #[account(seeds = [IDENTITY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, IdentityConfig>>,
    #[account(
        mut,
        seeds = [ATTESTATION_SEED, attestation_link.attestation_id.as_ref()],
        bump = attestation_link.bump,
        has_one = rent_payer,
        close = rent_payer,
    )]
    pub attestation_link: Box<Account<'info, AttestationLink>>,
    /// The owner's identity, if it still exists.
    #[account(mut, seeds = [IDENTITY_SEED, attestation_link.owner.as_ref()], bump = identity.bump)]
    pub identity: Option<Box<Account<'info, Identity>>>,
    /// CHECK: bound by `has_one`.
    #[account(mut)]
    pub rent_payer: UncheckedAccount<'info>,
    pub revoker: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdminLowerLevel<'info> {
    #[account(seeds = [IDENTITY_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, IdentityConfig>>,
    #[account(mut, seeds = [IDENTITY_SEED, identity.owner.as_ref()], bump = identity.bump)]
    pub identity: Box<Account<'info, Identity>>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(seeds = [IDENTITY_CONFIG_SEED], bump = config.bump, has_one = usdc_mint, has_one = treasury_token)]
    pub config: Box<Account<'info, IdentityConfig>>,
    #[account(
        init_if_needed,
        payer = user,
        space = Subscription::LEN,
        seeds = [SUBSCRIPTION_SEED, user.key().as_ref()],
        bump,
    )]
    pub subscription: Box<Account<'info, Subscription>>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut, token::mint = usdc_mint, token::authority = user)]
    pub user_usdc: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub treasury_token: Box<InterfaceAccount<'info, TokenAccount>>,
    pub usdc_mint: Box<InterfaceAccount<'info, Mint>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}
