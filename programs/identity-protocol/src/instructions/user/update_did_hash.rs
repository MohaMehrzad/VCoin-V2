use anchor_lang::prelude::*;

use crate::contexts::UpdateIdentity;

/// Update DID document hash
pub fn handler(ctx: Context<UpdateIdentity>, new_did_hash: [u8; 32]) -> Result<()> {
    let clock = Clock::get()?;
    let identity = &mut ctx.accounts.identity;
    
    identity.did_hash = new_did_hash;
    identity.updated_at = clock.unix_timestamp;
    
    msg!("DID hash updated for: {}", identity.owner);
    Ok(())
}

