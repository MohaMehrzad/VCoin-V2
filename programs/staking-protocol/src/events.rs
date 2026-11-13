use anchor_lang::prelude::*;

#[event]
pub struct RegistrarCreated {
    pub registrar: Pubkey,
    pub authority: Pubkey,
    pub vcoin_mint: Pubkey,
    pub vevcoin_mint: Pubkey,
    pub vault: Pubkey,
}

#[event]
pub struct VoterCreated {
    pub voter: Pubkey,
    pub owner: Pubkey,
}

#[event]
pub struct Deposited {
    pub owner: Pubkey,
    pub index: u8,
    pub amount: u64,
    pub lock_end: i64,
    pub total_locked: u64,
}

#[event]
pub struct LockExtended {
    pub owner: Pubkey,
    pub index: u8,
    pub old_lock_end: i64,
    pub new_lock_end: i64,
}

#[event]
pub struct Withdrawn {
    pub owner: Pubkey,
    pub index: u8,
    pub amount: u64,
    pub total_locked: u64,
}

#[event]
pub struct PauseChanged {
    pub paused: bool,
}

#[event]
pub struct WeightParamsUpdated {
    pub baseline_factor: u64,
    pub max_extra_factor: u64,
    pub saturation_secs: u64,
    pub min_lock_remaining_to_vote: u64,
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

#[event]
pub struct GovernanceConfigured {
    pub realm: Pubkey,
    pub governance_program_id: Pubkey,
    pub slash_destination: Pubkey,
}

#[event]
pub struct GateParamsUpdated {
    pub min_vote_score: u16,
    pub min_propose_score: u16,
}
