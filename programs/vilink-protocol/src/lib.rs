use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("FYaKjTU8fq6W8nBQB6LhFBvCzYtvNzYNd6Gdr4dQELfT");

/// ViLink Protocol - Cross-dApp Action Deep Links
/// 
/// One-tap social actions with shareable URLs.
/// 
/// Key Features:
/// - Shareable action links (viwo://action/xxx)
/// - Cross-dApp integration
/// - Gasless execution via session keys
/// - Action types: Tip, Vouch, Follow, Challenge, Stake
/// 
/// URI Format:
/// viwo://action/{action_id}?amount=X&target=Y
/// 
/// Flow:
/// 1. Creator generates action link
/// 2. User clicks link → app opens
/// 3. Session key executes action (gasless)
/// 4. User notified of result

pub mod constants {
    /// Seeds
    pub const CONFIG_SEED: &[u8] = b"vilink-config";
    pub const ACTION_SEED: &[u8] = b"action";
    pub const DAPP_REGISTRY_SEED: &[u8] = b"dapp";
    pub const USER_STATS_SEED: &[u8] = b"user-stats";
    pub const BATCH_SEED: &[u8] = b"batch";
    
    /// Action types
    pub const ACTION_TIP: u8 = 0;
    pub const ACTION_VOUCH: u8 = 1;
    pub const ACTION_FOLLOW: u8 = 2;
    pub const ACTION_CHALLENGE: u8 = 3;
    pub const ACTION_STAKE: u8 = 4;
    pub const ACTION_CONTENT_REACT: u8 = 5;
    pub const ACTION_DELEGATE: u8 = 6;
    pub const ACTION_VOTE: u8 = 7;
    
    /// Limits
    pub const MAX_ACTIONS_PER_BATCH: usize = 10;
    pub const MAX_ACTION_EXPIRY: i64 = 7 * 24 * 60 * 60; // 7 days
    pub const MIN_TIP_AMOUNT: u64 = 100_000_000; // 0.1 VCoin
    pub const MAX_TIP_AMOUNT: u64 = 10_000_000_000_000; // 10,000 VCoin
    
    /// Fee configuration
    pub const PLATFORM_FEE_BPS: u16 = 250; // 2.5%
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum ViLinkError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("ViLink Protocol is paused")]
        ProtocolPaused,
        #[msg("Action has expired")]
        ActionExpired,
        #[msg("Action already executed")]
        ActionAlreadyExecuted,
        #[msg("Invalid action type")]
        InvalidActionType,
        #[msg("Invalid action amount")]
        InvalidAmount,
        #[msg("Action creator cannot execute own action")]
        SelfExecutionNotAllowed,
        #[msg("dApp not registered")]
        DAppNotRegistered,
        #[msg("Batch size exceeds maximum")]
        BatchTooLarge,
        #[msg("Action not found")]
        ActionNotFound,
        #[msg("Insufficient balance")]
        InsufficientBalance,
        #[msg("Action type disabled")]
        ActionTypeDisabled,
        #[msg("Target user not valid")]
        InvalidTarget,
        #[msg("Arithmetic overflow")]
        Overflow,
        #[msg("Invalid token account owner")]
        InvalidTokenAccount,
        #[msg("Invalid token mint")]
        InvalidMint,
        #[msg("Invalid treasury account")]
        InvalidTreasury,
    }
}

pub mod state {
    use super::*;
    use crate::constants::*;
    
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
    
    /// Individual action link
    #[account]
    #[derive(Default)]
    pub struct ViLinkAction {
        /// Unique action ID (hash)
        pub action_id: [u8; 32],
        /// Action creator
        pub creator: Pubkey,
        /// Target user (recipient of tip, vouch target, etc.)
        pub target: Pubkey,
        /// Action type
        pub action_type: u8,
        /// Amount (for tips, stakes)
        pub amount: u64,
        /// Optional metadata hash (IPFS CID, etc.)
        pub metadata_hash: [u8; 32],
        /// Creation timestamp
        pub created_at: i64,
        /// Expiry timestamp
        pub expires_at: i64,
        /// Whether action has been executed
        pub executed: bool,
        /// Executor (who executed the action)
        pub executor: Pubkey,
        /// Execution timestamp
        pub executed_at: i64,
        /// Associated content ID (for content reactions)
        pub content_id: Option<[u8; 32]>,
        /// Source dApp
        pub source_dapp: Pubkey,
        /// One-time use?
        pub one_time: bool,
        /// Execution count (for reusable actions)
        pub execution_count: u32,
        /// Max executions (0 = unlimited)
        pub max_executions: u32,
        /// PDA bump
        pub bump: u8,
    }
    
