use anchor_lang::prelude::*;
use crate::constants::*;

/// Content type enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContentType {
    #[default]
    Post = 0,
    Article = 1,
    Media = 2,
    NFT = 3,
    Thread = 4,
}

impl ContentType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ContentType::Post),
            1 => Some(ContentType::Article),
            2 => Some(ContentType::Media),
            3 => Some(ContentType::NFT),
            4 => Some(ContentType::Thread),
            _ => None,
        }
    }
    
    pub fn energy_cost(&self) -> u16 {
        match self {
            ContentType::Post => ENERGY_COST_TEXT_POST,
            ContentType::Article => ENERGY_COST_IMAGE_POST, // Same as image
            ContentType::Media => ENERGY_COST_VIDEO_POST,
            ContentType::NFT => ENERGY_COST_IMAGE_POST,
            ContentType::Thread => ENERGY_COST_THREAD,
        }
    }
}
