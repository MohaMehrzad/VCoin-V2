use anchor_lang::prelude::*;
use vcoin_common::AdminState;

use crate::constants::*;

#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq, InitSpace,
)]
pub struct OracleEntry {
    /// `Pubkey::default()` = empty slot.
    pub key: Pubkey,
    /// Votes count only from this time on (registration + 24h).
    pub active_from: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub struct PoolParams {
    pub epoch_duration: i64,
    /// How long claims stay open after a root activates (also the deadline for activating one).
    pub claim_window: i64,
    /// Wait between a root reaching quorum and becoming claimable (DAO can veto meanwhile).
    pub dispute_delay: i64,
    /// Each epoch's budget is the previous one × (1 − decay_bps / 10000).
    pub decay_bps: u16,
}

impl Default for PoolParams {
    fn default() -> Self {
        Self {
            epoch_duration: DEFAULT_EPOCH_DURATION,
            claim_window: DEFAULT_CLAIM_WINDOW,
            dispute_delay: DEFAULT_DISPUTE_DELAY,
            decay_bps: DEFAULT_DECAY_BPS,
        }
    }
}

impl PoolParams {
    pub fn is_valid(&self) -> bool {
        (MIN_EPOCH_DURATION..=MAX_EPOCH_DURATION).contains(&self.epoch_duration)
            && (MIN_CLAIM_WINDOW..=MAX_CLAIM_WINDOW).contains(&self.claim_window)
            && (MIN_DISPUTE_DELAY..=MAX_DISPUTE_DELAY).contains(&self.dispute_delay)
            // A root must be proposed, survive the dispute delay (possibly twice, after a
            // veto or an oracle change) and activate within the claim window after the
            // epoch ends, so the window must leave room for that.
            && self.claim_window >= 2 * self.dispute_delay
            && self.decay_bps <= MAX_DECAY_BPS
    }
}

/// Singleton, PDA `["pool-config"]`. Owns the reserve vault and every epoch vault.
#[account]
#[derive(InitSpace)]
pub struct PoolConfig {
    /// Bootstrapped by the upgrade authority, handed to the Realms governance PDA.
    pub admin: AdminState,
    pub vcoin_mint: Pubkey,
    /// PDA `["reserve-vault"]`: the undistributed rewards (e.g. the 350M allocation).
    pub reserve_vault: Pubkey,
    pub fee_bps: u16,
    /// PDA `["fee-vault"]`: claim fees collect here, so a claim never depends on an
    /// outside account. Anyone may sweep it to `fee_destination`.
    pub fee_vault: Pubkey,
    pub fee_destination: Pubkey,
    /// Timelocked fee change (`pending_fee_eta == 0` = none).
    pub pending_fee_bps: u16,
    pub pending_fee_destination: Pubkey,
    pub pending_fee_eta: i64,
    pub params: PoolParams,
    /// Ceiling for any epoch's budget, fixed at initialization.
    pub max_epoch_budget: u64,
    pub next_epoch_budget: u64,
    pub epoch_count: u64,
    pub last_epoch_start: i64,
    pub oracles: [OracleEntry; MAX_ORACLES],
    pub root_quorum: u8,
    /// Bumped whenever the oracle set or the quorum changes; a proposed root is only valid
    /// under the version that produced it.
    pub oracle_set_version: u64,
    pub paused: bool,
    pub paused_since: i64,
    /// Total seconds spent paused, so deadlines can be extended by downtime (SSC-05).
    pub pause_accum: i64,
    pub bump: u8,
    pub reserve_bump: u8,
    pub fee_vault_bump: u8,
}

impl PoolConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn registered_oracles(&self) -> usize {
        self.oracles
            .iter()
            .filter(|o| o.key != Pubkey::default())
            .count()
    }

    pub fn is_active_oracle(&self, key: &Pubkey, now: i64) -> bool {
        *key != Pubkey::default()
            && self
                .oracles
                .iter()
                .any(|o| o.key == *key && o.active_from <= now)
    }

    pub fn active_oracles(&self, now: i64) -> usize {
        self.oracles
            .iter()
            .filter(|o| o.key != Pubkey::default() && o.active_from <= now)
            .count()
    }

    /// Votes a candidate needs: the configured quorum, and never less than a strict
    /// majority of the active oracles, so two candidates can never both qualify and a
    /// minority cannot settle a root on its own.
    pub fn effective_quorum(&self, now: i64) -> u8 {
        let majority = (self.active_oracles(now) / 2 + 1) as u8; // ≤ MAX_ORACLES / 2 + 1
        self.root_quorum.max(majority)
    }

