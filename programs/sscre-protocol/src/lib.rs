use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("6AJNcQSfoiE2UAeUDyJUBumS9SBwhAdSznoAeYpXrxXZ");

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

pub mod constants;
pub mod errors;
pub mod events;
pub mod state;

#[cfg(test)]
mod tests;

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
        config.five_a_program = Pubkey::default();
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
    pub fn initialize_funding_layers(ctx: Context<InitializeFundingLayers>) -> Result<()> {
        let funding = &mut ctx.accounts.funding_config;
        
        funding.authority = ctx.accounts.authority.key();
        funding.l1_primary_remaining = PRIMARY_RESERVES;
        funding.l2_secondary_remaining = SECONDARY_RESERVES;
        funding.l3_buyback_rate_bps = 1000;
        funding.l4_profit_rate_bps = 2500;
        funding.l5_fee_recycling_rate_bps = 5000;
        funding.active_layer = 1;
        funding.total_recycled = 0;
        funding.last_layer_switch = 0;
        funding.months_remaining_estimate = 60;
        funding.bump = ctx.bumps.funding_config;
        
        msg!("Funding layers initialized");
        Ok(())
    }
    
    /// Initialize circuit breaker
    pub fn initialize_circuit_breaker(ctx: Context<InitializeCircuitBreaker>) -> Result<()> {
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
    pub fn register_oracle(ctx: Context<RegisterOracle>, oracle: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        
        require!(config.oracle_count < 5, SSCREError::Overflow);
        
        let idx = config.oracle_count as usize;
        config.oracles[idx] = oracle;
        config.oracle_count += 1;
        
        msg!("Oracle registered: {}", oracle);
        Ok(())
    }
    
    /// Start a new epoch
    pub fn start_epoch(ctx: Context<StartEpoch>, total_allocation: u64) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        let epoch_dist = &mut ctx.accounts.epoch_distribution;
        let cb = &mut ctx.accounts.circuit_breaker;
        
        require!(!config.paused, SSCREError::ProtocolPaused);
        require!(!cb.is_active, SSCREError::CircuitBreakerEpochMax);
        require!(total_allocation <= MAX_EPOCH_EMISSION, SSCREError::CircuitBreakerEpochMax);
        require!(total_allocation <= config.remaining_reserves, SSCREError::InsufficientPoolBalance);
        
        let clock = Clock::get()?;
        
        config.current_epoch = config.current_epoch.saturating_add(1);
        
        cb.current_epoch_emission = 0;
        cb.largest_claim_this_epoch = 0;
        
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
        
        msg!("Epoch {} started with {} VCoin allocation", epoch_dist.epoch, total_allocation);
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
        
        let oracle_key = ctx.accounts.oracle.key();
        let is_oracle = config.oracles[..config.oracle_count as usize].contains(&oracle_key);
        require!(is_oracle, SSCREError::OracleNotRegistered);
        
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
        
        require!(clock.unix_timestamp <= epoch_dist.claim_expiry, SSCREError::ClaimWindowExpired);
        require!(!user_claim.is_epoch_claimed(epoch_dist.epoch), SSCREError::AlreadyClaimed);
        
        let user_key = ctx.accounts.user.key();
        let leaf = compute_leaf(&user_key, amount, epoch_dist.epoch);
        require!(verify_merkle_proof(&merkle_proof, &epoch_dist.merkle_root, &leaf), SSCREError::InvalidMerkleProof);
        
        let fee = (amount as u128 * GASLESS_FEE_BPS as u128 / 10000) as u64;
        let net_amount = amount.saturating_sub(fee);
        
        let pool_bump = config.bump;
        let seeds = &[POOL_CONFIG_SEED, &[pool_bump]];
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
        
        user_claim.user = user_key;
        user_claim.mark_epoch_claimed(epoch_dist.epoch)?;
        user_claim.total_claimed = user_claim.total_claimed.saturating_add(net_amount);
        user_claim.claims_count = user_claim.claims_count.saturating_add(1);
        
        if user_claim.first_claim_at == 0 {
            user_claim.first_claim_at = clock.unix_timestamp;
        }
        user_claim.last_claim_at = clock.unix_timestamp;
        user_claim.bump = ctx.bumps.user_claim;
        
        epoch_dist.total_claimed = epoch_dist.total_claimed.saturating_add(amount);
        epoch_dist.claims_count = epoch_dist.claims_count.saturating_add(1);
        
        config.total_distributed = config.total_distributed.saturating_add(amount);
        config.remaining_reserves = config.remaining_reserves.saturating_sub(amount);
        
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
    pub fn trigger_circuit_breaker(ctx: Context<TriggerCircuitBreaker>, reason: u8) -> Result<()> {
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
    /// M-05 Security Fix: Requires cooldown period before reset
    pub fn reset_circuit_breaker(ctx: Context<ResetCircuitBreaker>) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        let cb = &mut ctx.accounts.circuit_breaker;
        
        // M-05: Enforce 6-hour cooldown period after trigger
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp >= cb.last_trigger_at + CIRCUIT_BREAKER_COOLDOWN,
            SSCREError::CircuitBreakerCooldown
        );
        
        cb.is_active = false;
        config.circuit_breaker_active = false;
        
        msg!("Circuit breaker reset after {} hours cooldown", 
            (clock.unix_timestamp - cb.last_trigger_at) / 3600);
        Ok(())
    }
    
    /// Pause/unpause protocol
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        ctx.accounts.pool_config.paused = paused;
        msg!("SSCRE Protocol paused: {}", paused);
        Ok(())
    }
    
    /// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
    pub fn propose_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        
        require!(
            new_authority != config.authority,
            SSCREError::CannotProposeSelf
        );
        
        require!(
            new_authority != Pubkey::default(),
            SSCREError::InvalidAuthority
        );
        
        config.pending_authority = new_authority;
        
        msg!("Authority transfer proposed to: {}", new_authority);
        Ok(())
    }
    
    /// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        
        let old_authority = config.authority;
        let new_authority = ctx.accounts.new_authority.key();
        
        config.authority = new_authority;
        config.pending_authority = Pubkey::default();
        
        msg!("Authority transferred from {} to {}", old_authority, new_authority);
        Ok(())
    }
    
    /// Cancel a pending authority transfer (H-02 security fix)
    pub fn cancel_authority_transfer(ctx: Context<UpdateAuthority>) -> Result<()> {
        let config = &mut ctx.accounts.pool_config;
        
        require!(
            config.pending_authority != Pubkey::default(),
            SSCREError::NoPendingTransfer
        );
        
        let cancelled = config.pending_authority;
        config.pending_authority = Pubkey::default();
        
        msg!("Authority transfer to {} cancelled", cancelled);
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

// Account contexts

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = authority, space = RewardsPoolConfig::LEN, seeds = [POOL_CONFIG_SEED], bump)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    #[account(init, payer = authority, seeds = [b"pool-vault"], bump, token::mint = vcoin_mint, token::authority = pool_config, token::token_program = token_program)]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeFundingLayers<'info> {
    #[account(seeds = [POOL_CONFIG_SEED], bump = pool_config.bump, has_one = authority @ SSCREError::Unauthorized)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    #[account(init, payer = authority, space = FundingLayerConfig::LEN, seeds = [FUNDING_LAYER_SEED], bump)]
    pub funding_config: Account<'info, FundingLayerConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeCircuitBreaker<'info> {
    #[account(seeds = [POOL_CONFIG_SEED], bump = pool_config.bump, has_one = authority @ SSCREError::Unauthorized)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    #[account(init, payer = authority, space = CircuitBreaker::LEN, seeds = [CIRCUIT_BREAKER_SEED], bump)]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterOracle<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = pool_config.bump, has_one = authority @ SSCREError::Unauthorized)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct StartEpoch<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = pool_config.bump, has_one = authority @ SSCREError::Unauthorized)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    #[account(init, payer = authority, space = EpochDistribution::LEN, seeds = [EPOCH_SEED, (pool_config.current_epoch + 1).to_le_bytes().as_ref()], bump)]
    pub epoch_distribution: Account<'info, EpochDistribution>,
    #[account(mut, seeds = [CIRCUIT_BREAKER_SEED], bump = circuit_breaker.bump)]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateMerkleRoot<'info> {
    #[account(seeds = [POOL_CONFIG_SEED], bump = pool_config.bump)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    #[account(mut, seeds = [EPOCH_SEED, epoch_distribution.epoch.to_le_bytes().as_ref()], bump = epoch_distribution.bump)]
    pub epoch_distribution: Account<'info, EpochDistribution>,
    pub oracle: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = pool_config.bump)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    #[account(mut, seeds = [EPOCH_SEED, epoch_distribution.epoch.to_le_bytes().as_ref()], bump = epoch_distribution.bump)]
    pub epoch_distribution: Account<'info, EpochDistribution>,
    #[account(init_if_needed, payer = user, space = UserClaim::LEN, seeds = [USER_CLAIM_SEED, user.key().as_ref()], bump)]
    pub user_claim: Account<'info, UserClaim>,
    #[account(mut, seeds = [CIRCUIT_BREAKER_SEED], bump = circuit_breaker.bump)]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    #[account(constraint = vcoin_mint.key() == pool_config.vcoin_mint)]
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    #[account(mut, seeds = [b"pool-vault"], bump)]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, constraint = user_token_account.owner == user.key() @ SSCREError::InvalidTokenAccount, constraint = user_token_account.mint == pool_config.vcoin_mint @ SSCREError::InvalidMint)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, constraint = fee_account.owner == pool_config.fee_recipient @ SSCREError::InvalidTokenAccount, constraint = fee_account.mint == pool_config.vcoin_mint @ SSCREError::InvalidMint)]
    pub fee_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TriggerCircuitBreaker<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = pool_config.bump, has_one = authority @ SSCREError::Unauthorized)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    #[account(mut, seeds = [CIRCUIT_BREAKER_SEED], bump = circuit_breaker.bump)]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ResetCircuitBreaker<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = pool_config.bump, has_one = authority @ SSCREError::Unauthorized)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    #[account(mut, seeds = [CIRCUIT_BREAKER_SEED], bump = circuit_breaker.bump)]
    pub circuit_breaker: Account<'info, CircuitBreaker>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = pool_config.bump, has_one = authority @ SSCREError::Unauthorized)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(mut, seeds = [POOL_CONFIG_SEED], bump = pool_config.bump, has_one = authority @ SSCREError::Unauthorized)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    pub authority: Signer<'info>,
}

/// Context for accepting authority transfer (H-02 security fix)
#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(
        mut,
        seeds = [POOL_CONFIG_SEED],
        bump = pool_config.bump,
        constraint = pool_config.pending_authority == new_authority.key() @ SSCREError::NotPendingAuthority,
        constraint = pool_config.pending_authority != Pubkey::default() @ SSCREError::NoPendingTransfer
    )]
    pub pool_config: Account<'info, RewardsPoolConfig>,
    pub new_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetPoolStats<'info> {
    #[account(seeds = [POOL_CONFIG_SEED], bump = pool_config.bump)]
    pub pool_config: Account<'info, RewardsPoolConfig>,
}

#[derive(Accounts)]
pub struct GetUserStats<'info> {
    #[account(seeds = [USER_CLAIM_SEED, user_claim.user.as_ref()], bump = user_claim.bump)]
    pub user_claim: Account<'info, UserClaim>,
}
