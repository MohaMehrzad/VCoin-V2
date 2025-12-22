use anchor_lang::prelude::*;

/// Delegation account (PDA per delegator)
#[account]
#[derive(Default)]
pub struct Delegation {
    /// Who is delegating
    pub delegator: Pubkey,
    /// Who receives voting power
    pub delegate: Pubkey,
    /// Delegation type
    pub delegation_type: u8,
    /// Category bitmap (for PerCategory)
    pub categories: u8,
    /// Amount of veVCoin delegated
    pub delegated_amount: u64,
    /// When delegated
    pub delegated_at: i64,
    /// Expiration (0 = never)
    pub expires_at: i64,
    /// Whether revocable mid-vote
    pub revocable: bool,
    /// PDA bump
    pub bump: u8,
}

impl Delegation {
    pub const LEN: usize = 8 + // discriminator
        32 + // delegator
        32 + // delegate
        1 +  // delegation_type
        1 +  // categories
        8 +  // delegated_amount
        8 +  // delegated_at
        8 +  // expires_at
        1 +  // revocable
        1;   // bump
}

