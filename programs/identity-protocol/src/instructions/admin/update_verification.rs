use anchor_lang::prelude::*;

use crate::contexts::AdminUpdateIdentity;
use crate::errors::IdentityError;
use crate::events::VerificationUpdated;
use crate::state::VerificationLevel;

/// Update verification level (admin only)
pub fn handler(
    ctx: Context<AdminUpdateIdentity>,
    new_level: u8,
    verification_hash: [u8; 32],
) -> Result<()> {
    let _level = VerificationLevel::from_u8(new_level)
        .ok_or(IdentityError::InvalidVerificationLevel)?;
    
    let clock = Clock::get()?;
    let identity = &mut ctx.accounts.identity;
    let old_level = identity.verification_level;
    
    // Cannot downgrade verification
    require!(
        new_level >= old_level,
        IdentityError::CannotDowngradeVerification
    );
    
    identity.verification_level = new_level;
    identity.verification_hash = verification_hash;
    identity.updated_at = clock.unix_timestamp;
    
    // Update verified count if upgrading from None
    if old_level == 0 && new_level > 0 {
        let config = &mut ctx.accounts.identity_config;
        config.verified_identities = config.verified_identities.saturating_add(1);
    }
    
    emit!(VerificationUpdated {
        owner: identity.owner,
        old_level,
        new_level,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Verification updated: {} -> {}", old_level, new_level);
    Ok(())
}

