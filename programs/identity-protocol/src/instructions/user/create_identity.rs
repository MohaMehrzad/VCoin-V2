use anchor_lang::prelude::*;

use crate::contexts::CreateIdentity;
use crate::errors::IdentityError;
use crate::events::IdentityCreated;
use crate::state::VerificationLevel;

/// Create a new identity for a user
pub fn handler(ctx: Context<CreateIdentity>, did_hash: [u8; 32], username: String) -> Result<()> {
    require!(!ctx.accounts.identity_config.paused, IdentityError::ProtocolPaused);
    require!(username.len() <= 32, IdentityError::InvalidVerificationLevel);
    
    let clock = Clock::get()?;
    let identity = &mut ctx.accounts.identity;
    
    identity.owner = ctx.accounts.owner.key();
    identity.did_hash = did_hash;
    identity.verification_level = VerificationLevel::None as u8;
    identity.verification_hash = [0u8; 32];
    
    // Store username
    let username_bytes = username.as_bytes();
    identity.username[..username_bytes.len()].copy_from_slice(username_bytes);
    identity.username_len = username_bytes.len() as u8;
    
    identity.created_at = clock.unix_timestamp;
    identity.updated_at = clock.unix_timestamp;
    identity.is_active = true;
    identity.bump = ctx.bumps.identity;
    
    // Update global stats
    let config = &mut ctx.accounts.identity_config;
    config.total_identities = config.total_identities.saturating_add(1);
    
    emit!(IdentityCreated {
        owner: identity.owner,
        username,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Identity created for: {}", identity.owner);
    Ok(())
}

