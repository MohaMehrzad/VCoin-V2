use anchor_lang::prelude::*;
use vcoin_common::AdminState;

use crate::constants::*;
use crate::errors::GaslessError;

/// Singleton configuration, PDA `["gasless-config"]`.
#[account]
#[derive(Default, InitSpace)]
pub struct GaslessConfig {
    pub admin: AdminState,
    pub vcoin_mint: Pubkey,
    /// VCoin vault (PDA `["fee-vault"]`, authority = this config) collecting fees.
    pub fee_vault: Pubkey,
    /// VCoin token account that `withdraw_fees` pays out to.
    pub treasury: Pubkey,
    pub pending_treasury: Pubkey,
    pub treasury_eta: i64,
    /// VCoin charged per `pay_vcoin_fee` (base units).
    pub vcoin_fee_per_tx: u64,
    pub pending_fee: u64,
    pub fee_eta: i64,
    pub paused: bool,
    pub bump: u8,
    pub vault_bump: u8,
}

impl GaslessConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;
}

/// A scoped, expiring session key, PDA
/// `["session", target_program, session_signer, authority]` (MagicBlock gpl-session model).
///
/// Consuming programs load it as `Account<SessionToken>` with
/// `seeds::program = gasless_protocol::ID` and call [`SessionToken::verify`].
#[account]
#[derive(Default, InitSpace)]
pub struct SessionToken {
    /// The wallet the session acts for.
    pub authority: Pubkey,
    /// The only program this session may be used with.
    pub target_program: Pubkey,
    /// Ephemeral key that signs on the authority's behalf.
    pub session_signer: Pubkey,
    pub scope: u16,
    pub valid_until: i64,
    /// Account that paid rent; refunded on revoke.
    pub payer: Pubkey,
    pub bump: u8,
}

impl SessionToken {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub fn is_expired(&self, now: i64) -> bool {
        now >= self.valid_until
    }

    /// Verifies that `signer` may perform `action` for `authority` in `target_program` now.
    /// `action` must be exactly one scope bit (audit GAS-04: `scope & action != 0` used to
    /// accept 0xFFFF for any non-empty scope).
    pub fn verify(
        &self,
        authority: &Pubkey,
        signer: &Pubkey,
        target_program: &Pubkey,
        action: u16,
        now: i64,
    ) -> Result<()> {
        require!(
            self.authority == *authority
                && self.session_signer == *signer
                && self.target_program == *target_program,
            GaslessError::SessionMismatch
        );
        require!(!self.is_expired(now), GaslessError::SessionExpired);
        require!(
            action.count_ones() == 1
                && action & VALID_SCOPE_MASK != 0
                && self.scope & action == action,
            GaslessError::ActionNotInScope
        );
        Ok(())
    }
}

/// Validates a requested scope: non-empty and only allowed bits (audit TOOL-11/GAS-06).
pub fn validate_scope(scope: u16) -> Result<()> {
    require!(
        scope != 0 && scope & !VALID_SCOPE_MASK == 0,
        GaslessError::InvalidScope
    );
    Ok(())
}