    impl ViLinkAction {
        pub const LEN: usize = 8 + // discriminator
            32 + // action_id
            32 + // creator
            32 + // target
            1 +  // action_type
            8 +  // amount
            32 + // metadata_hash
            8 +  // created_at
            8 +  // expires_at
            1 +  // executed
            32 + // executor
            8 +  // executed_at
            (1 + 32) + // content_id (Option)
            32 + // source_dapp
            1 +  // one_time
            4 +  // execution_count
            4 +  // max_executions
            1;   // bump
    }
    
    /// Registered external dApp
    #[account]
    #[derive(Default)]
    pub struct RegisteredDApp {
        /// dApp identifier (domain hash or pubkey)
        pub dapp_id: [u8; 32],
        /// dApp name
        pub name: [u8; 32],
        /// dApp authority
        pub authority: Pubkey,
        /// dApp webhook URL hash
        pub webhook_hash: [u8; 32],
        /// Whether dApp is active
        pub is_active: bool,
        /// Registration timestamp
        pub registered_at: i64,
        /// Total actions from this dApp
        pub action_count: u64,
        /// Allowed action types bitmap
        pub allowed_actions: u8,
        /// Fee share (for affiliate model)
        pub fee_share_bps: u16,
        /// PDA bump
        pub bump: u8,
    }
    
    impl RegisteredDApp {
        pub const LEN: usize = 8 + // discriminator
            32 + // dapp_id
            32 + // name
            32 + // authority
            32 + // webhook_hash
            1 +  // is_active
            8 +  // registered_at
            8 +  // action_count
            1 +  // allowed_actions
            2 +  // fee_share_bps
            1;   // bump
    }
    
    /// User action statistics
    #[account]
    #[derive(Default)]
    pub struct UserActionStats {
        /// User wallet
        pub user: Pubkey,
        /// Total actions created
        pub actions_created: u64,
        /// Total actions executed
        pub actions_executed: u64,
        /// Total tips sent
        pub tips_sent: u64,
        /// Total tips received
        pub tips_received: u64,
        /// Total VCoin sent via tips
        pub vcoin_sent: u64,
        /// Total VCoin received via tips
        pub vcoin_received: u64,
        /// Total vouches given via actions
        pub vouches_given: u64,
        /// Total follows via actions
        pub follows_given: u64,
        /// First action timestamp
        pub first_action_at: i64,
        /// Last action timestamp
        pub last_action_at: i64,
        /// PDA bump
        pub bump: u8,
    }
    
    impl UserActionStats {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            8 +  // actions_created
            8 +  // actions_executed
            8 +  // tips_sent
            8 +  // tips_received
            8 +  // vcoin_sent
            8 +  // vcoin_received
            8 +  // vouches_given
            8 +  // follows_given
            8 +  // first_action_at
            8 +  // last_action_at
            1;   // bump
    }
    
    /// Batch action container
    #[account]
    #[derive(Default)]
    pub struct ActionBatch {
        /// Batch ID
        pub batch_id: [u8; 32],
        /// Creator
        pub creator: Pubkey,
        /// Action IDs in this batch
        pub action_ids: Vec<[u8; 32]>,
        /// Batch created timestamp
        pub created_at: i64,
        /// Total actions in batch
        pub total_actions: u8,
        /// Executed actions count
        pub executed_count: u8,
        /// PDA bump
        pub bump: u8,
    }
    