    /// A vote counts if its oracle is registered and active, and the vote was cast under
    /// the oracle's current registration (a removed and re-added oracle must vote again).
    pub fn vote_counts(&self, vote: &OracleVote, now: i64) -> bool {
        vote.oracle != Pubkey::default()
            && self.oracles.iter().any(|o| {
                o.key == vote.oracle && o.active_from <= now && vote.voted_at >= o.active_from
            })
    }

    /// Paused seconds so far, including a pause in progress.
    pub fn paused_total(&self, now: i64) -> i64 {
        self.pause_accum
            + if self.paused {
                now.saturating_sub(self.paused_since)
            } else {
                0
            }
    }
}

#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq, InitSpace,
)]
pub struct OracleVote {
    /// `Pubkey::default()` = empty slot.
    pub oracle: Pubkey,
    pub root: [u8; 32],
    pub max_total_claim: u64,
    pub voted_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum EpochState {
    /// Funded; oracles vote on a root once the epoch ends.
    Voting,
    /// Root active; claims open until the deadline.
    Active,
    /// Unclaimed funds returned to the reserve.
    Closed,
}

/// One distribution, PDA `["epoch", index]`, with its own vault `["epoch-vault", index]`
/// holding exactly its budget, so no epoch can pay out another's funds (SSC-02).
#[account]
#[derive(InitSpace)]
pub struct Epoch {
    pub index: u64,
    pub budget: u64,
    pub vault: Pubkey,
    pub start_ts: i64,
    pub end_ts: i64,
    pub pause_accum_at_start: i64,
    /// `PoolParams` snapshot at start, so later parameter changes never move this epoch's
    /// deadlines.
    pub claim_window: i64,
    pub dispute_delay: i64,
    pub votes: [OracleVote; MAX_ORACLES],
    /// Candidate that reached quorum (`proposed_at == 0` = none).
    pub proposed_root: [u8; 32],
    pub proposed_max_total: u64,
    pub proposed_at: i64,
    /// `PoolConfig::oracle_set_version` when the candidate was proposed.
    pub proposed_version: u64,
    pub root: [u8; 32],
    pub max_total_claim: u64,
    /// Fee snapshot at activation, so every claimant of an epoch pays the same rate.
    pub fee_bps: u16,
    pub activated_at: i64,
    pub claim_deadline: i64,
    pub pause_accum_at_activation: i64,
    /// Gross (before fee), like `max_total_claim` (SSC-13).
    pub total_claimed: u64,
    pub num_claims: u64,
    pub state: EpochState,
    pub rent_payer: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
}

impl Epoch {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    /// Matching votes for `(root, max_total)` that still count (see `vote_counts`).
    pub fn count_votes(
        &self,
        config: &PoolConfig,
        root: &[u8; 32],
        max_total: u64,
        now: i64,
    ) -> u8 {
        self.votes
            .iter()
            .filter(|v| {
                v.root == *root && v.max_total_claim == max_total && config.vote_counts(v, now)
            })
            .count() as u8
    }

    /// The proposed candidate still stands: same oracle set and quorum as when it was
    /// proposed, and it still has the votes.
    pub fn proposal_stands(&self, config: &PoolConfig, now: i64) -> bool {
        self.proposed_at != 0
            && self.proposed_version == config.oracle_set_version
            && self.count_votes(config, &self.proposed_root, self.proposed_max_total, now)
                >= config.effective_quorum(now)
    }

    /// Claims close at activation + window, extended by any pause since activation.
    pub fn effective_claim_deadline(&self, config: &PoolConfig, now: i64) -> i64 {
        self.claim_deadline.saturating_add(
            config
                .paused_total(now)
                .saturating_sub(self.pause_accum_at_activation),
        )
    }

    /// A root must activate before end + window (extended by pauses), or the budget returns.
    pub fn root_deadline(&self, config: &PoolConfig, now: i64) -> i64 {
        self.end_ts
            .saturating_add(self.claim_window)
            .saturating_add(
                config
                    .paused_total(now)
                    .saturating_sub(self.pause_accum_at_start),
            )
    }
}

/// Marks one claimant's claim in one epoch, PDA `["claim", epoch, claimant]` (SSC-10: no
/// per-user epoch bitmap to run out of).
#[account]
#[derive(InitSpace)]
pub struct ClaimStatus {
    pub epoch: Pubkey,
    pub claimant: Pubkey,
    pub amount: u64,
    pub claimed_at: i64,
    pub rent_payer: Pubkey,
    pub bump: u8,
}

impl ClaimStatus {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}
