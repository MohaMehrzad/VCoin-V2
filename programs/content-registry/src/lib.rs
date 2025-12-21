use anchor_lang::prelude::*;

declare_id!("3Ex3eTSLUcLdfkMdUD91FH3a5CaFSzydMtCwAWGvW5vY");

/// Content Registry Protocol
/// 
/// On-chain tracking hash with state management.
/// Each piece of content gets a unique tracking hash stored on-chain.
/// 
/// Content Types:
/// - Post (text/image)
/// - Article (long-form)
/// - Media (video/audio)
/// - NFT (tokenized content)
/// - Thread (multi-post)
/// 
/// State Flow: Active → Edited → Deleted (soft) → Archived

pub mod constants {
    /// Seeds
    pub const REGISTRY_CONFIG_SEED: &[u8] = b"registry-config";
    pub const CONTENT_RECORD_SEED: &[u8] = b"content-record";
    pub const USER_ENERGY_SEED: &[u8] = b"user-energy";
    pub const RATE_LIMIT_SEED: &[u8] = b"rate-limit";
    pub const ENERGY_CONFIG_SEED: &[u8] = b"energy-config";
    
    /// Energy costs by action
    pub const ENERGY_COST_TEXT_POST: u16 = 10;
    pub const ENERGY_COST_IMAGE_POST: u16 = 20;
    pub const ENERGY_COST_VIDEO_POST: u16 = 50;
    pub const ENERGY_COST_THREAD: u16 = 40;
    pub const ENERGY_COST_REPLY: u16 = 5;
    pub const ENERGY_COST_REPOST: u16 = 8;
    pub const ENERGY_COST_EDIT_AFTER_1H: u16 = 5;
    
    /// Energy regen rate per hour by tier
    pub const REGEN_RATE_NONE: u16 = 20;
    pub const REGEN_RATE_BRONZE: u16 = 50;
    pub const REGEN_RATE_SILVER: u16 = 80;
    pub const REGEN_RATE_GOLD: u16 = 120;
    pub const REGEN_RATE_PLATINUM: u16 = 200;
    
    /// Max energy by tier
    pub const MAX_ENERGY_NONE: u16 = 200;
    pub const MAX_ENERGY_BRONZE: u16 = 500;
    pub const MAX_ENERGY_SILVER: u16 = 800;
    pub const MAX_ENERGY_GOLD: u16 = 1200;
    pub const MAX_ENERGY_PLATINUM: u16 = 2000;
    
    /// Engagement thresholds for refunds
    pub const REFUND_THRESHOLD_10: u32 = 10;    // 25% refund
    pub const REFUND_THRESHOLD_50: u32 = 50;    // 50% refund
    pub const REFUND_THRESHOLD_100: u32 = 100;  // 100% refund
    pub const REFUND_THRESHOLD_1000: u32 = 1000; // 150% refund (viral)
    
    /// Timing
    pub const ENGAGEMENT_CHECK_DELAY: i64 = 24 * 60 * 60; // 24 hours
    pub const FREE_EDIT_WINDOW: i64 = 60 * 60; // 1 hour
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum ContentError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("Content registry is paused")]
        RegistryPaused,
        #[msg("Content not found")]
        ContentNotFound,
        #[msg("Content already deleted")]
        ContentAlreadyDeleted,
        #[msg("Cannot edit deleted content")]
        CannotEditDeleted,
        #[msg("Insufficient energy for this action")]
        InsufficientEnergy,
        #[msg("Daily post cap exceeded")]
        DailyCapExceeded,
        #[msg("Cooldown period not elapsed")]
        CooldownNotElapsed,
        #[msg("Invalid content type")]
        InvalidContentType,
        #[msg("Content URI too long (max 128 chars)")]
        ContentURITooLong,
        #[msg("Energy refund already claimed")]
        RefundAlreadyClaimed,
        #[msg("Engagement check period not elapsed")]
        RefundNotReady,
        #[msg("Arithmetic overflow")]
        Overflow,
    }
}

pub mod state {
    use super::*;
    use crate::constants::*;
    
