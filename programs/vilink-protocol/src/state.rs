use anchor_lang::prelude::*;
use vcoin_common::AdminState;

use crate::constants::*;

/// Singleton, PDA `["vilink-config"]`. Read-only on create and execute (audit VL-13).
#[account]
#[derive(InitSpace)]
pub struct ViLinkConfig {
    pub admin: AdminState,
    pub vcoin_mint: Pubkey,
    /// VCoin token account receiving tip fees.
    pub treasury: Pubkey,
    /// Fee for new links (each link keeps the fee it was created with, audit VL-16).
    pub fee_bps: u16,
    /// Timelocked change of fee and treasury (`pending_eta == 0` = none, audit VL-4).
    pub pending_fee_bps: u16,
    pub pending_treasury: Pubkey,
    pub pending_eta: i64,
    /// Bit per action type; disabling stops creation and execution (audit VL-2).
    pub enabled_actions: u8,
    pub paused: bool,
    pub bump: u8,
}

impl ViLinkConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn is_enabled(&self, action_type: u8) -> bool {
        action_type < NUM_ACTION_TYPES && self.enabled_actions & (1 << action_type) != 0
    }
}

/// A shareable action, PDA `["action", creator, nonce]`.
#[account]
#[derive(InitSpace)]
pub struct ViLinkAction {
    /// Unique per (creator, nonce); used in share URIs.
    pub action_id: [u8; 32],
    pub creator: Pubkey,
    pub nonce: u64,
    pub action_type: u8,
    /// Tip amount (gross); 0 for vouch / follow.
    pub amount: u64,
    pub target: Pubkey,
    pub metadata_hash: [u8; 32],
    pub fee_bps: u16,
    pub created_at: i64,
    pub expires_at: i64,
    pub max_executions: u32,
    pub execution_count: u32,
    pub total_volume: u64,
    pub bump: u8,
}

impl ViLinkAction {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn is_open(&self, now: i64) -> bool {
        now < self.expires_at && self.execution_count < self.max_executions
    }
}

/// One execution of an action by one executor, PDA `["exec", action, executor]`: each
/// executor can act on a link once, whatever its type (audit VL-3).
#[account]
#[derive(InitSpace)]
pub struct ExecutionReceipt {
    pub action: Pubkey,
    pub executor: Pubkey,
    pub executed_at: i64,
    pub rent_payer: Pubkey,
    pub bump: u8,
}

impl ExecutionReceipt {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// Per-user counters and the next action nonce, PDA `["user-stats", user]`.
#[account]
#[derive(Default, InitSpace)]
pub struct UserActionStats {
    pub user: Pubkey,
    /// Nonce for the next action this user creates.
    pub actions_created: u64,
    pub tips_sent: u64,
    pub tips_received: u64,
    pub vouches_given: u32,
    pub vouches_received: u32,
    pub follows_given: u32,
    pub follows_received: u32,
    pub bump: u8,
}

impl UserActionStats {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// `sha256(creator ‖ nonce ‖ type ‖ target ‖ amount ‖ created_at)`; the nonce makes two
/// identical links created in the same second distinct (audit VL-11).
pub fn action_id(
    creator: &Pubkey,
    nonce: u64,
    action_type: u8,
    target: &Pubkey,
    amount: u64,
    created_at: i64,
) -> [u8; 32] {
    solana_sha256_hasher::hashv(&[
        creator.as_ref(),
        &nonce.to_le_bytes(),
        &[action_type],
        target.as_ref(),
        &amount.to_le_bytes(),
        &created_at.to_le_bytes(),
    ])
    .to_bytes()
}

/// Fee rounded up, never more than the amount.
pub fn tip_fee(amount: u64, fee_bps: u16) -> u64 {
    ((amount as u128 * fee_bps as u128).div_ceil(BPS as u128) as u64).min(amount)
}
