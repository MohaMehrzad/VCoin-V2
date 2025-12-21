use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("STKGnmRk9hYxBGhH3JY8bQbqjUWCNEAVCiqUGmPjCrR");

/// VCoin Staking Protocol
/// 
/// Stake VCoin → Get veVCoin (voting power)
/// - Longer stake = More voting power
/// - Higher tier = Bonus veVCoin
/// 
/// Staking Tiers:
/// - None:     0 VCoin        → 0% fee discount, 1.0x veVCoin boost
/// - Bronze:   1,000 VCoin    → 10% fee discount, 1.1x veVCoin boost
/// - Silver:   5,000 VCoin    → 20% fee discount, 1.2x veVCoin boost
/// - Gold:     20,000 VCoin   → 30% fee discount, 1.3x veVCoin boost
/// - Platinum: 100,000 VCoin  → 50% fee discount, 1.4x veVCoin boost

pub mod constants {
    // Tier thresholds in base units (9 decimals)
    pub const BRONZE_THRESHOLD: u64 = 1_000 * 1_000_000_000;      // 1,000 VCoin
    pub const SILVER_THRESHOLD: u64 = 5_000 * 1_000_000_000;      // 5,000 VCoin
    pub const GOLD_THRESHOLD: u64 = 20_000 * 1_000_000_000;       // 20,000 VCoin
    pub const PLATINUM_THRESHOLD: u64 = 100_000 * 1_000_000_000;  // 100,000 VCoin
    
    // Lock duration limits in seconds
    pub const MIN_LOCK_DURATION: i64 = 7 * 24 * 60 * 60;          // 1 week
    pub const MAX_LOCK_DURATION: i64 = 4 * 365 * 24 * 60 * 60;    // 4 years
    pub const FOUR_YEARS_SECONDS: i64 = 4 * 365 * 24 * 60 * 60;   // 4 years
    
    // Tier boost multipliers (x1000 for precision)
    pub const TIER_BOOST_NONE: u64 = 1000;     // 1.0x
    pub const TIER_BOOST_BRONZE: u64 = 1100;   // 1.1x
    pub const TIER_BOOST_SILVER: u64 = 1200;   // 1.2x
    pub const TIER_BOOST_GOLD: u64 = 1300;     // 1.3x
    pub const TIER_BOOST_PLATINUM: u64 = 1400; // 1.4x
    
    // Fee discount basis points
    pub const FEE_DISCOUNT_NONE: u16 = 0;       // 0%
    pub const FEE_DISCOUNT_BRONZE: u16 = 1000;  // 10%
    pub const FEE_DISCOUNT_SILVER: u16 = 2000;  // 20%
    pub const FEE_DISCOUNT_GOLD: u16 = 3000;    // 30%
    pub const FEE_DISCOUNT_PLATINUM: u16 = 5000;// 50%
    
    // Seeds
    pub const STAKING_POOL_SEED: &[u8] = b"staking-pool";
    pub const USER_STAKE_SEED: &[u8] = b"user-stake";
    pub const POOL_VAULT_SEED: &[u8] = b"pool-vault";
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum StakingError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("Staking pool is paused")]
        PoolPaused,
        #[msg("Cannot stake zero tokens")]
        ZeroStakeAmount,
        #[msg("Lock duration below minimum (1 week)")]
        LockDurationTooShort,
        #[msg("Lock duration exceeds maximum (4 years)")]
        LockDurationTooLong,
        #[msg("Tokens are still locked")]
        TokensStillLocked,
        #[msg("Cannot unstake more than staked amount")]
        InsufficientStake,
        #[msg("Cannot extend lock to a shorter duration")]
        CannotShortenLock,
        #[msg("New lock end must be after current lock end")]
        InvalidLockExtension,
        #[msg("Arithmetic overflow")]
        Overflow,
        #[msg("User has no active stake")]
        NoActiveStake,
    }
}

pub mod state {
    use super::*;
    use crate::constants::*;
    
    /// Staking tier enum
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum StakingTier {
        #[default]
        None = 0,
        Bronze = 1,
        Silver = 2,
        Gold = 3,
        Platinum = 4,
    }
    
    impl StakingTier {
        pub fn from_amount(amount: u64) -> Self {
            if amount >= PLATINUM_THRESHOLD {
                StakingTier::Platinum
            } else if amount >= GOLD_THRESHOLD {
                StakingTier::Gold
            } else if amount >= SILVER_THRESHOLD {
                StakingTier::Silver
            } else if amount >= BRONZE_THRESHOLD {
                StakingTier::Bronze
            } else {
                StakingTier::None
            }
        }
        
