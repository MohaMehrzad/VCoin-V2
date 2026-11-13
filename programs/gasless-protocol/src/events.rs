use anchor_lang::prelude::*;

#[event]
pub struct GaslessInitialized {
    pub config: Pubkey,
    pub authority: Pubkey,
    pub fee_vault: Pubkey,
    pub treasury: Pubkey,
    pub vcoin_fee_per_tx: u64,
}

#[event]
pub struct SessionCreated {
    pub session: Pubkey,
    pub authority: Pubkey,
    pub session_signer: Pubkey,
    pub target_program: Pubkey,
    pub scope: u16,
    pub valid_until: i64,
}

#[event]
pub struct SessionRevoked {
    pub session: Pubkey,
    pub authority: Pubkey,
    pub revoked_by: Pubkey,
}

#[event]
pub struct FeePaid {
    pub payer: Pubkey,
    pub amount: u64,
}

#[event]
pub struct FeeChangeScheduled {
    pub new_fee: u64,
    pub effective_at: i64,
}

#[event]
pub struct FeeChanged {
    pub old_fee: u64,
    pub new_fee: u64,
}

#[event]
pub struct TreasuryChangeScheduled {
    pub new_treasury: Pubkey,
    pub effective_at: i64,
}

#[event]
pub struct TreasuryChanged {
    pub old_treasury: Pubkey,
    pub new_treasury: Pubkey,
}

#[event]
pub struct FeesWithdrawn {
    pub treasury: Pubkey,
    pub amount: u64,
}

#[event]
pub struct PauseChanged {
    pub paused: bool,
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
    pub cancelled: Pubkey,
}

#[event]
pub struct AuthorityTransferred {
    pub previous: Pubkey,
    pub new_authority: Pubkey,
}
