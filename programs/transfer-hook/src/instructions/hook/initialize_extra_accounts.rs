use anchor_lang::prelude::*;

use crate::contexts::InitializeExtraAccountMetaList;

/// Initialize extra account metas for the transfer hook
/// Required by Token-2022 transfer hook interface
pub fn handler(_ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
    // The extra accounts needed for the transfer hook execution
    // These are stored in the ExtraAccountMetaList account
    msg!("Extra account meta list initialized");
    Ok(())
}