        pub fn boost_multiplier(&self) -> u64 {
            match self {
                StakingTier::None => TIER_BOOST_NONE,
                StakingTier::Bronze => TIER_BOOST_BRONZE,
                StakingTier::Silver => TIER_BOOST_SILVER,
                StakingTier::Gold => TIER_BOOST_GOLD,
                StakingTier::Platinum => TIER_BOOST_PLATINUM,
            }
        }
        
        pub fn fee_discount_bps(&self) -> u16 {
            match self {
                StakingTier::None => FEE_DISCOUNT_NONE,
                StakingTier::Bronze => FEE_DISCOUNT_BRONZE,
                StakingTier::Silver => FEE_DISCOUNT_SILVER,
                StakingTier::Gold => FEE_DISCOUNT_GOLD,
                StakingTier::Platinum => FEE_DISCOUNT_PLATINUM,
            }
        }
        
        pub fn as_u8(&self) -> u8 {
            match self {
                StakingTier::None => 0,
                StakingTier::Bronze => 1,
                StakingTier::Silver => 2,
                StakingTier::Gold => 3,
                StakingTier::Platinum => 4,
            }
        }
    }
    
    /// Staking Pool Account (Singleton PDA)
    #[account]
    #[derive(Default)]
    pub struct StakingPool {
        /// Admin authority
        pub authority: Pubkey,
        /// VCoin mint address
        pub vcoin_mint: Pubkey,
        /// veVCoin mint address
        pub vevcoin_mint: Pubkey,
        /// veVCoin program address
        pub vevcoin_program: Pubkey,
        /// Pool vault for staked VCoin
        pub pool_vault: Pubkey,
        /// Total VCoin staked in the pool
        pub total_staked: u64,
        /// Total number of stakers
        pub total_stakers: u64,
        /// Whether the pool is paused
        pub paused: bool,
        /// Bump seed for PDA
        pub bump: u8,
        /// Vault bump seed
        pub vault_bump: u8,
    }
    
    impl StakingPool {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // vcoin_mint
            32 + // vevcoin_mint
            32 + // vevcoin_program
            32 + // pool_vault
            8 +  // total_staked
            8 +  // total_stakers
            1 +  // paused
            1 +  // bump
            1;   // vault_bump
    }
    
    /// User Stake Account (PDA per user)
    #[account]
    #[derive(Default)]
    pub struct UserStake {
        /// Owner of this stake
        pub owner: Pubkey,
        /// Amount of VCoin staked
        pub staked_amount: u64,
        /// Lock duration in seconds
        pub lock_duration: i64,
        /// Timestamp when lock ends
        pub lock_end: i64,
        /// When the stake was created
        pub stake_start: i64,
        /// Current staking tier
        pub tier: u8,
        /// Current veVCoin amount minted
        pub ve_vcoin_amount: u64,
        /// Bump seed for PDA
        pub bump: u8,
    }
    
    impl UserStake {
        pub const LEN: usize = 8 + // discriminator
            32 + // owner
            8 +  // staked_amount
            8 +  // lock_duration
            8 +  // lock_end
            8 +  // stake_start
            1 +  // tier
            8 +  // ve_vcoin_amount
            1;   // bump
    }
}

use constants::*;
use errors::*;
use state::*;

/// Calculate veVCoin amount based on staked amount, lock duration, and tier
/// Formula: ve_vcoin = staked_amount * (lock_duration / 4_years) * tier_boost
fn calculate_vevcoin(staked_amount: u64, lock_duration: i64, tier: StakingTier) -> Result<u64> {
    // ve_vcoin = staked_amount * (lock_duration / 4_years) * tier_boost
    // To avoid floating point, we multiply first then divide
    // tier_boost is already multiplied by 1000
    
    let duration_factor = (lock_duration as u128) * 1000 / (FOUR_YEARS_SECONDS as u128);
    let tier_boost = tier.boost_multiplier() as u128;
    
    let ve_vcoin = (staked_amount as u128)
        .checked_mul(duration_factor)
        .ok_or(StakingError::Overflow)?
        .checked_mul(tier_boost)
        .ok_or(StakingError::Overflow)?
        .checked_div(1_000_000) // Divide by 1000 * 1000 to normalize
        .ok_or(StakingError::Overflow)?;
    
    Ok(ve_vcoin as u64)
}

#[program]
pub mod staking_protocol {
    use super::*;

