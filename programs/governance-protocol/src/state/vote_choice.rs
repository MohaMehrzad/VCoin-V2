use anchor_lang::prelude::*;

/// Vote choice enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum VoteChoice {
    #[default]
    Abstain = 0,
    For = 1,
    Against = 2,
}

impl VoteChoice {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(VoteChoice::Abstain),
            1 => Some(VoteChoice::For),
            2 => Some(VoteChoice::Against),
            _ => None,
        }
    }
}

