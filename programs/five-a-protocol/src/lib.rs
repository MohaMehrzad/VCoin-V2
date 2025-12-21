use anchor_lang::prelude::*;

declare_id!("EPVUXY5NSTxWRGU4JF3zowtc5wB6HE9aUwFHG61W9CCH");

/// 5A Reputation Protocol
/// 
/// Anti-bot reputation scoring with oracle model and periodic snapshots.
/// 
/// The 5 Stars (Open Source - MIT Licensed - Public Good):
/// - A1: Authenticity (25%) - "Are you a real person?"
/// - A2: Accuracy (20%) - "Is your content quality?"
/// - A3: Agility (15%) - "Are you fast?"
/// - A4: Activity (25%) - "Do you show up daily?"
/// - A5: Approved (15%) - "Does the community like you?"
/// 
/// Score Range: 0-100 (stored as 0-10000 for precision)

pub mod constants {
    /// Seeds
    pub const FIVE_A_CONFIG_SEED: &[u8] = b"five-a-config";
    pub const USER_SCORE_SEED: &[u8] = b"user-score";
    pub const SCORE_SNAPSHOT_SEED: &[u8] = b"score-snapshot";
    pub const VOUCH_RECORD_SEED: &[u8] = b"vouch-record";
    pub const VOUCH_STATUS_SEED: &[u8] = b"vouch-status";
    pub const VOUCHER_STATS_SEED: &[u8] = b"voucher-stats";
    pub const ORACLE_SEED: &[u8] = b"oracle";
    
    /// Score weights (out of 10000)
    pub const AUTHENTICITY_WEIGHT: u16 = 2500;  // 25%
    pub const ACCURACY_WEIGHT: u16 = 2000;      // 20%
    pub const AGILITY_WEIGHT: u16 = 1500;       // 15%
    pub const ACTIVITY_WEIGHT: u16 = 2500;      // 25%
    pub const APPROVED_WEIGHT: u16 = 1500;      // 15%
    
    /// Vouch system
    pub const MIN_VOUCHER_SCORE: u16 = 6000;    // 60% 5A score to vouch
    pub const VOUCH_STAKE_AMOUNT: u64 = 5_000_000_000; // 5 VCoin
    pub const VOUCHES_REQUIRED: u8 = 3;
    pub const VOUCH_EVALUATION_PERIOD: i64 = 90 * 24 * 60 * 60; // 90 days
    pub const VOUCH_REWARD: u64 = 10_000_000_000; // 10 VCoin bonus for successful vouch
    
    /// Score update intervals
    pub const SNAPSHOT_INTERVAL: i64 = 24 * 60 * 60; // Daily snapshots
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum FiveAError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("Protocol is paused")]
        ProtocolPaused,
        #[msg("Caller is not a registered oracle")]
        NotOracle,
        #[msg("Invalid score value (must be 0-10000)")]
        InvalidScore,
        #[msg("Voucher 5A score too low (need 60%+)")]
        VoucherScoreTooLow,
        #[msg("User already has 3 vouches")]
        AlreadyFullyVouched,
        #[msg("Cannot vouch for self")]
        CannotVouchSelf,
        #[msg("Already vouched for this user")]
        AlreadyVouched,
        #[msg("Max concurrent vouches reached")]
        MaxVouchesReached,
        #[msg("Vouch stake amount incorrect")]
        InvalidStakeAmount,
        #[msg("Vouch evaluation period not complete")]
        EvaluationNotComplete,
        #[msg("Vouch already evaluated")]
        AlreadyEvaluated,
        #[msg("User not found")]
        UserNotFound,
        #[msg("Oracle already registered")]
        OracleAlreadyRegistered,
        #[msg("Maximum oracles reached")]
        MaxOraclesReached,
        #[msg("Arithmetic overflow")]
        Overflow,
    }
}

pub mod state {
    use super::*;
    use crate::constants::*;
    
