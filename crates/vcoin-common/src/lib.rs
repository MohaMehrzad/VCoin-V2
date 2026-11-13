//! Shared security primitives for the ViWo programs.
//!
//! * [`AdminState`]: two-step authority transfer with a mandatory 24 h delay
//!   (audit X-2 / H-NEW-01: previously present in only 4 of 11 programs).
//! * [`require_upgrade_authority`]: initializers may only be called by the
//!   program's upgrade authority (audit X-1: every `initialize` was front-runnable).
//!   Use it together with this Accounts pattern:
//!
//! ```ignore
//! #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
//! pub program: Program<'info, crate::program::MyProgram>,
//! pub program_data: Account<'info, ProgramData>,
//! ```
//! * [`validate_mint`] / [`validate_vcoin_mint`]: reject mints with unexpected
//!   Token-2022 extensions (transfer hooks, transfer fees, permanent delegates),
//!   a freeze authority, or the wrong decimals.

use anchor_lang::prelude::*;
use anchor_spl::token_2022::spl_token_2022::{
    self,
    extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions},
    state::Mint as Token2022Mint,
};

/// Delay between proposing and accepting a new authority, in seconds (24 h).
pub const AUTHORITY_TRANSFER_DELAY: i64 = 24 * 60 * 60;

/// VCoin decimals.
pub const VCOIN_DECIMALS: u8 = 9;

/// Token-2022 extensions the VCoin mint may carry: on-chain metadata only.
/// No transfer hook, transfer fee, permanent delegate, pausable or confidential extensions.
pub const VCOIN_ALLOWED_EXTENSIONS: &[ExtensionType] =
    &[ExtensionType::MetadataPointer, ExtensionType::TokenMetadata];

/// Shared errors. Offset 9000 keeps them disjoint from each program's own 6000+ codes.
#[error_code(offset = 9000)]
pub enum CommonError {
    #[msg("Signer is not the authority")]
    NotAuthority,
    #[msg("Signer is not the pending authority")]
    NotPendingAuthority,
    #[msg("No authority transfer is pending")]
    NoPendingAuthority,
    #[msg("The 24h authority transfer delay has not elapsed")]
    AuthorityTimelockActive,
    #[msg("New authority must be a non-default key different from the current authority")]
    InvalidNewAuthority,
    #[msg("Signer is not the program's upgrade authority")]
    NotUpgradeAuthority,
    #[msg("Mint is not owned by the Token-2022 program")]
    MintNotToken2022,
    #[msg("Mint account data is invalid")]
    InvalidMintData,
    #[msg("Mint has unexpected decimals")]
    InvalidMintDecimals,
    #[msg("Mint must not have a freeze authority")]
    MintHasFreezeAuthority,
    #[msg("Mint carries a Token-2022 extension that is not allowed")]
    ForbiddenMintExtension,
    #[msg("Arithmetic overflow")]
    Overflow,
}

/// Authority with a two-step, time-locked transfer.
///
/// `propose` (current authority) → wait [`AUTHORITY_TRANSFER_DELAY`] → `accept`
/// (new authority). The current authority can `cancel` at any time before acceptance.
#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, Default, PartialEq, Eq, InitSpace,
)]
pub struct AdminState {
    pub authority: Pubkey,
    /// `Pubkey::default()` when no transfer is pending.
    pub pending_authority: Pubkey,
    /// Unix time the pending transfer was proposed (0 when none).
    pub pending_since: i64,
}

impl AdminState {
    pub fn new(authority: Pubkey) -> Self {
        Self {
            authority,
            pending_authority: Pubkey::default(),
            pending_since: 0,
        }
    }

    pub fn has_pending(&self) -> bool {
        self.pending_authority != Pubkey::default()
    }

    pub fn require_authority(&self, signer: &Pubkey) -> Result<()> {
        require_keys_eq!(*signer, self.authority, CommonError::NotAuthority);
        Ok(())
    }

    /// Starts a transfer to `new_authority`. Replaces any previous pending transfer
    /// (restarting the delay).
    pub fn propose(&mut self, signer: &Pubkey, new_authority: Pubkey, now: i64) -> Result<()> {
        self.require_authority(signer)?;
        require!(
            new_authority != Pubkey::default() && new_authority != self.authority,
            CommonError::InvalidNewAuthority
        );
        self.pending_authority = new_authority;
        self.pending_since = now;
        Ok(())
    }

    /// Cancels a pending transfer.
    pub fn cancel(&mut self, signer: &Pubkey) -> Result<()> {
        self.require_authority(signer)?;
        require!(self.has_pending(), CommonError::NoPendingAuthority);
        self.pending_authority = Pubkey::default();
        self.pending_since = 0;
        Ok(())
    }

