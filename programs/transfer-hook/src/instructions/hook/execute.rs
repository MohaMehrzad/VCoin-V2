use anchor_lang::prelude::*;

use crate::contexts::Execute;
use crate::errors::TransferHookError;
use crate::events::{TransferProcessed, WashTradingDetected};
use crate::utils::{update_user_activity, check_wash_trading};

/// Execute transfer hook - called automatically on every VCoin transfer
/// This is the main entry point called by Token-2022
pub fn handler(ctx: Context<Execute>, amount: u64) -> Result<()> {
    let config = &ctx.accounts.hook_config;
    
    // M-AUDIT-11: When the hook is paused, ALL VCoin transfers are blocked.
    // This is an intentional design choice for emergency response scenarios
    // (e.g., exploit mitigation, contract migration). The pause affects every
    // transfer — not just flagged ones — because Token-2022 invokes this hook
    // on every token transfer. The authority can unpause via set_paused(false)
    // once the emergency is resolved. This trade-off prioritizes safety over
    // liveness: it is better to temporarily halt transfers than to allow
    // potentially exploited transfers to continue.
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
        
        // M-04 Security Fix: Optionally block transfers when wash trading is detected
        // When block_wash_trading is enabled, the transfer is rejected
        // Otherwise, we just emit the event and allow the transfer
        if config.block_wash_trading {
            return Err(TransferHookError::WashTradingDetected.into());
        }
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

