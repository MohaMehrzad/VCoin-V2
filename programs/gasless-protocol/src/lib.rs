use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("FZyRfP5qeChTZ9z2M2aHkXQ8QLHbRQ5aK7dJ2BpPtYXj");

/// Gasless Layer - Paymaster + Session Keys for Zero-Friction UX
/// 
/// Enables gasless transactions for ViWoApp users.
/// 
/// Key Features:
/// - Paymaster: Platform subsidizes transaction fees
/// - Session Keys: Temporary 24h signing keys with limited scope
/// - VCoin Deduction: Automatic fee deduction from VCoin balance
/// - SSCRE Integration: 1% deduction from reward claims
/// 
/// Fee Deduction Methods:
/// 1. Platform Subsidized (onboarding, governance)
/// 2. VCoin Deduction (tips, transfers)
/// 3. Reward Deduction (1% from SSCRE claims)
/// 
/// Session Key Scopes:
/// - Tip actions
/// - Vouch actions
/// - Content interactions
/// - Governance voting

pub mod constants {
    /// Seeds
    pub const GASLESS_CONFIG_SEED: &[u8] = b"gasless-config";
    pub const SESSION_KEY_SEED: &[u8] = b"session-key";
    pub const USER_GASLESS_SEED: &[u8] = b"user-gasless";
    pub const FEE_VAULT_SEED: &[u8] = b"fee-vault";
    pub const DAILY_BUDGET_SEED: &[u8] = b"daily-budget";
    
    /// Session configuration
    pub const SESSION_DURATION: i64 = 24 * 60 * 60;   // 24 hours
    pub const MAX_SESSION_ACTIONS: u32 = 1000;        // Max actions per session
    pub const MAX_SESSION_SPEND: u64 = 100_000_000_000_000; // 100,000 VCoin max per session
    
    /// Fee configuration
    pub const DEFAULT_SOL_FEE: u64 = 5_000;          // 0.000005 SOL per tx
    pub const VCOIN_FEE_MULTIPLIER: u64 = 100;      // 100x VCoin equivalent
    pub const SSCRE_DEDUCTION_BPS: u16 = 100;       // 1% from SSCRE claims
    
    /// Daily budget
    pub const DAILY_SUBSIDY_BUDGET_SOL: u64 = 10_000_000_000; // 10 SOL per day
    pub const MAX_SUBSIDIZED_TX_PER_USER: u32 = 50; // Max 50 free tx per user per day
    
    /// Action scope bits
    pub const SCOPE_TIP: u16 = 1 << 0;
    pub const SCOPE_VOUCH: u16 = 1 << 1;
    pub const SCOPE_CONTENT: u16 = 1 << 2;
    pub const SCOPE_GOVERNANCE: u16 = 1 << 3;
    pub const SCOPE_TRANSFER: u16 = 1 << 4;
    pub const SCOPE_STAKE: u16 = 1 << 5;
    pub const SCOPE_CLAIM: u16 = 1 << 6;
    pub const SCOPE_FOLLOW: u16 = 1 << 7;
    pub const SCOPE_ALL: u16 = 0xFFFF;
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum GaslessError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("Gasless Protocol is paused")]
        ProtocolPaused,
        #[msg("Session key expired")]
        SessionExpired,
        #[msg("Session key revoked")]
        SessionRevoked,
        #[msg("Action not in session scope")]
        ActionNotInScope,
        #[msg("Session action limit exceeded")]
        SessionActionLimitExceeded,
        #[msg("Session spend limit exceeded")]
        SessionSpendLimitExceeded,
        #[msg("Daily subsidy budget exceeded")]
        DailyBudgetExceeded,
        #[msg("User daily limit exceeded")]
        UserDailyLimitExceeded,
        #[msg("Insufficient VCoin balance for fee")]
        InsufficientVCoinBalance,
        #[msg("Invalid session key")]
        InvalidSessionKey,
        #[msg("Session already exists")]
        SessionAlreadyExists,
        #[msg("Fee deduction method not allowed")]
        FeeMethodNotAllowed,
        #[msg("Invalid action type")]
        InvalidActionType,
        #[msg("Arithmetic overflow")]
        Overflow,
    }
}

