use anchor_lang::prelude::*;

use crate::constants::*;

/// Subscription tiers
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubscriptionTier {
    #[default]
    Free = 0,
    Verified = 1,   // $4/month
    Premium = 2,    // $12/month
    Enterprise = 3, // $59/month
}

impl SubscriptionTier {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(SubscriptionTier::Free),
            1 => Some(SubscriptionTier::Verified),
            2 => Some(SubscriptionTier::Premium),
            3 => Some(SubscriptionTier::Enterprise),
            _ => None,
        }
    }
    
    pub fn price(&self) -> u64 {
        match self {
            SubscriptionTier::Free => SUBSCRIPTION_FREE,
            SubscriptionTier::Verified => SUBSCRIPTION_VERIFIED,
            SubscriptionTier::Premium => SUBSCRIPTION_PREMIUM,
            SubscriptionTier::Enterprise => SUBSCRIPTION_ENTERPRISE,
        }
    }
}

