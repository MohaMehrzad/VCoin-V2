use anchor_lang::prelude::*;

/// L-01: Events for staking protocol state changes

/// Emitted when a user stakes VCoin
#[event]
pub struct Staked {
    pub user: Pubkey,
    pub amount: u64,
    pub lock_duration: i64,
    pub vevcoin_minted: u64,
    pub tier: u8,
    pub timestamp: i64,
}

/// Emitted when a user unstakes VCoin
#[event]
pub struct Unstaked {
    pub user: Pubkey,
    pub amount: u64,
    pub vevcoin_burned: u64,
    pub remaining_stake: u64,
    pub timestamp: i64,
}

/// Emitted when a user extends their lock duration
#[event]
pub struct LockExtended {
    pub user: Pubkey,
    pub old_lock_end: i64,
    pub new_lock_end: i64,
    pub new_vevcoin: u64,
    pub timestamp: i64,
}

/// Emitted when a user's tier changes
#[event]
pub struct TierUpdated {
    pub user: Pubkey,
    pub old_tier: u8,
    pub new_tier: u8,
    pub timestamp: i64,
}

/// Emitted when the pool is paused
#[event]
pub struct PoolPaused {
    pub authority: Pubkey,
    pub timestamp: i64,
}

/// Emitted when the pool is unpaused
#[event]
pub struct PoolUnpaused {
    pub authority: Pubkey,
    pub timestamp: i64,
}

/// Emitted when authority transfer is proposed
#[event]
pub struct AuthorityTransferProposed {
    pub current_authority: Pubkey,
    pub proposed_authority: Pubkey,
    pub timestamp: i64,
}

/// Emitted when authority transfer is accepted
#[event]
pub struct AuthorityTransferAccepted {
    pub old_authority: Pubkey,
    pub new_authority: Pubkey,
    pub timestamp: i64,
}

/// Emitted when authority transfer is cancelled
#[event]
pub struct AuthorityTransferCancelled {
    pub authority: Pubkey,
    pub cancelled_pending: Pubkey,
    pub timestamp: i64,
}

/// Emitted when staking pool is initialized
#[event]
pub struct StakingPoolInitialized {
    pub authority: Pubkey,
    pub vcoin_mint: Pubkey,
    pub vevcoin_mint: Pubkey,
    pub timestamp: i64,
}