    /// Content type enum
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum ContentType {
        #[default]
        Post = 0,
        Article = 1,
        Media = 2,
        NFT = 3,
        Thread = 4,
    }
    
    impl ContentType {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(ContentType::Post),
                1 => Some(ContentType::Article),
                2 => Some(ContentType::Media),
                3 => Some(ContentType::NFT),
                4 => Some(ContentType::Thread),
                _ => None,
            }
        }
        
        pub fn energy_cost(&self) -> u16 {
            match self {
                ContentType::Post => ENERGY_COST_TEXT_POST,
                ContentType::Article => ENERGY_COST_IMAGE_POST, // Same as image
                ContentType::Media => ENERGY_COST_VIDEO_POST,
                ContentType::NFT => ENERGY_COST_IMAGE_POST,
                ContentType::Thread => ENERGY_COST_THREAD,
            }
        }
    }
    
    /// Content state enum
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum ContentState {
        #[default]
        Active = 0,
        Edited = 1,
        Deleted = 2,
        Archived = 3,
    }
    
    impl ContentState {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(ContentState::Active),
                1 => Some(ContentState::Edited),
                2 => Some(ContentState::Deleted),
                3 => Some(ContentState::Archived),
                _ => None,
            }
        }
    }
    
    /// Global registry configuration
    #[account]
    #[derive(Default)]
    pub struct RegistryConfig {
        /// Admin authority
        pub authority: Pubkey,
        /// Identity protocol for verification
        pub identity_program: Pubkey,
        /// Staking program for tier lookup
        pub staking_program: Pubkey,
        /// Total content count
        pub total_content_count: u64,
        /// Total active content
        pub active_content_count: u64,
        /// Whether registry is paused
        pub paused: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl RegistryConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // identity_program
            32 + // staking_program
            8 +  // total_content_count
            8 +  // active_content_count
            1 +  // paused
            1;   // bump
    }
    
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
    
    /// Individual content record
    #[account]
    pub struct ContentRecord {
        /// Unique tracking ID (hash)
        pub tracking_id: [u8; 32],
        /// Content author
        pub author: Pubkey,
        /// SHA256 of current content
        pub content_hash: [u8; 32],
        /// IPFS/Arweave CID (max 128)
        pub content_uri: [u8; 128],
        /// URI length
        pub uri_len: u8,
        /// Content type
        pub content_type: u8,
        /// Current state
        pub state: u8,
        /// Version (edit count)
        pub version: u16,
        /// Original creation timestamp
        pub created_at: i64,
        /// Last state change
        pub updated_at: i64,
        /// Hash before last edit (for history)
        pub previous_hash: [u8; 32],
        /// Energy spent on creation
        pub energy_spent: u16,
        /// Whether energy refund was claimed
        pub refund_claimed: bool,
        /// Engagement count (likes) for refund calculation
        pub engagement_count: u32,
        /// PDA bump
        pub bump: u8,
    }
    
    impl Default for ContentRecord {
        fn default() -> Self {
            Self {
                tracking_id: [0u8; 32],
                author: Pubkey::default(),
                content_hash: [0u8; 32],
                content_uri: [0u8; 128],
                uri_len: 0,
                content_type: 0,
                state: 0,
                version: 0,
                created_at: 0,
                updated_at: 0,
                previous_hash: [0u8; 32],
                energy_spent: 0,
                refund_claimed: false,
                engagement_count: 0,
                bump: 0,
            }
        }
    }
    
    impl ContentRecord {
        pub const LEN: usize = 8 + // discriminator
            32 + // tracking_id
            32 + // author
            32 + // content_hash
            128 + // content_uri
            1 +  // uri_len
            1 +  // content_type
            1 +  // state
            2 +  // version
            8 +  // created_at
            8 +  // updated_at
            32 + // previous_hash
            2 +  // energy_spent
            1 +  // refund_claimed
            4 +  // engagement_count
            1;   // bump
    }
    
    /// User energy account
    #[account]
    #[derive(Default)]
    pub struct UserEnergy {
        /// User wallet
        pub user: Pubkey,
        /// Current energy (scales with tier)
        pub current_energy: u16,
        /// Max energy (tier-based)
        pub max_energy: u16,
        /// Last regeneration time
        pub last_regen_time: i64,
        /// Regen rate per hour
        pub regen_rate: u16,
        /// Energy spent today
        pub energy_spent_today: u32,
        /// Energy refunded today
        pub energy_refunded_today: u32,
        /// Last daily reset
        pub last_reset: i64,
        /// User's staking tier (0-4)
        pub tier: u8,
        /// PDA bump
        pub bump: u8,
    }
    
    impl UserEnergy {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            2 +  // current_energy
            2 +  // max_energy
            8 +  // last_regen_time
            2 +  // regen_rate
            4 +  // energy_spent_today
            4 +  // energy_refunded_today
            8 +  // last_reset
            1 +  // tier
            1;   // bump
        
        pub fn max_energy_for_tier(tier: u8) -> u16 {
            match tier {
                0 => MAX_ENERGY_NONE,
                1 => MAX_ENERGY_BRONZE,
                2 => MAX_ENERGY_SILVER,
                3 => MAX_ENERGY_GOLD,
                4 => MAX_ENERGY_PLATINUM,
                _ => MAX_ENERGY_NONE,
            }
        }
        
        pub fn regen_rate_for_tier(tier: u8) -> u16 {
            match tier {
                0 => REGEN_RATE_NONE,
                1 => REGEN_RATE_BRONZE,
                2 => REGEN_RATE_SILVER,
                3 => REGEN_RATE_GOLD,
                4 => REGEN_RATE_PLATINUM,
                _ => REGEN_RATE_NONE,
            }
        }
    }
    
    /// Rate limit account per user
    #[account]
    #[derive(Default)]
    pub struct UserRateLimit {
        /// User wallet
        pub user: Pubkey,
        /// Posts today
        pub posts_today: u16,
        /// Edits this hour
        pub edits_this_hour: u8,
        /// Last post time
        pub last_post_time: i64,
        /// Day reset time
        pub day_reset_time: i64,
        /// Hour reset time
        pub hour_reset_time: i64,
        /// PDA bump
        pub bump: u8,
    }
    
    impl UserRateLimit {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            2 +  // posts_today
            1 +  // edits_this_hour
            8 +  // last_post_time
            8 +  // day_reset_time
            8 +  // hour_reset_time
            1;   // bump
        
        /// Get daily cap based on tier
        pub fn daily_cap_for_tier(tier: u8) -> u16 {
            match tier {
                0 => 50,
                1 => 100,
                2 => 200,
                3 => 400,
                4 => 1000,
                _ => 50,
            }
        }
    }
}