pub mod state {
    use super::*;
    use crate::constants::*;
    
    /// Fee deduction method
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum FeeMethod {
        #[default]
        PlatformSubsidized,
        VCoinDeduction,
        SSCREDeduction,
    }
    
    /// Global gasless configuration
    #[account]
    #[derive(Default)]
    pub struct GaslessConfig {
        /// Admin authority
        pub authority: Pubkey,
        /// Fee payer wallet (paymaster)
        pub fee_payer: Pubkey,
        /// VCoin mint
        pub vcoin_mint: Pubkey,
        /// Fee vault for VCoin fees
        pub fee_vault: Pubkey,
        /// SSCRE program for reward deduction
        pub sscre_program: Pubkey,
        /// Daily subsidy budget (SOL lamports)
        pub daily_subsidy_budget: u64,
        /// SOL fee per transaction (lamports)
        pub sol_fee_per_tx: u64,
        /// VCoin fee multiplier
        pub vcoin_fee_multiplier: u64,
        /// SSCRE deduction rate (bps)
        pub sscre_deduction_bps: u16,
        /// Max subsidized tx per user per day
        pub max_subsidized_per_user: u32,
        /// Total transactions subsidized
        pub total_subsidized_tx: u64,
        /// Total SOL spent on subsidies
        pub total_sol_spent: u64,
        /// Total VCoin collected as fees
        pub total_vcoin_collected: u64,
        /// Whether protocol is paused
        pub paused: bool,
        /// Current day (for daily budget reset)
        pub current_day: u32,
        /// Day's spent budget
        pub day_spent: u64,
        /// PDA bump
        pub bump: u8,
    }
    
    impl GaslessConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // fee_payer
            32 + // vcoin_mint
            32 + // fee_vault
            32 + // sscre_program
            8 +  // daily_subsidy_budget
            8 +  // sol_fee_per_tx
            8 +  // vcoin_fee_multiplier
            2 +  // sscre_deduction_bps
            4 +  // max_subsidized_per_user
            8 +  // total_subsidized_tx
            8 +  // total_sol_spent
            8 +  // total_vcoin_collected
            1 +  // paused
            4 +  // current_day
            8 +  // day_spent
            1;   // bump
        
        /// Get current day number
        pub fn get_day_number(timestamp: i64) -> u32 {
            (timestamp / 86400) as u32
        }
        