    /// Global 5A protocol configuration
    #[account]
    #[derive(Default)]
    pub struct FiveAConfig {
        /// Admin authority
        pub authority: Pubkey,
        /// Identity protocol program
        pub identity_program: Pubkey,
        /// VCoin mint for vouch stakes
        pub vcoin_mint: Pubkey,
        /// Vouch stake vault
        pub vouch_vault: Pubkey,
        /// Registered oracles (max 10)
        pub oracles: [Pubkey; 10],
        /// Number of active oracles
        pub oracle_count: u8,
        /// Required consensus (e.g., 5 of 7)
        pub required_consensus: u8,
        /// Total users with scores
        pub total_users: u64,
        /// Current snapshot epoch
        pub current_epoch: u64,
        /// Last snapshot timestamp
        pub last_snapshot_time: i64,
        /// Whether protocol is paused
        pub paused: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl FiveAConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // identity_program
            32 + // vcoin_mint
            32 + // vouch_vault
            (32 * 10) + // oracles
            1 +  // oracle_count
            1 +  // required_consensus
            8 +  // total_users
            8 +  // current_epoch
            8 +  // last_snapshot_time
            1 +  // paused
            1;   // bump
    }
    
    /// Individual user's 5A score
    #[account]
    #[derive(Default)]
    pub struct UserScore {
        /// User wallet
        pub user: Pubkey,
        /// Authenticity score (0-10000)
        pub authenticity: u16,
        /// Accuracy score (0-10000)
        pub accuracy: u16,
        /// Agility score (0-10000)
        pub agility: u16,
        /// Activity score (0-10000)
        pub activity: u16,
        /// Approved score (0-10000)
        pub approved: u16,
        /// Weighted composite score (0-10000)
        pub composite_score: u16,
        /// Score last updated
        pub last_updated: i64,
        /// Last snapshot epoch this user was included in
        pub last_snapshot_epoch: u64,
        /// Number of score updates
        pub update_count: u32,
        /// Whether user has private score mode enabled
        pub is_private: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl UserScore {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            2 +  // authenticity
            2 +  // accuracy
            2 +  // agility
            2 +  // activity
            2 +  // approved
            2 +  // composite_score
            8 +  // last_updated
            8 +  // last_snapshot_epoch
            4 +  // update_count
            1 +  // is_private
            1;   // bump
        
        /// Calculate weighted composite score
        pub fn calculate_composite(&self) -> u16 {
            let weighted = 
                (self.authenticity as u32 * AUTHENTICITY_WEIGHT as u32 +
                 self.accuracy as u32 * ACCURACY_WEIGHT as u32 +
                 self.agility as u32 * AGILITY_WEIGHT as u32 +
                 self.activity as u32 * ACTIVITY_WEIGHT as u32 +
                 self.approved as u32 * APPROVED_WEIGHT as u32) / 10000;
            
            weighted as u16
        }
    }
    
    /// Periodic score snapshot for epoch
    #[account]
    #[derive(Default)]
    pub struct ScoreSnapshot {
        /// Epoch number
        pub epoch: u64,
        /// Merkle root of all scores in this epoch
        pub merkle_root: [u8; 32],
        /// Total users in snapshot
        pub user_count: u64,
        /// Average composite score
        pub avg_score: u16,
        /// Snapshot timestamp
        pub timestamp: i64,
        /// Oracle that submitted snapshot
        pub submitter: Pubkey,
        /// PDA bump
        pub bump: u8,
    }
    
    impl ScoreSnapshot {
        pub const LEN: usize = 8 + // discriminator
            8 +  // epoch
            32 + // merkle_root
            8 +  // user_count
            2 +  // avg_score
            8 +  // timestamp
            32 + // submitter
            1;   // bump
    }
    
    /// Vouch record (PDA per vouch)
    #[account]
    #[derive(Default)]
    pub struct VouchRecord {
        /// Voucher (must have 60%+ 5A)
        pub voucher: Pubkey,
        /// Vouchee (new user)
        pub vouchee: Pubkey,
        /// Timestamp of vouch
        pub vouched_at: i64,
        /// VCoin staked (5 VCoin)
        pub vouch_stake: u64,
        /// Status: 0=Active, 1=Revoked, 2=Slashed, 3=Rewarded
        pub status: u8,
        /// Whether outcome has been evaluated
        pub outcome_evaluated: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl VouchRecord {
        pub const LEN: usize = 8 + // discriminator
            32 + // voucher
            32 + // vouchee
            8 +  // vouched_at
            8 +  // vouch_stake
            1 +  // status
            1 +  // outcome_evaluated
            1;   // bump
    }
    