    /// Initialize the staking pool
    pub fn initialize_pool(
        ctx: Context<InitializePool>,
        vevcoin_program: Pubkey,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        pool.authority = ctx.accounts.authority.key();
        pool.vcoin_mint = ctx.accounts.vcoin_mint.key();
        pool.vevcoin_mint = ctx.accounts.vevcoin_mint.key();
        pool.vevcoin_program = vevcoin_program;
        pool.pool_vault = ctx.accounts.pool_vault.key();
        pool.total_staked = 0;
        pool.total_stakers = 0;
        pool.paused = false;
        pool.bump = ctx.bumps.pool;
        pool.vault_bump = ctx.bumps.pool_vault;
        
        msg!("Staking pool initialized");
        msg!("VCoin Mint: {}", pool.vcoin_mint);
        msg!("veVCoin Mint: {}", pool.vevcoin_mint);
        
        Ok(())
    }

    /// Stake VCoin with a lock duration
    /// Mints veVCoin proportional to stake amount, lock duration, and tier
    pub fn stake(
        ctx: Context<Stake>,
        amount: u64,
        lock_duration: i64,
    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        let user_stake = &mut ctx.accounts.user_stake;
        
        // Validations
        require!(!pool.paused, StakingError::PoolPaused);
        require!(amount > 0, StakingError::ZeroStakeAmount);
        require!(lock_duration >= MIN_LOCK_DURATION, StakingError::LockDurationTooShort);
        require!(lock_duration <= MAX_LOCK_DURATION, StakingError::LockDurationTooLong);
        
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        
        // Calculate new total stake and tier
        let new_staked_amount = user_stake.staked_amount
            .checked_add(amount)
            .ok_or(StakingError::Overflow)?;
        let new_tier = StakingTier::from_amount(new_staked_amount);
        
        // Calculate lock end
        let lock_end = now.checked_add(lock_duration).ok_or(StakingError::Overflow)?;
        
        // For existing stakes, new lock must not be shorter
        if user_stake.staked_amount > 0 && lock_end < user_stake.lock_end {
            return Err(StakingError::InvalidLockExtension.into());
        }
        
        // Calculate veVCoin to mint
        // If adding to existing stake, calculate delta
        let old_vevcoin = user_stake.ve_vcoin_amount;
        let new_vevcoin = calculate_vevcoin(new_staked_amount, lock_duration, new_tier)?;
        let vevcoin_to_mint = new_vevcoin.checked_sub(old_vevcoin).unwrap_or(0);
        
        // Transfer VCoin to pool vault
        token_2022::transfer_checked(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                token_2022::TransferChecked {
                    from: ctx.accounts.user_vcoin_account.to_account_info(),
                    to: ctx.accounts.pool_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                    mint: ctx.accounts.vcoin_mint.to_account_info(),
                },
            ),
            amount,
            ctx.accounts.vcoin_mint.decimals,
        )?;
        
        // Update user stake
        let is_new_staker = user_stake.staked_amount == 0;
        user_stake.owner = ctx.accounts.user.key();
        user_stake.staked_amount = new_staked_amount;
        user_stake.lock_duration = lock_duration;
        user_stake.lock_end = lock_end;
        user_stake.tier = new_tier.as_u8();
        user_stake.ve_vcoin_amount = new_vevcoin;
        
        if is_new_staker {
            user_stake.stake_start = now;
            user_stake.bump = ctx.bumps.user_stake;
            pool.total_stakers = pool.total_stakers.checked_add(1).ok_or(StakingError::Overflow)?;
        }
        
        // Update pool
        pool.total_staked = pool.total_staked.checked_add(amount).ok_or(StakingError::Overflow)?;
        
        msg!("Staked {} VCoin", amount);
        msg!("Lock duration: {} seconds", lock_duration);
        msg!("Tier: {:?}", new_tier.as_u8());
        msg!("veVCoin minted: {}", vevcoin_to_mint);
        msg!("Total veVCoin: {}", new_vevcoin);
        