pub mod events {
    use super::*;
    
    #[event]
    pub struct ContentCreated {
        pub tracking_id: [u8; 32],
        pub author: Pubkey,
        pub content_type: u8,
        pub content_hash: [u8; 32],
        pub timestamp: i64,
    }
    
    #[event]
    pub struct ContentEdited {
        pub tracking_id: [u8; 32],
        pub author: Pubkey,
        pub version: u16,
        pub new_hash: [u8; 32],
        pub timestamp: i64,
    }
    
    #[event]
    pub struct ContentDeleted {
        pub tracking_id: [u8; 32],
        pub author: Pubkey,
        pub timestamp: i64,
    }
    
    #[event]
    pub struct EnergySpent {
        pub user: Pubkey,
        pub amount: u16,
        pub action: String,
        pub remaining: u16,
    }
    
    #[event]
    pub struct EnergyRefunded {
        pub user: Pubkey,
        pub content_id: [u8; 32],
        pub refund_amount: u16,
        pub engagement_count: u32,
    }
}

use constants::*;
use errors::*;
use state::*;
use events::*;

#[program]
pub mod content_registry {
    use super::*;

    /// Initialize the content registry
    pub fn initialize(
        ctx: Context<Initialize>,
        identity_program: Pubkey,
        staking_program: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.registry_config;
        
        config.authority = ctx.accounts.authority.key();
        config.identity_program = identity_program;
        config.staking_program = staking_program;
        config.total_content_count = 0;
        config.active_content_count = 0;
        config.paused = false;
        config.bump = ctx.bumps.registry_config;
        
        msg!("Content registry initialized");
        Ok(())
    }
    
