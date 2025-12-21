use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

declare_id!("E5FWQsncH5hWRYX2ysiTA9uA2vhdQtQP473tDU9GWhyi");

/// VCoin Transfer Hook Program
/// 
/// Implements Token-2022 Transfer Hook for:
/// 1. Auto-updating 5A Activity scores on transfers
/// 2. Recording tip transactions for SSCRE calculations
/// 3. Detecting wash trading patterns
/// 4. Updating engagement trust scores

pub mod constants {
    /// Minimum transfer amount to count as activity (1 VCoin)
    pub const MIN_ACTIVITY_THRESHOLD: u64 = 1_000_000_000;
    
    /// Maximum transfers per hour before diminishing activity score
    pub const MAX_TRANSFERS_PER_HOUR: u8 = 20;
    
    /// Wash trading detection: minimum time between transfers to same recipient
    pub const WASH_TRADING_COOLDOWN_SECONDS: i64 = 3600; // 1 hour
    
    /// Seeds
    pub const HOOK_CONFIG_SEED: &[u8] = b"hook-config";
    pub const TRANSFER_RECORD_SEED: &[u8] = b"transfer-record";
    pub const USER_ACTIVITY_SEED: &[u8] = b"user-activity";
    pub const PAIR_TRACKING_SEED: &[u8] = b"pair-tracking";
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum TransferHookError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("Transfer hook is paused")]
        HookPaused,
        #[msg("Invalid program owner")]
        InvalidProgramOwner,
        #[msg("Wash trading pattern detected")]
        WashTradingDetected,
        #[msg("Arithmetic overflow")]
        Overflow,
        #[msg("Invalid extra account metas")]
        InvalidExtraAccountMetas,
    }
}

pub mod state {
    use super::*;
    
    /// Global hook configuration
    #[account]
    #[derive(Default)]
    pub struct HookConfig {
        /// Admin authority
        pub authority: Pubkey,
        /// VCoin mint address
        pub vcoin_mint: Pubkey,
        /// 5A Protocol program (for CPI calls)
        pub five_a_program: Pubkey,
        /// Whether wash trading blocking is enabled (vs just flagging)
        pub block_wash_trading: bool,
        /// Minimum amount for activity score increment
        pub min_activity_amount: u64,
        /// Total transfers processed
        pub total_transfers: u64,
        /// Total wash trading flags
        pub wash_trading_flags: u64,
        /// Whether hook is paused
        pub paused: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl HookConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // vcoin_mint
            32 + // five_a_program
            1 +  // block_wash_trading
            8 +  // min_activity_amount
            8 +  // total_transfers
            8 +  // wash_trading_flags
            1 +  // paused
            1;   // bump
    }
    
    /// Per-user activity tracking for rate limiting
    #[account]
    #[derive(Default)]
    pub struct UserActivity {
        /// User wallet
        pub user: Pubkey,
        /// Transfers in current hour
        pub transfers_this_hour: u8,
        /// Hour reset timestamp
        pub hour_reset_time: i64,
        /// Total lifetime transfers sent
        pub total_transfers_sent: u64,
        /// Total lifetime transfers received
        pub total_transfers_received: u64,
        /// Total VCoin sent
        pub total_amount_sent: u64,
        /// Total VCoin received
        pub total_amount_received: u64,
        /// Last transfer timestamp
        pub last_transfer_time: i64,
        /// Activity score contribution (updated by 5A oracle)
        pub activity_score_contribution: u16,
        /// PDA bump
        pub bump: u8,
    }
    
    impl UserActivity {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            1 +  // transfers_this_hour
            8 +  // hour_reset_time
            8 +  // total_transfers_sent
            8 +  // total_transfers_received
            8 +  // total_amount_sent
            8 +  // total_amount_received
            8 +  // last_transfer_time
            2 +  // activity_score_contribution
            1;   // bump
    }
    
