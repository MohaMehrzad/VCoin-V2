use anchor_lang::prelude::*;
use crate::constants::*;
use crate::contexts::ClaimRefund;
use crate::errors::ContentError;
use crate::events::EnergyRefunded;

pub fn handler(ctx: Context<ClaimRefund>) -> Result<()> {
    let content = &mut ctx.accounts.content_record;
    
    require!(!content.refund_claimed, ContentError::RefundAlreadyClaimed);
    
    let clock = Clock::get()?;
    let elapsed = clock.unix_timestamp - content.created_at;
    
    require!(
        elapsed >= ENGAGEMENT_CHECK_DELAY,
        ContentError::RefundNotReady
    );
    
    // Calculate refund based on engagement
    let refund_pct = if content.engagement_count >= REFUND_THRESHOLD_1000 {
        150 // 150% (bonus energy!)
    } else if content.engagement_count >= REFUND_THRESHOLD_100 {
        100 // 100%
    } else if content.engagement_count >= REFUND_THRESHOLD_50 {
        50 // 50%
    } else if content.engagement_count >= REFUND_THRESHOLD_10 {
        25 // 25%
    } else {
        0 // No refund
    };
    
    if refund_pct > 0 {
        let refund_amount = ((content.energy_spent as u32 * refund_pct) / 100) as u16;
        
        let user_energy = &mut ctx.accounts.user_energy;
        user_energy.current_energy = user_energy.current_energy
            .saturating_add(refund_amount)
            .min(user_energy.max_energy);
        user_energy.energy_refunded_today = user_energy.energy_refunded_today
            .saturating_add(refund_amount as u32);
        
        emit!(EnergyRefunded {
            user: user_energy.user,
            content_id: content.tracking_id,
            refund_amount,
            engagement_count: content.engagement_count,
        });
        
        msg!("Energy refunded: {} ({}%)", refund_amount, refund_pct);
    }
    
    content.refund_claimed = true;
    
    Ok(())
}