        /// Check if daily budget reset needed
        pub fn should_reset_daily_budget(&self, current_timestamp: i64) -> bool {
            Self::get_day_number(current_timestamp) > self.current_day
        }
    }
    
    /// Session key for temporary signing
    #[account]
    #[derive(Default)]
    pub struct SessionKey {
        /// User who owns this session
        pub user: Pubkey,
        /// The session key pubkey
        pub session_pubkey: Pubkey,
        /// Allowed action scope bitmap
        pub scope: u16,
        /// Session creation timestamp
        pub created_at: i64,
        /// Session expiry timestamp
        pub expires_at: i64,
        /// Actions executed in this session
        pub actions_used: u32,
        /// Max actions allowed
        pub max_actions: u32,
        /// VCoin spent via this session
        pub vcoin_spent: u64,
        /// Max VCoin spend allowed
        pub max_spend: u64,
        /// Whether session is revoked
        pub is_revoked: bool,
        /// Last action timestamp
        pub last_action_at: i64,
        /// Fee method for this session
        pub fee_method: FeeMethod,
        /// PDA bump
        pub bump: u8,
    }
    
    impl SessionKey {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            32 + // session_pubkey
            2 +  // scope
            8 +  // created_at
            8 +  // expires_at
            4 +  // actions_used
            4 +  // max_actions
            8 +  // vcoin_spent
            8 +  // max_spend
            1 +  // is_revoked
            8 +  // last_action_at
            1 +  // fee_method
            1;   // bump
        
        /// Check if action is in scope
        pub fn is_action_in_scope(&self, action_type: u16) -> bool {
            (self.scope & action_type) != 0
        }
        
        /// Check if session is valid
        pub fn is_valid(&self, current_timestamp: i64) -> bool {
            !self.is_revoked && 
            current_timestamp <= self.expires_at &&
            self.actions_used < self.max_actions
        }
    }
    
    /// User gasless statistics
    #[account]
    #[derive(Default)]
    pub struct UserGaslessStats {
        /// User wallet
        pub user: Pubkey,
        /// Total gasless transactions
        pub total_gasless_tx: u64,
        /// Total subsidized transactions
        pub total_subsidized: u64,
        /// Total VCoin paid as fees
        pub total_vcoin_fees: u64,
        /// Total SSCRE deductions
        pub total_sscre_deductions: u64,
        /// Sessions created
        pub sessions_created: u32,
        /// Active session (if any)
        pub active_session: Pubkey,
        /// Current day for daily limits
        pub current_day: u32,
        /// Today's subsidized tx count
        pub today_subsidized: u32,
        /// First gasless tx timestamp
        pub first_gasless_at: i64,
        /// Last gasless tx timestamp
        pub last_gasless_at: i64,
        /// PDA bump
        pub bump: u8,
    }
    
    impl UserGaslessStats {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            8 +  // total_gasless_tx
            8 +  // total_subsidized
            8 +  // total_vcoin_fees
            8 +  // total_sscre_deductions
            4 +  // sessions_created
            32 + // active_session
            4 +  // current_day
            4 +  // today_subsidized
            8 +  // first_gasless_at
            8 +  // last_gasless_at
            1;   // bump
        
        /// Reset daily limits if new day
        pub fn check_daily_reset(&mut self, current_timestamp: i64) {
            let day = GaslessConfig::get_day_number(current_timestamp);
            if day > self.current_day {
                self.current_day = day;
                self.today_subsidized = 0;
            }
        }
    }
    
    /// Daily budget tracking
    #[account]
    #[derive(Default)]
    pub struct DailyBudget {
        /// Day number
        pub day: u32,
        /// Total budget allocated
        pub total_budget: u64,
        /// Amount spent
        pub spent: u64,
        /// Transactions subsidized
        pub tx_count: u64,
        /// Unique users subsidized
        pub unique_users: u32,
        /// PDA bump
        pub bump: u8,
    }
    
    impl DailyBudget {
        pub const LEN: usize = 8 + // discriminator
            4 +  // day
            8 +  // total_budget
            8 +  // spent
            8 +  // tx_count
            4 +  // unique_users
            1;   // bump
    }
}

pub mod events {
    use super::*;
    use crate::state::FeeMethod;
    
    #[event]
    pub struct GaslessConfigInitialized {
        pub authority: Pubkey,
        pub fee_payer: Pubkey,
        pub daily_budget: u64,
    }
    
    #[event]
    pub struct SessionKeyCreated {
        pub user: Pubkey,
        pub session_pubkey: Pubkey,
        pub scope: u16,
        pub expires_at: i64,
        pub fee_method: u8,
    }
    
    #[event]
    pub struct SessionKeyRevoked {
        pub user: Pubkey,
        pub session_pubkey: Pubkey,
        pub actions_used: u32,
    }
    
    #[event]
    pub struct SessionActionExecuted {
        pub user: Pubkey,
        pub session_pubkey: Pubkey,
        pub action_type: u16,
        pub fee_method: u8,
        pub fee_amount: u64,
    }
    
    #[event]
    pub struct DailyBudgetReset {
        pub day: u32,
        pub previous_spent: u64,
        pub new_budget: u64,
    }
    
    #[event]
    pub struct FeeCollected {
        pub user: Pubkey,
        pub fee_method: u8,
        pub amount: u64,
        pub is_vcoin: bool,
    }
}

use constants::*;
use errors::*;
use state::*;
use events::*;

#[program]
pub mod gasless_protocol {
    use super::*;