    /// Initialize energy system config
    pub fn initialize_energy(
        ctx: Context<InitializeEnergy>,
    ) -> Result<()> {
        let energy_config = &mut ctx.accounts.energy_config;
        
        energy_config.authority = ctx.accounts.authority.key();
        energy_config.base_regen_rate = REGEN_RATE_NONE;
        energy_config.engagement_check_delay = ENGAGEMENT_CHECK_DELAY;
        energy_config.viral_threshold = REFUND_THRESHOLD_1000;
        energy_config.paused = false;
        energy_config.bump = ctx.bumps.energy_config;
        
        msg!("Energy system initialized");
        Ok(())
    }
    
    /// Create content record
    pub fn create_content(
        ctx: Context<CreateContent>,
        tracking_id: [u8; 32],
        content_hash: [u8; 32],
        content_uri: String,
        content_type: u8,
    ) -> Result<()> {
        require!(!ctx.accounts.registry_config.paused, ContentError::RegistryPaused);
        require!(content_uri.len() <= 128, ContentError::ContentURITooLong);
        
        let content_type_enum = ContentType::from_u8(content_type)
            .ok_or(ContentError::InvalidContentType)?;
        
        let clock = Clock::get()?;
        
        // Check and spend energy
        let user_energy = &mut ctx.accounts.user_energy;
        let energy_cost = content_type_enum.energy_cost();
        
        // Regenerate energy first
        regenerate_energy(user_energy, clock.unix_timestamp)?;
        
        require!(
            user_energy.current_energy >= energy_cost,
            ContentError::InsufficientEnergy
        );
        
        // Check rate limit
        let rate_limit = &mut ctx.accounts.rate_limit;
        check_and_update_rate_limit(rate_limit, user_energy.tier, clock.unix_timestamp)?;
        
        // Spend energy
        user_energy.current_energy = user_energy.current_energy.saturating_sub(energy_cost);
        user_energy.energy_spent_today = user_energy.energy_spent_today.saturating_add(energy_cost as u32);
        
        // Create content record
        let content = &mut ctx.accounts.content_record;
        content.tracking_id = tracking_id;
        content.author = ctx.accounts.author.key();
        content.content_hash = content_hash;
        
        let uri_bytes = content_uri.as_bytes();
        content.content_uri[..uri_bytes.len()].copy_from_slice(uri_bytes);
        content.uri_len = uri_bytes.len() as u8;
        
        content.content_type = content_type;
        content.state = ContentState::Active as u8;
        content.version = 1;
        content.created_at = clock.unix_timestamp;
        content.updated_at = clock.unix_timestamp;
        content.previous_hash = [0u8; 32];
        content.energy_spent = energy_cost;
        content.refund_claimed = false;
        content.engagement_count = 0;
        content.bump = ctx.bumps.content_record;
        
        // Update registry stats
        let config = &mut ctx.accounts.registry_config;
        config.total_content_count = config.total_content_count.saturating_add(1);
        config.active_content_count = config.active_content_count.saturating_add(1);
        
        emit!(ContentCreated {
            tracking_id,
            author: content.author,
            content_type,
            content_hash,
            timestamp: clock.unix_timestamp,
        });
        
        emit!(EnergySpent {
            user: content.author,
            amount: energy_cost,
            action: "create_content".to_string(),
            remaining: user_energy.current_energy,
        });
        
        msg!("Content created: {:?}", tracking_id);
        Ok(())
    }
    