    impl ActionBatch {
        pub const LEN: usize = 8 + // discriminator
            32 + // batch_id
            32 + // creator
            4 + (32 * 10) + // action_ids (Vec with max 10)
            8 +  // created_at
            1 +  // total_actions
            1 +  // executed_count
            1;   // bump
    }
    
    /// Get action type name
    pub fn action_type_name(action_type: u8) -> &'static str {
        match action_type {
            0 => "Tip",
            1 => "Vouch",
            2 => "Follow",
            3 => "Challenge",
            4 => "Stake",
            5 => "ContentReact",
            6 => "Delegate",
            7 => "Vote",
            _ => "Unknown",
        }
    }
}

pub mod events {
    use super::*;
    
    #[event]
    pub struct ViLinkConfigInitialized {
        pub authority: Pubkey,
        pub vcoin_mint: Pubkey,
        pub enabled_actions: u8,
    }
    
    #[event]
    pub struct ActionCreated {
        pub action_id: [u8; 32],
        pub creator: Pubkey,
        pub target: Pubkey,
        pub action_type: u8,
        pub amount: u64,
        pub expires_at: i64,
    }
    
    #[event]
    pub struct ActionExecuted {
        pub action_id: [u8; 32],
        pub executor: Pubkey,
        pub target: Pubkey,
        pub action_type: u8,
        pub amount: u64,
        pub fee_paid: u64,
    }
    
    #[event]
    pub struct DAppRegistered {
        pub dapp_id: [u8; 32],
        pub authority: Pubkey,
        pub allowed_actions: u8,
    }
    
    #[event]
    pub struct BatchCreated {
        pub batch_id: [u8; 32],
        pub creator: Pubkey,
        pub action_count: u8,
    }
}

use constants::*;
use errors::*;
use state::*;
use events::*;

#[program]
pub mod vilink_protocol {
    use super::*;

    /// Initialize ViLink configuration
    pub fn initialize(
        ctx: Context<Initialize>,
        treasury: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.authority = ctx.accounts.authority.key();
        config.vcoin_mint = ctx.accounts.vcoin_mint.key();
        config.treasury = treasury;
        config.five_a_program = Pubkey::default();
        config.staking_program = Pubkey::default();
        config.content_registry = Pubkey::default();
        config.governance_program = Pubkey::default();
        config.gasless_program = Pubkey::default();
        // Enable all action types by default
        config.enabled_actions = 0xFF;
        config.total_actions_created = 0;
        config.total_actions_executed = 0;
        config.total_tip_volume = 0;
        config.paused = false;
        config.platform_fee_bps = PLATFORM_FEE_BPS;
        config.bump = ctx.bumps.config;
        
        emit!(ViLinkConfigInitialized {
            authority: config.authority,
            vcoin_mint: config.vcoin_mint,
            enabled_actions: config.enabled_actions,
        });
        
        msg!("ViLink Protocol initialized");
        Ok(())
    }
    
