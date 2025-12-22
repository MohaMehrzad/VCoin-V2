use anchor_lang::prelude::*;

use crate::contexts::GetBalance;

/// Get user's veVCoin balance (view function)
pub fn handler(ctx: Context<GetBalance>) -> Result<u64> {
    Ok(ctx.accounts.user_account.balance)
}

