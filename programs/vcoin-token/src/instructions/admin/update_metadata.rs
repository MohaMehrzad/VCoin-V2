use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    spl_token_metadata_interface::state::Field, token_metadata_update_field,
    TokenMetadataUpdateField,
};

use super::fund_rent;
use crate::constants::*;
use crate::contexts::UpdateMetadata;
use crate::errors::VCoinError;
use crate::events::MetadataUpdated;
use crate::state::MetadataField;

pub fn handler(ctx: Context<UpdateMetadata>, field: MetadataField, value: String) -> Result<()> {
    let config = &ctx.accounts.config;
    config
        .admin
        .require_authority(&ctx.accounts.authority.key())?;
    let max_len = match field {
        MetadataField::Name => MAX_NAME_LEN,
        MetadataField::Symbol => MAX_SYMBOL_LEN,
        MetadataField::Uri => MAX_URI_LEN,
    };
    require!(value.len() <= max_len, VCoinError::MetadataTooLong);

    // Upper bound: the old value is replaced, so growth is at most the new value's length.
    let mint_info = ctx.accounts.mint.to_account_info();
    let target_len = mint_info
        .data_len()
        .checked_add(value.len())
        .ok_or(VCoinError::Overflow)?;
    fund_rent(
        &mint_info,
        &ctx.accounts.authority.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        target_len,
    )?;

    let spl_field = match field {
        MetadataField::Name => Field::Name,
        MetadataField::Symbol => Field::Symbol,
        MetadataField::Uri => Field::Uri,
    };
    let bump = [config.bump];
    let signer_seeds: &[&[u8]] = &[VCOIN_CONFIG_SEED, &bump];
    token_metadata_update_field(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataUpdateField {
                program_id: ctx.accounts.token_program.to_account_info(),
                metadata: mint_info,
                update_authority: ctx.accounts.config.to_account_info(),
            },
            &[signer_seeds],
        ),
        spl_field,
        value.clone(),
    )?;

    emit!(MetadataUpdated { field, value });
    Ok(())
}