    /// Create a new action link
    pub fn create_action(
        ctx: Context<CreateAction>,
        action_type: u8,
        amount: u64,
        target: Pubkey,
        metadata_hash: [u8; 32],
        expiry_seconds: i64,
        one_time: bool,
        max_executions: u32,
        content_id: Option<[u8; 32]>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let action = &mut ctx.accounts.action;
        let user_stats = &mut ctx.accounts.user_stats;
        
        require!(!config.paused, ViLinkError::ProtocolPaused);
        require!(action_type < 8, ViLinkError::InvalidActionType);
        require!(config.is_action_enabled(action_type), ViLinkError::ActionTypeDisabled);
        
        // Validate amount for tip actions
        if action_type == ACTION_TIP {
            require!(amount >= MIN_TIP_AMOUNT, ViLinkError::InvalidAmount);
            require!(amount <= MAX_TIP_AMOUNT, ViLinkError::InvalidAmount);
        }
        
        // Validate expiry
        let expiry = if expiry_seconds > 0 && expiry_seconds <= MAX_ACTION_EXPIRY {
            expiry_seconds
        } else {
            MAX_ACTION_EXPIRY
        };
        
        let clock = Clock::get()?;
        
        // Generate action ID from inputs
        let action_id = generate_action_id(
            &ctx.accounts.creator.key(),
            &target,
            action_type,
            amount,
            clock.unix_timestamp,
        );
        
        action.action_id = action_id;
        action.creator = ctx.accounts.creator.key();
        action.target = target;
        action.action_type = action_type;
        action.amount = amount;
        action.metadata_hash = metadata_hash;
        action.created_at = clock.unix_timestamp;
        action.expires_at = clock.unix_timestamp + expiry;
        action.executed = false;
        action.executor = Pubkey::default();
        action.executed_at = 0;
        action.content_id = content_id;
        action.source_dapp = Pubkey::default(); // Set if from registered dApp
        action.one_time = one_time;
        action.execution_count = 0;
        action.max_executions = max_executions;
        action.bump = ctx.bumps.action;
        
        // Update config stats
        config.total_actions_created = config.total_actions_created.saturating_add(1);
        
        // Update user stats
        user_stats.user = ctx.accounts.creator.key();
        user_stats.actions_created = user_stats.actions_created.saturating_add(1);
        if user_stats.first_action_at == 0 {
            user_stats.first_action_at = clock.unix_timestamp;
        }
        user_stats.last_action_at = clock.unix_timestamp;
        user_stats.bump = ctx.bumps.user_stats;
        
        emit!(ActionCreated {
            action_id,
            creator: action.creator,
            target,
            action_type,
            amount,
            expires_at: action.expires_at,
        });
        
        msg!("Action created: type={}, target={}", 
            action_type_name(action_type), target);
        Ok(())
    }
    
    /// Execute an action (tip execution with VCoin transfer)
    pub fn execute_tip_action(
        ctx: Context<ExecuteTipAction>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let action = &mut ctx.accounts.action;
        let executor_stats = &mut ctx.accounts.executor_stats;
        let target_stats = &mut ctx.accounts.target_stats;
        
        require!(!config.paused, ViLinkError::ProtocolPaused);
        
        // Validate token accounts (runtime checks to reduce stack size)
        require!(
            ctx.accounts.executor_token_account.owner == ctx.accounts.executor.key(),
            ViLinkError::InvalidTokenAccount
        );
        require!(
            ctx.accounts.executor_token_account.mint == config.vcoin_mint,
            ViLinkError::InvalidMint
        );
        require!(
            ctx.accounts.target_token_account.owner == action.target,
            ViLinkError::InvalidTarget
        );
        require!(
            ctx.accounts.target_token_account.mint == config.vcoin_mint,
            ViLinkError::InvalidMint
        );
        require!(
            ctx.accounts.treasury_token_account.owner == config.treasury,
            ViLinkError::InvalidTreasury
        );
        require!(
            ctx.accounts.treasury_token_account.mint == config.vcoin_mint,
            ViLinkError::InvalidMint
        );
        
        let clock = Clock::get()?;
        
        // Check action validity
        require!(!action.executed || !action.one_time, ViLinkError::ActionAlreadyExecuted);
        require!(clock.unix_timestamp <= action.expires_at, ViLinkError::ActionExpired);
        require!(action.action_type == ACTION_TIP, ViLinkError::InvalidActionType);
        
        // Check max executions
        if action.max_executions > 0 {
            require!(
                action.execution_count < action.max_executions,
                ViLinkError::ActionAlreadyExecuted
            );
        }
        
        // Cannot execute own tip action
        let executor_key = ctx.accounts.executor.key();
        require!(executor_key != action.creator, ViLinkError::SelfExecutionNotAllowed);
        
        // Calculate fee
        let fee = (action.amount as u128 * config.platform_fee_bps as u128 / 10000) as u64;
        let net_amount = action.amount.saturating_sub(fee);
        
        // Transfer VCoin from executor to target
        token_2022::transfer_checked(
            CpiContext::new(ctx.accounts.token_program.to_account_info(),
                token_2022::TransferChecked {
                    from: ctx.accounts.executor_token_account.to_account_info(),
                    to: ctx.accounts.target_token_account.to_account_info(),
                    authority: ctx.accounts.executor.to_account_info(),
                    mint: ctx.accounts.vcoin_mint.to_account_info(),
                },
            ),
            net_amount,
            ctx.accounts.vcoin_mint.decimals,
        )?;
        
        // Transfer fee to treasury
        if fee > 0 {
            token_2022::transfer_checked(
                CpiContext::new(ctx.accounts.token_program.to_account_info(),
                    token_2022::TransferChecked {
                        from: ctx.accounts.executor_token_account.to_account_info(),
                        to: ctx.accounts.treasury_token_account.to_account_info(),
                        authority: ctx.accounts.executor.to_account_info(),
                        mint: ctx.accounts.vcoin_mint.to_account_info(),
                    },
                ),
                fee,
                ctx.accounts.vcoin_mint.decimals,
            )?;
        }
        
        // Update action state
        action.execution_count = action.execution_count.saturating_add(1);
        if action.one_time {
            action.executed = true;
        }
        action.executor = executor_key;
        action.executed_at = clock.unix_timestamp;
        
        // Update config stats
        config.total_actions_executed = config.total_actions_executed.saturating_add(1);
        config.total_tip_volume = config.total_tip_volume.saturating_add(action.amount);
        
        // Update executor stats
        executor_stats.user = executor_key;
        executor_stats.actions_executed = executor_stats.actions_executed.saturating_add(1);
        executor_stats.tips_sent = executor_stats.tips_sent.saturating_add(1);
        executor_stats.vcoin_sent = executor_stats.vcoin_sent.saturating_add(action.amount);
        executor_stats.last_action_at = clock.unix_timestamp;
        executor_stats.bump = ctx.bumps.executor_stats;
        
        // Update target stats
        target_stats.user = action.target;
        target_stats.tips_received = target_stats.tips_received.saturating_add(1);
        target_stats.vcoin_received = target_stats.vcoin_received.saturating_add(net_amount);
        target_stats.bump = ctx.bumps.target_stats;
        
        emit!(ActionExecuted {
            action_id: action.action_id,
            executor: executor_key,
            target: action.target,
            action_type: action.action_type,
            amount: action.amount,
            fee_paid: fee,
        });
        
        msg!("Tip action executed: {} VCoin to {}", action.amount, action.target);
        Ok(())
    }
    
