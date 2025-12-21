use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("FZrjuWJE6VW7qSxB8Jhd4hxv1fSnYBsRCzBTfWhVN8zC");

/// SSCRE Protocol - Self-Sustaining Circular Reward Economy
/// 
/// Merkle-based gasless reward claims with 6-layer funding hierarchy.
/// 
/// Key Features:
/// - 350M VCoin primary rewards pool
/// - Monthly epoch-based distribution
/// - Merkle tree claims for gas efficiency
/// - 5A score-weighted rewards
/// - 6-layer sustainable funding (post Year 5)
/// 
/// Reward Distribution Formula:
/// user_reward = base_allocation × five_a_multiplier × streak_bonus × vouch_multiplier

pub mod constants {
    /// Seeds
    pub const POOL_CONFIG_SEED: &[u8] = b"pool-config";
    pub const EPOCH_SEED: &[u8] = b"epoch";
    pub const USER_CLAIM_SEED: &[u8] = b"user-claim";
    pub const FUNDING_LAYER_SEED: &[u8] = b"funding-layer";
    pub const CIRCUIT_BREAKER_SEED: &[u8] = b"circuit-breaker";
    
    /// Pool configuration
    pub const PRIMARY_RESERVES: u64 = 350_000_000 * 1_000_000_000;  // 350M VCoin (35% of 1B)
    pub const SECONDARY_RESERVES: u64 = 40_000_000 * 1_000_000_000; // 40M VCoin buyback buffer
    pub const EPOCH_DURATION: i64 = 30 * 24 * 60 * 60;              // 30 days
    pub const CLAIM_WINDOW: i64 = 90 * 24 * 60 * 60;                // 90 days to claim
    
    /// Fee deduction for gasless claims
    pub const GASLESS_FEE_BPS: u16 = 100; // 1% deducted for gas
    
    /// Minimum claim amount
    pub const MIN_CLAIM_AMOUNT: u64 = 1_000_000_000; // 1 VCoin minimum
    
    /// Circuit breaker thresholds
    pub const MAX_EPOCH_EMISSION: u64 = 10_000_000 * 1_000_000_000; // 10M VCoin max per epoch
    pub const MAX_SINGLE_CLAIM: u64 = 100_000 * 1_000_000_000;      // 100K VCoin max single claim
    
    /// 5A Score multipliers (x1000 for precision)
    pub const SCORE_MULT_0_20: u64 = 100;   // 0.1x (10%)
    pub const SCORE_MULT_20_40: u64 = 400;  // 0.4x (40%)
    pub const SCORE_MULT_40_60: u64 = 700;  // 0.7x (70%)
    pub const SCORE_MULT_60_80: u64 = 1000; // 1.0x (100%)
    pub const SCORE_MULT_80_100: u64 = 1200; // 1.2x (120%)
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum SSCREError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("SSCRE Protocol is paused")]
        ProtocolPaused,
        #[msg("Invalid merkle proof")]
        InvalidMerkleProof,
        #[msg("Already claimed for this epoch")]
        AlreadyClaimed,
        #[msg("Claim window expired")]
        ClaimWindowExpired,
        #[msg("Epoch not finalized")]
        EpochNotFinalized,
        #[msg("Insufficient pool balance")]
        InsufficientPoolBalance,
        #[msg("Claim amount below minimum")]
        ClaimBelowMinimum,
        #[msg("Circuit breaker triggered: max epoch emission exceeded")]
        CircuitBreakerEpochMax,
        #[msg("Circuit breaker triggered: max single claim exceeded")]
        CircuitBreakerClaimMax,
        #[msg("Invalid epoch number")]
        InvalidEpoch,
        #[msg("Epoch already exists")]
        EpochAlreadyExists,
        #[msg("Oracle not registered")]
        OracleNotRegistered,
        #[msg("Funding layer inactive")]
        FundingLayerInactive,
        #[msg("Arithmetic overflow")]
        Overflow,
    }
}

pub mod state {
    use super::*;
    use crate::constants::*;
    
