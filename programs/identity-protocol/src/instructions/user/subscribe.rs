use anchor_lang::prelude::*;
use anchor_spl::token;

use crate::constants::SUBSCRIPTION_DURATION;
use crate::contexts::Subscribe;
use crate::errors::IdentityError;
use crate::events::SubscriptionUpdated;
use crate::state::SubscriptionTier;

/// Subscribe to a tier
pub fn handler(ctx: Context<Subscribe>, tier: u8) -> Result<()> {
    let tier_enum = SubscriptionTier::from_u8(tier)
        .ok_or(IdentityError::InvalidSubscriptionTier)?;

    let price = tier_enum.price();

    // C-AUDIT-22: Enforce USDC payment for non-free tiers
    if price > 0 {
        let cpi_accounts = token::TransferChecked {
            from: ctx.accounts.user_token_account.to_account_info(),
            mint: ctx.accounts.usdc_mint.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
        );
        token::transfer_checked(cpi_ctx, price, ctx.accounts.usdc_mint.decimals)?;
    }

    let clock = Clock::get()?;
    let subscription = &mut ctx.accounts.subscription;

    subscription.user = ctx.accounts.user.key();
    subscription.tier = tier;
    subscription.started_at = clock.unix_timestamp;
    subscription.expires_at = clock.unix_timestamp + SUBSCRIPTION_DURATION;
    subscription.auto_renew = false;
    subscription.total_paid = subscription.total_paid.saturating_add(price);
    subscription.bump = ctx.bumps.subscription;

    emit!(SubscriptionUpdated {
        user: subscription.user,
        tier,
        expires_at: subscription.expires_at,
    });

    msg!("Subscription activated: tier {}", tier);
    Ok(())
}