    /// Execute a vouch action (integrates with 5A Protocol)
    pub fn execute_vouch_action(
        ctx: Context<ExecuteGenericAction>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let action = &mut ctx.accounts.action;
        let executor_stats = &mut ctx.accounts.executor_stats;
        
        require!(!config.paused, ViLinkError::ProtocolPaused);
        require!(action.action_type == ACTION_VOUCH, ViLinkError::InvalidActionType);
        
        let clock = Clock::get()?;
        
        // Check action validity
        require!(!action.executed || !action.one_time, ViLinkError::ActionAlreadyExecuted);
        require!(clock.unix_timestamp <= action.expires_at, ViLinkError::ActionExpired);
        
        let executor_key = ctx.accounts.executor.key();
        require!(executor_key != action.creator, ViLinkError::SelfExecutionNotAllowed);
        
        // Mark action as executed
        action.execution_count = action.execution_count.saturating_add(1);
        if action.one_time {
            action.executed = true;
        }
        action.executor = executor_key;
        action.executed_at = clock.unix_timestamp;
        
        // Update stats
        config.total_actions_executed = config.total_actions_executed.saturating_add(1);
        
        executor_stats.user = executor_key;
        executor_stats.actions_executed = executor_stats.actions_executed.saturating_add(1);
        executor_stats.vouches_given = executor_stats.vouches_given.saturating_add(1);
        executor_stats.last_action_at = clock.unix_timestamp;
        executor_stats.bump = ctx.bumps.executor_stats;
        
        emit!(ActionExecuted {
            action_id: action.action_id,
            executor: executor_key,
            target: action.target,
            action_type: action.action_type,
            amount: 0,
            fee_paid: 0,
        });
        
        // NOTE: Actual vouch CPI to 5A Protocol would happen here
        // This requires the 5A program accounts to be passed in
        
        msg!("Vouch action executed for {}", action.target);
        Ok(())
    }
    