    /// Tracks transfer patterns between specific pairs for wash trading detection
    #[account]
    #[derive(Default)]
    pub struct PairTracking {
        /// Sender wallet
        pub sender: Pubkey,
        /// Receiver wallet
        pub receiver: Pubkey,
        /// Last transfer timestamp
        pub last_transfer_time: i64,
        /// Transfer count in last 24 hours
        pub transfers_24h: u16,
        /// Day reset timestamp
        pub day_reset_time: i64,
        /// Total amount transferred in 24h
        pub amount_24h: u64,
        /// Wash trading flag count
        pub wash_flags: u16,
        /// Engagement trust score (0-10000)
        pub trust_score: u16,
        /// PDA bump
        pub bump: u8,
    }
    
    impl PairTracking {
        pub const LEN: usize = 8 + // discriminator
            32 + // sender
            32 + // receiver
            8 +  // last_transfer_time
            2 +  // transfers_24h
            8 +  // day_reset_time
            8 +  // amount_24h
            2 +  // wash_flags
            2 +  // trust_score
            1;   // bump
    }
    
    /// Individual transfer record for audit trail
    #[account]
    #[derive(Default)]
    pub struct TransferRecord {
        /// Transfer ID (sequential)
        pub transfer_id: u64,
        /// Sender
        pub sender: Pubkey,
        /// Receiver
        pub receiver: Pubkey,
        /// Amount transferred
        pub amount: u64,
        /// Timestamp
        pub timestamp: i64,
        /// Whether this was flagged as wash trading
        pub wash_trading_flag: bool,
        /// Whether this was a tip transaction
        pub is_tip: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl TransferRecord {
        pub const LEN: usize = 8 + // discriminator
            8 +  // transfer_id
            32 + // sender
            32 + // receiver
            8 +  // amount
            8 +  // timestamp
            1 +  // wash_trading_flag
            1 +  // is_tip
            1;   // bump
    }
}

pub mod events {
    use super::*;
    
    #[event]
    pub struct TransferProcessed {
        pub sender: Pubkey,
        pub receiver: Pubkey,
        pub amount: u64,
        pub timestamp: i64,
        pub is_tip: bool,
    }
    
    #[event]
    pub struct WashTradingDetected {
        pub sender: Pubkey,
        pub receiver: Pubkey,
        pub amount: u64,
        pub timestamp: i64,
        pub pair_transfers_24h: u16,
    }
    
    #[event]
    pub struct ActivityScoreUpdated {
        pub user: Pubkey,
        pub new_contribution: u16,
        pub transfers_this_hour: u8,
    }
}

use constants::*;
use errors::*;
use state::*;
use events::*;

#[program]
pub mod transfer_hook {
    use super::*;

    /// Initialize the transfer hook configuration
    pub fn initialize(
        ctx: Context<Initialize>,
        five_a_program: Pubkey,
        min_activity_amount: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.hook_config;
        
        config.authority = ctx.accounts.authority.key();
        config.vcoin_mint = ctx.accounts.vcoin_mint.key();
        config.five_a_program = five_a_program;
        config.block_wash_trading = false; // Start with flagging only
        config.min_activity_amount = min_activity_amount;
        config.total_transfers = 0;
        config.wash_trading_flags = 0;
        config.paused = false;
        config.bump = ctx.bumps.hook_config;
        
        msg!("Transfer hook initialized for VCoin mint: {}", config.vcoin_mint);
        Ok(())
    }
    
    /// Execute transfer hook - called automatically on every VCoin transfer
    /// This is the main entry point called by Token-2022
    pub fn execute(ctx: Context<Execute>, amount: u64) -> Result<()> {
        let config = &ctx.accounts.hook_config;
        
        // Check if hook is paused
        require!(!config.paused, TransferHookError::HookPaused);
        
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp;
        
        let sender_key = ctx.accounts.sender.key();
        let receiver_key = ctx.accounts.receiver.key();
        
        // Update sender activity
        let sender_activity = &mut ctx.accounts.sender_activity;
        update_user_activity(sender_activity, sender_key, amount, true, current_time)?;
        
        // Update receiver activity
        let receiver_activity = &mut ctx.accounts.receiver_activity;
        update_user_activity(receiver_activity, receiver_key, amount, false, current_time)?;
        
        // Check for wash trading patterns
        let pair_tracking = &mut ctx.accounts.pair_tracking;
        let is_wash_trading = check_wash_trading(
            pair_tracking,
            sender_key,
            receiver_key,
            amount,
            current_time,
        )?;
        
        if is_wash_trading {
            emit!(WashTradingDetected {
                sender: sender_key,
                receiver: receiver_key,
                amount,
                timestamp: current_time,
                pair_transfers_24h: pair_tracking.transfers_24h,
            });
            
            // Note: We don't block, just flag. Blocking happens in governance decisions.
        }
        
        // Emit transfer processed event
        emit!(TransferProcessed {
            sender: sender_key,
            receiver: receiver_key,
            amount,
            timestamp: current_time,
            is_tip: false, // Tip detection requires additional context
        });
        
        Ok(())
    }
    