    /// Global rewards pool configuration
    #[account]
    #[derive(Default)]
    pub struct RewardsPoolConfig {
        /// Admin authority
        pub authority: Pubkey,
        /// VCoin mint
        pub vcoin_mint: Pubkey,
        /// Pool vault holding VCoin rewards
        pub pool_vault: Pubkey,
        /// 5A Protocol for score verification
        pub five_a_program: Pubkey,
        /// Registered oracles (max 5)
        pub oracles: [Pubkey; 5],
        /// Number of active oracles
        pub oracle_count: u8,
        /// Current epoch number
        pub current_epoch: u64,
        /// Total VCoin distributed all-time
        pub total_distributed: u64,
        /// Remaining primary reserves
        pub remaining_reserves: u64,
        /// Whether protocol is paused
        pub paused: bool,
        /// Whether circuit breaker is active
        pub circuit_breaker_active: bool,
        /// Fee recipient for gasless fee
        pub fee_recipient: Pubkey,
        /// PDA bump
        pub bump: u8,
        /// Vault bump
        pub vault_bump: u8,
    }
    
    impl RewardsPoolConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // vcoin_mint
            32 + // pool_vault
            32 + // five_a_program
            (32 * 5) + // oracles
            1 +  // oracle_count
            8 +  // current_epoch
            8 +  // total_distributed
            8 +  // remaining_reserves
            1 +  // paused
            1 +  // circuit_breaker_active
            32 + // fee_recipient
            1 +  // bump
            1;   // vault_bump
    }
    
    /// Epoch distribution account
    #[account]
    #[derive(Default)]
    pub struct EpochDistribution {
        /// Epoch number
        pub epoch: u64,
        /// Merkle root of all user allocations
        pub merkle_root: [u8; 32],
        /// Total VCoin allocated for this epoch
        pub total_allocation: u64,
        /// Total VCoin claimed so far
        pub total_claimed: u64,
        /// Number of users who claimed
        pub claims_count: u64,
        /// Epoch start timestamp
        pub start_time: i64,
        /// Epoch end timestamp
        pub end_time: i64,
        /// Claim window expiry
        pub claim_expiry: i64,
        /// Whether epoch is finalized (merkle root set)
        pub is_finalized: bool,
        /// Oracle that submitted the merkle root
        pub submitter: Pubkey,
        /// Average 5A score for this epoch
        pub avg_five_a_score: u16,
        /// Total eligible users
        pub eligible_users: u64,
        /// PDA bump
        pub bump: u8,
    }
    
    impl EpochDistribution {
        pub const LEN: usize = 8 + // discriminator
            8 +  // epoch
            32 + // merkle_root
            8 +  // total_allocation
            8 +  // total_claimed
            8 +  // claims_count
            8 +  // start_time
            8 +  // end_time
            8 +  // claim_expiry
            1 +  // is_finalized
            32 + // submitter
            2 +  // avg_five_a_score
            8 +  // eligible_users
            1;   // bump
    }
    
    /// User claim record (tracks all epochs claimed)
    #[account]
    #[derive(Default)]
    pub struct UserClaim {
        /// User wallet
        pub user: Pubkey,
        /// Last claimed epoch
        pub last_claimed_epoch: u64,
        /// Total VCoin claimed all-time
        pub total_claimed: u64,
        /// Total claims made
        pub claims_count: u32,
        /// First claim timestamp
        pub first_claim_at: i64,
        /// Last claim timestamp
        pub last_claim_at: i64,
        /// Bitmap of claimed epochs (last 256 epochs)
        pub claimed_epochs_bitmap: [u64; 4],
        /// PDA bump
        pub bump: u8,
    }
    
    impl UserClaim {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            8 +  // last_claimed_epoch
            8 +  // total_claimed
            4 +  // claims_count
            8 +  // first_claim_at
            8 +  // last_claim_at
            (8 * 4) + // claimed_epochs_bitmap
            1;   // bump
        
        /// Check if a specific epoch has been claimed
        pub fn is_epoch_claimed(&self, epoch: u64) -> bool {
            if epoch > 255 {
                // For epochs > 255, check last_claimed_epoch
                return epoch <= self.last_claimed_epoch;
            }
            let bitmap_index = (epoch / 64) as usize;
            let bit_position = epoch % 64;
            if bitmap_index >= 4 {
                return false;
            }
            (self.claimed_epochs_bitmap[bitmap_index] & (1 << bit_position)) != 0
        }
        
        /// Mark an epoch as claimed
        pub fn mark_epoch_claimed(&mut self, epoch: u64) {
            if epoch <= 255 {
                let bitmap_index = (epoch / 64) as usize;
                let bit_position = epoch % 64;
                if bitmap_index < 4 {
                    self.claimed_epochs_bitmap[bitmap_index] |= 1 << bit_position;
                }
            }
            if epoch > self.last_claimed_epoch {
                self.last_claimed_epoch = epoch;
            }
        }
    }
    
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
    
    /// Circuit breaker state
    #[account]
    #[derive(Default)]
    pub struct CircuitBreaker {
        /// Authority
        pub authority: Pubkey,
        /// Whether circuit breaker is active
        pub is_active: bool,
        /// Max emission per epoch
        pub max_epoch_emission: u64,
        /// Max single claim
        pub max_single_claim: u64,
        /// Current epoch emission so far
        pub current_epoch_emission: u64,
        /// Largest claim this epoch
        pub largest_claim_this_epoch: u64,
        /// Number of triggers
        pub trigger_count: u32,
        /// Last trigger timestamp
        pub last_trigger_at: i64,
        /// Trigger reason
        pub last_trigger_reason: u8,
        /// PDA bump
        pub bump: u8,
    }
    
    impl CircuitBreaker {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            1 +  // is_active
            8 +  // max_epoch_emission
            8 +  // max_single_claim
            8 +  // current_epoch_emission
            8 +  // largest_claim_this_epoch
            4 +  // trigger_count
            8 +  // last_trigger_at
            1 +  // last_trigger_reason
            1;   // bump
    }
    
    /// Get 5A score multiplier
    pub fn get_five_a_multiplier(score: u16) -> u64 {
        if score >= 8000 {
            SCORE_MULT_80_100
        } else if score >= 6000 {
            SCORE_MULT_60_80
        } else if score >= 4000 {
            SCORE_MULT_40_60
        } else if score >= 2000 {
            SCORE_MULT_20_40
        } else {
            SCORE_MULT_0_20
        }
    }
}

