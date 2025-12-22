use anchor_lang::prelude::*;

/// Global ViLink configuration
#[account]
#[derive(Default)]
pub struct ViLinkConfig {
    /// Admin authority
    pub authority: Pubkey,
    /// VCoin mint
    pub vcoin_mint: Pubkey,
    /// Treasury for platform fees
    pub treasury: Pubkey,
    /// 5A Protocol for vouch integration
    pub five_a_program: Pubkey,
    /// Staking protocol for stake actions
    pub staking_program: Pubkey,
    /// Content registry for react actions
    pub content_registry: Pubkey,
    /// Governance protocol for vote/delegate actions
    pub governance_program: Pubkey,
    /// Gasless protocol for session key execution
    pub gasless_program: Pubkey,
    /// Enabled action types bitmap (8 bits, one per action type)
    pub enabled_actions: u8,
    /// Total actions created
    pub total_actions_created: u64,
    /// Total actions executed
    pub total_actions_executed: u64,
    /// Total VCoin volume through tips
    pub total_tip_volume: u64,
    /// Whether protocol is paused
    pub paused: bool,
    /// Platform fee in basis points
    pub platform_fee_bps: u16,
    /// PDA bump
    pub bump: u8,
}

impl ViLinkConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        32 + // vcoin_mint
        32 + // treasury
        32 + // five_a_program
        32 + // staking_program
        32 + // content_registry
        32 + // governance_program
        32 + // gasless_program
        1 +  // enabled_actions
        8 +  // total_actions_created
        8 +  // total_actions_executed
        8 +  // total_tip_volume
        1 +  // paused
        2 +  // platform_fee_bps
        1;   // bump
    
    /// Check if action type is enabled
    pub fn is_action_enabled(&self, action_type: u8) -> bool {
        (self.enabled_actions & (1 << action_type)) != 0
    }
}

