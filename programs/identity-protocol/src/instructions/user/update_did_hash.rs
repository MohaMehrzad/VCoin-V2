use anchor_lang::prelude::*;

use crate::contexts::UpdateIdentity;
use crate::errors::IdentityError;

/// Update DID document hash
pub fn handler(ctx: Context<UpdateIdentity>, new_did_hash: [u8; 32]) -> Result<()> {
    let clock = Clock::get()?;
    let identity = &mut ctx.accounts.identity;

    // H-AUDIT-13: Require verification before DID hash changes
    require!(identity.verification_level > 0, IdentityError::IdentityNotFound);

    identity.did_hash = new_did_hash;
    identity.updated_at = clock.unix_timestamp;

    msg!("DID hash updated for: {}", identity.owner);
    Ok(())
}