    /// Initialize gasless configuration
    pub fn initialize(
        ctx: Context<Initialize>,
        fee_payer: Pubkey,
        daily_budget: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.authority = ctx.accounts.authority.key();
        config.fee_payer = fee_payer;
        config.vcoin_mint = ctx.accounts.vcoin_mint.key();
        config.fee_vault = ctx.accounts.fee_vault.key();
        config.sscre_program = Pubkey::default();
        config.daily_subsidy_budget = daily_budget;
        config.sol_fee_per_tx = DEFAULT_SOL_FEE;
        config.vcoin_fee_multiplier = VCOIN_FEE_MULTIPLIER;
        config.sscre_deduction_bps = SSCRE_DEDUCTION_BPS;
        config.max_subsidized_per_user = MAX_SUBSIDIZED_TX_PER_USER;
        config.total_subsidized_tx = 0;
        config.total_sol_spent = 0;
        config.total_vcoin_collected = 0;
        config.paused = false;
        config.current_day = GaslessConfig::get_day_number(Clock::get()?.unix_timestamp);
        config.day_spent = 0;
        config.bump = ctx.bumps.config;
        
        emit!(GaslessConfigInitialized {
            authority: config.authority,
            fee_payer,
            daily_budget,
        });
        
        msg!("Gasless Protocol initialized with {} lamports daily budget", daily_budget);
        Ok(())
    }
    
    /// Create a session key for gasless transactions
    pub fn create_session_key(
        ctx: Context<CreateSessionKey>,
        session_pubkey: Pubkey,
        scope: u16,
        duration_seconds: i64,
        max_actions: u32,
        max_spend: u64,
        fee_method: u8,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let session = &mut ctx.accounts.session_key;
        let user_stats = &mut ctx.accounts.user_stats;
        
        require!(!config.paused, GaslessError::ProtocolPaused);
        
        let clock = Clock::get()?;
        
        // Limit session duration
        let duration = if duration_seconds > 0 && duration_seconds <= SESSION_DURATION {
            duration_seconds
        } else {
            SESSION_DURATION
        };
        
        // Limit max actions
        let actions = if max_actions > 0 && max_actions <= MAX_SESSION_ACTIONS {
            max_actions
        } else {
            MAX_SESSION_ACTIONS
        };
        
        // Limit max spend
        let spend = if max_spend > 0 && max_spend <= MAX_SESSION_SPEND {
            max_spend
        } else {
            MAX_SESSION_SPEND
        };
        
        // Parse fee method
        let method = match fee_method {
            0 => FeeMethod::PlatformSubsidized,
            1 => FeeMethod::VCoinDeduction,
            2 => FeeMethod::SSCREDeduction,
            _ => FeeMethod::VCoinDeduction,
        };
        
        session.user = ctx.accounts.user.key();
        session.session_pubkey = session_pubkey;
        session.scope = scope;
        session.created_at = clock.unix_timestamp;
        session.expires_at = clock.unix_timestamp + duration;
        session.actions_used = 0;
        session.max_actions = actions;
        session.vcoin_spent = 0;
        session.max_spend = spend;
        session.is_revoked = false;
        session.last_action_at = 0;
        session.fee_method = method;
        session.bump = ctx.bumps.session_key;
        
        // Update user stats
        user_stats.user = ctx.accounts.user.key();
        user_stats.sessions_created = user_stats.sessions_created.saturating_add(1);
        user_stats.active_session = session_pubkey;
        user_stats.bump = ctx.bumps.user_stats;
        
        emit!(SessionKeyCreated {
            user: session.user,
            session_pubkey,
            scope,
            expires_at: session.expires_at,
            fee_method,
        });
        
        msg!("Session key created: scope={:#06x}, expires in {}s", scope, duration);
        Ok(())
    }
    
