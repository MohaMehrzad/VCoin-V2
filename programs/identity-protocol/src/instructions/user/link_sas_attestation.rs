use anchor_lang::prelude::*;

use crate::contexts::LinkSASAttestation;
use crate::errors::IdentityError;
use crate::events::SASAttestationLinked;
use crate::utils::derive_verification_level;

/// Link SAS attestation to identity
pub fn handler(
    ctx: Context<LinkSASAttestation>,
    sas_attestation_id: Pubkey,
    attestation_type: u8,
    verified_claims: u16,
    expires_at: i64,
    portable_score: u16,
) -> Result<()> {
    let config = &ctx.accounts.identity_config;
    
    // Verify attester is trusted
    let attester = ctx.accounts.attester.key();
    let is_trusted = config.trusted_attesters[..config.attester_count as usize]
        .contains(&attester);
    require!(is_trusted, IdentityError::UntrustedAttester);
    
    let clock = Clock::get()?;
    let sas = &mut ctx.accounts.sas_attestation;
    
    sas.user = ctx.accounts.user.key();
    sas.sas_attestation_id = sas_attestation_id;
    sas.attestation_type = attestation_type;
    sas.attester = attester;
    sas.verified_claims = verified_claims;
    
    // Derive verification level from claims
    sas.verification_level = derive_verification_level(verified_claims);
    
    sas.first_verified_at = clock.unix_timestamp;
    sas.last_verified_at = clock.unix_timestamp;
    sas.expires_at = expires_at;
    sas.portable_score = portable_score;
    sas.bump = ctx.bumps.sas_attestation;
    
    // Update identity verification level if higher
    let identity = &mut ctx.accounts.identity;
    if sas.verification_level > identity.verification_level {
        identity.verification_level = sas.verification_level;
        identity.updated_at = clock.unix_timestamp;
    }
    
    emit!(SASAttestationLinked {
        user: sas.user,
        attestation_id: sas_attestation_id,
        attester,
        verification_level: sas.verification_level,
    });
    
    msg!("SAS attestation linked for: {}", sas.user);
    Ok(())
}

