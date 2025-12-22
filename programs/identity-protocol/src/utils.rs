use crate::state::VerificationLevel;

/// Derive verification level from claims bitmap
pub fn derive_verification_level(claims: u16) -> u8 {
    // Claims bitmap:
    // bit 0: Email verified
    // bit 1: Phone verified
    // bit 2: Social verified
    // bit 3: KYC verified
    // bit 4: Biometric verified
    // bit 5: UniqueHuman verified
    
    let has_email = claims & 0x01 != 0;
    let has_phone = claims & 0x02 != 0;
    let has_social = claims & 0x04 != 0;
    let has_kyc = claims & 0x08 != 0;
    let has_biometric = claims & 0x10 != 0;
    let has_unique_human = claims & 0x20 != 0;
    
    if has_kyc && has_biometric && has_unique_human {
        VerificationLevel::Enhanced as u8
    } else if has_kyc && has_biometric {
        VerificationLevel::Full as u8
    } else if has_kyc || (has_email && has_phone && has_social) {
        VerificationLevel::KYC as u8
    } else if has_email && has_phone {
        VerificationLevel::Basic as u8
    } else {
        VerificationLevel::None as u8
    }
}