    /// Vouch status enum
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum VouchStatus {
        #[default]
        Active = 0,
        Revoked = 1,
        Slashed = 2,
        Rewarded = 3,
    }
    
    /// User's vouch status (how many vouches received)
    #[account]
    #[derive(Default)]
    pub struct UserVouchStatus {
        /// User wallet
        pub user: Pubkey,
        /// Number of vouches received (0-3)
        pub vouches_received: u8,
        /// Who vouched (max 3)
        pub vouchers: [Pubkey; 3],
        /// Timestamp when 3 vouches received
        pub vouch_completed_at: i64,
        /// Reward multiplier (0-10000 = 0-100%)
        pub reward_multiplier: u16,
        /// Whether fully vouched
        pub is_fully_vouched: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl UserVouchStatus {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            1 +  // vouches_received
            (32 * 3) + // vouchers
            8 +  // vouch_completed_at
            2 +  // reward_multiplier
            1 +  // is_fully_vouched
            1;   // bump
        
        /// Get reward multiplier based on vouch count
        pub fn get_multiplier(&self) -> u16 {
            match self.vouches_received {
                0 => 1000,  // 10%
                1 => 4000,  // 40%
                2 => 7000,  // 70%
                _ => 10000, // 100%
            }
        }
    }
    
    /// Voucher statistics
    #[account]
    #[derive(Default)]
    pub struct VoucherStats {
        /// Voucher wallet
        pub user: Pubkey,
        /// Total vouches given
        pub total_vouches_given: u32,
        /// Successful vouches (vouchee reached 50%+)
        pub successful_vouches: u32,
        /// Failed vouches (vouchee banned/inactive)
        pub failed_vouches: u32,
        /// Vouch accuracy (0-10000)
        pub vouch_accuracy: u16,
        /// Current active vouches
        pub vouches_active: u8,
        /// Max concurrent vouches (based on 5A score)
        pub max_concurrent_vouches: u8,
        /// Total rewards earned
        pub total_rewards_earned: u64,
        /// Total stake lost
        pub total_stake_lost: u64,
        /// PDA bump
        pub bump: u8,
    }
    
    impl VoucherStats {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            4 +  // total_vouches_given
            4 +  // successful_vouches
            4 +  // failed_vouches
            2 +  // vouch_accuracy
            1 +  // vouches_active
            1 +  // max_concurrent_vouches
            8 +  // total_rewards_earned
            8 +  // total_stake_lost
            1;   // bump
        
        /// Calculate max vouches based on 5A score
        pub fn max_vouches_for_score(score: u16) -> u8 {
            if score >= 9000 { 10 }
            else if score >= 8000 { 8 }
            else if score >= 7000 { 5 }
            else if score >= 6000 { 3 }
            else { 0 }
        }
    }
    
    /// Registered oracle
    #[account]
    #[derive(Default)]
    pub struct Oracle {
        /// Oracle wallet
        pub wallet: Pubkey,
        /// Oracle name (max 32 chars)
        pub name: [u8; 32],
        /// Whether oracle is active
        pub is_active: bool,
        /// Total score submissions
        pub total_submissions: u64,
        /// Accuracy rate (0-10000)
        pub accuracy_rate: u16,
        /// Last submission timestamp
        pub last_submission: i64,
        /// PDA bump
        pub bump: u8,
    }
    
    impl Oracle {
        pub const LEN: usize = 8 + // discriminator
            32 + // wallet
            32 + // name
            1 +  // is_active
            8 +  // total_submissions
            2 +  // accuracy_rate
            8 +  // last_submission
            1;   // bump
    }
}

pub mod events {
    use super::*;
    
    #[event]
    pub struct ScoreUpdated {
        pub user: Pubkey,
        pub authenticity: u16,
        pub accuracy: u16,
        pub agility: u16,
        pub activity: u16,
        pub approved: u16,
        pub composite: u16,
        pub timestamp: i64,
    }
    
    #[event]
    pub struct SnapshotCreated {
        pub epoch: u64,
        pub merkle_root: [u8; 32],
        pub user_count: u64,
        pub timestamp: i64,
    }
    
    #[event]
    pub struct VouchCreated {
        pub voucher: Pubkey,
        pub vouchee: Pubkey,
        pub stake: u64,
        pub timestamp: i64,
    }
    
