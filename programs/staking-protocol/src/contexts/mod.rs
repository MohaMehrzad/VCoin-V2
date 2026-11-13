use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use vevcoin_token::program::VevcoinToken;

use crate::constants::*;
use crate::errors::StakingError;
use crate::state::{MaxVoterWeightRecord, Registrar, SlashRequest, Voter, VoterWeightRecord};

/// Creates the registrar and its VCoin vault. Upgrade authority only (audit X-1/STK-07).
#[derive(Accounts)]
pub struct CreateRegistrar<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = Registrar::LEN, seeds = [REGISTRAR_SEED], bump)]
    pub registrar: Box<Account<'info, Registrar>>,

    /// Must be the vcoin-token program's mint PDA (checked in the handler) and pass
    /// `validate_vcoin_mint` (no hook, fee, freeze authority or permanent delegate).
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,

    /// Must be the vevcoin-token program's mint PDA (checked in the handler).
    pub vevcoin_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init,
        payer = authority,
        seeds = [VAULT_SEED],
        bump,
        token::mint = vcoin_mint,
        token::authority = registrar,
        token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ StakingError::InvalidProgramData)]
    pub program: Program<'info, crate::program::StakingProtocol>,
    pub program_data: Account<'info, ProgramData>,

    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateVoter<'info> {
    #[account(seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,

    #[account(init, payer = payer, space = Voter::LEN, seeds = [VOTER_SEED, owner.key().as_ref()], bump)]
    pub voter: Box<Account<'info, Voter>>,

    pub owner: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Accounts shared by deposit and withdraw: VCoin vault leg + veVCoin receipt leg.
#[derive(Accounts)]
pub struct MoveStake<'info> {
    #[account(
        mut,
        seeds = [REGISTRAR_SEED],
        bump = registrar.bump,
        has_one = vault,
        has_one = vcoin_mint,
        has_one = vevcoin_mint,
    )]
    pub registrar: Box<Account<'info, Registrar>>,

    #[account(mut, seeds = [VOTER_SEED, owner.key().as_ref()], bump = voter.bump, has_one = owner)]
    pub voter: Box<Account<'info, Voter>>,

    pub owner: Signer<'info>,

    #[account(mut)]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut, token::mint = vcoin_mint, token::authority = owner, token::token_program = token_program)]
    pub owner_vcoin: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub vevcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    /// The owner's canonical veVCoin ATA: receipts live in exactly one account per owner, so
    /// a slash can always burn them (anyone can recreate the ATA).
    #[account(
        mut,
        associated_token::mint = vevcoin_mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub owner_vevcoin: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: veVCoin config PDA; validated by the veVCoin program (seeds + has_one).
    pub vevcoin_config: UncheckedAccount<'info>,
    pub vevcoin_program: Program<'info, VevcoinToken>,

    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct ExtendLock<'info> {
    #[account(mut, seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,
    #[account(mut, seeds = [VOTER_SEED, owner.key().as_ref()], bump = voter.bump, has_one = owner)]
    pub voter: Box<Account<'info, Voter>>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseVoter<'info> {
    #[account(mut, seeds = [VOTER_SEED, owner.key().as_ref()], bump = voter.bump, has_one = owner, close = owner)]
    pub voter: Box<Account<'info, Voter>>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateRegistrar<'info> {
    #[account(mut, seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,
    pub new_authority: Signer<'info>,
}

/// Withdraw = stake accounts + the owner's Realms TokenOwnerRecord (checked for open votes).
#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub stake: MoveStake<'info>,
    /// CHECK: address, owner and type are checked by `governance::assert_can_withdraw`.
    pub token_owner_record: UncheckedAccount<'info>,
}

/// One-shot wiring to a Realms realm; creates the realm-wide MaxVoterWeightRecord.
#[derive(Accounts)]
pub struct ConfigureGovernance<'info> {
    #[account(mut, seeds = [REGISTRAR_SEED], bump = registrar.bump, has_one = vcoin_mint)]
    pub registrar: Box<Account<'info, Registrar>>,
    pub authority: Signer<'info>,
    /// CHECK: validated by `governance::validate_realm` (owner, RealmV2 tag, community mint).
    pub realm: UncheckedAccount<'info>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    /// DAO treasury token account that receives slashed VCoin.
    #[account(token::mint = vcoin_mint, token::token_program = token_program)]
    pub slash_destination: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(init, payer = payer, space = MaxVoterWeightRecord::LEN, seeds = [MAX_VOTER_WEIGHT_RECORD_SEED], bump)]
    pub max_voter_weight_record: Box<Account<'info, MaxVoterWeightRecord>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

/// Creates `owner`'s VoterWeightRecord (anyone may pay).
#[derive(Accounts)]
pub struct CreateVoterWeightRecord<'info> {
    #[account(seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,
    #[account(
        init,
        payer = payer,
        space = VoterWeightRecord::LEN,
        seeds = [VOTER_WEIGHT_RECORD_SEED, owner.key().as_ref()],
        bump,
    )]
    pub voter_weight_record: Box<Account<'info, VoterWeightRecord>>,
    /// CHECK: the governing token owner the record is for.
    pub owner: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Writes the voter's current weight for one Realms action; call it in the same
