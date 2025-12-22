use anchor_lang::prelude::*;

/// L-01: Events for veVCoin token protocol state changes

/// Emitted when veVCoin is initialized
#[event]
pub struct VeVCoinInitialized {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub staking_protocol: Pubkey,
    pub timestamp: i64,
}

/// Emitted when veVCoin is minted
#[event]
pub struct VeVCoinMinted {
    pub user: Pubkey,
    pub amount: u64,
    pub total_supply: u64,
    pub timestamp: i64,
}

/// Emitted when veVCoin is burned
#[event]
pub struct VeVCoinBurned {
    pub user: Pubkey,
    pub amount: u64,
    pub remaining_supply: u64,
    pub timestamp: i64,
}

/// Emitted when staking protocol is updated
#[event]
pub struct StakingProtocolUpdated {
    pub old_staking_protocol: Pubkey,
    pub new_staking_protocol: Pubkey,
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

