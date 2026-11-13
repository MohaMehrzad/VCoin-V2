//! Reads of foreign accounts: the Realms realm / token owner record and the 5A score.
//! Every read checks owner, PDA address and type tag (audit X-5).

use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::StakingError;
use crate::state::Registrar;

/// The realm must be a RealmV2 of `governance_program` whose community mint is `vcoin_mint`.
pub fn validate_realm(
    realm: &AccountInfo,
    governance_program: &Pubkey,
    vcoin_mint: &Pubkey,
) -> Result<()> {
    require_keys_eq!(
        *realm.owner,
        *governance_program,
        StakingError::InvalidRealm
    );
    let data = realm.try_borrow_data()?;
    require!(
        data.len() >= 33 && data[0] == REALM_V2_ACCOUNT_TYPE,
        StakingError::InvalidRealm
    );
    require!(
        data[1..33] == vcoin_mint.to_bytes(),
        StakingError::InvalidRealm
    );
    Ok(())
}

/// Address of `owner`'s TokenOwnerRecord in the configured realm.
pub fn token_owner_record_address(registrar: &Registrar, owner: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            GOVERNANCE_SEED,
            registrar.realm.as_ref(),
            registrar.realm_governing_token_mint.as_ref(),
            owner.as_ref(),
        ],
        &registrar.governance_program_id,
    )
    .0
}

/// Blocks withdrawals while the owner has unrelinquished votes or outstanding proposals,
/// as the Voter-Stake-Registry does (audit GOV-03: tokens voted, were withdrawn and voted
/// again from another wallet).
pub fn assert_can_withdraw(
    registrar: &Registrar,
    owner: &Pubkey,
    record: &AccountInfo,
) -> Result<()> {
    if registrar.governance_program_id == Pubkey::default() {
        return Ok(()); // governance not wired yet: no votes can exist
    }
    require_keys_eq!(
        record.key(),
        token_owner_record_address(registrar, owner),
        StakingError::InvalidTokenOwnerRecord
    );
    // Only the governance program can create an account at this PDA; anything else means
    // the owner never joined the realm.
    if *record.owner != registrar.governance_program_id || record.data_is_empty() {
        return Ok(());
    }
    let data = record.try_borrow_data()?;
    require!(
        data.len() >= 115 && data[0] == TOKEN_OWNER_RECORD_V2_ACCOUNT_TYPE,
        StakingError::InvalidTokenOwnerRecord
    );
    require!(
        data[1..33] == registrar.realm.to_bytes()
            && data[33..65] == registrar.realm_governing_token_mint.to_bytes()
            && data[65..97] == owner.to_bytes(),
        StakingError::InvalidTokenOwnerRecord
    );
    let mut unrelinquished = u64::from_le_bytes(data[105..113].try_into().unwrap());
    if data[114] < 1 {
        // Pre-v1 records stored a u32 count followed by unused bytes.
        unrelinquished &= u32::MAX as u64;
    }
    let outstanding_proposals = data[113];
    require!(
        unrelinquished == 0 && outstanding_proposals == 0,
        StakingError::VotesNotRelinquished
    );
    Ok(())
}

/// The owner's 5A composite score (0 when the owner has never been scored). The 5A program
/// id is pinned at compile time, so no configuration can point the gate at a look-alike.
pub fn five_a_score(owner: &Pubkey, score_account: &AccountInfo) -> Result<u16> {
    let expected = Pubkey::find_program_address(
        &[five_a_protocol::constants::USER_SCORE_SEED, owner.as_ref()],
        &five_a_protocol::ID,
    )
    .0;
    require_keys_eq!(
        score_account.key(),
        expected,
        StakingError::InvalidScoreAccount
    );
    if *score_account.owner != five_a_protocol::ID || score_account.data_is_empty() {
        return Ok(0);
    }
    let data = score_account.try_borrow_data()?;
    let score = five_a_protocol::state::UserScore::try_deserialize(&mut &data[..])?;
    Ok(score.composite_score.min(MAX_FIVE_A_SCORE))
}