/// transaction as that action (the record expires after the current slot).
#[derive(Accounts)]
pub struct UpdateVoterWeightRecord<'info> {
    #[account(seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,
    #[account(seeds = [VOTER_SEED, voter.owner.as_ref()], bump = voter.bump)]
    pub voter: Box<Account<'info, Voter>>,
    #[account(
        mut,
        seeds = [VOTER_WEIGHT_RECORD_SEED, voter.owner.as_ref()],
        bump,
    )]
    pub voter_weight_record: Box<Account<'info, VoterWeightRecord>>,
    /// CHECK: the owner's 5A UserScore PDA (may not exist); checked by `governance::five_a_score`.
    pub user_score: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct UpdateMaxVoterWeightRecord<'info> {
    #[account(seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,
    #[account(mut, seeds = [MAX_VOTER_WEIGHT_RECORD_SEED], bump)]
    pub max_voter_weight_record: Box<Account<'info, MaxVoterWeightRecord>>,
}

/// Opens a slash request against `voter` (registrar authority = the DAO).
#[derive(Accounts)]
pub struct ProposeSlash<'info> {
    #[account(mut, seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,
    pub authority: Signer<'info>,
    #[account(mut, seeds = [VOTER_SEED, voter.owner.as_ref()], bump = voter.bump)]
    pub voter: Box<Account<'info, Voter>>,
    #[account(
        init,
        payer = payer,
        space = SlashRequest::LEN,
        seeds = [SLASH_SEED, voter.key().as_ref(), &registrar.slash_nonce.to_le_bytes()],
        bump,
    )]
    pub slash_request: Box<Account<'info, SlashRequest>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Cancel (authority) or expire (anyone, after the window): closes the request.
#[derive(Accounts)]
pub struct CloseSlash<'info> {
    #[account(seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,
    #[account(mut, seeds = [VOTER_SEED, voter.owner.as_ref()], bump = voter.bump)]
    pub voter: Box<Account<'info, Voter>>,
    #[account(
        mut,
        seeds = [SLASH_SEED, voter.key().as_ref(), &slash_request.nonce.to_le_bytes()],
        bump = slash_request.bump,
        has_one = voter,
        has_one = rent_payer,
        close = rent_payer,
    )]
    pub slash_request: Box<Account<'info, SlashRequest>>,
    /// CHECK: receives the request's rent; bound by `has_one`.
    #[account(mut)]
    pub rent_payer: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CancelSlash<'info> {
    pub close: CloseSlash<'info>,
    pub authority: Signer<'info>,
}

/// Executes a ready slash request (permissionless): vault -> slash destination, receipts
/// burned from the owner's ATA.
#[derive(Accounts)]
pub struct ExecuteSlash<'info> {
    #[account(
        mut,
        seeds = [REGISTRAR_SEED],
        bump = registrar.bump,
        has_one = vault,
        has_one = vcoin_mint,
        has_one = vevcoin_mint,
        has_one = slash_destination,
    )]
    pub registrar: Box<Account<'info, Registrar>>,
    #[account(mut, seeds = [VOTER_SEED, voter.owner.as_ref()], bump = voter.bump)]
    pub voter: Box<Account<'info, Voter>>,
    #[account(
        mut,
        seeds = [SLASH_SEED, voter.key().as_ref(), &slash_request.nonce.to_le_bytes()],
        bump = slash_request.bump,
        has_one = voter,
        has_one = rent_payer,
        close = rent_payer,
    )]
    pub slash_request: Box<Account<'info, SlashRequest>>,
    /// CHECK: receives the request's rent; bound by `has_one`.
    #[account(mut)]
    pub rent_payer: UncheckedAccount<'info>,
    #[account(mut)]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub slash_destination: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub vevcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    /// CHECK: the slashed owner; only used to derive the receipt ATA.
    #[account(address = voter.owner)]
    pub owner: UncheckedAccount<'info>,
    #[account(
        mut,
        associated_token::mint = vevcoin_mint,
        associated_token::authority = owner,
        associated_token::token_program = token_program,
    )]
    pub owner_vevcoin: Box<InterfaceAccount<'info, TokenAccount>>,
    /// CHECK: veVCoin config PDA; validated by the veVCoin program (seeds + has_one).
    pub vevcoin_config: UncheckedAccount<'info>,
    pub vevcoin_program: Program<'info, VevcoinToken>,
    pub token_program: Program<'info, Token2022>,
}

/// Repoints slashed funds to another VCoin token account (authority = the DAO).
#[derive(Accounts)]
pub struct SetSlashDestination<'info> {
    #[account(mut, seeds = [REGISTRAR_SEED], bump = registrar.bump, has_one = vcoin_mint)]
    pub registrar: Box<Account<'info, Registrar>>,
    pub authority: Signer<'info>,
    pub vcoin_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(token::mint = vcoin_mint, token::token_program = token_program)]
    pub slash_destination: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Program<'info, Token2022>,
}

/// Permissionless: stops an expired deposit from counting toward the max voter weight.
#[derive(Accounts)]
pub struct DeactivateExpired<'info> {
    #[account(mut, seeds = [REGISTRAR_SEED], bump = registrar.bump)]
    pub registrar: Box<Account<'info, Registrar>>,
    #[account(mut, seeds = [VOTER_SEED, voter.owner.as_ref()], bump = voter.bump)]
    pub voter: Box<Account<'info, Voter>>,
}
