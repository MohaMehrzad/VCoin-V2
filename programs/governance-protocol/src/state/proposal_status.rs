use anchor_lang::prelude::*;

/// Proposal status enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProposalStatus {
    #[default]
    Pending = 0,
    Active = 1,
    Passed = 2,
    Rejected = 3,
    Executed = 4,
    Cancelled = 5,
}

impl ProposalStatus {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ProposalStatus::Pending),
            1 => Some(ProposalStatus::Active),
            2 => Some(ProposalStatus::Passed),
            3 => Some(ProposalStatus::Rejected),
            4 => Some(ProposalStatus::Executed),
            5 => Some(ProposalStatus::Cancelled),
            _ => None,
        }
    }
}