    /// Execute an action using session key
    pub fn execute_session_action(
        ctx: Context<ExecuteSessionAction>,
        action_type: u16,
        spend_amount: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let session = &mut ctx.accounts.session_key;
        let user_stats = &mut ctx.accounts.user_stats;
        
        require!(!config.paused, GaslessError::ProtocolPaused);
        
        let clock = Clock::get()?;
        
        // Check session validity
        require!(session.is_valid(clock.unix_timestamp), GaslessError::SessionExpired);
        require!(session.is_action_in_scope(action_type), GaslessError::ActionNotInScope);
        
        // Check action limit
        require!(
            session.actions_used < session.max_actions,
            GaslessError::SessionActionLimitExceeded
        );
        
        // Check spend limit
        require!(
            session.vcoin_spent.saturating_add(spend_amount) <= session.max_spend,
            GaslessError::SessionSpendLimitExceeded
        );
        
        // Handle daily budget for subsidized transactions
        if matches!(session.fee_method, FeeMethod::PlatformSubsidized) {
            // Reset daily budget if needed
            if config.should_reset_daily_budget(clock.unix_timestamp) {
                emit!(DailyBudgetReset {
                    day: config.current_day,
                    previous_spent: config.day_spent,
                    new_budget: config.daily_subsidy_budget,
                });
                config.current_day = GaslessConfig::get_day_number(clock.unix_timestamp);
                config.day_spent = 0;
            }
            
            // Check daily budget
            require!(
                config.day_spent.saturating_add(config.sol_fee_per_tx) <= config.daily_subsidy_budget,
                GaslessError::DailyBudgetExceeded
            );
            
            // Check user daily limit
            user_stats.check_daily_reset(clock.unix_timestamp);
            require!(
                user_stats.today_subsidized < config.max_subsidized_per_user,
                GaslessError::UserDailyLimitExceeded
            );
            
            // Update budget tracking
            config.day_spent = config.day_spent.saturating_add(config.sol_fee_per_tx);
            config.total_sol_spent = config.total_sol_spent.saturating_add(config.sol_fee_per_tx);
            config.total_subsidized_tx = config.total_subsidized_tx.saturating_add(1);
            user_stats.today_subsidized = user_stats.today_subsidized.saturating_add(1);
            user_stats.total_subsidized = user_stats.total_subsidized.saturating_add(1);
            
            emit!(FeeCollected {
                user: session.user,
                fee_method: 0,
                amount: config.sol_fee_per_tx,
                is_vcoin: false,
            });
        }
        
        // Update session
        session.actions_used = session.actions_used.saturating_add(1);
        session.vcoin_spent = session.vcoin_spent.saturating_add(spend_amount);
        session.last_action_at = clock.unix_timestamp;
        
        // Update user stats
        user_stats.total_gasless_tx = user_stats.total_gasless_tx.saturating_add(1);
        if user_stats.first_gasless_at == 0 {
            user_stats.first_gasless_at = clock.unix_timestamp;
        }
        user_stats.last_gasless_at = clock.unix_timestamp;
        
        emit!(SessionActionExecuted {
            user: session.user,
            session_pubkey: session.session_pubkey,
            action_type,
            fee_method: session.fee_method as u8,
            fee_amount: spend_amount,
        });
        
        msg!("Session action executed: type={:#06x}, spend={}", action_type, spend_amount);
        Ok(())
    }
    
    /// Deduct VCoin fee for gasless transaction
    pub fn deduct_vcoin_fee(
        ctx: Context<DeductVCoinFee>,
        amount: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let user_stats = &mut ctx.accounts.user_stats;
        
        require!(!config.paused, GaslessError::ProtocolPaused);
        
        let clock = Clock::get()?;
        
        // Calculate VCoin fee equivalent
        let vcoin_fee = config.sol_fee_per_tx
            .saturating_mul(config.vcoin_fee_multiplier);
        
        let fee_to_deduct = if amount > 0 { amount } else { vcoin_fee };
        
        // Transfer VCoin from user to fee vault
        token_2022::transfer_checked(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                token_2022::TransferChecked {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.fee_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                    mint: ctx.accounts.vcoin_mint.to_account_info(),
                },
            ),
            fee_to_deduct,
            ctx.accounts.vcoin_mint.decimals,
        )?;
        
