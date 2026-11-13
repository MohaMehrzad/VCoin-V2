//! Identities, usernames and attester-signed verification levels.

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::contexts::*;
use crate::errors::IdentityError;
use crate::events::*;
use crate::state::{is_valid_username, level_for_claims, Identity};

pub fn create_identity(
    ctx: Context<CreateIdentity>,
    did_hash: [u8; 32],
    username: String,
) -> Result<()> {
    require!(!ctx.accounts.config.paused, IdentityError::Paused);
    require!(is_valid_username(&username), IdentityError::InvalidUsername);
    let now = Clock::get()?.unix_timestamp;
    let owner = ctx.accounts.owner.key();
    let identity = &mut ctx.accounts.identity;
    identity.owner = owner;
    identity.did_hash = did_hash;
    identity.username = [0u8; MAX_USERNAME_LEN];
    identity.username[..username.len()].copy_from_slice(username.as_bytes());
    identity.username_len = username.len() as u8;
    identity.level = LEVEL_NONE;
    identity.claims = 0;
    identity.attester = Pubkey::default();
    identity.attestation_id = [0u8; 32];
    identity.level_expires_at = 0;
    identity.created_at = now;
    identity.updated_at = now;
    identity.bump = ctx.bumps.identity;
    let record = &mut ctx.accounts.username_record;
    record.owner = owner;
    record.bump = ctx.bumps.username_record;
    emit!(IdentityCreated {
        owner,
        username,
        did_hash
    });
    Ok(())
}

/// Any owner may fix its DID hash, verified or not (audit ID-L1).
pub fn update_did_hash(ctx: Context<UpdateIdentity>, did_hash: [u8; 32]) -> Result<()> {
    require!(!ctx.accounts.config.paused, IdentityError::Paused);
    let identity = &mut ctx.accounts.identity;
    identity.did_hash = did_hash;
    identity.updated_at = Clock::get()?.unix_timestamp;
    emit!(DidHashUpdated {
        owner: identity.owner,
        did_hash
    });
    Ok(())
}

pub fn close_identity(ctx: Context<CloseIdentity>) -> Result<()> {
    emit!(IdentityClosed {
        owner: ctx.accounts.owner.key()
    });
    Ok(())
}

/// Links (or refreshes) an attestation. The identity takes the attestation's level, up or
/// down (audit ID-01); one attestation id binds to one wallet (ID-03).
pub fn link_attestation(
    ctx: Context<LinkAttestation>,
    attestation_id: [u8; 32],
    claims: u16,
    expires_at: i64,
) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, IdentityError::Paused);
    let now = Clock::get()?.unix_timestamp;
    let owner = ctx.accounts.owner.key();
    let attester = ctx.accounts.attester.key();
    require!(
        config.is_attester(&attester),
        IdentityError::NotTrustedAttester
    );
    require_keys_neq!(attester, owner, IdentityError::SelfAttestation);
    require!(
        claims & !VALID_CLAIMS_MASK == 0,
        IdentityError::InvalidClaims
    );
    require!(
        expires_at > now && expires_at - now <= MAX_ATTESTATION_VALIDITY,
        IdentityError::InvalidExpiry
    );

    let link = &mut ctx.accounts.attestation_link;
    if link.owner == Pubkey::default() {
        link.attestation_id = attestation_id;
        link.owner = owner;
        link.rent_payer = owner;
        link.bump = ctx.bumps.attestation_link;
    }
    require_keys_eq!(link.owner, owner, IdentityError::AttestationAlreadyLinked);
    // A refresh must come from the attester that issued this id: another attester could
    // otherwise take the link over and strip the issuer's right to revoke it.
    require!(
        link.attester == Pubkey::default() || link.attester == attester,
        IdentityError::AttesterMismatch
    );
    let level = level_for_claims(claims);
    link.attester = attester;
    link.claims = claims;
    link.level = level;
    link.expires_at = expires_at;
    link.linked_at = now;

    let identity = &mut ctx.accounts.identity;
    identity.level = level;
    identity.claims = claims;
    identity.attester = attester;
    identity.attestation_id = attestation_id;
    identity.level_expires_at = expires_at;
    identity.updated_at = now;

    emit!(AttestationLinked {
        owner,
        attester,
        attestation_id,
        claims,
        level,
        expires_at
    });
    Ok(())
}

/// The issuing attester (even if no longer trusted) or the authority revokes (audit ID-02).
pub fn revoke_attestation(ctx: Context<RevokeAttestation>) -> Result<()> {
    let revoker = ctx.accounts.revoker.key();
    let link = &ctx.accounts.attestation_link;
    require!(
        revoker == link.attester || revoker == ctx.accounts.config.admin.authority,
        IdentityError::NotRevoker
    );
    // The identity PDA is always passed (it is derived from the link), so a revocation can
    // never close the link and leave the level standing. It may have been closed already.
    let info = ctx.accounts.identity.to_account_info();
    if info.owner == &crate::ID && !info.data_is_empty() {
        let mut data = info.try_borrow_mut_data()?;
        let mut identity = Identity::try_deserialize(&mut &data[..])?;
        if identity.attestation_id == link.attestation_id {
            identity.level = LEVEL_NONE;
            identity.claims = 0;
            identity.attester = Pubkey::default();
            identity.attestation_id = [0u8; 32];
            identity.level_expires_at = 0;
            identity.updated_at = Clock::get()?.unix_timestamp;
            identity.try_serialize(&mut &mut data[..])?;
        }
    }
    emit!(AttestationRevoked {
        owner: link.owner,
        attestation_id: link.attestation_id,
        revoked_by: revoker
    });
    Ok(())
}

/// The authority may only lower a level, never grant one (audit ID-01/02).
pub fn admin_lower_level(ctx: Context<AdminLowerLevel>, new_level: u8) -> Result<()> {
    ctx.accounts
        .config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    let identity = &mut ctx.accounts.identity;
    require!(new_level < identity.level, IdentityError::NotADowngrade);
    let old_level = identity.level;
    identity.level = new_level;
    identity.updated_at = Clock::get()?.unix_timestamp;
    emit!(LevelLowered {
        owner: identity.owner,
        old_level,
        new_level
    });
    Ok(())
}
