use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer_checked, TransferChecked};

use crate::constants::GASLESS_CONFIG_SEED;
use crate::contexts::{PayVCoinFee, WithdrawFees};
use crate::errors::GaslessError;
use crate::events::{FeePaid, FeesWithdrawn};

/// Pays the current per-transaction VCoin fee. `max_fee` is signed by the user, so no
/// admin change can ever take more than they agreed to (audit GAS-16 / SEC-2).
pub fn pay_vcoin_fee(ctx: Context<PayVCoinFee>, max_fee: u64) -> Result<()> {
    let config = &ctx.accounts.config;
    require!(!config.paused, GaslessError::Paused);
    let fee = config.vcoin_fee_per_tx;
    require!(fee <= max_fee, GaslessError::FeeAboveMax);
    if fee > 0 {
        transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                TransferChecked {
                    from: ctx.accounts.payer_vcoin.to_account_info(),
                    mint: ctx.accounts.vcoin_mint.to_account_info(),
                    to: ctx.accounts.fee_vault.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            fee,
            ctx.accounts.vcoin_mint.decimals,
        )?;
    }
    emit!(FeePaid {
        payer: ctx.accounts.payer.key(),
        amount: fee
    });
    Ok(())
}

/// Sends collected fees to the configured treasury (audit GAS-03: fees were locked forever).
pub fn withdraw_fees(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
    require!(amount > 0, GaslessError::ZeroAmount);
    let bump = [ctx.accounts.config.bump];
    let seeds: &[&[u8]] = &[GASLESS_CONFIG_SEED, &bump];
    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.fee_vault.to_account_info(),
                mint: ctx.accounts.vcoin_mint.to_account_info(),
                to: ctx.accounts.treasury.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[seeds],
        ),
        amount,
        ctx.accounts.vcoin_mint.decimals,
    )?;
    emit!(FeesWithdrawn {
        treasury: ctx.accounts.treasury.key(),
        amount
    });
    Ok(())
}