    /// Completes a pending transfer once the delay has elapsed. Returns the old authority.
    pub fn accept(&mut self, signer: &Pubkey, now: i64) -> Result<Pubkey> {
        require!(self.has_pending(), CommonError::NoPendingAuthority);
        require_keys_eq!(
            *signer,
            self.pending_authority,
            CommonError::NotPendingAuthority
        );
        let ready_at = self
            .pending_since
            .checked_add(AUTHORITY_TRANSFER_DELAY)
            .ok_or(CommonError::Overflow)?;
        require!(now >= ready_at, CommonError::AuthorityTimelockActive);
        let old = self.authority;
        self.authority = self.pending_authority;
        self.pending_authority = Pubkey::default();
        self.pending_since = 0;
        Ok(old)
    }
}

/// True when a scheduled change (`eta != 0`) may be applied at `now`.
pub fn timelock_ready(eta: i64, now: i64) -> bool {
    eta != 0 && now >= eta
}

/// Requires `signer` to be the program's upgrade authority (see the module docs for
/// the matching Accounts constraints that bind `program_data` to the running program).
pub fn require_upgrade_authority(program_data: &ProgramData, signer: &Pubkey) -> Result<()> {
    require!(
        program_data.upgrade_authority_address == Some(*signer),
        CommonError::NotUpgradeAuthority
    );
    Ok(())
}

/// Validates a Token-2022 mint: owner, decimals, no freeze authority, and only
/// extensions from `allowed`.
pub fn validate_mint(mint: &AccountInfo, decimals: u8, allowed: &[ExtensionType]) -> Result<()> {
    require_keys_eq!(
        *mint.owner,
        spl_token_2022::ID,
        CommonError::MintNotToken2022
    );
    let data = mint.try_borrow_data()?;
    let state = StateWithExtensions::<Token2022Mint>::unpack(&data)
        .map_err(|_| error!(CommonError::InvalidMintData))?;
    require!(state.base.is_initialized, CommonError::InvalidMintData);
    require!(
        state.base.decimals == decimals,
        CommonError::InvalidMintDecimals
    );
    require!(
        state.base.freeze_authority.is_none(),
        CommonError::MintHasFreezeAuthority
    );
    let extensions = state
        .get_extension_types()
        .map_err(|_| error!(CommonError::InvalidMintData))?;
    for extension in extensions {
        require!(
            allowed.contains(&extension),
            CommonError::ForbiddenMintExtension
        );
    }
    Ok(())
}

