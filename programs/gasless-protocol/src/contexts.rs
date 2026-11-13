use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};

use crate::constants::*;
use crate::errors::GaslessError;
use crate::state::{GaslessConfig, SessionToken};

/// Upgrade-authority–gated setup (audit GAS-07 / X-1).
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = GaslessConfig::LEN, seeds = [GASLESS_CONFIG_SEED], bump)]
    pub config: Box<Account<'info, GaslessConfig>>,

    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = authority,
        seeds = [FEE_VAULT_SEED],
        bump,
        token::mint = vcoin_mint,
        token::authority = config,
        token::token_program = token_program,
    )]
    pub fee_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(token::mint = vcoin_mint, token::token_program = token_program)]
    pub treasury: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ GaslessError::InvalidProgramData)]
    pub program: Program<'info, crate::program::GaslessProtocol>,
    pub program_data: Account<'info, ProgramData>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/// The authority, the session key and the rent payer all sign: the session key proves
/// possession, and the payer may be a relayer (users need no SOL).
#[derive(Accounts)]
#[instruction(target_program: Pubkey)]
pub struct CreateSession<'info> {
    #[account(seeds = [GASLESS_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, GaslessConfig>>,

    #[account(
        init,
        payer = payer,
        space = SessionToken::LEN,
        seeds = [SESSION_SEED, target_program.as_ref(), session_signer.key().as_ref(), authority.key().as_ref()],
        bump,
    )]
    pub session: Box<Account<'info, SessionToken>>,

    pub authority: Signer<'info>,
    pub session_signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Closes a session and refunds its rent to the original payer (audit GAS-05).
#[derive(Accounts)]
pub struct RevokeSession<'info> {
    #[account(
        mut,
        seeds = [SESSION_SEED, session.target_program.as_ref(), session.session_signer.as_ref(), session.authority.as_ref()],
        bump = session.bump,
        has_one = payer,
        close = payer,
    )]
    pub session: Box<Account<'info, SessionToken>>,
    /// CHECK: rent recipient recorded at creation (`has_one = payer`).
    #[account(mut)]
    pub payer: UncheckedAccount<'info>,
    pub revoker: Signer<'info>,
}

#[derive(Accounts)]
pub struct PayVCoinFee<'info> {
    /// Read-only: paying a fee never write-locks global state (audit GAS-09).
    #[account(seeds = [GASLESS_CONFIG_SEED], bump = config.bump, has_one = vcoin_mint, has_one = fee_vault)]
    pub config: Box<Account<'info, GaslessConfig>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub fee_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut, token::mint = vcoin_mint, token::authority = payer, token::token_program = token_program)]
    pub payer_vcoin: Box<InterfaceAccount<'info, TokenAccount>>,
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
}

/// Permissionless: fees can only ever go to the configured treasury (audit GAS-03).
#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(seeds = [GASLESS_CONFIG_SEED], bump = config.bump, has_one = vcoin_mint, has_one = fee_vault, has_one = treasury)]
    pub config: Box<Account<'info, GaslessConfig>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub fee_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub treasury: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [GASLESS_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, GaslessConfig>>,
    pub authority: Signer<'info>,
}

/// Permissionless application of a scheduled fee change once its delay has passed.
#[derive(Accounts)]
pub struct ApplyFee<'info> {
    #[account(mut, seeds = [GASLESS_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, GaslessConfig>>,
}

/// Permissionless application of a scheduled treasury change once its delay has passed.
#[derive(Accounts)]
pub struct ApplyTreasury<'info> {
    #[account(mut, seeds = [GASLESS_CONFIG_SEED], bump = config.bump, has_one = vcoin_mint)]
    pub config: Box<Account<'info, GaslessConfig>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        address = config.pending_treasury @ GaslessError::NothingScheduled,
        token::mint = vcoin_mint,
        token::token_program = token_program,
    )]
    pub new_treasury: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [GASLESS_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, GaslessConfig>>,
    pub new_authority: Signer<'info>,
}
