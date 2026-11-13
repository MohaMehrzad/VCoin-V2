use anchor_lang::prelude::*;
use vcoin_common::AdminState;

use crate::constants::*;

/// Singleton, PDA `["identity-config"]`.
#[account]
#[derive(InitSpace)]
pub struct IdentityConfig {
    pub admin: AdminState,
    /// Trusted attesters (`Pubkey::default()` = empty slot). Removing one immediately voids
    /// the levels it granted (see [`Identity::effective_level`]).
    pub attesters: [Pubkey; MAX_ATTESTERS],
    /// USDC mint (6 decimals), fixed at initialization.
    pub usdc_mint: Pubkey,
    /// Token account (USDC) that receives subscription payments.
    pub treasury_token: Pubkey,
    /// Price per 30 days for tiers 1..=3.
    pub prices: [u64; NUM_TIERS],
    /// Timelocked payment change (`pending_eta == 0` = none).
    pub pending_prices: [u64; NUM_TIERS],
    pub pending_treasury_token: Pubkey,
    pub pending_eta: i64,
    pub paused: bool,
    pub bump: u8,
}

impl IdentityConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn is_attester(&self, key: &Pubkey) -> bool {
        *key != Pubkey::default() && self.attesters.contains(key)
    }
}

/// One per wallet, PDA `["identity", owner]`.
#[account]
#[derive(InitSpace)]
pub struct Identity {
    pub owner: Pubkey,
    pub did_hash: [u8; 32],
    pub username: [u8; MAX_USERNAME_LEN],
    pub username_len: u8,
    /// Level from the current attestation (see `effective_level`).
    pub level: u8,
    pub claims: u16,
    pub attester: Pubkey,
    pub attestation_id: [u8; 32],
    pub level_expires_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

impl Identity {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    /// The level consumers should trust: zero once the attestation expired or its attester
    /// is no longer trusted (audit ID-02).
    pub fn effective_level(&self, now: i64, config: &IdentityConfig) -> u8 {
        if now < self.level_expires_at && config.is_attester(&self.attester) {
            self.level
        } else {
            LEVEL_NONE
        }
    }

    pub fn username(&self) -> &[u8] {
        &self.username[..self.username_len as usize]
    }
}

/// Reserves a username, PDA `["username", name]`.
#[account]
#[derive(InitSpace)]
pub struct UsernameRecord {
    pub owner: Pubkey,
    pub bump: u8,
}

impl UsernameRecord {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// Binds an attestation id to exactly one wallet, PDA `["attestation", attestation_id]`
/// (audit ID-03: one proof cannot verify many wallets).
#[account]
#[derive(InitSpace)]
pub struct AttestationLink {
    pub attestation_id: [u8; 32],
    pub owner: Pubkey,
    pub attester: Pubkey,
    pub claims: u16,
    pub level: u8,
    pub expires_at: i64,
    pub linked_at: i64,
    pub rent_payer: Pubkey,
    pub bump: u8,
}

impl AttestationLink {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// PDA `["subscription", user]`.
#[account]
#[derive(InitSpace)]
pub struct Subscription {
    pub user: Pubkey,
    /// 1..=3 (0 = never subscribed).
    pub tier: u8,
    pub started_at: i64,
    pub expires_at: i64,
    pub total_paid: u64,
    /// What the remaining time cost per 30-day period (a time-weighted average across
    /// renewals). Upgrade credit uses this, not the current price list, so a price change
    /// can never be turned into free time.
    pub unit_price: u64,
    pub bump: u8,
}

impl Subscription {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn active_tier(&self, now: i64) -> u8 {
        if now < self.expires_at {
            self.tier
        } else {
            0
        }
    }
}

/// Level for a claims bitmap. KYC (level 2+) requires the KYC claim itself (audit ID-04).
pub fn level_for_claims(claims: u16) -> u8 {
    let has = |bit: u16| claims & bit != 0;
    if has(CLAIM_KYC) && has(CLAIM_BIOMETRIC) && has(CLAIM_UNIQUE_HUMAN) {
        LEVEL_ENHANCED
    } else if has(CLAIM_KYC) && has(CLAIM_BIOMETRIC) {
        LEVEL_FULL
    } else if has(CLAIM_KYC) {
        LEVEL_KYC
    } else if has(CLAIM_EMAIL) && has(CLAIM_PHONE) {
        LEVEL_BASIC
    } else {
        LEVEL_NONE
    }
}

/// `[a-z0-9_]{3,32}`.
pub fn is_valid_username(name: &str) -> bool {
    (MIN_USERNAME_LEN..=MAX_USERNAME_LEN).contains(&name.len())
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

/// Result of [`subscription_quote`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubscriptionQuote {
    /// USDC to charge now.
    pub cost: u64,
    pub expires_at: i64,
    /// New `Subscription::unit_price`.
    pub unit_price: u64,
}

/// Cost of buying `tier` given the current subscription, and the new expiry: same tier
/// extends from max(now, expiry) by 30 days; an upgrade credits the unused time of the
/// current tier; a downgrade while active is refused (`None`) (audit ID-05).
///
/// The credit is what the remaining time actually cost (`unit_price`), never the current
/// list price. An upgrade normally costs `price − credit` for 30 days. If the prepaid time
/// is worth a full period of the new tier or more, the upgrade is free and the time carries
/// over at that value, so no paid value is dropped and none is created.
pub fn subscription_quote(
    current: &Subscription,
    tier: u8,
    prices: &[u64; NUM_TIERS],
    now: i64,
) -> Option<SubscriptionQuote> {
    let price = prices[tier as usize - 1];
    let period = SUBSCRIPTION_DURATION as u128;
    let active = current.active_tier(now);
    if active == 0 {
        return Some(SubscriptionQuote {
            cost: price,
            expires_at: now.checked_add(SUBSCRIPTION_DURATION)?,
            unit_price: price,
        });
    }
    let remaining = (current.expires_at - now) as u128; // active, so > 0
    let paid = current.unit_price as u128;
    if tier == active {
        // The remaining time keeps its cost; the new period costs today's price.
        let unit_price = (remaining * paid + period * price as u128) / (remaining + period);
        return Some(SubscriptionQuote {
            cost: price,
            expires_at: current.expires_at.checked_add(SUBSCRIPTION_DURATION)?,
            unit_price: u64::try_from(unit_price).ok()?,
        });
    }
    if tier < active {
        return None;
    }
    let credit = remaining * paid / period;
    if credit < price as u128 {
        return Some(SubscriptionQuote {
            cost: (price as u128 - credit) as u64,
            expires_at: now.checked_add(SUBSCRIPTION_DURATION)?,
            unit_price: price,
        });
    }
    let carried = if price == 0 {
        remaining
    } else {
        remaining * paid / price as u128
    };
    let carried = i64::try_from(carried).ok()?.max(SUBSCRIPTION_DURATION);
    Some(SubscriptionQuote {
        cost: 0,
        expires_at: now.checked_add(carried)?,
        unit_price: price,
    })
}
