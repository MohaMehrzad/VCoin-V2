use anchor_lang::prelude::*;

use crate::contexts::CreateSnapshot;
use crate::errors::FiveAError;
use crate::events::SnapshotCreated;

/// Create a score snapshot (oracle only)
pub fn handler(
    ctx: Context<CreateSnapshot>,
    merkle_root: [u8; 32],
    user_count: u64,
    avg_score: u16,
) -> Result<()> {
    let config = &mut ctx.accounts.five_a_config;
    require!(!config.paused, FiveAError::ProtocolPaused);
    
    // Verify oracle
    let oracle_key = ctx.accounts.oracle.key();
    let is_oracle = config.oracles[..config.oracle_count as usize]
        .contains(&oracle_key);
    require!(is_oracle, FiveAError::NotOracle);
    
    let clock = Clock::get()?;
    
    // Increment epoch
    config.current_epoch = config.current_epoch.saturating_add(1);
    config.last_snapshot_time = clock.unix_timestamp;
    
    // Create snapshot
    let snapshot = &mut ctx.accounts.snapshot;
    snapshot.epoch = config.current_epoch;
    snapshot.merkle_root = merkle_root;
    snapshot.user_count = user_count;
    snapshot.avg_score = avg_score;
    snapshot.timestamp = clock.unix_timestamp;
    snapshot.submitter = oracle_key;
    snapshot.bump = ctx.bumps.snapshot;
    
    emit!(SnapshotCreated {
        epoch: snapshot.epoch,
        merkle_root,
        user_count,
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Snapshot created: epoch {}", snapshot.epoch);
    Ok(())
}

