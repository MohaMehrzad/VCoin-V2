use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("CFGXTS2MueQwTYTMMTBQbRWzJtSTC2p4ZRuKPpLDmrv7");

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

pub mod constants;
pub mod errors;
pub mod events;
pub mod state;

use constants::*;
use errors::*;
use state::*;
use events::*;

#[program]
pub mod vilink_protocol {
    use super::*;

    /// Initialize ViLink configuration
    pub fn initialize(ctx: Context<Initialize>, treasury: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.authority = ctx.accounts.authority.key();
        config.vcoin_mint = ctx.accounts.vcoin_mint.key();
        config.treasury = treasury;
        config.five_a_program = Pubkey::default();
        config.staking_program = Pubkey::default();
        config.content_registry = Pubkey::default();
        config.governance_program = Pubkey::default();
        config.gasless_program = Pubkey::default();
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
        
        if action_type == ACTION_TIP {
            require!(amount >= MIN_TIP_AMOUNT, ViLinkError::InvalidAmount);
            require!(amount <= MAX_TIP_AMOUNT, ViLinkError::InvalidAmount);
        }
        
        let expiry = if expiry_seconds > 0 && expiry_seconds <= MAX_ACTION_EXPIRY {
            expiry_seconds
        } else {
            MAX_ACTION_EXPIRY
        };
        
        let clock = Clock::get()?;
        
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
        action.source_dapp = Pubkey::default();
        action.one_time = one_time;
        action.execution_count = 0;
        action.max_executions = max_executions;
        action.bump = ctx.bumps.action;
        
        config.total_actions_created = config.total_actions_created.saturating_add(1);
        
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
        
        msg!("Action created: type={}, target={}", action_type_name(action_type), target);
        Ok(())
    }
    
