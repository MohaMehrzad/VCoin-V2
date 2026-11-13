//! USDC subscriptions (audit ID-05).

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::constants::*;
use crate::contexts::*;
use crate::errors::IdentityError;
use crate::events::*;
use crate::state::subscription_quote;

/// Buys 30 days of `tier` (1..=3). The same tier extends from the current expiry, an upgrade
/// is prorated, a downgrade waits until the active tier ends. `max_payment` caps the charge.
pub fn subscribe(ctx: Context<Subscribe>, tier: u8, max_payment: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, IdentityError::Paused);
    require!(
        (1..=NUM_TIERS as u8).contains(&tier),
        IdentityError::InvalidTier
    );
    let now = Clock::get()?.unix_timestamp;
    let user = ctx.accounts.user.key();
    let sub = &mut ctx.accounts.subscription;
    if sub.user == Pubkey::default() {
        sub.user = user;
        sub.bump = ctx.bumps.subscription;
    }
    let (cost, expires_at) = subscription_quote(sub, tier, &config.prices, now)
        .ok_or(IdentityError::CannotDowngradeActive)?;
    require!(cost <= max_payment, IdentityError::PriceAboveMax);

    if cost > 0 {
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.user_usdc.to_account_info(),
                    mint: ctx.accounts.usdc_mint.to_account_info(),
                    to: ctx.accounts.treasury_token.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            cost,
            ctx.accounts.usdc_mint.decimals,
        )?;
    }
    if sub.active_tier(now) != tier {
        sub.started_at = now;
    }
    sub.tier = tier;
    sub.expires_at = expires_at;
    sub.total_paid = sub
        .total_paid
        .checked_add(cost)
        .ok_or(IdentityError::Overflow)?;
    emit!(Subscribed {
        user,
        tier,
        paid: cost,
        expires_at
    });
    Ok(())
}