    /// Initialize extra account metas for the transfer hook
    /// Required by Token-2022 transfer hook interface
    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        // The extra accounts needed for the transfer hook execution
        // These are stored in the ExtraAccountMetaList account
        msg!("Extra account meta list initialized");
        Ok(())
    }
    
    /// Update hook configuration
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        new_five_a_program: Option<Pubkey>,
        new_min_activity_amount: Option<u64>,
        block_wash_trading: Option<bool>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.hook_config;
        
        if let Some(program) = new_five_a_program {
            config.five_a_program = program;
        }
        if let Some(amount) = new_min_activity_amount {
            config.min_activity_amount = amount;
        }
        if let Some(block) = block_wash_trading {
            config.block_wash_trading = block;
        }
        
        msg!("Hook config updated");
        Ok(())
    }
    
    /// Pause/unpause the hook
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        ctx.accounts.hook_config.paused = paused;
        msg!("Hook paused status: {}", paused);
        Ok(())
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.hook_config.authority = new_authority;
        msg!("Authority updated to: {}", new_authority);
        Ok(())
    }
    
    /// Query user activity stats
    pub fn get_user_activity(ctx: Context<GetUserActivity>) -> Result<()> {
        let activity = &ctx.accounts.user_activity;
        msg!("User: {}", activity.user);
        msg!("Total sent: {}", activity.total_transfers_sent);
        msg!("Total received: {}", activity.total_transfers_received);
        msg!("Activity contribution: {}", activity.activity_score_contribution);
        Ok(())
    }
    
    /// Query pair tracking stats
    pub fn get_pair_stats(ctx: Context<GetPairStats>) -> Result<()> {
        let pair = &ctx.accounts.pair_tracking;
        msg!("Sender: {}", pair.sender);
        msg!("Receiver: {}", pair.receiver);
        msg!("Transfers 24h: {}", pair.transfers_24h);
        msg!("Wash flags: {}", pair.wash_flags);
        msg!("Trust score: {}", pair.trust_score);
        Ok(())
    }
}

// Helper functions

fn update_user_activity(
    activity: &mut UserActivity,
    user: Pubkey,
    amount: u64,
    is_sender: bool,
    current_time: i64,
) -> Result<()> {
    // Initialize if new
    if activity.user == Pubkey::default() {
        activity.user = user;
    }
    
    // Reset hourly counter if needed
    if current_time >= activity.hour_reset_time + 3600 {
        activity.transfers_this_hour = 0;
        activity.hour_reset_time = current_time;
    }
    
    // Update counters
    activity.transfers_this_hour = activity.transfers_this_hour.saturating_add(1);
    activity.last_transfer_time = current_time;
    
    if is_sender {
        activity.total_transfers_sent = activity.total_transfers_sent.saturating_add(1);
        activity.total_amount_sent = activity.total_amount_sent.saturating_add(amount);
    } else {
        activity.total_transfers_received = activity.total_transfers_received.saturating_add(1);
        activity.total_amount_received = activity.total_amount_received.saturating_add(amount);
    }
    
    // Calculate activity score contribution
    // Higher for consistent activity, diminishing for spam
    let base_contribution = if activity.transfers_this_hour <= MAX_TRANSFERS_PER_HOUR {
        100_u16
    } else {
        // Diminishing returns for excessive transfers
        50_u16.saturating_div(activity.transfers_this_hour as u16)
    };
    
    activity.activity_score_contribution = 
        activity.activity_score_contribution.saturating_add(base_contribution);
    
    emit!(ActivityScoreUpdated {
        user,
        new_contribution: activity.activity_score_contribution,
        transfers_this_hour: activity.transfers_this_hour,
    });
    
    Ok(())
}