pub mod events {
    use super::*;
    
    #[event]
    pub struct PoolInitialized {
        pub authority: Pubkey,
        pub vcoin_mint: Pubkey,
        pub initial_reserves: u64,
    }
    
    #[event]
    pub struct EpochStarted {
        pub epoch: u64,
        pub start_time: i64,
        pub end_time: i64,
    }
    
    #[event]
    pub struct EpochFinalized {
        pub epoch: u64,
        pub merkle_root: [u8; 32],
        pub total_allocation: u64,
        pub eligible_users: u64,
    }
    
    #[event]
    pub struct RewardsClaimed {
        pub user: Pubkey,
        pub epoch: u64,
        pub gross_amount: u64,
        pub fee_deducted: u64,
        pub net_amount: u64,
    }
    
    #[event]
    pub struct CircuitBreakerTriggered {
        pub reason: u8,
        pub value: u64,
        pub threshold: u64,
        pub timestamp: i64,
    }
    
    #[event]
    pub struct FundingLayerSwitch {
        pub from_layer: u8,
        pub to_layer: u8,
        pub reason: String,
        pub timestamp: i64,
    }
}

use constants::*;
use errors::*;
use state::*;
use events::*;

#[program]
pub mod sscre_protocol {
    use super::*;

    /// Initialize the SSCRE rewards pool
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        fee_recipient: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        
        config.authority = ctx.accounts.authority.key();
        config.vcoin_mint = ctx.accounts.vcoin_mint.key();
        config.pool_vault = ctx.accounts.pool_vault.key();
        config.five_a_program = Pubkey::default(); // Set later
        config.oracles = [Pubkey::default(); 5];
        config.oracle_count = 0;
        config.current_epoch = 0;
        config.total_distributed = 0;
        config.remaining_reserves = PRIMARY_RESERVES;
        config.paused = false;
        config.circuit_breaker_active = false;
        config.fee_recipient = fee_recipient;
        config.bump = ctx.bumps.pool_config;
        config.vault_bump = ctx.bumps.pool_vault;
        
        emit!(PoolInitialized {
            authority: config.authority,
            vcoin_mint: config.vcoin_mint,
            initial_reserves: PRIMARY_RESERVES,
        });
        
