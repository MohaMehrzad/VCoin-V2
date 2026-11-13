pub mod authority;
pub mod initialize;
pub mod set_paused;
pub mod update_metadata;

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

/// Tops `account` up to rent exemption for `target_len` bytes, paid by `payer`.
/// Token-2022 reallocates the mint for metadata but does not fund the extra rent.
pub(crate) fn fund_rent<'info>(
    account: &AccountInfo<'info>,
    payer: &AccountInfo<'info>,
    system_program: &AccountInfo<'info>,
    target_len: usize,
) -> Result<()> {
    let required = Rent::get()?.minimum_balance(target_len);
    let top_up = required.saturating_sub(account.lamports());
    if top_up > 0 {
        transfer(
            CpiContext::new(
                system_program.clone(),
                Transfer {
                    from: payer.clone(),
                    to: account.clone(),
                },
            ),
            top_up,
        )?;
    }
    Ok(())
}
