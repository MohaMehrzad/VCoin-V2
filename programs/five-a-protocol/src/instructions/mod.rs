pub mod admin;
pub mod score;
pub mod vouch;

use anchor_lang::prelude::*;

use crate::constants::USER_SCORE_SEED;
use crate::errors::FiveAError;
use crate::state::UserScore;

/// Reads `user`'s score from its PDA; `None` if it was never created.
pub fn read_score(user: &Pubkey, account: &AccountInfo) -> Result<Option<UserScore>> {
    let expected = Pubkey::find_program_address(&[USER_SCORE_SEED, user.as_ref()], &crate::ID).0;
    require_keys_eq!(account.key(), expected, FiveAError::InvalidScoreAccount);
    if *account.owner != crate::ID || account.data_is_empty() {
        return Ok(None);
    }
    let data = account.try_borrow_data()?;
    Ok(Some(UserScore::try_deserialize(&mut &data[..])?))
}
