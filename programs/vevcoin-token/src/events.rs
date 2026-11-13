use anchor_lang::prelude::*;

#[event]
pub struct VeVCoinInitialized {
    pub config: Pubkey,
    pub mint: Pubkey,
    pub authority: Pubkey,
    pub staking_authority: Pubkey,
}

#[event]
pub struct VeVCoinMinted {
    pub destination: Pubkey,
    pub amount: u64,
}

#[event]
pub struct VeVCoinBurned {
    pub source: Pubkey,
    /// Amount the staking program asked to burn.
    pub requested: u64,
    /// Amount actually burned: `min(requested, balance)` (a holder may have burned
    /// their own receipt; that must never block an unstake).
    pub burned: u64,
}

#[event]
pub struct AuthorityProposed {
    pub current: Pubkey,
    pub proposed: Pubkey,
    pub ready_at: i64,
}

#[event]
pub struct AuthorityTransferCancelled {
    pub authority: Pubkey,
    pub cancelled: Pubkey,
}

#[event]
pub struct AuthorityTransferred {
    pub previous: Pubkey,
    pub new_authority: Pubkey,
}