    /// Execute a follow action
    pub fn execute_follow_action(
        ctx: Context<ExecuteGenericAction>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let action = &mut ctx.accounts.action;
        let executor_stats = &mut ctx.accounts.executor_stats;
        
        require!(!config.paused, ViLinkError::ProtocolPaused);
        require!(action.action_type == ACTION_FOLLOW, ViLinkError::InvalidActionType);
        
        let clock = Clock::get()?;
        
        require!(!action.executed || !action.one_time, ViLinkError::ActionAlreadyExecuted);
        require!(clock.unix_timestamp <= action.expires_at, ViLinkError::ActionExpired);
        
        let executor_key = ctx.accounts.executor.key();
        require!(executor_key != action.creator, ViLinkError::SelfExecutionNotAllowed);
        
        action.execution_count = action.execution_count.saturating_add(1);
        if action.one_time {
            action.executed = true;
        }
        action.executor = executor_key;
        action.executed_at = clock.unix_timestamp;
        
        config.total_actions_executed = config.total_actions_executed.saturating_add(1);
        
        executor_stats.user = executor_key;
        executor_stats.actions_executed = executor_stats.actions_executed.saturating_add(1);
        executor_stats.follows_given = executor_stats.follows_given.saturating_add(1);
        executor_stats.last_action_at = clock.unix_timestamp;
        executor_stats.bump = ctx.bumps.executor_stats;
        
        emit!(ActionExecuted {
            action_id: action.action_id,
            executor: executor_key,
            target: action.target,
            action_type: action.action_type,
            amount: 0,
            fee_paid: 0,
        });
        
        msg!("Follow action executed for {}", action.target);
        Ok(())
    }
    
    /// Register an external dApp
    pub fn register_dapp(
        ctx: Context<RegisterDApp>,
        name: [u8; 32],
        webhook_hash: [u8; 32],
        allowed_actions: u8,
        fee_share_bps: u16,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let dapp = &mut ctx.accounts.dapp;
        
        require!(!config.paused, ViLinkError::ProtocolPaused);
        
        let clock = Clock::get()?;
        
        // Generate dApp ID from authority
        let dapp_id = generate_dapp_id(&ctx.accounts.dapp_authority.key());
        
        dapp.dapp_id = dapp_id;
        dapp.name = name;
        dapp.authority = ctx.accounts.dapp_authority.key();
        dapp.webhook_hash = webhook_hash;
        dapp.is_active = true;
        dapp.registered_at = clock.unix_timestamp;
        dapp.action_count = 0;
        dapp.allowed_actions = allowed_actions;
        dapp.fee_share_bps = fee_share_bps;
        dapp.bump = ctx.bumps.dapp;
        
        emit!(DAppRegistered {
            dapp_id,
            authority: dapp.authority,
            allowed_actions,
        });
        
        msg!("dApp registered: {:?}", name);
        Ok(())
    }
    
    /// Create a batch of actions
    pub fn create_batch(
        ctx: Context<CreateBatch>,
        action_ids: Vec<[u8; 32]>,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let batch = &mut ctx.accounts.batch;
        
        require!(!config.paused, ViLinkError::ProtocolPaused);
        require!(action_ids.len() <= MAX_ACTIONS_PER_BATCH, ViLinkError::BatchTooLarge);
        
        let clock = Clock::get()?;
        
        let batch_id = generate_batch_id(
            &ctx.accounts.creator.key(),
            clock.unix_timestamp,
        );
        
        batch.batch_id = batch_id;
        batch.creator = ctx.accounts.creator.key();
        batch.action_ids = action_ids.clone();
        batch.created_at = clock.unix_timestamp;
        batch.total_actions = action_ids.len() as u8;
        batch.executed_count = 0;
        batch.bump = ctx.bumps.batch;
        
        emit!(BatchCreated {
            batch_id,
            creator: batch.creator,
            action_count: batch.total_actions,
        });
        
        msg!("Batch created with {} actions", action_ids.len());
        Ok(())
    }
    
