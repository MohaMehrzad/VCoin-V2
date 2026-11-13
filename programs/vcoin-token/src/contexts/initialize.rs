use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022};

use crate::constants::*;
use crate::errors::VCoinError;
use crate::state::VCoinConfig;

/// Creates the config and the VCoin mint. Only the program's upgrade authority may call
/// it (audit TOK-04 / X-1: initializers were front-runnable).
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = VCoinConfig::LEN,
        seeds = [VCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Box<Account<'info, VCoinConfig>>,

    /// The VCoin mint: Token-2022, 9 decimals, mint authority = config PDA,
    /// no freeze authority, and only the MetadataPointer extension (pointing at itself;
    /// the TokenMetadata extension is written by the handler).
    #[account(
        init,
        payer = authority,
        seeds = [VCOIN_MINT_SEED],
        bump,
        mint::decimals = VCOIN_DECIMALS,
        mint::authority = config,
        mint::token_program = token_program,
        extensions::metadata_pointer::authority = config,
        extensions::metadata_pointer::metadata_address = mint,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ VCoinError::InvalidProgramData)]
    pub program: Program<'info, crate::program::VcoinToken>,
    pub program_data: Account<'info, ProgramData>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