        msg!("SSCRE Pool initialized with {} VCoin reserves", PRIMARY_RESERVES);
        Ok(())
    }
    
    /// Initialize the 6-layer funding configuration
    pub fn initialize_funding_layers(
        ctx: Context<InitializeFundingLayers>,
    ) -> Result<()> {
        let funding = &mut ctx.accounts.funding_config;
        
        funding.authority = ctx.accounts.authority.key();
        funding.l1_primary_remaining = PRIMARY_RESERVES;
        funding.l2_secondary_remaining = SECONDARY_RESERVES;
        funding.l3_buyback_rate_bps = 1000;  // 10%
        funding.l4_profit_rate_bps = 2500;   // 25%
        funding.l5_fee_recycling_rate_bps = 5000; // 50%
        funding.active_layer = 1;
        funding.total_recycled = 0;
        funding.last_layer_switch = 0;
        funding.months_remaining_estimate = 60; // 5 years
        funding.bump = ctx.bumps.funding_config;
        
        msg!("Funding layers initialized");
        Ok(())
    }
    
    /// Initialize circuit breaker
    pub fn initialize_circuit_breaker(
        ctx: Context<InitializeCircuitBreaker>,
    ) -> Result<()> {
        let cb = &mut ctx.accounts.circuit_breaker;
        
        cb.authority = ctx.accounts.authority.key();
        cb.is_active = false;
        cb.max_epoch_emission = MAX_EPOCH_EMISSION;
        cb.max_single_claim = MAX_SINGLE_CLAIM;
        cb.current_epoch_emission = 0;
        cb.largest_claim_this_epoch = 0;
        cb.trigger_count = 0;
        cb.last_trigger_at = 0;
        cb.last_trigger_reason = 0;
        cb.bump = ctx.bumps.circuit_breaker;
        
        msg!("Circuit breaker initialized");
        Ok(())
    }
    
    /// Register an oracle for merkle root submission
    pub fn register_oracle(
        ctx: Context<RegisterOracle>,
        oracle: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        
        require!(config.oracle_count < 5, SSCREError::Overflow);
        
        let idx = config.oracle_count as usize;
        config.oracles[idx] = oracle;
        config.oracle_count += 1;
        
        msg!("Oracle registered: {}", oracle);
        Ok(())
    }
    
    /// Start a new epoch
    pub fn start_epoch(
        ctx: Context<StartEpoch>,
        total_allocation: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        let epoch_dist = &mut ctx.accounts.epoch_distribution;
        let cb = &mut ctx.accounts.circuit_breaker;
        
        require!(!config.paused, SSCREError::ProtocolPaused);
        require!(!cb.is_active, SSCREError::CircuitBreakerEpochMax);
        
        // Check allocation doesn't exceed max
        require!(
            total_allocation <= MAX_EPOCH_EMISSION,
            SSCREError::CircuitBreakerEpochMax
        );
        
        // Check sufficient reserves
        require!(
            total_allocation <= config.remaining_reserves,
            SSCREError::InsufficientPoolBalance
        );
        
        let clock = Clock::get()?;
        
        // Increment epoch
        config.current_epoch = config.current_epoch.saturating_add(1);
        
        // Reset circuit breaker for new epoch
        cb.current_epoch_emission = 0;
        cb.largest_claim_this_epoch = 0;
        
        // Initialize epoch distribution
        epoch_dist.epoch = config.current_epoch;
        epoch_dist.merkle_root = [0u8; 32];
        epoch_dist.total_allocation = total_allocation;
        epoch_dist.total_claimed = 0;
        epoch_dist.claims_count = 0;
        epoch_dist.start_time = clock.unix_timestamp;
        epoch_dist.end_time = clock.unix_timestamp + EPOCH_DURATION;
        epoch_dist.claim_expiry = clock.unix_timestamp + EPOCH_DURATION + CLAIM_WINDOW;
        epoch_dist.is_finalized = false;
        epoch_dist.submitter = Pubkey::default();
        epoch_dist.avg_five_a_score = 0;
        epoch_dist.eligible_users = 0;
        epoch_dist.bump = ctx.bumps.epoch_distribution;
        
        emit!(EpochStarted {
            epoch: epoch_dist.epoch,
            start_time: epoch_dist.start_time,
            end_time: epoch_dist.end_time,
        });
        
        msg!("Epoch {} started with {} VCoin allocation", 
            epoch_dist.epoch, total_allocation);
        Ok(())
    }
    
    /// Update merkle root (finalize epoch distribution)
    pub fn update_merkle_root(
        ctx: Context<UpdateMerkleRoot>,
        merkle_root: [u8; 32],
        eligible_users: u64,
        avg_five_a_score: u16,
    ) -> Result<()> {
        let config = &ctx.accounts.pool_config;
        let epoch_dist = &mut ctx.accounts.epoch_distribution;
        
        require!(!config.paused, SSCREError::ProtocolPaused);
        require!(!epoch_dist.is_finalized, SSCREError::EpochAlreadyExists);
        
        // Verify oracle is registered
        let oracle_key = ctx.accounts.oracle.key();
        let is_oracle = config.oracles[..config.oracle_count as usize]
            .contains(&oracle_key);
        require!(is_oracle, SSCREError::OracleNotRegistered);
        
        // Set merkle root
        epoch_dist.merkle_root = merkle_root;
        epoch_dist.is_finalized = true;
        epoch_dist.submitter = oracle_key;
        epoch_dist.eligible_users = eligible_users;
        epoch_dist.avg_five_a_score = avg_five_a_score;
        
        emit!(EpochFinalized {
            epoch: epoch_dist.epoch,
            merkle_root,
            total_allocation: epoch_dist.total_allocation,
            eligible_users,
        });
        
        msg!("Epoch {} finalized with merkle root", epoch_dist.epoch);
        Ok(())
    }
    
    /// Claim rewards with merkle proof
    pub fn claim_rewards(
        ctx: Context<ClaimRewards>,
        amount: u64,
        merkle_proof: Vec<[u8; 32]>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        let epoch_dist = &mut ctx.accounts.epoch_distribution;
        let user_claim = &mut ctx.accounts.user_claim;
        let cb = &mut ctx.accounts.circuit_breaker;
        
        require!(!config.paused, SSCREError::ProtocolPaused);
        require!(!cb.is_active, SSCREError::CircuitBreakerEpochMax);
        require!(epoch_dist.is_finalized, SSCREError::EpochNotFinalized);
        require!(amount >= MIN_CLAIM_AMOUNT, SSCREError::ClaimBelowMinimum);
        require!(amount <= MAX_SINGLE_CLAIM, SSCREError::CircuitBreakerClaimMax);
        
        let clock = Clock::get()?;
        
        // Check claim window
        require!(
            clock.unix_timestamp <= epoch_dist.claim_expiry,
            SSCREError::ClaimWindowExpired
        );
        
        // Check not already claimed
        require!(
            !user_claim.is_epoch_claimed(epoch_dist.epoch),
            SSCREError::AlreadyClaimed
        );
        
        // Verify merkle proof
        let user_key = ctx.accounts.user.key();
        let leaf = compute_leaf(&user_key, amount, epoch_dist.epoch);
        require!(
            verify_merkle_proof(&merkle_proof, &epoch_dist.merkle_root, &leaf),
            SSCREError::InvalidMerkleProof
        );
        
        // Calculate fee for gasless claim
        let fee = (amount as u128 * GASLESS_FEE_BPS as u128 / 10000) as u64;
        let net_amount = amount.saturating_sub(fee);
        
        // Get bumps for transfer
        let pool_bump = config.bump;
        
        // Transfer VCoin to user
        let seeds = &[
            POOL_CONFIG_SEED,
            &[pool_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        
        token_2022::transfer_checked(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
                token_2022::TransferChecked {
                    from: ctx.accounts.pool_vault.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: config.to_account_info(),
                    mint: ctx.accounts.vcoin_mint.to_account_info(),
                },
                signer_seeds,
            ),
            net_amount,
            ctx.accounts.vcoin_mint.decimals,
        )?;
        
        // Transfer fee to fee recipient
        if fee > 0 {
            token_2022::transfer_checked(
                CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
                    token_2022::TransferChecked {
                        from: ctx.accounts.pool_vault.to_account_info(),
                        to: ctx.accounts.fee_account.to_account_info(),
                        authority: config.to_account_info(),
                        mint: ctx.accounts.vcoin_mint.to_account_info(),
                    },
                    signer_seeds,
                ),
                fee,
                ctx.accounts.vcoin_mint.decimals,
            )?;
        }
        
        // Update user claim record
        user_claim.user = user_key;
        user_claim.mark_epoch_claimed(epoch_dist.epoch);
        user_claim.total_claimed = user_claim.total_claimed.saturating_add(net_amount);
        user_claim.claims_count = user_claim.claims_count.saturating_add(1);
        
        if user_claim.first_claim_at == 0 {
            user_claim.first_claim_at = clock.unix_timestamp;
        }
        user_claim.last_claim_at = clock.unix_timestamp;
        user_claim.bump = ctx.bumps.user_claim;
        
        // Update epoch distribution
        epoch_dist.total_claimed = epoch_dist.total_claimed.saturating_add(amount);
        epoch_dist.claims_count = epoch_dist.claims_count.saturating_add(1);
        
        // Update pool config
        config.total_distributed = config.total_distributed.saturating_add(amount);
        config.remaining_reserves = config.remaining_reserves.saturating_sub(amount);
        
        // Update circuit breaker tracking
        cb.current_epoch_emission = cb.current_epoch_emission.saturating_add(amount);
        if amount > cb.largest_claim_this_epoch {
            cb.largest_claim_this_epoch = amount;
        }
        
        emit!(RewardsClaimed {
            user: user_key,
            epoch: epoch_dist.epoch,
            gross_amount: amount,
            fee_deducted: fee,
            net_amount,
        });
        
        msg!("Claimed {} VCoin (net: {} after {} fee)", amount, net_amount, fee);
        Ok(())
    }
    
    /// Trigger circuit breaker (emergency)
    pub fn trigger_circuit_breaker(
        ctx: Context<TriggerCircuitBreaker>,
        reason: u8,
    ) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        let cb = &mut ctx.accounts.circuit_breaker;
        
        let clock = Clock::get()?;
        
        cb.is_active = true;
        cb.trigger_count = cb.trigger_count.saturating_add(1);
        cb.last_trigger_at = clock.unix_timestamp;
        cb.last_trigger_reason = reason;
        
        config.circuit_breaker_active = true;
        
        emit!(CircuitBreakerTriggered {
            reason,
            value: 0,
            threshold: 0,
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Circuit breaker triggered: reason {}", reason);
        Ok(())
    }
    
    /// Reset circuit breaker (after investigation)
    pub fn reset_circuit_breaker(
        ctx: Context<ResetCircuitBreaker>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        let cb = &mut ctx.accounts.circuit_breaker;
        
        cb.is_active = false;
        config.circuit_breaker_active = false;
        
        msg!("Circuit breaker reset");
        Ok(())
    }
    
    /// Pause/unpause protocol
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        ctx.accounts.pool_config.paused = paused;
        msg!("SSCRE Protocol paused: {}", paused);
        Ok(())
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.pool_config.authority = new_authority;
        msg!("Authority updated to: {}", new_authority);
        Ok(())
    }
    
    /// Update 5A program reference
    pub fn set_five_a_program(ctx: Context<UpdateConfig>, five_a_program: Pubkey) -> Result<()> {
        ctx.accounts.pool_config.five_a_program = five_a_program;
        msg!("5A Program set to: {}", five_a_program);
        Ok(())
    }
    
    /// Get pool stats
    pub fn get_pool_stats(ctx: Context<GetPoolStats>) -> Result<()> {
        let config = &ctx.accounts.pool_config;
        msg!("Current epoch: {}", config.current_epoch);
        msg!("Total distributed: {}", config.total_distributed);
        msg!("Remaining reserves: {}", config.remaining_reserves);
        msg!("Paused: {}", config.paused);
        Ok(())
    }
    
    /// Get user claim stats
    pub fn get_user_stats(ctx: Context<GetUserStats>) -> Result<()> {
        let claim = &ctx.accounts.user_claim;
        msg!("User: {}", claim.user);
        msg!("Total claimed: {}", claim.total_claimed);
        msg!("Claims count: {}", claim.claims_count);
        msg!("Last claimed epoch: {}", claim.last_claimed_epoch);
        Ok(())
    }
}

