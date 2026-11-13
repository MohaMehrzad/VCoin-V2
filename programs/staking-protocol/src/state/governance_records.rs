//! SPL Governance (Realms) voter-weight addin accounts.
//!
//! Layouts match `spl-governance-addin-api` exactly (verified in LiteSVM against the
//! deployed mainnet and devnet governance programs). Anchor's account discriminator
//! `sha256("account:<Name>")[..8]` equals the addin API's discriminator, so plain
//! `#[account]` structs are byte-compatible.

use anchor_lang::prelude::*;

/// Realms action a voter-weight record is valid for (order is part of the wire format).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum VoterWeightAction {
    CastVote,
    CommentProposal,
    CreateGovernance,
    CreateProposal,
    SignOffProposal,
}

/// Per-owner voter weight, PDA `["voter-weight-record", owner]`. Rewritten by
/// `update_voter_weight_record` in the same transaction as the Realms action
/// (`voter_weight_expiry = current slot`).
#[account]
#[derive(Default, InitSpace)]
pub struct VoterWeightRecord {
    pub realm: Pubkey,
    pub governing_token_mint: Pubkey,
    pub governing_token_owner: Pubkey,
    pub voter_weight: u64,
    pub voter_weight_expiry: Option<u64>,
    pub weight_action: Option<VoterWeightAction>,
    pub weight_action_target: Option<Pubkey>,
    pub reserved: [u8; 8],
}

impl VoterWeightRecord {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// Realm-wide maximum voter weight, PDA `["max-voter-weight-record"]`.
#[account]
#[derive(Default, InitSpace)]
pub struct MaxVoterWeightRecord {
    pub realm: Pubkey,
    pub governing_token_mint: Pubkey,
    pub max_voter_weight: u64,
    pub max_voter_weight_expiry: Option<u64>,
    pub reserved: [u8; 8],
}

impl MaxVoterWeightRecord {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}
