use anchor_lang::prelude::*;
use crate::contexts::ExecuteSessionAction;
use crate::errors::GaslessError;
use crate::events::{DailyBudgetReset, FeeCollected, SessionActionExecuted};
use crate::state::{FeeMethod, GaslessConfig};

pub fn handler(
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

