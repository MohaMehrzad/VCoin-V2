use anchor_lang::prelude::*;
use crate::constants::USER_STAKE_SEED;
use crate::contexts::DelegateVotes;
use crate::errors::GovernanceError;
use crate::events::DelegationCreated;

pub fn handler(
    ctx: Context<DelegateVotes>,
    delegation_type: u8,
    categories: u8,
    vevcoin_amount: u64,
    expires_at: i64,
    revocable: bool,
) -> Result<()> {
    let delegator_key = ctx.accounts.delegator.key();
    let delegate_key = ctx.accounts.delegate.key();

    require!(delegator_key != delegate_key, GovernanceError::CannotDelegateSelf);

    // H-NEW-03: Validate delegation amount against actual veVCoin balance
    let delegator_balance = get_delegator_vevcoin_balance(&ctx)?;
    require!(
        vevcoin_amount <= delegator_balance,
        GovernanceError::ExceedsDelegatedAmount
    );

    let clock = Clock::get()?;

    let delegation = &mut ctx.accounts.delegation;
    delegation.delegator = delegator_key;
    delegation.delegate = delegate_key;
    delegation.delegation_type = delegation_type;
    delegation.categories = categories;
    delegation.delegated_amount = vevcoin_amount;
    delegation.delegated_at = clock.unix_timestamp;
    delegation.expires_at = expires_at;
    delegation.revocable = revocable;
    delegation.bump = ctx.bumps.delegation;
    
    // Update delegate stats
    let delegate_stats = &mut ctx.accounts.delegate_stats;
    delegate_stats.delegate = delegate_key;
    delegate_stats.unique_delegators = delegate_stats.unique_delegators.saturating_add(1);
    delegate_stats.total_delegated_vevcoin = delegate_stats
        .total_delegated_vevcoin
        .saturating_add(vevcoin_amount);
    delegate_stats.bump = ctx.bumps.delegate_stats;
    
    emit!(DelegationCreated {
        delegator: delegator_key,
        delegate: delegate_key,
        amount: vevcoin_amount,
        delegation_type,
    });

    msg!("Delegation created: {} veVCoin", vevcoin_amount);
    Ok(())
}

/// H-NEW-03: Query delegator's actual veVCoin balance from staking protocol
#[inline(never)]
fn get_delegator_vevcoin_balance(ctx: &Context<DelegateVotes>) -> Result<u64> {
    let delegator_key = ctx.accounts.delegator.key();
    let config = &ctx.accounts.config;
    let user_stake = &ctx.accounts.user_stake;

    // Validate the user_stake PDA matches expected address
    let (expected_user_stake_pda, _) = Pubkey::find_program_address(
        &[USER_STAKE_SEED, delegator_key.as_ref()],
        &config.staking_program,
    );
    require!(
        user_stake.key() == expected_user_stake_pda,
        GovernanceError::InvalidUserStakePDA
    );

    // If account doesn't exist or is empty, balance is zero
    if user_stake.data_is_empty() {
        return Ok(0);
    }

    // Read veVCoin balance from staking account
    // Layout: UserStake struct has vevcoin_amount at bytes 73-81
    let stake_data = user_stake.try_borrow_data()?;
    require!(stake_data.len() >= 81, GovernanceError::InvalidUserStakeData);

    let vevcoin_balance = u64::from_le_bytes(
        stake_data[73..81]
            .try_into()
            .map_err(|_| GovernanceError::InvalidUserStakeData)?,
    );

    Ok(vevcoin_balance)
}

