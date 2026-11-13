use anchor_lang::prelude::*;
use vcoin_common::AdminState;

use crate::constants::*;

/// Voting-weight parameters (fixed point, [`FACTOR_SCALE`] = 1.0×).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub struct WeightParams {
    pub baseline_factor: u64,
    pub max_extra_factor: u64,
    pub saturation_secs: u64,
    pub min_lock_remaining_to_vote: u64,
}

impl Default for WeightParams {
    fn default() -> Self {
        Self {
            baseline_factor: DEFAULT_BASELINE_FACTOR,
            max_extra_factor: DEFAULT_MAX_EXTRA_FACTOR,
            saturation_secs: DEFAULT_SATURATION_SECS,
            min_lock_remaining_to_vote: DEFAULT_MIN_LOCK_REMAINING_TO_VOTE,
        }
    }
}

impl WeightParams {
    pub fn is_valid(&self) -> bool {
        self.baseline_factor <= MAX_BASELINE_FACTOR
            && self.max_extra_factor <= MAX_EXTRA_FACTOR_CAP
            && self.saturation_secs >= MIN_SATURATION_SECS
            && self.saturation_secs <= MAX_LOCK_SECS as u64
            && self.min_lock_remaining_to_vote <= MAX_LOCK_SECS as u64
    }
}

/// Singleton registrar, PDA `["registrar"]`. Owns the VCoin vault and is the only
/// account the veVCoin program accepts as mint/burn signer.
#[account]
#[derive(Default, InitSpace)]
pub struct Registrar {
    /// Authority for pause, parameters and slashing. Bootstrapped by the deployer and handed
    /// (24h two-step) to the Realms governance PDA after launch, so every change is a DAO vote.
    pub admin: AdminState,
    /// Realms (SPL Governance) program and realm this registrar serves as voter-weight plugin.
    pub governance_program_id: Pubkey,
    pub realm: Pubkey,
    /// The realm's community governing-token mint (the mint Realms votes are cast for).
    pub realm_governing_token_mint: Pubkey,
    pub vcoin_mint: Pubkey,
    pub vault: Pubkey,
    pub vevcoin_mint: Pubkey,
    /// Where slashed VCoin goes (DAO treasury token account).
    pub slash_destination: Pubkey,
    pub weight: WeightParams,
    /// 5A composite score (0..=10000) required to vote / to create proposals.
    pub min_vote_score: u16,
    pub min_propose_score: u16,
    /// Sum of all deposits (receipt supply upper bound; drives MaxVoterWeightRecord).
    pub total_locked: u64,
    /// Principal in deposits not yet deactivated after their lock ended. The max voter weight
    /// is computed from this, so abandoned expired deposits cannot inflate it forever and
    /// make percentage thresholds unreachable.
    pub active_locked: u64,
    pub slash_nonce: u64,
    pub paused: bool,
    pub bump: u8,
    pub vault_bump: u8,
}

impl Registrar {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}