    /// Update protocol programs
    pub fn update_programs(
        ctx: Context<UpdateConfig>,
        five_a_program: Pubkey,
        staking_program: Pubkey,
        content_registry: Pubkey,
        governance_program: Pubkey,
        gasless_program: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.five_a_program = five_a_program;
        config.staking_program = staking_program;
        config.content_registry = content_registry;
        config.governance_program = governance_program;
        config.gasless_program = gasless_program;
        
        msg!("Protocol programs updated");
        Ok(())
    }
    
    /// Enable/disable action types
    pub fn set_enabled_actions(
        ctx: Context<UpdateConfig>,
        enabled_actions: u8,
    ) -> Result<()> {
        ctx.accounts.config.enabled_actions = enabled_actions;
        msg!("Enabled actions updated: {:#010b}", enabled_actions);
        Ok(())
    }
    
    /// Update platform fee
    pub fn set_platform_fee(
        ctx: Context<UpdateConfig>,
        fee_bps: u16,
    ) -> Result<()> {
        ctx.accounts.config.platform_fee_bps = fee_bps;
        msg!("Platform fee updated to {} bps", fee_bps);
        Ok(())
    }
    
    /// Pause/unpause protocol
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        ctx.accounts.config.paused = paused;
        msg!("ViLink Protocol paused: {}", paused);
        Ok(())
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.config.authority = new_authority;
        msg!("Authority updated to: {}", new_authority);
        Ok(())
    }
    
    /// Get action details
    pub fn get_action(ctx: Context<GetAction>) -> Result<()> {
        let action = &ctx.accounts.action;
        msg!("Action ID: {:?}", action.action_id);
        msg!("Type: {}", action_type_name(action.action_type));
        msg!("Creator: {}", action.creator);
        msg!("Target: {}", action.target);
        msg!("Amount: {}", action.amount);
        msg!("Executed: {}", action.executed);
        msg!("Executions: {}/{}", action.execution_count, action.max_executions);
        Ok(())
    }
    
    /// Get user stats
    pub fn get_user_stats(ctx: Context<GetUserStats>) -> Result<()> {
        let stats = &ctx.accounts.user_stats;
        msg!("User: {}", stats.user);
        msg!("Actions created: {}", stats.actions_created);
        msg!("Actions executed: {}", stats.actions_executed);
        msg!("Tips sent: {}, VCoin: {}", stats.tips_sent, stats.vcoin_sent);
        msg!("Tips received: {}, VCoin: {}", stats.tips_received, stats.vcoin_received);
        Ok(())
    }
}

// Helper functions

fn generate_action_id(
    creator: &Pubkey,
    target: &Pubkey,
    action_type: u8,
    amount: u64,
    timestamp: i64,
) -> [u8; 32] {
    use solana_program::keccak;
    
    let mut data = Vec::with_capacity(81);
    data.extend_from_slice(creator.as_ref());
    data.extend_from_slice(target.as_ref());
    data.push(action_type);
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(&timestamp.to_le_bytes());
    
    keccak::hash(&data).to_bytes()
}

fn generate_dapp_id(authority: &Pubkey) -> [u8; 32] {
    use solana_program::keccak;
    
    let mut data = Vec::with_capacity(40);
    data.extend_from_slice(b"vilink-dapp");
    data.extend_from_slice(authority.as_ref());
    
    keccak::hash(&data).to_bytes()
}

fn generate_batch_id(creator: &Pubkey, timestamp: i64) -> [u8; 32] {
    use solana_program::keccak;
    
    let mut data = Vec::with_capacity(48);
    data.extend_from_slice(b"vilink-batch");
    data.extend_from_slice(creator.as_ref());
    data.extend_from_slice(&timestamp.to_le_bytes());
    
    keccak::hash(&data).to_bytes()
}