/// Validates the VCoin mint: Token-2022, 9 decimals, no freeze authority, metadata-only.
pub fn validate_vcoin_mint(mint: &AccountInfo) -> Result<()> {
    validate_mint(mint, VCOIN_DECIMALS, VCOIN_ALLOWED_EXTENSIONS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::solana_program::program_option::COption;
    use anchor_spl::token_2022::spl_token_2022::extension::{
        metadata_pointer::MetadataPointer, transfer_hook::TransferHook, BaseStateWithExtensionsMut,
        StateWithExtensionsMut,
    };

    fn code(err: anchor_lang::error::Error) -> u32 {
        match err {
            anchor_lang::error::Error::AnchorError(e) => e.error_code_number,
            other => panic!("unexpected error {other:?}"),
        }
    }

    fn assert_err<T: std::fmt::Debug>(r: Result<T>, expected: CommonError) {
        let e = r.expect_err("expected an error");
        assert_eq!(code(e), u32::from(expected));
    }

    #[test]
    fn authority_transfer_requires_24h_and_pending_signer() {
        let (a, b, x) = (
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        );
        let mut s = AdminState::new(a);

        assert_err(s.propose(&x, b, 100), CommonError::NotAuthority);
        assert_err(
            s.propose(&a, Pubkey::default(), 100),
            CommonError::InvalidNewAuthority,
        );
        assert_err(s.propose(&a, a, 100), CommonError::InvalidNewAuthority);
        s.propose(&a, b, 100).unwrap();

        assert_err(
            s.accept(&x, 100 + AUTHORITY_TRANSFER_DELAY),
            CommonError::NotPendingAuthority,
        );
        assert_err(
            s.accept(&b, 100 + AUTHORITY_TRANSFER_DELAY - 1),
            CommonError::AuthorityTimelockActive,
        );
        assert_eq!(s.accept(&b, 100 + AUTHORITY_TRANSFER_DELAY).unwrap(), a);
        assert_eq!(s.authority, b);
        assert!(!s.has_pending());
        assert_err(s.accept(&b, i64::MAX), CommonError::NoPendingAuthority);
    }

    #[test]
    fn cancel_clears_pending_and_reproposal_restarts_delay() {
        let (a, b, c) = (
            Pubkey::new_unique(),
            Pubkey::new_unique(),
            Pubkey::new_unique(),
        );
        let mut s = AdminState::new(a);
        assert_err(s.cancel(&a), CommonError::NoPendingAuthority);
        s.propose(&a, b, 0).unwrap();
        s.cancel(&a).unwrap();
        assert_err(
            s.accept(&b, AUTHORITY_TRANSFER_DELAY),
            CommonError::NoPendingAuthority,
        );

        s.propose(&a, b, 0).unwrap();
        s.propose(&a, c, 1_000).unwrap(); // replaces b and restarts the clock
        assert_err(
            s.accept(&b, 1_000 + AUTHORITY_TRANSFER_DELAY),
            CommonError::NotPendingAuthority,
        );
        assert_err(
            s.accept(&c, AUTHORITY_TRANSFER_DELAY),
            CommonError::AuthorityTimelockActive,
        );
        s.accept(&c, 1_000 + AUTHORITY_TRANSFER_DELAY).unwrap();
    }

    #[test]
    fn accept_overflow_is_an_error_not_a_panic() {
        let (a, b) = (Pubkey::new_unique(), Pubkey::new_unique());
        let mut s = AdminState::new(a);
        s.propose(&a, b, i64::MAX).unwrap();
        assert_err(s.accept(&b, i64::MAX), CommonError::Overflow);
    }

    #[test]
    fn timelock_ready_semantics() {
        assert!(
            !timelock_ready(0, i64::MAX),
            "eta 0 means nothing scheduled"
        );
        assert!(!timelock_ready(100, 99));
        assert!(timelock_ready(100, 100));
    }

    #[test]
    fn upgrade_authority_gate() {
        let auth = Pubkey::new_unique();
        let pd = ProgramData {
            slot: 0,
            upgrade_authority_address: Some(auth),
        };
        require_upgrade_authority(&pd, &auth).unwrap();
        assert_err(
            require_upgrade_authority(&pd, &Pubkey::new_unique()),
            CommonError::NotUpgradeAuthority,
        );
        let immutable = ProgramData {
            slot: 0,
            upgrade_authority_address: None,
        };
        assert_err(
            require_upgrade_authority(&immutable, &auth),
            CommonError::NotUpgradeAuthority,
        );
    }

    /// Builds Token-2022 mint account bytes with the given extensions.
    fn mint_bytes(decimals: u8, freeze: Option<Pubkey>, extensions: &[ExtensionType]) -> Vec<u8> {
        let len = ExtensionType::try_calculate_account_len::<Token2022Mint>(extensions).unwrap();
        let mut data = vec![0u8; len];
        let mut state =
            StateWithExtensionsMut::<Token2022Mint>::unpack_uninitialized(&mut data).unwrap();
        for ext in extensions {
            match ext {
                ExtensionType::MetadataPointer => {
                    state.init_extension::<MetadataPointer>(true).unwrap();
                }
                ExtensionType::TransferHook => {
                    state.init_extension::<TransferHook>(true).unwrap();
                }
                other => panic!("test helper does not build {other:?}"),
            }
        }
        state.base = Token2022Mint {
            mint_authority: COption::Some(Pubkey::new_unique()),
            supply: 0,
            decimals,
            is_initialized: true,
            freeze_authority: freeze.map(COption::Some).unwrap_or(COption::None),
        };
        state.pack_base();
        state.init_account_type().unwrap();
        data
    }

    fn check(data: &mut [u8], owner: &Pubkey) -> Result<()> {
        let key = Pubkey::new_unique();
        let mut lamports = 1_000_000u64;
        let ai = AccountInfo::new(&key, false, false, &mut lamports, data, owner, false, 0);
        validate_vcoin_mint(&ai)
    }

    #[test]
    fn vcoin_mint_validation() {
        let t22 = spl_token_2022::ID;
        check(
            &mut mint_bytes(9, None, &[ExtensionType::MetadataPointer]),
            &t22,
        )
        .unwrap();
        check(&mut mint_bytes(9, None, &[]), &t22).unwrap();

        assert_err(
            check(
                &mut mint_bytes(9, None, &[ExtensionType::TransferHook]),
                &t22,
            ),
            CommonError::ForbiddenMintExtension,
        );
        assert_err(
            check(&mut mint_bytes(6, None, &[]), &t22),
            CommonError::InvalidMintDecimals,
        );
        assert_err(
            check(&mut mint_bytes(9, Some(Pubkey::new_unique()), &[]), &t22),
            CommonError::MintHasFreezeAuthority,
        );
        assert_err(
            check(&mut mint_bytes(9, None, &[]), &anchor_spl::token::ID),
            CommonError::MintNotToken2022,
        );
    }
}
