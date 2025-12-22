use anchor_lang::prelude::*;

/// Content state enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContentState {
    #[default]
    Active = 0,
    Edited = 1,
    Deleted = 2,
    Archived = 3,
}

impl ContentState {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ContentState::Active),
            1 => Some(ContentState::Edited),
            2 => Some(ContentState::Deleted),
            3 => Some(ContentState::Archived),
            _ => None,
        }
    }
}