// Account contexts

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = ViLinkConfig::LEN,
        seeds = [CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, ViLinkConfig>,
    
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateAction<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, ViLinkConfig>,
    
    #[account(
        init,
        payer = creator,
        space = ViLinkAction::LEN,
        seeds = [ACTION_SEED, creator.key().as_ref(), &Clock::get()?.unix_timestamp.to_le_bytes()],
        bump
    )]
    pub action: Account<'info, ViLinkAction>,
    
    #[account(
        init_if_needed,
        payer = creator,
        space = UserActionStats::LEN,
        seeds = [USER_STATS_SEED, creator.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserActionStats>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTipAction<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, ViLinkConfig>,
    
    #[account(
        mut,
        seeds = [ACTION_SEED, action.creator.as_ref(), &action.created_at.to_le_bytes()],
        bump = action.bump
    )]
    pub action: Account<'info, ViLinkAction>,
    
    #[account(
        init_if_needed,
        payer = executor,
        space = UserActionStats::LEN,
        seeds = [USER_STATS_SEED, executor.key().as_ref()],
        bump
    )]
    pub executor_stats: Account<'info, UserActionStats>,
    
    #[account(
        init_if_needed,
        payer = executor,
        space = UserActionStats::LEN,
        seeds = [USER_STATS_SEED, action.target.as_ref()],
        bump
    )]
    pub target_stats: Account<'info, UserActionStats>,
    
    /// VCoin mint
    #[account(constraint = vcoin_mint.key() == config.vcoin_mint @ ViLinkError::InvalidMint)]
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    /// Executor's token account (validated in instruction handler to reduce stack size)
    #[account(mut)]
    pub executor_token_account: InterfaceAccount<'info, TokenAccount>,
    
    /// Target's token account (validated in instruction handler to reduce stack size)
    #[account(mut)]
    pub target_token_account: InterfaceAccount<'info, TokenAccount>,
    
    /// Treasury token account (validated in instruction handler to reduce stack size)
    #[account(mut)]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub executor: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteGenericAction<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, ViLinkConfig>,
    
    #[account(
        mut,
        seeds = [ACTION_SEED, action.creator.as_ref(), &action.created_at.to_le_bytes()],
        bump = action.bump
    )]
    pub action: Account<'info, ViLinkAction>,
    
    #[account(
        init_if_needed,
        payer = executor,
        space = UserActionStats::LEN,
        seeds = [USER_STATS_SEED, executor.key().as_ref()],
        bump
    )]
    pub executor_stats: Account<'info, UserActionStats>,
    
    #[account(mut)]
    pub executor: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterDApp<'info> {
    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump,
        has_one = authority @ ViLinkError::Unauthorized
    )]
    pub config: Account<'info, ViLinkConfig>,
    
    #[account(
        init,
        payer = authority,
        space = RegisteredDApp::LEN,
        seeds = [DAPP_REGISTRY_SEED, dapp_authority.key().as_ref()],
        bump
    )]
    pub dapp: Account<'info, RegisteredDApp>,
    
    /// CHECK: dApp authority being registered
    pub dapp_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateBatch<'info> {
    #[account(
        seeds = [CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, ViLinkConfig>,
    
    #[account(
        init,
        payer = creator,
        space = ActionBatch::LEN,
        seeds = [BATCH_SEED, creator.key().as_ref(), &Clock::get()?.unix_timestamp.to_le_bytes()],
        bump
    )]
    pub batch: Account<'info, ActionBatch>,
    
    #[account(mut)]
    pub creator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = config.bump,
        has_one = authority @ ViLinkError::Unauthorized
    )]
    pub config: Account<'info, ViLinkConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = config.bump,
        has_one = authority @ ViLinkError::Unauthorized
    )]
    pub config: Account<'info, ViLinkConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetAction<'info> {
    #[account(
        seeds = [ACTION_SEED, action.creator.as_ref(), &action.created_at.to_le_bytes()],
        bump = action.bump
    )]
    pub action: Account<'info, ViLinkAction>,
}

#[derive(Accounts)]
pub struct GetUserStats<'info> {
    #[account(
        seeds = [USER_STATS_SEED, user_stats.user.as_ref()],
        bump = user_stats.bump
    )]
    pub user_stats: Account<'info, UserActionStats>,
}