        // Update stats
        config.total_vcoin_collected = config.total_vcoin_collected.saturating_add(fee_to_deduct);
        user_stats.total_vcoin_fees = user_stats.total_vcoin_fees.saturating_add(fee_to_deduct);
        user_stats.total_gasless_tx = user_stats.total_gasless_tx.saturating_add(1);
        user_stats.last_gasless_at = clock.unix_timestamp;
        user_stats.bump = ctx.bumps.user_stats;
        
        emit!(FeeCollected {
            user: ctx.accounts.user.key(),
            fee_method: 1,
            amount: fee_to_deduct,
            is_vcoin: true,
        });
        
        msg!("VCoin fee deducted: {} VCoin", fee_to_deduct);
        Ok(())
    }
    
    /// Revoke a session key
    pub fn revoke_session_key(
        ctx: Context<RevokeSessionKey>,
    ) -> Result<()> {
        let session = &mut ctx.accounts.session_key;
        let user_stats = &mut ctx.accounts.user_stats;
        
        require!(!session.is_revoked, GaslessError::SessionRevoked);
        
        session.is_revoked = true;
        
        // Clear active session if this was it
        if user_stats.active_session == session.session_pubkey {
            user_stats.active_session = Pubkey::default();
        }
        
        emit!(SessionKeyRevoked {
            user: session.user,
            session_pubkey: session.session_pubkey,
            actions_used: session.actions_used,
        });
        
        msg!("Session key revoked after {} actions", session.actions_used);
        Ok(())
    }
    
    /// Update fee configuration
    pub fn update_fee_config(
        ctx: Context<UpdateConfig>,
        sol_fee_per_tx: u64,
        vcoin_fee_multiplier: u64,
        sscre_deduction_bps: u16,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.sol_fee_per_tx = sol_fee_per_tx;
        config.vcoin_fee_multiplier = vcoin_fee_multiplier;
        config.sscre_deduction_bps = sscre_deduction_bps;
        
        msg!("Fee config updated: SOL={}, mult={}, SSCRE={}bps",
            sol_fee_per_tx, vcoin_fee_multiplier, sscre_deduction_bps);
        Ok(())
    }
    
    /// Update daily budget
    pub fn update_daily_budget(
        ctx: Context<UpdateConfig>,
        daily_budget: u64,
        max_per_user: u32,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.daily_subsidy_budget = daily_budget;
        config.max_subsidized_per_user = max_per_user;
        
        msg!("Daily budget updated: {} lamports, {} max per user",
            daily_budget, max_per_user);
        Ok(())
    }
    
    /// Set fee payer (paymaster wallet)
    pub fn set_fee_payer(ctx: Context<UpdateConfig>, new_fee_payer: Pubkey) -> Result<()> {
        ctx.accounts.config.fee_payer = new_fee_payer;
        msg!("Fee payer updated to: {}", new_fee_payer);
        Ok(())
    }
    
    /// Set SSCRE program reference
    pub fn set_sscre_program(ctx: Context<UpdateConfig>, sscre_program: Pubkey) -> Result<()> {
        ctx.accounts.config.sscre_program = sscre_program;
        msg!("SSCRE program set to: {}", sscre_program);
        Ok(())
    }
    
    /// Pause/unpause protocol
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        ctx.accounts.config.paused = paused;
        msg!("Gasless Protocol paused: {}", paused);
        Ok(())
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.config.authority = new_authority;
        msg!("Authority updated to: {}", new_authority);
        Ok(())
    }
    
    /// Get session key info
    pub fn get_session_info(ctx: Context<GetSessionInfo>) -> Result<()> {
        let session = &ctx.accounts.session_key;
        msg!("User: {}", session.user);
        msg!("Session Key: {}", session.session_pubkey);
        msg!("Scope: {:#06x}", session.scope);
        msg!("Actions: {}/{}", session.actions_used, session.max_actions);
        msg!("Spent: {}/{} VCoin", session.vcoin_spent, session.max_spend);
        msg!("Revoked: {}", session.is_revoked);
        Ok(())
    }
    
    /// Get user gasless stats
    pub fn get_user_gasless_stats(ctx: Context<GetUserStats>) -> Result<()> {
        let stats = &ctx.accounts.user_stats;
        msg!("User: {}", stats.user);
        msg!("Total gasless tx: {}", stats.total_gasless_tx);
        msg!("Total subsidized: {}", stats.total_subsidized);
        msg!("Total VCoin fees: {}", stats.total_vcoin_fees);
        msg!("Sessions created: {}", stats.sessions_created);
        Ok(())
    }
    
    /// Get config stats
    pub fn get_config_stats(ctx: Context<GetConfigStats>) -> Result<()> {
        let config = &ctx.accounts.config;
        msg!("Total subsidized tx: {}", config.total_subsidized_tx);
        msg!("Total SOL spent: {}", config.total_sol_spent);
        msg!("Total VCoin collected: {}", config.total_vcoin_collected);
        msg!("Daily budget: {}", config.daily_subsidy_budget);
        msg!("Today spent: {}", config.day_spent);
        msg!("Paused: {}", config.paused);
        Ok(())
    }
}