    #[event]
    pub struct VouchEvaluated {
        pub voucher: Pubkey,
        pub vouchee: Pubkey,
        pub success: bool,
        pub reward_or_slash: u64,
    }
}

use constants::*;
use errors::*;
use state::*;
use events::*;

#[program]
pub mod five_a_protocol {
    use super::*;

    /// Initialize the 5A protocol
    pub fn initialize(
        ctx: Context<Initialize>,
        identity_program: Pubkey,
        vcoin_mint: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.five_a_config;
        
        config.authority = ctx.accounts.authority.key();
        config.identity_program = identity_program;
        config.vcoin_mint = vcoin_mint;
        config.vouch_vault = ctx.accounts.vouch_vault.key();
        config.oracles = [Pubkey::default(); 10];
        config.oracle_count = 0;
        config.required_consensus = 1; // Start with single oracle, increase later
        config.total_users = 0;
        config.current_epoch = 0;
        config.last_snapshot_time = 0;
        config.paused = false;
        config.bump = ctx.bumps.five_a_config;
        
        msg!("5A Protocol initialized");
        Ok(())
    }
    
    /// Register an oracle
    pub fn register_oracle(
        ctx: Context<RegisterOracle>,
        name: String,
    ) -> Result<()> {
        let config = &mut ctx.accounts.five_a_config;
        
        require!(config.oracle_count < 10, FiveAError::MaxOraclesReached);
        
        // Check if already registered
        let oracle_key = ctx.accounts.oracle_wallet.key();
        for i in 0..config.oracle_count as usize {
            require!(
                config.oracles[i] != oracle_key,
                FiveAError::OracleAlreadyRegistered
            );
        }
        
        // Add to config
        let idx = config.oracle_count as usize;
        config.oracles[idx] = oracle_key;
        config.oracle_count += 1;
        
        // Initialize oracle account
        let oracle = &mut ctx.accounts.oracle;
        oracle.wallet = oracle_key;
        
        let name_bytes = name.as_bytes();
        let len = name_bytes.len().min(32);
        oracle.name[..len].copy_from_slice(&name_bytes[..len]);
        
        oracle.is_active = true;
        oracle.total_submissions = 0;
        oracle.accuracy_rate = 10000; // Start at 100%
        oracle.last_submission = 0;
        oracle.bump = ctx.bumps.oracle;
        
        msg!("Oracle registered: {}", oracle_key);
        Ok(())
    }
    