        Ok(())
    }

    /// Extend lock duration to increase veVCoin
    pub fn extend_lock(
        ctx: Context<ExtendLock>,
        new_lock_duration: i64,
    ) -> Result<()> {
        let user_stake = &mut ctx.accounts.user_stake;
        
        require!(user_stake.staked_amount > 0, StakingError::NoActiveStake);
        require!(new_lock_duration >= MIN_LOCK_DURATION, StakingError::LockDurationTooShort);
        require!(new_lock_duration <= MAX_LOCK_DURATION, StakingError::LockDurationTooLong);
        
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        
        // Calculate new lock end
        let new_lock_end = now.checked_add(new_lock_duration).ok_or(StakingError::Overflow)?;
        
        // New lock end must be after current lock end
        require!(new_lock_end > user_stake.lock_end, StakingError::CannotShortenLock);
        
        // Calculate new veVCoin
        let tier = StakingTier::from_amount(user_stake.staked_amount);
        let old_vevcoin = user_stake.ve_vcoin_amount;
        let new_vevcoin = calculate_vevcoin(user_stake.staked_amount, new_lock_duration, tier)?;
        let vevcoin_to_mint = new_vevcoin.checked_sub(old_vevcoin).unwrap_or(0);
        
        // Update stake
        user_stake.lock_duration = new_lock_duration;
        user_stake.lock_end = new_lock_end;
        user_stake.ve_vcoin_amount = new_vevcoin;
        
        msg!("Extended lock to {} seconds", new_lock_duration);
        msg!("New lock end: {}", new_lock_end);
        msg!("Additional veVCoin: {}", vevcoin_to_mint);
        msg!("Total veVCoin: {}", new_vevcoin);
        
        Ok(())
    }

    /// Unstake VCoin after lock expires
    /// Burns all veVCoin
    pub fn unstake(
        ctx: Context<Unstake>,
        amount: u64,
    ) -> Result<()> {
        // Get values for validation first
        let staked_amount = ctx.accounts.user_stake.staked_amount;
        let lock_end = ctx.accounts.user_stake.lock_end;
        let ve_vcoin_amount = ctx.accounts.user_stake.ve_vcoin_amount;
        
        require!(staked_amount > 0, StakingError::NoActiveStake);
        require!(amount > 0, StakingError::ZeroStakeAmount);
        require!(amount <= staked_amount, StakingError::InsufficientStake);
        
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        
        // Check lock has expired
        require!(now >= lock_end, StakingError::TokensStillLocked);
        
        // Calculate new stake and tier
        let new_staked_amount = staked_amount.checked_sub(amount).ok_or(StakingError::Overflow)?;
        let new_tier = StakingTier::from_amount(new_staked_amount);
        
        // Calculate new veVCoin (0 if fully unstaking)
        let new_vevcoin = if new_staked_amount > 0 {
            // Maintain proportional veVCoin for remaining stake
            let remaining_ratio = (new_staked_amount as u128) * 1000 / (staked_amount as u128);
            (ve_vcoin_amount as u128 * remaining_ratio / 1000) as u64
        } else {
            0
        };
        let vevcoin_to_burn = ve_vcoin_amount.checked_sub(new_vevcoin).unwrap_or(0);
        
        // Get bump from context
        let pool_bump = ctx.bumps.pool;
        let current_total_stakers = ctx.accounts.pool.total_stakers;
        let current_total_staked = ctx.accounts.pool.total_staked;
        
        // Transfer VCoin back to user
        let seeds = &[
            STAKING_POOL_SEED,
            &[pool_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        
        token_2022::transfer_checked(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
                token_2022::TransferChecked {
                    from: ctx.accounts.pool_vault.to_account_info(),
                    to: ctx.accounts.user_vcoin_account.to_account_info(),
                    authority: ctx.accounts.pool.to_account_info(),
                    mint: ctx.accounts.vcoin_mint.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            ctx.accounts.vcoin_mint.decimals,
        )?;
        
        // Update state after CPI
        let user_stake = &mut ctx.accounts.user_stake;
        let pool = &mut ctx.accounts.pool;
        
        // Update user stake
        let is_full_unstake = new_staked_amount == 0;
        user_stake.staked_amount = new_staked_amount;
        user_stake.tier = new_tier.as_u8();
        user_stake.ve_vcoin_amount = new_vevcoin;
        
        if is_full_unstake {
            user_stake.lock_duration = 0;
            user_stake.lock_end = 0;
            pool.total_stakers = current_total_stakers.checked_sub(1).ok_or(StakingError::Overflow)?;
        }
        
        // Update pool
        pool.total_staked = current_total_staked.checked_sub(amount).ok_or(StakingError::Overflow)?;
        
        msg!("Unstaked {} VCoin", amount);
        msg!("veVCoin burned: {}", vevcoin_to_burn);
        msg!("Remaining stake: {}", new_staked_amount);
        
        Ok(())
    }

    /// Update user's tier based on current stake
    pub fn update_tier(ctx: Context<UpdateTier>) -> Result<()> {
        let user_stake = &mut ctx.accounts.user_stake;
        
        require!(user_stake.staked_amount > 0, StakingError::NoActiveStake);
        
        let new_tier = StakingTier::from_amount(user_stake.staked_amount);
        let old_tier = user_stake.tier;
        
        user_stake.tier = new_tier.as_u8();
        
        // Recalculate veVCoin with new tier
        let new_vevcoin = calculate_vevcoin(
            user_stake.staked_amount,
            user_stake.lock_duration,
            new_tier,
        )?;
        user_stake.ve_vcoin_amount = new_vevcoin;
        
        msg!("Tier updated from {} to {}", old_tier, new_tier.as_u8());
        msg!("veVCoin updated to: {}", new_vevcoin);
        
        Ok(())
    }

    /// Pause/unpause the staking pool
    pub fn set_paused(ctx: Context<AdminAction>, paused: bool) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        require!(
            ctx.accounts.authority.key() == pool.authority,
            StakingError::Unauthorized
        );
        
        pool.paused = paused;
        
        msg!("Pool paused status: {}", paused);
        
        Ok(())
    }

    /// Update the pool authority
    pub fn update_authority(ctx: Context<AdminAction>, new_authority: Pubkey) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        
        require!(
            ctx.accounts.authority.key() == pool.authority,
            StakingError::Unauthorized
        );
        
        pool.authority = new_authority;
        
        msg!("Authority updated to: {}", new_authority);
        
        Ok(())
    }

    /// Get user's staking info (view function)
    pub fn get_stake_info(ctx: Context<GetStakeInfo>) -> Result<UserStakeInfo> {
        let user_stake = &ctx.accounts.user_stake;
        let clock = Clock::get()?;
        
        Ok(UserStakeInfo {
            staked_amount: user_stake.staked_amount,
            lock_end: user_stake.lock_end,
            tier: user_stake.tier,
            ve_vcoin_amount: user_stake.ve_vcoin_amount,
            is_locked: clock.unix_timestamp < user_stake.lock_end,
            fee_discount_bps: StakingTier::from_amount(user_stake.staked_amount).fee_discount_bps(),
        })
    }
}

