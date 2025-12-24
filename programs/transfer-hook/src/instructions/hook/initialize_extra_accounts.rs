use anchor_lang::prelude::*;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta,
    seeds::Seed,
    state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::constants::{HOOK_CONFIG_SEED, USER_ACTIVITY_SEED, PAIR_TRACKING_SEED};
use crate::contexts::InitializeExtraAccountMetaList;

/// H-01 Security Fix: Initialize extra account metas for the transfer hook
/// Required by Token-2022 transfer hook interface
/// 
/// This populates the ExtraAccountMetaList with accounts needed during Execute:
/// 1. hook_config - PDA for hook configuration
/// 2. sender_activity - PDA for sender's activity tracking  
/// 3. receiver_activity - PDA for receiver's activity tracking
/// 4. pair_tracking - PDA for sender->receiver pair tracking
/// 5. payer - for rent on init_if_needed accounts
/// 6. system_program - for account creation
pub fn handler(ctx: Context<InitializeExtraAccountMetaList>) -> Result<()> {
    // Define the extra account metas needed for the Execute instruction
    let extra_metas = vec![
        // 1. hook_config - PDA derived from HOOK_CONFIG_SEED
        ExtraAccountMeta::new_with_seeds(
            &[Seed::Literal { bytes: HOOK_CONFIG_SEED.to_vec() }],
            false, // is_signer
            false, // is_writable
        )?,
        
        // 2. sender_activity - PDA derived from USER_ACTIVITY_SEED + sender owner
        // The sender owner comes from the source token account (index 0)
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: USER_ACTIVITY_SEED.to_vec() },
                Seed::AccountData { account_index: 0, data_index: 32, length: 32 }, // owner field at offset 32
            ],
            false, // is_signer
            true,  // is_writable (init_if_needed)
        )?,
        
        // 3. receiver_activity - PDA derived from USER_ACTIVITY_SEED + receiver owner
        // The receiver owner comes from the destination token account (index 1)
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: USER_ACTIVITY_SEED.to_vec() },
                Seed::AccountData { account_index: 1, data_index: 32, length: 32 }, // owner field at offset 32
            ],
            false, // is_signer
            true,  // is_writable (init_if_needed)
        )?,
        
        // 4. pair_tracking - PDA derived from PAIR_TRACKING_SEED + sender owner + receiver owner
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: PAIR_TRACKING_SEED.to_vec() },
                Seed::AccountData { account_index: 0, data_index: 32, length: 32 }, // sender owner
                Seed::AccountData { account_index: 1, data_index: 32, length: 32 }, // receiver owner
            ],
            false, // is_signer
            true,  // is_writable (init_if_needed)
        )?,
        
        // 5. payer - external account that pays for rent on init_if_needed
        // This must be provided by the caller as a signer
        ExtraAccountMeta::new_external_pda_with_seeds(
            4, // Account index in remaining accounts
            &[], // No seeds - provided externally
            false, // is_signer - will be set by caller
            true,  // is_writable
        )?,
        
        // 6. system_program - for account creation
        ExtraAccountMeta::new_with_pubkey(&anchor_lang::system_program::ID, false, false)?,
    ];

    // Calculate space needed for extra account meta list
    let account_size = ExtraAccountMetaList::size_of(extra_metas.len())?;
    
    // Allocate space for the extra account meta list if needed
    let extra_account_meta_list = &ctx.accounts.extra_account_meta_list;
    if extra_account_meta_list.data_is_empty() {
        // Create the account with sufficient space
        let lamports = Rent::get()?.minimum_balance(account_size);
        let cpi_accounts = anchor_lang::system_program::CreateAccount {
            from: ctx.accounts.payer.to_account_info(),
            to: extra_account_meta_list.to_account_info(),
        };
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
        );
        anchor_lang::system_program::create_account(
            cpi_context,
            lamports,
            account_size as u64,
            &crate::ID,
        )?;
    }
    
    // Initialize the extra account meta list
    let mut data = extra_account_meta_list.try_borrow_mut_data()?;
    ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, &extra_metas)?;
    
    msg!("H-01 Fix: Extra account meta list initialized with {} accounts", extra_metas.len());
    msg!("Accounts: hook_config, sender_activity, receiver_activity, pair_tracking, payer, system_program");
    
    Ok(())
}
