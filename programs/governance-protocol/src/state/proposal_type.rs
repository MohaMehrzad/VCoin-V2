use anchor_lang::prelude::*;

/// Proposal type enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProposalType {
    #[default]
    Parameter = 0,
    Treasury = 1,
    Protocol = 2,
    Emissions = 3,
}

impl ProposalType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ProposalType::Parameter),
            1 => Some(ProposalType::Treasury),
            2 => Some(ProposalType::Protocol),
            3 => Some(ProposalType::Emissions),
            _ => None,
        }
    }
}

