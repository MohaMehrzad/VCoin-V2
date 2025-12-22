use anchor_lang::prelude::*;

/// Verification levels
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum VerificationLevel {
    #[default]
    None = 0,
    Basic = 1,      // Email + phone
    KYC = 2,        // Identity documents
    Full = 3,       // KYC + biometric
    Enhanced = 4,   // Full + UniqueHuman
}

impl VerificationLevel {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(VerificationLevel::None),
            1 => Some(VerificationLevel::Basic),
            2 => Some(VerificationLevel::KYC),
            3 => Some(VerificationLevel::Full),
            4 => Some(VerificationLevel::Enhanced),
            _ => None,
        }
    }
}