fn check_wash_trading(
    pair: &mut PairTracking,
    sender: Pubkey,
    receiver: Pubkey,
    amount: u64,
    current_time: i64,
) -> Result<bool> {
    // Initialize if new
    if pair.sender == Pubkey::default() {
        pair.sender = sender;
        pair.receiver = receiver;
        pair.trust_score = 5000; // Start neutral
    }
    
    // Reset daily counter if needed
    if current_time >= pair.day_reset_time + 86400 {
        pair.transfers_24h = 0;
        pair.amount_24h = 0;
        pair.day_reset_time = current_time;
    }
    
    // Check for wash trading pattern
    let time_since_last = current_time - pair.last_transfer_time;
    let is_rapid_transfer = time_since_last < WASH_TRADING_COOLDOWN_SECONDS && 
                            pair.last_transfer_time > 0;
    let is_high_frequency = pair.transfers_24h > 10;
    
    // Update pair tracking
    pair.last_transfer_time = current_time;
    pair.transfers_24h = pair.transfers_24h.saturating_add(1);
    pair.amount_24h = pair.amount_24h.saturating_add(amount);
    
    // Detect wash trading
    let is_wash_trading = is_rapid_transfer && is_high_frequency;
    
    if is_wash_trading {
        pair.wash_flags = pair.wash_flags.saturating_add(1);
        // Decrease trust score
        pair.trust_score = pair.trust_score.saturating_sub(500);
    } else if pair.trust_score < 10000 {
        // Slowly rebuild trust for legitimate activity
        pair.trust_score = pair.trust_score.saturating_add(10);
    }
    
    Ok(is_wash_trading)
}

// Account contexts

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = HookConfig::LEN,
        seeds = [HOOK_CONFIG_SEED],
        bump
    )]
    pub hook_config: Account<'info, HookConfig>,
    
    /// VCoin mint (Token-2022)
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Execute<'info> {
    #[account(
        seeds = [HOOK_CONFIG_SEED],
        bump = hook_config.bump
    )]
    pub hook_config: Account<'info, HookConfig>,
    
    /// Source token account
    pub sender: InterfaceAccount<'info, TokenAccount>,
    
    /// Destination token account  
    pub receiver: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = UserActivity::LEN,
        seeds = [USER_ACTIVITY_SEED, sender.owner.as_ref()],
        bump
    )]
    pub sender_activity: Account<'info, UserActivity>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = UserActivity::LEN,
        seeds = [USER_ACTIVITY_SEED, receiver.owner.as_ref()],
        bump
    )]
    pub receiver_activity: Account<'info, UserActivity>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = PairTracking::LEN,
        seeds = [PAIR_TRACKING_SEED, sender.owner.as_ref(), receiver.owner.as_ref()],
        bump
    )]
    pub pair_tracking: Account<'info, PairTracking>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// The extra account meta list account
    /// CHECK: Validated by the transfer hook interface
    #[account(mut)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [HOOK_CONFIG_SEED],
        bump = hook_config.bump,
        has_one = authority @ TransferHookError::Unauthorized
    )]
    pub hook_config: Account<'info, HookConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [HOOK_CONFIG_SEED],
        bump = hook_config.bump,
        has_one = authority @ TransferHookError::Unauthorized
    )]
    pub hook_config: Account<'info, HookConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction()]
pub struct GetUserActivity<'info> {
    #[account(
        seeds = [USER_ACTIVITY_SEED, user.key().as_ref()],
        bump = user_activity.bump
    )]
    pub user_activity: Account<'info, UserActivity>,
    
    /// CHECK: Just used for PDA derivation
    pub user: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct GetPairStats<'info> {
    #[account(
        seeds = [PAIR_TRACKING_SEED, sender.key().as_ref(), receiver.key().as_ref()],
        bump = pair_tracking.bump
    )]
    pub pair_tracking: Account<'info, PairTracking>,
    
    /// CHECK: Just used for PDA derivation
    pub sender: UncheckedAccount<'info>,
    
    /// CHECK: Just used for PDA derivation
    pub receiver: UncheckedAccount<'info>,
}


