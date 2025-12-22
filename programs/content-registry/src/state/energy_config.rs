use anchor_lang::prelude::*;

/// Energy system configuration
#[account]
#[derive(Default)]
pub struct EnergyConfig {
    /// Admin authority
    pub authority: Pubkey,
    /// Base regen rate per hour
    pub base_regen_rate: u16,
    /// Engagement check delay (seconds)
    pub engagement_check_delay: i64,
    /// Viral threshold (likes)
    pub viral_threshold: u32,
    /// Whether energy system is paused
    pub paused: bool,
    /// PDA bump
    pub bump: u8,
}

impl EnergyConfig {
    pub const LEN: usize = 8 + // discriminator
        32 + // authority
        2 +  // base_regen_rate
        8 +  // engagement_check_delay
        4 +  // viral_threshold
        1 +  // paused
        1;   // bump
}
