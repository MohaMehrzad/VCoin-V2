use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    spl_pod::optional_keys::OptionalNonZeroPubkey,
    spl_token_metadata_interface::state::TokenMetadata, token_metadata_initialize,
    TokenMetadataInitialize,
};
use vcoin_common::{require_upgrade_authority, AdminState};

use super::fund_rent;
use crate::constants::*;
use crate::contexts::Initialize;
use crate::errors::VCoinError;
use crate::events::VCoinInitialized;

pub fn handler(ctx: Context<Initialize>, name: String, symbol: String, uri: String) -> Result<()> {
    require_upgrade_authority(&ctx.accounts.program_data, &ctx.accounts.authority.key())?;
    require!(
        name.len() <= MAX_NAME_LEN && symbol.len() <= MAX_SYMBOL_LEN && uri.len() <= MAX_URI_LEN,
        VCoinError::MetadataTooLong
    );

    let config_key = ctx.accounts.config.key();
    let mint_key = ctx.accounts.mint.key();
    {
        let config = &mut ctx.accounts.config;
        config.admin = AdminState::new(ctx.accounts.authority.key());
        config.mint = mint_key;
        config.paused = false;
        config.bump = ctx.bumps.config;
        config.mint_bump = ctx.bumps.mint;
    }

    // Fund the TokenMetadata TLV entry before Token-2022 appends it to the mint.
    let metadata = TokenMetadata {
        update_authority: OptionalNonZeroPubkey::try_from(Some(config_key))?,
        mint: mint_key,
        name: name.clone(),
        symbol: symbol.clone(),
        uri: uri.clone(),
        additional_metadata: vec![],
    };
    let mint_info = ctx.accounts.mint.to_account_info();
    let target_len = mint_info
        .data_len()
        .checked_add(metadata.tlv_size_of()?)
        .ok_or(VCoinError::Overflow)?;
    fund_rent(
        &mint_info,
        &ctx.accounts.authority.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
        target_len,
    )?;

    let config_info = ctx.accounts.config.to_account_info();
    let bump = [ctx.bumps.config];
    let signer_seeds: &[&[u8]] = &[VCOIN_CONFIG_SEED, &bump];
    token_metadata_initialize(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TokenMetadataInitialize {
                program_id: ctx.accounts.token_program.to_account_info(),
                metadata: mint_info.clone(),
                update_authority: config_info.clone(),
                mint_authority: config_info,
                mint: mint_info,
            },
            &[signer_seeds],
        ),
        name.clone(),
        symbol.clone(),
        uri.clone(),
    )?;

    emit!(VCoinInitialized {
        config: config_key,
        mint: mint_key,
        authority: ctx.accounts.authority.key(),
        name,
        symbol,
        uri,
    });
    Ok(())
}
