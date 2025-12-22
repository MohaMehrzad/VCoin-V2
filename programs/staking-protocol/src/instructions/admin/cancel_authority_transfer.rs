use anchor_lang::prelude::*;

use crate::contexts::AdminAction;
use crate::errors::StakingError;
use crate::events::AuthorityTransferCancelled;

/// Cancel a pending authority transfer (H-02 security fix)
/// Only the current authority can cancel
pub fn handler(ctx: Context<AdminAction>) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    
    require!(
        ctx.accounts.authority.key() == pool.authority,
        StakingError::Unauthorized
    );
    
    require!(
        pool.pending_authority != Pubkey::default(),
        StakingError::NoPendingTransfer
    );
    
    let cancelled = pool.pending_authority;
    pool.pending_authority = Pubkey::default();
    
    let clock = Clock::get()?;
    
    // L-01: Emit authority transfer cancelled event
    emit!(AuthorityTransferCancelled {
        authority: pool.authority,
        cancelled_pending: cancelled,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Authority transfer to {} cancelled", cancelled);
    
    Ok(())
}

