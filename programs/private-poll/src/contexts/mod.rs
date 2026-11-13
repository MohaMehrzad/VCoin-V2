use anchor_lang::prelude::*;
use staking_protocol::constants::{REGISTRAR_SEED, VOTER_SEED};
use staking_protocol::state::{Registrar, Voter};

use crate::constants::*;
use crate::errors::PollError;
use crate::state::{BallotRecord, Poll, PollConfig};

/// Creates the config. Upgrade authority only (audit X-1 / GOV-10).
#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = PollConfig::LEN, seeds = [CONFIG_SEED], bump)]
    pub config: Box<Account<'info, PollConfig>>,
    #[account(seeds = [REGISTRAR_SEED], bump = registrar.bump, seeds::program = staking_protocol::ID)]
    pub registrar: Box<Account<'info, Registrar>>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ PollError::InvalidProgramData)]
    pub program: Program<'info, crate::program::PrivatePoll>,
    pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PollConfig>>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PollConfig>>,
    pub new_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CreatePoll<'info> {
    #[account(mut, seeds = [CONFIG_SEED], bump = config.bump, has_one = registrar)]
    pub config: Box<Account<'info, PollConfig>>,
    #[account(seeds = [REGISTRAR_SEED], bump = registrar.bump, seeds::program = staking_protocol::ID)]
    pub registrar: Box<Account<'info, Registrar>>,
    #[account(
        seeds = [VOTER_SEED, creator.key().as_ref()],
        bump = creator_voter.bump,
        seeds::program = staking_protocol::ID,
    )]
    pub creator_voter: Box<Account<'info, Voter>>,
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        init,
        payer = creator,
        space = Poll::LEN,
        seeds = [POLL_SEED, &config.poll_count.to_le_bytes()],
        bump,
    )]
    pub poll: Box<Account<'info, Poll>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump, has_one = registrar)]
    pub config: Box<Account<'info, PollConfig>>,
    #[account(mut, seeds = [POLL_SEED, &poll.id.to_le_bytes()], bump = poll.bump)]
    pub poll: Box<Account<'info, Poll>>,
    #[account(seeds = [REGISTRAR_SEED], bump = registrar.bump, seeds::program = staking_protocol::ID)]
    pub registrar: Box<Account<'info, Registrar>>,
    /// The voter's staking deposits.
    #[account(
        seeds = [VOTER_SEED, owner.key().as_ref()],
        bump = stake.bump,
        seeds::program = staking_protocol::ID,
    )]
    pub stake: Box<Account<'info, Voter>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    /// CHECK: the owner's 5A UserScore PDA (may not exist); checked by
    /// `staking_protocol::governance::five_a_score`.
    pub user_score: UncheckedAccount<'info>,
    #[account(
        init,
        payer = owner,
        space = BallotRecord::LEN,
        seeds = [BALLOT_SEED, poll.key().as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub ballot_record: Box<Account<'info, BallotRecord>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitShare<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PollConfig>>,
    #[account(mut, seeds = [POLL_SEED, &poll.id.to_le_bytes()], bump = poll.bump)]
    pub poll: Box<Account<'info, Poll>>,
    pub member: Signer<'info>,
}

/// Finalize or expire (permissionless).
#[derive(Accounts)]
pub struct SettlePoll<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PollConfig>>,
    #[account(mut, seeds = [POLL_SEED, &poll.id.to_le_bytes()], bump = poll.bump)]
    pub poll: Box<Account<'info, Poll>>,
}

#[derive(Accounts)]
pub struct AssertPollPassed<'info> {
    #[account(seeds = [CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, PollConfig>>,
    #[account(mut, seeds = [POLL_SEED, &poll.id.to_le_bytes()], bump = poll.bump)]
    pub poll: Box<Account<'info, Poll>>,
    pub consumer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClosePoll<'info> {
    #[account(mut, seeds = [POLL_SEED, &poll.id.to_le_bytes()], bump = poll.bump, has_one = creator, close = creator)]
    pub poll: Box<Account<'info, Poll>>,
    #[account(mut)]
    pub creator: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseBallot<'info> {
    #[account(
        mut,
        seeds = [BALLOT_SEED, ballot_record.poll.as_ref(), voter.key().as_ref()],
        bump = ballot_record.bump,
        has_one = voter,
        close = voter,
    )]
    pub ballot_record: Box<Account<'info, BallotRecord>>,
    /// CHECK: the record's poll; must be final, or already closed.
    #[account(address = ballot_record.poll)]
    pub poll: UncheckedAccount<'info>,
    #[account(mut)]
    pub voter: Signer<'info>,
}