// Helper functions

/// Compute merkle leaf from user, amount, and epoch
fn compute_leaf(user: &Pubkey, amount: u64, epoch: u64) -> [u8; 32] {
    use solana_program::keccak;
    
    let mut data = Vec::with_capacity(48);
    data.extend_from_slice(user.as_ref());
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(&epoch.to_le_bytes());
    
    keccak::hash(&data).to_bytes()
}

/// Verify merkle proof
fn verify_merkle_proof(proof: &[[u8; 32]], root: &[u8; 32], leaf: &[u8; 32]) -> bool {
    use solana_program::keccak;
    
    let mut computed_hash = *leaf;
    
    for proof_element in proof {
        // Sort the hashes to ensure consistent ordering
        let (left, right) = if computed_hash < *proof_element {
            (computed_hash, *proof_element)
        } else {
            (*proof_element, computed_hash)
        };
        
        let mut combined = [0u8; 64];
        combined[..32].copy_from_slice(&left);
        combined[32..].copy_from_slice(&right);
        
        computed_hash = keccak::hash(&combined).to_bytes();
    }
    
    computed_hash == *root
}

// Account contexts

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(
        init,
        payer = authority,
        space = RewardsPoolConfig::LEN,
        seeds = [POOL_CONFIG_SEED],
        bump
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    /// VCoin mint
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// Pool vault for VCoin rewards
    #[account(
        init,
        payer = authority,
        seeds = [b"pool-vault"],
        bump,
        token::mint = vcoin_mint,
        token::authority = pool_config,
        token::token_program = token_program,
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeFundingLayers<'info> {
    #[account(
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump,
        has_one = authority @ SSCREError::Unauthorized
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    #[account(
        init,
        payer = authority,
        space = FundingLayerConfig::LEN,
        seeds = [FUNDING_LAYER_SEED],
        bump
    )]
    pub funding_config: Account<'info, FundingLayerConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeCircuitBreaker<'info> {
    #[account(
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump,
        has_one = authority @ SSCREError::Unauthorized
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    #[account(
        init,
        payer = authority,
        space = CircuitBreaker::LEN,
        seeds = [CIRCUIT_BREAKER_SEED],
        bump
    )]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterOracle<'info> {
    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump,
        has_one = authority @ SSCREError::Unauthorized
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct StartEpoch<'info> {
    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump,
        has_one = authority @ SSCREError::Unauthorized
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    #[account(
        init,
        payer = authority,
        space = EpochDistribution::LEN,
        seeds = [EPOCH_SEED, (pool_config.current_epoch + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub epoch_distribution: Account<'info, EpochDistribution>,
    
    #[account(
        mut,
        seeds = [CIRCUIT_BREAKER_SEED],
        bump = circuit_breaker.bump
    )]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMerkleRoot<'info> {
    #[account(
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    #[account(
        mut,
        seeds = [EPOCH_SEED, epoch_distribution.epoch.to_le_bytes().as_ref()],
        bump = epoch_distribution.bump
    )]
    pub epoch_distribution: Account<'info, EpochDistribution>,
    
    pub oracle: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    #[account(
        mut,
        seeds = [EPOCH_SEED, epoch_distribution.epoch.to_le_bytes().as_ref()],
        bump = epoch_distribution.bump
    )]
    pub epoch_distribution: Account<'info, EpochDistribution>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = UserClaim::LEN,
        seeds = [USER_CLAIM_SEED, user.key().as_ref()],
        bump
    )]
    pub user_claim: Account<'info, UserClaim>,
    
    #[account(
        mut,
        seeds = [CIRCUIT_BREAKER_SEED],
        bump = circuit_breaker.bump
    )]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    
    /// VCoin mint
    #[account(constraint = vcoin_mint.key() == pool_config.vcoin_mint)]
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// Pool vault
    #[account(
        mut,
        seeds = [b"pool-vault"],
        bump
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    
    /// User's token account
    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    
    /// Fee recipient's token account
    #[account(
        mut,
        constraint = fee_account.owner == pool_config.fee_recipient
    )]
    pub fee_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TriggerCircuitBreaker<'info> {
    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump,
        has_one = authority @ SSCREError::Unauthorized
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    #[account(
        mut,
        seeds = [CIRCUIT_BREAKER_SEED],
        bump = circuit_breaker.bump
    )]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetCircuitBreaker<'info> {
    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump,
        has_one = authority @ SSCREError::Unauthorized
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    #[account(
        mut,
        seeds = [CIRCUIT_BREAKER_SEED],
        bump = circuit_breaker.bump
    )]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump,
        has_one = authority @ SSCREError::Unauthorized
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump,
        has_one = authority @ SSCREError::Unauthorized
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetPoolStats<'info> {
    #[account(
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
}

#[derive(Accounts)]
pub struct GetUserStats<'info> {
    #[account(
        seeds = [USER_CLAIM_SEED, user_claim.user.as_ref()],
        bump = user_claim.bump
    )]
    pub user_claim: Account<'info, UserClaim>,
}


