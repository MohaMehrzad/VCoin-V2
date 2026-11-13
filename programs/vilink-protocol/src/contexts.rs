use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use gasless_protocol::{constants::SESSION_SEED, state::SessionToken};

use crate::constants::*;
use crate::errors::ViLinkError;
use crate::state::*;

/// Upgrade authority only; VCoin (Token-2022) mint pinned (audit VL-1).
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = ViLinkConfig::LEN, seeds = [CONFIG_SEED], bump)]
    pub config: Box<Account<'info, ViLinkConfig>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(token::mint = vcoin_mint, token::token_program = token_program)]
    pub treasury: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ ViLinkError::InvalidProgramData)]
    pub program: Program<'info, crate::program::VilinkProtocol>,
    pub program_data: Account<'info, ProgramData>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, ViLinkConfig>>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, ViLinkConfig>>,
    pub new_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ProposeConfig<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = vcoin_mint)]
    pub config: Box<Account<'info, ViLinkConfig>>,
    pub authority: Signer<'info>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(token::mint = vcoin_mint, token::token_program = token_program)]
    pub treasury: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
}

/// Permissionless once the timelock passed.
#[derive(Accounts)]
pub struct ApplyConfig<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, ViLinkConfig>>,
}

#[derive(Accounts)]
#[instruction(action_type: u8, amount: u64, target: Pubkey, metadata_hash: [u8; 32], expiry_secs: i64, max_executions: u32, nonce: u64)]
pub struct CreateAction<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, ViLinkConfig>>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init_if_needed,
        payer = creator,
        space = UserActionStats::LEN,
        seeds = [USER_STATS_SEED, creator.key().as_ref()],
        bump,
    )]
    pub creator_stats: Box<Account<'info, UserActionStats>>,
    #[account(
        init,
        payer = creator,
        space = ViLinkAction::LEN,
        seeds = [ACTION_SEED, creator.key().as_ref(), &nonce.to_le_bytes()],
        bump,
    )]
    pub action: Box<Account<'info, ViLinkAction>>,
    pub system_program: Program<'info, System>,
}

/// Tips move tokens, so the executor's wallet always signs.
#[derive(Accounts)]
pub struct ExecuteTip<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = vcoin_mint, has_one = treasury)]
    pub config: Box<Account<'info, ViLinkConfig>>,
    #[account(mut, seeds = [ACTION_SEED, action.creator.as_ref(), &action.nonce.to_le_bytes()], bump = action.bump)]
    pub action: Box<Account<'info, ViLinkAction>>,
    #[account(mut)]
    pub executor: Signer<'info>,
    #[account(mut, token::mint = vcoin_mint, token::authority = executor, token::token_program = token_program)]
    pub executor_token: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = vcoin_mint,
        token::token_program = token_program,
        constraint = target_token.owner == action.target @ ViLinkError::InvalidTarget,
    )]
    pub target_token: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub treasury: Box<InterfaceAccount<'info, TokenAccount>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = executor,
        space = ExecutionReceipt::LEN,
        seeds = [RECEIPT_SEED, action.key().as_ref(), executor.key().as_ref()],
        bump,
    )]
    pub receipt: Box<Account<'info, ExecutionReceipt>>,
    #[account(
        init_if_needed,
        payer = executor,
        space = UserActionStats::LEN,
        seeds = [USER_STATS_SEED, executor.key().as_ref()],
        bump,
    )]
    pub executor_stats: Box<Account<'info, UserActionStats>>,
    #[account(
        init_if_needed,
        payer = executor,
        space = UserActionStats::LEN,
        seeds = [USER_STATS_SEED, action.target.as_ref()],
        bump,
    )]
    pub target_stats: Box<Account<'info, UserActionStats>>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/// Vouch / follow: the executor signs, or a gasless session key for the executor with the
/// matching scope; `payer` (e.g. a relayer) funds the receipt.
#[derive(Accounts)]
pub struct ExecuteSocial<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, ViLinkConfig>>,
    #[account(mut, seeds = [ACTION_SEED, action.creator.as_ref(), &action.nonce.to_le_bytes()], bump = action.bump)]
    pub action: Box<Account<'info, ViLinkAction>>,
    /// CHECK: the executor wallet; authorized through `signer` (and `session`).
    pub executor: UncheckedAccount<'info>,
    pub signer: Signer<'info>,
    #[account(
        seeds = [SESSION_SEED, crate::ID.as_ref(), signer.key().as_ref(), executor.key().as_ref()],
        bump = session.bump,
        seeds::program = gasless_protocol::ID,
    )]
    pub session: Option<Box<Account<'info, SessionToken>>>,
    #[account(
        init,
        payer = payer,
        space = ExecutionReceipt::LEN,
        seeds = [RECEIPT_SEED, action.key().as_ref(), executor.key().as_ref()],
        bump,
    )]
    pub receipt: Box<Account<'info, ExecutionReceipt>>,
    #[account(
        init_if_needed,
        payer = payer,
        space = UserActionStats::LEN,
        seeds = [USER_STATS_SEED, executor.key().as_ref()],
        bump,
    )]
    pub executor_stats: Box<Account<'info, UserActionStats>>,
    #[account(
        init_if_needed,
        payer = payer,
        space = UserActionStats::LEN,
        seeds = [USER_STATS_SEED, action.target.as_ref()],
        bump,
    )]
    pub target_stats: Box<Account<'info, UserActionStats>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// The creator closes an expired or exhausted action (audit VL-6).
#[derive(Accounts)]
pub struct CloseAction<'info> {
    #[account(
        mut,
        seeds = [ACTION_SEED, creator.key().as_ref(), &action.nonce.to_le_bytes()],
        bump = action.bump,
        has_one = creator,
        close = creator,
    )]
    pub action: Box<Account<'info, ViLinkAction>>,
    #[account(mut)]
    pub creator: Signer<'info>,
}

/// Anyone closes a receipt once its action is closed or no longer executable.
#[derive(Accounts)]
pub struct CloseReceipt<'info> {
    #[account(
        mut,
        seeds = [RECEIPT_SEED, receipt.action.as_ref(), receipt.executor.as_ref()],
        bump = receipt.bump,
        has_one = rent_payer,
        close = rent_payer,
    )]
    pub receipt: Box<Account<'info, ExecutionReceipt>>,
    /// CHECK: the receipt's action (may already be closed).
    #[account(address = receipt.action)]
    pub action: UncheckedAccount<'info>,
    /// CHECK: bound by `has_one`.
    #[account(mut)]
    pub rent_payer: UncheckedAccount<'info>,
}
