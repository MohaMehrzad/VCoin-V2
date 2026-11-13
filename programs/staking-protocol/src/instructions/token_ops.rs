//! Token CPIs signed by the registrar PDA (vault owner and veVCoin staking authority).

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::constants::REGISTRAR_SEED;

/// The registrar account and its bump, used as PDA signer.
pub struct RegistrarSigner<'info> {
    pub info: AccountInfo<'info>,
    pub bump: u8,
}

/// Moves `amount` VCoin out of the vault.
pub fn vault_transfer<'info>(
    token_program: AccountInfo<'info>,
    vault: AccountInfo<'info>,
    mint: AccountInfo<'info>,
    to: AccountInfo<'info>,
    registrar: &RegistrarSigner<'info>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    let bump = [registrar.bump];
    let seeds: &[&[u8]] = &[REGISTRAR_SEED, &bump];
    transfer_checked(
        CpiContext::new_with_signer(
            token_program,
            TransferChecked {
                from: vault,
                mint,
                to,
                authority: registrar.info.clone(),
            },
            &[seeds],
        ),
        amount,
        decimals,
    )
}

/// veVCoin program accounts for receipt mint/burn.
pub struct ReceiptAccounts<'info> {
    pub vevcoin_program: AccountInfo<'info>,
    pub config: AccountInfo<'info>,
    pub mint: AccountInfo<'info>,
    /// The owner's receipt ATA.
    pub account: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
}

pub fn mint_receipt<'info>(
    r: ReceiptAccounts<'info>,
    registrar: &RegistrarSigner<'info>,
    amount: u64,
) -> Result<()> {
    let bump = [registrar.bump];
    let seeds: &[&[u8]] = &[REGISTRAR_SEED, &bump];
    vevcoin_token::cpi::mint_vevcoin(
        CpiContext::new_with_signer(
            r.vevcoin_program,
            vevcoin_token::cpi::accounts::MintVeVCoin {
                config: r.config,
                mint: r.mint,
                destination: r.account,
                staking_authority: registrar.info.clone(),
                token_program: r.token_program,
            },
            &[seeds],
        ),
        amount,
    )
}

/// Burns `min(amount, balance)` receipts (the veVCoin program clamps), so a holder who
/// already burned their own receipt can always withdraw and can still be slashed.
pub fn burn_receipt<'info>(
    r: ReceiptAccounts<'info>,
    registrar: &RegistrarSigner<'info>,
    amount: u64,
) -> Result<()> {
    let bump = [registrar.bump];
    let seeds: &[&[u8]] = &[REGISTRAR_SEED, &bump];
    vevcoin_token::cpi::burn_vevcoin(
        CpiContext::new_with_signer(
            r.vevcoin_program,
            vevcoin_token::cpi::accounts::BurnVeVCoin {
                config: r.config,
                mint: r.mint,
                source: r.account,
                staking_authority: registrar.info.clone(),
                token_program: r.token_program,
            },
            &[seeds],
        ),
        amount,
    )
}
