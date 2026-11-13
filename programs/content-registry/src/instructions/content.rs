//! Energy, content records, engagement and refunds.

use anchor_lang::prelude::*;
use gasless_protocol::{constants::SCOPE_CONTENT, state::SessionToken};
use staking_protocol::state::Voter;

use crate::constants::*;
use crate::contexts::*;
use crate::errors::ContentError;
use crate::events::*;
use crate::state::*;

fn now() -> Result<i64> {
    Ok(Clock::get()?.unix_timestamp)
}

/// The author signs, or a session key acting for the author in this program with the
/// CONTENT scope (gasless-protocol).
fn authorize(
    author: &Pubkey,
    signer: &Pubkey,
    session: &Option<Box<Account<SessionToken>>>,
    now: i64,
) -> Result<()> {
    if author == signer {
        return Ok(());
    }
    let session = session.as_ref().ok_or(ContentError::Unauthorized)?;
    session.verify(author, signer, &crate::ID, SCOPE_CONTENT, now)
}

/// Tier from the user's staking `Voter` PDA (its address is checked by the accounts
/// struct). No account there means the wallet never staked: tier 0.
fn stake_tier(stake: &UncheckedAccount, now: i64) -> Result<u8> {
    let info = stake.to_account_info();
    if info.owner != &staking_protocol::ID || info.data_is_empty() {
        return Ok(0);
    }
    let voter = Voter::try_deserialize(&mut &info.try_borrow_data()?[..])?;
    Ok(tier_for_locked(voter.locked_amount(now)))
}

/// Applies the user's current staking tier (settling regeneration at the old rate first).
fn refresh_tier(energy: &mut UserEnergy, tier: u8, now: i64) {
    if tier != energy.tier {
        emit!(TierChanged {
            user: energy.user,
            old_tier: energy.tier,
            new_tier: tier
        });
    }
    energy.set_tier(tier, now);
}

fn spend(energy: &mut UserEnergy, cost: u16) -> Result<()> {
    require!(
        energy.current_energy >= cost,
        ContentError::InsufficientEnergy
    );
    energy.current_energy -= cost;
    emit!(EnergySpent {
        user: energy.user,
        amount: cost,
        remaining: energy.current_energy
    });
    Ok(())
}

pub fn init_user_energy(ctx: Context<InitUserEnergy>) -> Result<()> {
    require!(!ctx.accounts.config.paused, ContentError::Paused);
    let now = now()?;
    let tier = stake_tier(&ctx.accounts.stake, now)?;
    let energy = &mut ctx.accounts.user_energy;
    energy.user = ctx.accounts.user.key();
    energy.bump = ctx.bumps.user_energy;
    energy.day_start = now;
    energy.last_regen_time = now;
    energy.set_tier(tier, now);
    energy.current_energy = energy.max_energy;
    emit!(EnergyInitialized {
        user: energy.user,
        tier
    });
    Ok(())
}

pub fn create_content(
    ctx: Context<CreateContent>,
    tracking_id: [u8; 32],
    content_hash: [u8; 32],
    uri: String,
    content_type: u8,
) -> Result<()> {
    require!(!ctx.accounts.config.paused, ContentError::Paused);
    let now = now()?;
    let author = ctx.accounts.author.key();
    let signer = ctx.accounts.signer.key();
    authorize(&author, &signer, &ctx.accounts.session, now)?;
    require!(uri.len() <= MAX_URI_LEN, ContentError::UriTooLong);
    let cost = *CONTENT_COSTS
        .get(content_type as usize)
        .ok_or(ContentError::InvalidContentType)?;

    let tier = stake_tier(&ctx.accounts.stake, now)?;
    let energy = &mut ctx.accounts.user_energy;
    refresh_tier(energy, tier, now);
    energy.roll_day(now);
    require!(
        energy.posts_today < DAILY_POST_CAP[tier as usize],
        ContentError::DailyCapExceeded
    );
    spend(energy, cost)?;
    energy.posts_today += 1;

    let record = &mut ctx.accounts.content;
    record.author = author;
    record.tracking_id = tracking_id;
    record.content_hash = content_hash;
    record.uri = [0u8; MAX_URI_LEN];
    record.uri[..uri.len()].copy_from_slice(uri.as_bytes());
    record.uri_len = uri.len() as u8;
    record.content_type = content_type;
    record.state = ContentState::Active;
    record.version = 1;
    record.created_at = now;
    record.updated_at = now;
    record.energy_spent = cost;
    record.engagement_count = 0;
    record.refund_claimed = false;
    record.rent_payer = ctx.accounts.payer.key();
    record.bump = ctx.bumps.content;
    emit!(ContentCreated {
        author,
        tracking_id,
        content_hash,
        content_type,
        signer
    });
    Ok(())
}

