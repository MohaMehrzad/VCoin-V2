use anchor_lang::prelude::*;

/// Delegation type enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum DelegationType {
    #[default]
    Full = 0,
    PerCategory = 1,
    PerProposal = 2,
}

