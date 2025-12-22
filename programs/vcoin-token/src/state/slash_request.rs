use anchor_lang::prelude::*;

use crate::constants::{SLASH_STATUS_PENDING, SLASH_STATUS_APPROVED, SLASH_STATUS_EXECUTED};

/// Slash request requiring governance approval (H-01 Security Fix)
/// 
/// Slashing flow:
/// 1. Permanent delegate proposes slash -> creates SlashRequest (Pending)
/// 2. Governance votes and approves -> status becomes Approved, timelock starts
/// 3. After 48 hour timelock -> slash can be executed
/// 4. Execute burns tokens from target account
#[account]
#[derive(Default)]
pub struct SlashRequest {
    /// Target account to slash tokens from
    pub target: Pubkey,
    /// Amount of tokens to slash
    pub amount: u64,
    /// Hash of the reason/evidence for slashing
    pub reason_hash: [u8; 32],
    /// Who proposed the slash (permanent delegate)
    pub proposer: Pubkey,
    /// Governance proposal ID that approved this slash
    pub proposal_id: u64,
    /// Status: 0=Pending, 1=Approved, 2=Executed, 3=Rejected, 4=Cancelled
    pub status: u8,
    /// Timestamp when request was created
    pub created_at: i64,
    /// Timestamp when 48h timelock ends (set after governance approval)
    pub timelock_end: i64,
    /// Timestamp when slash was executed
    pub executed_at: i64,
    /// PDA bump
    pub bump: u8,
}

impl SlashRequest {
    pub const LEN: usize = 8 + // discriminator
        32 + // target
        8 +  // amount
        32 + // reason_hash
        32 + // proposer
        8 +  // proposal_id
        1 +  // status
        8 +  // created_at
        8 +  // timelock_end
        8 +  // executed_at
        1;   // bump
    
    /// Check if the slash request is pending
    pub fn is_pending(&self) -> bool {
        self.status == SLASH_STATUS_PENDING
    }
    
    /// Check if the slash request is approved and ready for execution
    pub fn is_approved(&self) -> bool {
        self.status == SLASH_STATUS_APPROVED
    }
    
    /// Check if the slash request has been executed
    pub fn is_executed(&self) -> bool {
        self.status == SLASH_STATUS_EXECUTED
    }
    
    /// Check if timelock has expired (can execute)
    pub fn is_timelock_expired(&self, current_timestamp: i64) -> bool {
        self.is_approved() && current_timestamp >= self.timelock_end
    }
}