    /// Submit score update (oracle only)
    pub fn submit_score(
        ctx: Context<SubmitScore>,
        authenticity: u16,
        accuracy: u16,
        agility: u16,
        activity: u16,
        approved: u16,
    ) -> Result<()> {
        let config = &ctx.accounts.five_a_config;
        require!(!config.paused, FiveAError::ProtocolPaused);
        
        // Validate scores
        require!(authenticity <= 10000, FiveAError::InvalidScore);
        require!(accuracy <= 10000, FiveAError::InvalidScore);
        require!(agility <= 10000, FiveAError::InvalidScore);
        require!(activity <= 10000, FiveAError::InvalidScore);
        require!(approved <= 10000, FiveAError::InvalidScore);
        
        // Verify oracle is registered
        let oracle_key = ctx.accounts.oracle.key();
        let is_oracle = config.oracles[..config.oracle_count as usize]
            .contains(&oracle_key);
        require!(is_oracle, FiveAError::NotOracle);
        
        let clock = Clock::get()?;
        let user_score = &mut ctx.accounts.user_score;
        
        // Initialize if new
        if user_score.user == Pubkey::default() {
            user_score.user = ctx.accounts.user.key();
            let config = &mut ctx.accounts.five_a_config;
            config.total_users = config.total_users.saturating_add(1);
        }
        
        // Update scores
        user_score.authenticity = authenticity;
        user_score.accuracy = accuracy;
        user_score.agility = agility;
        user_score.activity = activity;
        user_score.approved = approved;
        user_score.composite_score = user_score.calculate_composite();
        user_score.last_updated = clock.unix_timestamp;
        user_score.update_count = user_score.update_count.saturating_add(1);
        user_score.bump = ctx.bumps.user_score;
        
        // Update oracle stats
        let oracle_account = &mut ctx.accounts.oracle_account;
        oracle_account.total_submissions = oracle_account.total_submissions.saturating_add(1);
        oracle_account.last_submission = clock.unix_timestamp;
        
        emit!(ScoreUpdated {
            user: user_score.user,
            authenticity,
            accuracy,
            agility,
            activity,
            approved,
            composite: user_score.composite_score,
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Score updated for: {}, composite: {}", user_score.user, user_score.composite_score);
        Ok(())
    }
    
    /// Create a score snapshot (oracle only)
    pub fn create_snapshot(
        ctx: Context<CreateSnapshot>,
        merkle_root: [u8; 32],
        user_count: u64,
        avg_score: u16,
    ) -> Result<()> {
        let config = &mut ctx.accounts.five_a_config;
        require!(!config.paused, FiveAError::ProtocolPaused);
        
        // Verify oracle
        let oracle_key = ctx.accounts.oracle.key();
        let is_oracle = config.oracles[..config.oracle_count as usize]
            .contains(&oracle_key);
        require!(is_oracle, FiveAError::NotOracle);
        
        let clock = Clock::get()?;
        
        // Increment epoch
        config.current_epoch = config.current_epoch.saturating_add(1);
        config.last_snapshot_time = clock.unix_timestamp;
        
        // Create snapshot
        let snapshot = &mut ctx.accounts.snapshot;
        snapshot.epoch = config.current_epoch;
        snapshot.merkle_root = merkle_root;
        snapshot.user_count = user_count;
        snapshot.avg_score = avg_score;
        snapshot.timestamp = clock.unix_timestamp;
        snapshot.submitter = oracle_key;
        snapshot.bump = ctx.bumps.snapshot;
        
        emit!(SnapshotCreated {
            epoch: snapshot.epoch,
            merkle_root,
            user_count,
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Snapshot created: epoch {}", snapshot.epoch);
        Ok(())
    }
    
    /// Vouch for a new user
    pub fn vouch_for_user(
        ctx: Context<VouchForUser>,
    ) -> Result<()> {
        let config = &ctx.accounts.five_a_config;
        require!(!config.paused, FiveAError::ProtocolPaused);
        
        let voucher_key = ctx.accounts.voucher.key();
        let vouchee_key = ctx.accounts.vouchee.key();
        
        // Cannot vouch for self
        require!(voucher_key != vouchee_key, FiveAError::CannotVouchSelf);
        
        // Check voucher's 5A score
        let voucher_score = &ctx.accounts.voucher_score;
        require!(
            voucher_score.composite_score >= MIN_VOUCHER_SCORE,
            FiveAError::VoucherScoreTooLow
        );
        
        // Check voucher stats
        let voucher_stats = &ctx.accounts.voucher_stats;
        let max_vouches = VoucherStats::max_vouches_for_score(voucher_score.composite_score);
        require!(
            voucher_stats.vouches_active < max_vouches,
            FiveAError::MaxVouchesReached
        );
        
        // Check vouchee status
        let vouchee_status = &ctx.accounts.vouchee_status;
        require!(
            vouchee_status.vouches_received < VOUCHES_REQUIRED,
            FiveAError::AlreadyFullyVouched
        );
        
        // Check not already vouched by this voucher
        for i in 0..vouchee_status.vouches_received as usize {
            require!(
                vouchee_status.vouchers[i] != voucher_key,
                FiveAError::AlreadyVouched
            );
        }
        
        let clock = Clock::get()?;
        
        // Create vouch record
        let vouch = &mut ctx.accounts.vouch_record;
        vouch.voucher = voucher_key;
        vouch.vouchee = vouchee_key;
        vouch.vouched_at = clock.unix_timestamp;
        vouch.vouch_stake = VOUCH_STAKE_AMOUNT;
        vouch.status = VouchStatus::Active as u8;
        vouch.outcome_evaluated = false;
        vouch.bump = ctx.bumps.vouch_record;
        
        // Update vouchee status
        let vouchee_status = &mut ctx.accounts.vouchee_status;
        vouchee_status.user = vouchee_key;
        let vouch_idx = vouchee_status.vouches_received as usize;
        vouchee_status.vouchers[vouch_idx] = voucher_key;
        vouchee_status.vouches_received += 1;
        let multiplier = vouchee_status.get_multiplier();
        vouchee_status.reward_multiplier = multiplier;
        
        if vouchee_status.vouches_received >= VOUCHES_REQUIRED {
            vouchee_status.is_fully_vouched = true;
            vouchee_status.vouch_completed_at = clock.unix_timestamp;
        }
        vouchee_status.bump = ctx.bumps.vouchee_status;
        
        // Update voucher stats
        let voucher_stats = &mut ctx.accounts.voucher_stats;
        voucher_stats.user = voucher_key;
        voucher_stats.total_vouches_given = voucher_stats.total_vouches_given.saturating_add(1);
        voucher_stats.vouches_active = voucher_stats.vouches_active.saturating_add(1);
        voucher_stats.max_concurrent_vouches = max_vouches;
        voucher_stats.bump = ctx.bumps.voucher_stats;
        
        emit!(VouchCreated {
            voucher: voucher_key,
            vouchee: vouchee_key,
            stake: VOUCH_STAKE_AMOUNT,
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Vouch created: {} -> {}", voucher_key, vouchee_key);
        Ok(())
    }
    
    /// Evaluate vouch outcome after 90 days
    pub fn evaluate_vouch(
        ctx: Context<EvaluateVouch>,
    ) -> Result<()> {
        let vouch = &mut ctx.accounts.vouch_record;
        
        require!(!vouch.outcome_evaluated, FiveAError::AlreadyEvaluated);
        
        let clock = Clock::get()?;
        let elapsed = clock.unix_timestamp - vouch.vouched_at;
        require!(elapsed >= VOUCH_EVALUATION_PERIOD, FiveAError::EvaluationNotComplete);
        
        // Check vouchee's current score
        let vouchee_score = &ctx.accounts.vouchee_score;
        let is_successful = vouchee_score.composite_score >= 5000; // 50%+
        
        vouch.outcome_evaluated = true;
        
        let voucher_stats = &mut ctx.accounts.voucher_stats;
        voucher_stats.vouches_active = voucher_stats.vouches_active.saturating_sub(1);
        
        if is_successful {
            vouch.status = VouchStatus::Rewarded as u8;
            voucher_stats.successful_vouches = voucher_stats.successful_vouches.saturating_add(1);
            voucher_stats.total_rewards_earned = voucher_stats.total_rewards_earned.saturating_add(VOUCH_REWARD);
            
            emit!(VouchEvaluated {
                voucher: vouch.voucher,
                vouchee: vouch.vouchee,
                success: true,
                reward_or_slash: VOUCH_REWARD,
            });
        } else {
            vouch.status = VouchStatus::Slashed as u8;
            voucher_stats.failed_vouches = voucher_stats.failed_vouches.saturating_add(1);
            voucher_stats.total_stake_lost = voucher_stats.total_stake_lost.saturating_add(vouch.vouch_stake);
            
            emit!(VouchEvaluated {
                voucher: vouch.voucher,
                vouchee: vouch.vouchee,
                success: false,
                reward_or_slash: vouch.vouch_stake,
            });
        }
        
        // Update accuracy
        let total = voucher_stats.successful_vouches + voucher_stats.failed_vouches;
        if total > 0 {
            voucher_stats.vouch_accuracy = 
                ((voucher_stats.successful_vouches as u32 * 10000) / total as u32) as u16;
        }
        
        msg!("Vouch evaluated: success={}", is_successful);
        Ok(())
    }
    
    /// Enable private score mode
    pub fn enable_private_score(ctx: Context<UpdateUserScore>) -> Result<()> {
        ctx.accounts.user_score.is_private = true;
        msg!("Private score mode enabled");
        Ok(())
    }
    
    /// Disable private score mode
    pub fn disable_private_score(ctx: Context<UpdateUserScore>) -> Result<()> {
        ctx.accounts.user_score.is_private = false;
        msg!("Private score mode disabled");
        Ok(())
    }
    
    /// Pause/unpause protocol
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        ctx.accounts.five_a_config.paused = paused;
        msg!("5A Protocol paused: {}", paused);
        Ok(())
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.five_a_config.authority = new_authority;
        msg!("Authority updated to: {}", new_authority);
        Ok(())
    }
    
    /// Get user score
    pub fn get_score(ctx: Context<GetScore>) -> Result<()> {
        let score = &ctx.accounts.user_score;
        msg!("User: {}", score.user);
        msg!("Composite: {}", score.composite_score);
        msg!("A1 Authenticity: {}", score.authenticity);
        msg!("A2 Accuracy: {}", score.accuracy);
        msg!("A3 Agility: {}", score.agility);
        msg!("A4 Activity: {}", score.activity);
        msg!("A5 Approved: {}", score.approved);
        Ok(())
    }
}

// Account contexts

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = FiveAConfig::LEN,
        seeds = [FIVE_A_CONFIG_SEED],
        bump
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    /// CHECK: Vouch stake vault
    pub vouch_vault: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterOracle<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump,
        has_one = authority @ FiveAError::Unauthorized
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    #[account(
        init,
        payer = authority,
        space = Oracle::LEN,
        seeds = [ORACLE_SEED, oracle_wallet.key().as_ref()],
        bump
    )]
    pub oracle: Account<'info, Oracle>,
    
    /// CHECK: Oracle wallet to register
    pub oracle_wallet: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    #[account(
        init_if_needed,
        payer = oracle,
        space = UserScore::LEN,
        seeds = [USER_SCORE_SEED, user.key().as_ref()],
        bump
    )]
    pub user_score: Account<'info, UserScore>,
    