    /// Edit content (update hash and URI)
    pub fn edit_content(
        ctx: Context<EditContent>,
        new_content_hash: [u8; 32],
        new_content_uri: String,
    ) -> Result<()> {
        let content = &mut ctx.accounts.content_record;
        
        require!(
            content.state != ContentState::Deleted as u8,
            ContentError::CannotEditDeleted
        );
        require!(new_content_uri.len() <= 128, ContentError::ContentURITooLong);
        
        let clock = Clock::get()?;
        let time_since_creation = clock.unix_timestamp - content.created_at;
        
        // Free edits within 1 hour of creation
        if time_since_creation > FREE_EDIT_WINDOW {
            let user_energy = &mut ctx.accounts.user_energy;
            regenerate_energy(user_energy, clock.unix_timestamp)?;
            
            require!(
                user_energy.current_energy >= ENERGY_COST_EDIT_AFTER_1H,
                ContentError::InsufficientEnergy
            );
            
            user_energy.current_energy = user_energy.current_energy.saturating_sub(ENERGY_COST_EDIT_AFTER_1H);
        }
        
        // Store previous hash for history
        content.previous_hash = content.content_hash;
        content.content_hash = new_content_hash;
        
        let uri_bytes = new_content_uri.as_bytes();
        content.content_uri = [0u8; 128];
        content.content_uri[..uri_bytes.len()].copy_from_slice(uri_bytes);
        content.uri_len = uri_bytes.len() as u8;
        
        content.state = ContentState::Edited as u8;
        content.version = content.version.saturating_add(1);
        content.updated_at = clock.unix_timestamp;
        
        emit!(ContentEdited {
            tracking_id: content.tracking_id,
            author: content.author,
            version: content.version,
            new_hash: new_content_hash,
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Content edited: version {}", content.version);
        Ok(())
    }
    
    /// Soft delete content
    pub fn delete_content(ctx: Context<DeleteContent>) -> Result<()> {
        let content = &mut ctx.accounts.content_record;
        
        require!(
            content.state != ContentState::Deleted as u8,
            ContentError::ContentAlreadyDeleted
        );
        
        let clock = Clock::get()?;
        
        content.state = ContentState::Deleted as u8;
        content.updated_at = clock.unix_timestamp;
        
        // Update registry stats
        let config = &mut ctx.accounts.registry_config;
        config.active_content_count = config.active_content_count.saturating_sub(1);
        
        emit!(ContentDeleted {
            tracking_id: content.tracking_id,
            author: content.author,
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Content deleted");
        Ok(())
    }
    
    /// Update engagement count (oracle/backend call)
    pub fn update_engagement(
        ctx: Context<UpdateEngagement>,
        engagement_count: u32,
    ) -> Result<()> {
        let content = &mut ctx.accounts.content_record;
        content.engagement_count = engagement_count;
        
        msg!("Engagement updated: {}", engagement_count);
        Ok(())
    }
    
    /// Claim energy refund based on engagement
    pub fn claim_energy_refund(ctx: Context<ClaimRefund>) -> Result<()> {
        let content = &mut ctx.accounts.content_record;
        
        require!(!content.refund_claimed, ContentError::RefundAlreadyClaimed);
        
        let clock = Clock::get()?;
        let elapsed = clock.unix_timestamp - content.created_at;
        
        require!(
            elapsed >= ENGAGEMENT_CHECK_DELAY,
            ContentError::RefundNotReady
        );
        
        // Calculate refund based on engagement
        let refund_pct = if content.engagement_count >= REFUND_THRESHOLD_1000 {
            150 // 150% (bonus energy!)
        } else if content.engagement_count >= REFUND_THRESHOLD_100 {
            100 // 100%
        } else if content.engagement_count >= REFUND_THRESHOLD_50 {
            50 // 50%
        } else if content.engagement_count >= REFUND_THRESHOLD_10 {
            25 // 25%
        } else {
            0 // No refund
        };
        
        if refund_pct > 0 {
            let refund_amount = ((content.energy_spent as u32 * refund_pct) / 100) as u16;
            
            let user_energy = &mut ctx.accounts.user_energy;
            user_energy.current_energy = user_energy.current_energy
                .saturating_add(refund_amount)
                .min(user_energy.max_energy);
            user_energy.energy_refunded_today = user_energy.energy_refunded_today
                .saturating_add(refund_amount as u32);
            
            emit!(EnergyRefunded {
                user: user_energy.user,
                content_id: content.tracking_id,
                refund_amount,
                engagement_count: content.engagement_count,
            });
            
            msg!("Energy refunded: {} ({}%)", refund_amount, refund_pct);
        }
        
        content.refund_claimed = true;
        
        Ok(())
    }
    
    /// Initialize user energy account
    pub fn initialize_user_energy(
        ctx: Context<InitializeUserEnergy>,
        tier: u8,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let user_energy = &mut ctx.accounts.user_energy;
        
        user_energy.user = ctx.accounts.user.key();
        user_energy.tier = tier;
        user_energy.max_energy = UserEnergy::max_energy_for_tier(tier);
        user_energy.regen_rate = UserEnergy::regen_rate_for_tier(tier);
        user_energy.current_energy = user_energy.max_energy; // Start full
        user_energy.last_regen_time = clock.unix_timestamp;
        user_energy.energy_spent_today = 0;
        user_energy.energy_refunded_today = 0;
        user_energy.last_reset = clock.unix_timestamp;
        user_energy.bump = ctx.bumps.user_energy;
        
        msg!("User energy initialized: tier {}, max {}", tier, user_energy.max_energy);
        Ok(())
    }
    
    /// Update user tier (from staking program)
    pub fn update_user_tier(
        ctx: Context<UpdateUserTier>,
        new_tier: u8,
    ) -> Result<()> {
        let user_energy = &mut ctx.accounts.user_energy;
        
        user_energy.tier = new_tier;
        user_energy.max_energy = UserEnergy::max_energy_for_tier(new_tier);
        user_energy.regen_rate = UserEnergy::regen_rate_for_tier(new_tier);
        
        // Cap current energy at new max
        if user_energy.current_energy > user_energy.max_energy {
            user_energy.current_energy = user_energy.max_energy;
        }
        
        msg!("User tier updated: {}", new_tier);
        Ok(())
    }
    
    /// Pause/unpause registry
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        ctx.accounts.registry_config.paused = paused;
        msg!("Content registry paused: {}", paused);
        Ok(())
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.registry_config.authority = new_authority;
        msg!("Authority updated to: {}", new_authority);
        Ok(())
    }
    
    /// Get content info
    pub fn get_content(ctx: Context<GetContent>) -> Result<()> {
        let content = &ctx.accounts.content_record;
        msg!("Tracking ID: {:?}", content.tracking_id);
        msg!("Author: {}", content.author);
        msg!("Type: {}", content.content_type);
        msg!("State: {}", content.state);
        msg!("Version: {}", content.version);
        Ok(())
    }
    
    /// Get user energy stats
    pub fn get_energy(ctx: Context<GetEnergy>) -> Result<()> {
        let energy = &ctx.accounts.user_energy;
        msg!("User: {}", energy.user);
        msg!("Current: {}/{}", energy.current_energy, energy.max_energy);
        msg!("Regen rate: {}/hr", energy.regen_rate);
        msg!("Tier: {}", energy.tier);
        Ok(())
    }
}

// Helper functions

fn regenerate_energy(energy: &mut UserEnergy, current_time: i64) -> Result<()> {
    let elapsed_seconds = current_time - energy.last_regen_time;
    if elapsed_seconds > 0 {
        let hours_elapsed = elapsed_seconds as f64 / 3600.0;
        let regen_amount = (hours_elapsed * energy.regen_rate as f64) as u16;
        
        energy.current_energy = energy.current_energy
            .saturating_add(regen_amount)
            .min(energy.max_energy);
        energy.last_regen_time = current_time;
    }
    
    // Reset daily counters if new day
    if current_time >= energy.last_reset + 86400 {
        energy.energy_spent_today = 0;
        energy.energy_refunded_today = 0;
        energy.last_reset = current_time;
    }
    
    Ok(())
}

fn check_and_update_rate_limit(
    rate_limit: &mut UserRateLimit,
    tier: u8,
    current_time: i64,
) -> Result<()> {
    // Reset daily counter if new day
    if current_time >= rate_limit.day_reset_time + 86400 {
        rate_limit.posts_today = 0;
        rate_limit.day_reset_time = current_time;
    }
    
    // Reset hourly counter if new hour
    if current_time >= rate_limit.hour_reset_time + 3600 {
        rate_limit.edits_this_hour = 0;
        rate_limit.hour_reset_time = current_time;
    }
    
    // Check daily cap
    let daily_cap = UserRateLimit::daily_cap_for_tier(tier);
    require!(
        rate_limit.posts_today < daily_cap,
        ContentError::DailyCapExceeded
    );
    
    rate_limit.posts_today += 1;
    rate_limit.last_post_time = current_time;
    
    Ok(())
}

// Account contexts

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = RegistryConfig::LEN,
        seeds = [REGISTRY_CONFIG_SEED],
        bump
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeEnergy<'info> {
    #[account(
        init,
        payer = authority,
        space = EnergyConfig::LEN,
        seeds = [ENERGY_CONFIG_SEED],
        bump
    )]
    pub energy_config: Account<'info, EnergyConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(tracking_id: [u8; 32])]
pub struct CreateContent<'info> {
    #[account(
        mut,
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    #[account(
        init,
        payer = author,
        space = ContentRecord::LEN,
        seeds = [CONTENT_RECORD_SEED, tracking_id.as_ref()],
        bump
    )]
    pub content_record: Account<'info, ContentRecord>,
    
    #[account(
        mut,
        seeds = [USER_ENERGY_SEED, author.key().as_ref()],
        bump = user_energy.bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
    
    #[account(
        init_if_needed,
        payer = author,
        space = UserRateLimit::LEN,
        seeds = [RATE_LIMIT_SEED, author.key().as_ref()],
        bump
    )]
    pub rate_limit: Account<'info, UserRateLimit>,
    
    #[account(mut)]
    pub author: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditContent<'info> {
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, content_record.tracking_id.as_ref()],
        bump = content_record.bump,
        has_one = author
    )]
    pub content_record: Account<'info, ContentRecord>,
    
    #[account(
        mut,
        seeds = [USER_ENERGY_SEED, author.key().as_ref()],
        bump = user_energy.bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
    
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteContent<'info> {
    #[account(
        mut,
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, content_record.tracking_id.as_ref()],
        bump = content_record.bump,
        has_one = author
    )]
    pub content_record: Account<'info, ContentRecord>,
    
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateEngagement<'info> {
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, content_record.tracking_id.as_ref()],
        bump = content_record.bump
    )]
    pub content_record: Account<'info, ContentRecord>,
    
    #[account(
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump,
        has_one = authority @ ContentError::Unauthorized
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimRefund<'info> {
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, content_record.tracking_id.as_ref()],
        bump = content_record.bump,
        has_one = author
    )]
    pub content_record: Account<'info, ContentRecord>,
    
    #[account(
        mut,
        seeds = [USER_ENERGY_SEED, author.key().as_ref()],
        bump = user_energy.bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
    
    pub author: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitializeUserEnergy<'info> {
    #[account(
        init,
        payer = user,
        space = UserEnergy::LEN,
        seeds = [USER_ENERGY_SEED, user.key().as_ref()],
        bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUserTier<'info> {
    #[account(
        mut,
        seeds = [USER_ENERGY_SEED, user_energy.user.as_ref()],
        bump = user_energy.bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
    
    #[account(
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump,
        has_one = authority @ ContentError::Unauthorized
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump,
        has_one = authority @ ContentError::Unauthorized
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [REGISTRY_CONFIG_SEED],
        bump = registry_config.bump,
        has_one = authority @ ContentError::Unauthorized
    )]
    pub registry_config: Account<'info, RegistryConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetContent<'info> {
    #[account(
        seeds = [CONTENT_RECORD_SEED, content_record.tracking_id.as_ref()],
        bump = content_record.bump
    )]
    pub content_record: Account<'info, ContentRecord>,
}

#[derive(Accounts)]
pub struct GetEnergy<'info> {
    #[account(
        seeds = [USER_ENERGY_SEED, user_energy.user.as_ref()],
        bump = user_energy.bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
}


