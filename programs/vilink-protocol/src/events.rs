use anchor_lang::prelude::*;

#[event]
pub struct ViLinkInitialized {
    pub authority: Pubkey,
    pub treasury: Pubkey,
    pub fee_bps: u16,
}

#[event]
pub struct ConfigChangeProposed {
    pub fee_bps: u16,
    pub treasury: Pubkey,
    pub eta: i64,
}

#[event]
pub struct ConfigChangeApplied {
    pub fee_bps: u16,
    pub treasury: Pubkey,
}

#[event]
pub struct ConfigChangeCancelled {}

#[event]
pub struct EnabledActionsSet {
    pub enabled_actions: u8,
}

#[event]
pub struct PauseChanged {
    pub paused: bool,
}

#[event]
pub struct ActionCreated {
    pub action: Pubkey,
    pub action_id: [u8; 32],
    pub creator: Pubkey,
    pub action_type: u8,
    pub target: Pubkey,
    pub amount: u64,
    pub fee_bps: u16,
    pub expires_at: i64,
    pub max_executions: u32,
}

#[event]
pub struct ActionExecuted {
    pub action: Pubkey,
    pub action_type: u8,
    pub executor: Pubkey,
    pub target: Pubkey,
    pub amount: u64,
    pub fee: u64,
    pub execution_count: u32,
    pub signer: Pubkey,
}

#[event]
pub struct ActionClosed {
    pub action: Pubkey,
    pub execution_count: u32,
}

#[event]
pub struct AuthorityProposed {
    pub current: Pubkey,
    pub proposed: Pubkey,
    pub ready_at: i64,
}

#[event]
pub struct AuthorityTransferCancelled {
    pub authority: Pubkey,
}

#[event]
pub struct AuthorityTransferred {
    pub previous: Pubkey,
    pub new_authority: Pubkey,
}

/// The emergency guardian may pause (never unpause); `Pubkey::default()` = none.
#[event]
pub struct GuardianSet {
    pub guardian: Pubkey,
}
