use anchor_lang::prelude::*;

/// Global hook configuration
#[account]
#[derive(Default)]
pub struct HookConfig {
    /// Admin authority
    pub authority: Pubkey,
    /// Pending authority for two-step transfer (H-02 security fix)
    pub pending_authority: Pubkey,
    /// VCoin mint address
    pub vcoin_mint: Pubkey,
    /// 5A Protocol program (for CPI calls)
    pub five_a_program: Pubkey,
    /// Whether wash trading blocking is enabled (vs just flagging)
    pub block_wash_trading: bool,
    /// Minimum amount for activity score increment
    pub min_activity_amount: u64,
    /// Total transfers processed
    pub total_transfers: u64,
    /// Total wash trading flags
    pub wash_trading_flags: u64,
    /// Whether hook is paused
    pub paused: bool,
    /// PDA bump
    pub bump: u8,
}

impl HookConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // pending_authority (NEW - H-02)
        32 + // vcoin_mint
        32 + // five_a_program
        1 +  // block_wash_trading
        8 +  // min_activity_amount
        8 +  // total_transfers
        8 +  // wash_trading_flags
        1 +  // paused
        1;   // bump
}

