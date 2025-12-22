use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    /// The extra account meta list account
    /// CHECK: Validated by the transfer hook interface
    #[account(mut)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    
    pub vcoin_mint: InterfaceAccount<'info, Mint>,
    
    pub system_program: Program<'info, System>,
}