    #[account(
        mut,
        seeds = [ORACLE_SEED, oracle.key().as_ref()],
        bump = oracle_account.bump
    )]
    pub oracle_account: Account<'info, Oracle>,
    
    /// CHECK: User whose score is being updated
    pub user: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub oracle: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateSnapshot<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    #[account(
        init,
        payer = oracle,
        space = ScoreSnapshot::LEN,
        seeds = [SCORE_SNAPSHOT_SEED, (five_a_config.current_epoch + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub snapshot: Account<'info, ScoreSnapshot>,
    
    #[account(mut)]
    pub oracle: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VouchForUser<'info> {
    #[account(
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    #[account(
        seeds = [USER_SCORE_SEED, voucher.key().as_ref()],
        bump = voucher_score.bump
    )]
    pub voucher_score: Account<'info, UserScore>,
    
    #[account(
        init,
        payer = voucher,
        space = VouchRecord::LEN,
        seeds = [VOUCH_RECORD_SEED, voucher.key().as_ref(), vouchee.key().as_ref()],
        bump
    )]
    pub vouch_record: Account<'info, VouchRecord>,
    
    #[account(
        init_if_needed,
        payer = voucher,
        space = UserVouchStatus::LEN,
        seeds = [VOUCH_STATUS_SEED, vouchee.key().as_ref()],
        bump
    )]
    pub vouchee_status: Account<'info, UserVouchStatus>,
    
    #[account(
        init_if_needed,
        payer = voucher,
        space = VoucherStats::LEN,
        seeds = [VOUCHER_STATS_SEED, voucher.key().as_ref()],
        bump
    )]
    pub voucher_stats: Account<'info, VoucherStats>,
    
    #[account(mut)]
    pub voucher: Signer<'info>,
    
    /// CHECK: User receiving vouch
    pub vouchee: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EvaluateVouch<'info> {
    #[account(
        mut,
        seeds = [VOUCH_RECORD_SEED, vouch_record.voucher.as_ref(), vouch_record.vouchee.as_ref()],
        bump = vouch_record.bump
    )]
    pub vouch_record: Account<'info, VouchRecord>,
    
    #[account(
        seeds = [USER_SCORE_SEED, vouch_record.vouchee.as_ref()],
        bump = vouchee_score.bump
    )]
    pub vouchee_score: Account<'info, UserScore>,
    
    #[account(
        mut,
        seeds = [VOUCHER_STATS_SEED, vouch_record.voucher.as_ref()],
        bump = voucher_stats.bump
    )]
    pub voucher_stats: Account<'info, VoucherStats>,
    
    pub evaluator: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateUserScore<'info> {
    #[account(
        mut,
        seeds = [USER_SCORE_SEED, user.key().as_ref()],
        bump = user_score.bump,
        constraint = user_score.user == user.key()
    )]
    pub user_score: Account<'info, UserScore>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump,
        has_one = authority @ FiveAError::Unauthorized
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump,
        has_one = authority @ FiveAError::Unauthorized
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetScore<'info> {
    #[account(
        seeds = [USER_SCORE_SEED, user_score.user.as_ref()],
        bump = user_score.bump
    )]
    pub user_score: Account<'info, UserScore>,
}


