use anchor_lang::prelude::*;

use crate::constants::SUBSCRIPTION_DURATION;
use crate::contexts::Subscribe;
use crate::errors::IdentityError;
use crate::events::SubscriptionUpdated;
use crate::state::SubscriptionTier;

/// Subscribe to a tier
pub fn handler(ctx: Context<Subscribe>, tier: u8) -> Result<()> {
    let tier_enum = SubscriptionTier::from_u8(tier)
        .ok_or(IdentityError::InvalidSubscriptionTier)?;
    
    let clock = Clock::get()?;
    let subscription = &mut ctx.accounts.subscription;
    
    subscription.user = ctx.accounts.user.key();
    subscription.tier = tier;
    subscription.started_at = clock.unix_timestamp;
    subscription.expires_at = clock.unix_timestamp + SUBSCRIPTION_DURATION;
    subscription.auto_renew = false;
    subscription.total_paid = subscription.total_paid.saturating_add(tier_enum.price());
    subscription.bump = ctx.bumps.subscription;
    
    emit!(SubscriptionUpdated {
        user: subscription.user,
        tier,
        expires_at: subscription.expires_at,
    });
    
    msg!("Subscription activated: tier {}", tier);
    Ok(())
}