/// Every edit costs energy and is evented with the previous hash (audit CR-10).
pub fn edit_content(ctx: Context<EditContent>, content_hash: [u8; 32], uri: String) -> Result<()> {
    require!(!ctx.accounts.config.paused, ContentError::Paused);
    let now = now()?;
    let author = ctx.accounts.author.key();
    let signer = ctx.accounts.signer.key();
    authorize(&author, &signer, &ctx.accounts.session, now)?;
    require!(uri.len() <= MAX_URI_LEN, ContentError::UriTooLong);

    // Edits use the author's current tier, not the one from their last post.
    let tier = stake_tier(&ctx.accounts.stake, now)?;
    let energy = &mut ctx.accounts.user_energy;
    refresh_tier(energy, tier, now);
    spend(energy, EDIT_COST)?;

    let record = &mut ctx.accounts.content;
    let previous_hash = record.content_hash;
    record.content_hash = content_hash;
    record.uri = [0u8; MAX_URI_LEN];
    record.uri[..uri.len()].copy_from_slice(uri.as_bytes());
    record.uri_len = uri.len() as u8;
    record.state = ContentState::Edited;
    record.version = record
        .version
        .checked_add(1)
        .ok_or(ContentError::Overflow)?;
    record.updated_at = now;
    record.energy_spent = record.energy_spent.saturating_add(EDIT_COST);
    emit!(ContentEdited {
        author,
        tracking_id: record.tracking_id,
        previous_hash,
        content_hash,
        version: record.version,
        signer,
    });
    Ok(())
}

pub fn delete_content(ctx: Context<DeleteContent>) -> Result<()> {
    let record = &ctx.accounts.content;
    emit!(ContentDeleted {
        author: record.author,
        tracking_id: record.tracking_id,
        content_hash: record.content_hash
    });
    Ok(())
}

/// The engagement oracle (not the admin) reports engagement (audit CR-07).
pub fn update_engagement(ctx: Context<UpdateEngagement>, engagement_count: u32) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, ContentError::Paused);
    require_keys_eq!(
        ctx.accounts.oracle.key(),
        config.engagement_oracle,
        ContentError::NotEngagementOracle
    );
    let record = &mut ctx.accounts.content;
    record.engagement_count = engagement_count;
    emit!(EngagementUpdated {
        author: record.author,
        tracking_id: record.tracking_id,
        engagement_count
    });
    Ok(())
}

/// Returns up to 100% of the energy spent on a post with enough engagement, between 24h and
/// 7 days after posting; nothing is consumed while the post does not qualify (audit CR-06).
pub fn claim_refund(ctx: Context<ClaimRefund>) -> Result<()> {
    require!(!ctx.accounts.config.paused, ContentError::Paused);
    let now = now()?;
    let record = &mut ctx.accounts.content;
    require!(!record.refund_claimed, ContentError::RefundAlreadyClaimed);
    let age = now.saturating_sub(record.created_at);
    require!(age >= REFUND_DELAY, ContentError::RefundTooEarly);
    require!(age <= REFUND_DEADLINE, ContentError::RefundWindowClosed);
    let percent = refund_percent(record.engagement_count);
    require!(percent > 0, ContentError::RefundNotReady);
    let amount = (record.energy_spent as u32 * percent as u32 / 100) as u16;

    let tier = stake_tier(&ctx.accounts.stake, now)?;
    let energy = &mut ctx.accounts.user_energy;
    refresh_tier(energy, tier, now);
    energy.roll_day(now);
    let refunded = energy
        .refunded_today
        .checked_add(amount)
        .ok_or(ContentError::Overflow)?;
    require!(
        refunded <= energy.max_energy,
        ContentError::DailyRefundCapExceeded
    );
    energy.refunded_today = refunded;
    energy.current_energy = energy
        .current_energy
        .saturating_add(amount)
        .min(energy.max_energy);
    record.refund_claimed = true;
    emit!(RefundClaimed {
        author: record.author,
        tracking_id: record.tracking_id,
        percent,
        amount
    });
    Ok(())
}
