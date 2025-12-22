use anchor_lang::prelude::*;

/// 6-Layer Funding Configuration (for post Year 5 sustainability)
#[account]
#[derive(Default)]
pub struct FundingLayerConfig {
    /// Authority
    pub authority: Pubkey,
    /// Layer 1: Primary reserves remaining
    pub l1_primary_remaining: u64,
    /// Layer 2: Secondary reserves (buyback buffer)
    pub l2_secondary_remaining: u64,
    /// Layer 3: Buyback recycling (10% monthly revenue)
    pub l3_buyback_rate_bps: u16,
    /// Layer 4: Profit buybacks (25% quarterly profit)
    pub l4_profit_rate_bps: u16,
    /// Layer 5: Fee recycling (50% platform fees)
    pub l5_fee_recycling_rate_bps: u16,
    /// Current active layer (1-5)
    pub active_layer: u8,
    /// Total recycled through L3-L5
    pub total_recycled: u64,
    /// Last layer switch timestamp
    pub last_layer_switch: i64,
    /// Months until primary depletion (estimate)
    pub months_remaining_estimate: u16,
    /// PDA bump
    pub bump: u8,
}

impl FundingLayerConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        8 +  // l1_primary_remaining
        8 +  // l2_secondary_remaining
        2 +  // l3_buyback_rate_bps
        2 +  // l4_profit_rate_bps
        2 +  // l5_fee_recycling_rate_bps
        1 +  // active_layer
        8 +  // total_recycled
        8 +  // last_layer_switch
        2 +  // months_remaining_estimate
        1;   // bump
}

