use anchor_lang::prelude::*;

/// L-01: Events for VCoin token protocol state changes

/// Emitted when VCoin is initialized
#[event]
pub struct VCoinInitialized {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub permanent_delegate: Pubkey,
    pub timestamp: i64,
}

/// Emitted when tokens are minted
#[event]
pub struct TokensMinted {
    pub recipient: Pubkey,
    pub amount: u64,
    pub total_minted: u64,
    pub timestamp: i64,
}

/// Emitted when tokens are slashed (legacy)
#[event]
pub struct TokensSlashed {
    pub target: Pubkey,
    pub amount: u64,
    pub reason_hash: [u8; 32],
    pub timestamp: i64,
}

/// Emitted when a slash is proposed (H-01)
#[event]
pub struct SlashProposed {
    pub target: Pubkey,
    pub amount: u64,
    pub reason_hash: [u8; 32],
    pub proposer: Pubkey,
    pub timestamp: i64,
}

/// Emitted when a slash is approved (H-01)
#[event]
pub struct SlashApproved {
    pub target: Pubkey,
    pub amount: u64,
    pub approver: Pubkey,
    pub timelock_end: i64,
    pub timestamp: i64,
}

/// Emitted when a slash is executed (H-01)
#[event]
pub struct SlashExecuted {
    pub target: Pubkey,
    pub amount: u64,
    pub executor: Pubkey,
    pub timestamp: i64,
}

/// Emitted when the protocol is paused
#[event]
pub struct ProtocolPaused {
    pub authority: Pubkey,
    pub timestamp: i64,
}

/// Emitted when the protocol is unpaused
#[event]
pub struct ProtocolUnpaused {
    pub authority: Pubkey,
    pub timestamp: i64,
}

/// Emitted when the permanent delegate is updated
#[event]
pub struct PermanentDelegateUpdated {
    pub old_delegate: Pubkey,
    pub new_delegate: Pubkey,
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