/// Return type for get_stake_info
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserStakeInfo {
    pub staked_amount: u64,
    pub lock_end: i64,
    pub tier: u8,
    pub ve_vcoin_amount: u64,
    pub is_locked: bool,
    pub fee_discount_bps: u16,
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = StakingPool::LEN,
        seeds = [STAKING_POOL_SEED],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    /// VCoin mint
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// veVCoin mint
    pub vevcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// Pool vault for staked VCoin
    #[account(
        init,
        payer = authority,
        seeds = [POOL_VAULT_SEED],
        bump,
        token::mint = vcoin_mint,
        token::authority = pool,
        token::token_program = token_program,
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [STAKING_POOL_SEED],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = UserStake::LEN,
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,
    
    /// VCoin mint
    #[account(constraint = vcoin_mint.key() == pool.vcoin_mint)]
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// User's VCoin token account
    #[account(mut)]
    pub user_vcoin_account: InterfaceAccount<'info, TokenAccount>,
    
    /// Pool vault for staked VCoin
    #[account(
        mut,
        seeds = [POOL_VAULT_SEED],
        bump
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExtendLock<'info> {
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump,
        constraint = user_stake.owner == user.key()
    )]
    pub user_stake: Account<'info, UserStake>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [STAKING_POOL_SEED],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
    
    #[account(
        mut,
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump,
        constraint = user_stake.owner == user.key()
    )]
    pub user_stake: Account<'info, UserStake>,
    
    /// VCoin mint
    #[account(constraint = vcoin_mint.key() == pool.vcoin_mint)]
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// User's VCoin token account
    #[account(mut)]
    pub user_vcoin_account: InterfaceAccount<'info, TokenAccount>,
    
    /// Pool vault
    #[account(
        mut,
        seeds = [POOL_VAULT_SEED],
        bump
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct UpdateTier<'info> {
    pub user: Signer<'info>,
    
    #[account(
        mut,
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump,
        constraint = user_stake.owner == user.key()
    )]
    pub user_stake: Account<'info, UserStake>,
}

#[derive(Accounts)]
pub struct AdminAction<'info> {
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [STAKING_POOL_SEED],
        bump
    )]
    pub pool: Account<'info, StakingPool>,
}

#[derive(Accounts)]
pub struct GetStakeInfo<'info> {
    /// CHECK: Just for PDA derivation
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [USER_STAKE_SEED, user.key().as_ref()],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,
}