// Account contexts

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = GaslessConfig::LEN,
        seeds = [GASLESS_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, GaslessConfig>,
    
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// Fee vault for VCoin fees
    #[account(
        init,
        payer = authority,
        seeds = [FEE_VAULT_SEED],
        bump,
        token::mint = vcoin_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(session_pubkey: Pubkey)]
pub struct CreateSessionKey<'info> {
    #[account(
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GaslessConfig>,
    
    #[account(
        init,
        payer = user,
        space = SessionKey::LEN,
        seeds = [SESSION_KEY_SEED, user.key().as_ref(), session_pubkey.as_ref()],
        bump
    )]
    pub session_key: Account<'info, SessionKey>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = UserGaslessStats::LEN,
        seeds = [USER_GASLESS_SEED, user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteSessionAction<'info> {
    #[account(
        mut,
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GaslessConfig>,
    
    #[account(
        mut,
        seeds = [SESSION_KEY_SEED, session_key.user.as_ref(), session_key.session_pubkey.as_ref()],
        bump = session_key.bump,
        constraint = session_key.user == user.key() @ GaslessError::Unauthorized
    )]
    pub session_key: Account<'info, SessionKey>,
    
    #[account(
        mut,
        seeds = [USER_GASLESS_SEED, user.key().as_ref()],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeductVCoinFee<'info> {
    #[account(
        mut,
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GaslessConfig>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = UserGaslessStats::LEN,
        seeds = [USER_GASLESS_SEED, user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
    
    #[account(constraint = vcoin_mint.key() == config.vcoin_mint)]
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// User's VCoin account
    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    
    /// Fee vault
    #[account(
        mut,
        seeds = [FEE_VAULT_SEED],
        bump
    )]
    pub fee_vault: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeSessionKey<'info> {
    #[account(
        mut,
        seeds = [SESSION_KEY_SEED, session_key.user.as_ref(), session_key.session_pubkey.as_ref()],
        bump = session_key.bump,
        constraint = session_key.user == user.key() @ GaslessError::Unauthorized
    )]
    pub session_key: Account<'info, SessionKey>,
    
    #[account(
        mut,
        seeds = [USER_GASLESS_SEED, user.key().as_ref()],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
    
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump,
        has_one = authority @ GaslessError::Unauthorized
    )]
    pub config: Account<'info, GaslessConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump,
        has_one = authority @ GaslessError::Unauthorized
    )]
    pub config: Account<'info, GaslessConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetSessionInfo<'info> {
    #[account(
        seeds = [SESSION_KEY_SEED, session_key.user.as_ref(), session_key.session_pubkey.as_ref()],
        bump = session_key.bump
    )]
    pub session_key: Account<'info, SessionKey>,
}

#[derive(Accounts)]
pub struct GetUserStats<'info> {
    #[account(
        seeds = [USER_GASLESS_SEED, user_stats.user.as_ref()],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserGaslessStats>,
}

#[derive(Accounts)]
pub struct GetConfigStats<'info> {
    #[account(
        seeds = [GASLESS_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GaslessConfig>,
}