    /// Execute an action (tip execution with VCoin transfer)
    pub fn execute_tip_action(ctx: Context<ExecuteTipAction>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let action = &mut ctx.accounts.action;
        let executor_stats = &mut ctx.accounts.executor_stats;
        let target_stats = &mut ctx.accounts.target_stats;
        
        require!(!config.paused, ViLinkError::ProtocolPaused);
        
        require!(ctx.accounts.executor_token_account.owner == ctx.accounts.executor.key(), ViLinkError::InvalidTokenAccount);
        require!(ctx.accounts.executor_token_account.mint == config.vcoin_mint, ViLinkError::InvalidMint);
        require!(ctx.accounts.target_token_account.owner == action.target, ViLinkError::InvalidTarget);
        require!(ctx.accounts.target_token_account.mint == config.vcoin_mint, ViLinkError::InvalidMint);
        require!(ctx.accounts.treasury_token_account.owner == config.treasury, ViLinkError::InvalidTreasury);
        require!(ctx.accounts.treasury_token_account.mint == config.vcoin_mint, ViLinkError::InvalidMint);
        
        let clock = Clock::get()?;
        
        require!(!action.executed || !action.one_time, ViLinkError::ActionAlreadyExecuted);
        require!(clock.unix_timestamp <= action.expires_at, ViLinkError::ActionExpired);
        require!(action.action_type == ACTION_TIP, ViLinkError::InvalidActionType);
        
        if action.max_executions > 0 {
            require!(action.execution_count < action.max_executions, ViLinkError::ActionAlreadyExecuted);
        }
        
        let executor_key = ctx.accounts.executor.key();
        require!(executor_key != action.creator, ViLinkError::SelfExecutionNotAllowed);
        
        let fee = (action.amount as u128 * config.platform_fee_bps as u128 / 10000) as u64;
        let net_amount = action.amount.saturating_sub(fee);
        
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
        
        action.execution_count = action.execution_count.saturating_add(1);
        if action.one_time {
            action.executed = true;
        }
        action.executor = executor_key;
        action.executed_at = clock.unix_timestamp;
        
        config.total_actions_executed = config.total_actions_executed.saturating_add(1);
        config.total_tip_volume = config.total_tip_volume.saturating_add(action.amount);
        
        executor_stats.user = executor_key;
        executor_stats.actions_executed = executor_stats.actions_executed.saturating_add(1);
        executor_stats.tips_sent = executor_stats.tips_sent.saturating_add(1);
        executor_stats.vcoin_sent = executor_stats.vcoin_sent.saturating_add(action.amount);
        executor_stats.last_action_at = clock.unix_timestamp;
        executor_stats.bump = ctx.bumps.executor_stats;
        
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
    
    /// Execute a vouch action
    pub fn execute_vouch_action(ctx: Context<ExecuteGenericAction>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let action = &mut ctx.accounts.action;
        let executor_stats = &mut ctx.accounts.executor_stats;
        
        require!(!config.paused, ViLinkError::ProtocolPaused);
        require!(action.action_type == ACTION_VOUCH, ViLinkError::InvalidActionType);
        
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
        
        msg!("Vouch action executed for {}", action.target);
        Ok(())
    }
    
    /// Execute a follow action
    pub fn execute_follow_action(ctx: Context<ExecuteGenericAction>) -> Result<()> {
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
    pub fn create_batch(ctx: Context<CreateBatch>, action_ids: Vec<[u8; 32]>) -> Result<()> {
        let config = &ctx.accounts.config;
        let batch = &mut ctx.accounts.batch;
        
        require!(!config.paused, ViLinkError::ProtocolPaused);
        require!(action_ids.len() <= MAX_ACTIONS_PER_BATCH, ViLinkError::BatchTooLarge);
        
        let clock = Clock::get()?;
        
        let batch_id = generate_batch_id(&ctx.accounts.creator.key(), clock.unix_timestamp);
        
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
    pub fn set_enabled_actions(ctx: Context<UpdateConfig>, enabled_actions: u8) -> Result<()> {
        ctx.accounts.config.enabled_actions = enabled_actions;
        msg!("Enabled actions updated: {:#010b}", enabled_actions);
        Ok(())
    }
    
    /// Update platform fee
    pub fn set_platform_fee(ctx: Context<UpdateConfig>, fee_bps: u16) -> Result<()> {
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

// Account contexts

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = ViLinkConfig::LEN, seeds = [CONFIG_SEED], bump)]
    pub config: Account<'info, ViLinkConfig>,
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateAction<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, ViLinkConfig>,
    #[account(init, payer = creator, space = ViLinkAction::LEN, seeds = [ACTION_SEED, creator.key().as_ref(), &Clock::get()?.unix_timestamp.to_le_bytes()], bump)]
    pub action: Account<'info, ViLinkAction>,
    #[account(init_if_needed, payer = creator, space = UserActionStats::LEN, seeds = [USER_STATS_SEED, creator.key().as_ref()], bump)]
    pub user_stats: Account<'info, UserActionStats>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTipAction<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, ViLinkConfig>,
    #[account(mut, seeds = [ACTION_SEED, action.creator.as_ref(), &action.created_at.to_le_bytes()], bump = action.bump)]
    pub action: Account<'info, ViLinkAction>,
    #[account(init_if_needed, payer = executor, space = UserActionStats::LEN, seeds = [USER_STATS_SEED, executor.key().as_ref()], bump)]
    pub executor_stats: Account<'info, UserActionStats>,
    #[account(init_if_needed, payer = executor, space = UserActionStats::LEN, seeds = [USER_STATS_SEED, action.target.as_ref()], bump)]
    pub target_stats: Account<'info, UserActionStats>,
    #[account(constraint = vcoin_mint.key() == config.vcoin_mint @ ViLinkError::InvalidMint)]
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub executor_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub target_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub executor: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteGenericAction<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, ViLinkConfig>,
    #[account(mut, seeds = [ACTION_SEED, action.creator.as_ref(), &action.created_at.to_le_bytes()], bump = action.bump)]
    pub action: Account<'info, ViLinkAction>,
    #[account(init_if_needed, payer = executor, space = UserActionStats::LEN, seeds = [USER_STATS_SEED, executor.key().as_ref()], bump)]
    pub executor_stats: Account<'info, UserActionStats>,
    #[account(mut)]
    pub executor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterDApp<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ ViLinkError::Unauthorized)]
    pub config: Account<'info, ViLinkConfig>,
    #[account(init, payer = authority, space = RegisteredDApp::LEN, seeds = [DAPP_REGISTRY_SEED, dapp_authority.key().as_ref()], bump)]
    pub dapp: Account<'info, RegisteredDApp>,
    /// CHECK: dApp authority being registered
    pub dapp_authority: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateBatch<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Account<'info, ViLinkConfig>,
    #[account(init, payer = creator, space = ActionBatch::LEN, seeds = [BATCH_SEED, creator.key().as_ref(), &Clock::get()?.unix_timestamp.to_le_bytes()], bump)]
    pub batch: Account<'info, ActionBatch>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ ViLinkError::Unauthorized)]
    pub config: Account<'info, ViLinkConfig>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = authority @ ViLinkError::Unauthorized)]
    pub config: Account<'info, ViLinkConfig>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetAction<'info> {
    #[account(seeds = [ACTION_SEED, action.creator.as_ref(), &action.created_at.to_le_bytes()], bump = action.bump)]
    pub action: Account<'info, ViLinkAction>,
}

#[derive(Accounts)]
pub struct GetUserStats<'info> {
    #[account(seeds = [USER_STATS_SEED, user_stats.user.as_ref()], bump = user_stats.bump)]
    pub user_stats: Account<'info, UserActionStats>,
}
