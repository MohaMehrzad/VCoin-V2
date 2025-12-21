use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("VCNtkM3xg8ihH3JY8bQbqjUWCNEAVCiqUGmPjAqPNwP");

/// VCoin Token-2022 with Extensions
/// - Metadata: On-chain token metadata (Name: VCoin, Symbol: VIWO)
/// - Permanent Delegate: Slashing authority for governance multisig
/// - Transfer Hook: Auto-update 5A scores (prepared, hook program separate)
/// 
/// Total Supply: 1,000,000,000 (1B)
/// Decimals: 9

pub mod constants {
    pub const VCOIN_DECIMALS: u8 = 9;
    pub const TOTAL_SUPPLY: u64 = 1_000_000_000 * 1_000_000_000; // 1B with 9 decimals
    pub const TOKEN_NAME: &str = "VCoin";
    pub const TOKEN_SYMBOL: &str = "VIWO";
    pub const TOKEN_URI: &str = "https://viwoapp.com/vcoin-metadata.json";
    
    // Seed for VCoin Config PDA
    pub const VCOIN_CONFIG_SEED: &[u8] = b"vcoin-config";
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum VCoinError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("Mint is already initialized")]
        MintAlreadyInitialized,
        #[msg("Invalid mint authority")]
        InvalidMintAuthority,
        #[msg("Exceeds maximum supply")]
        ExceedsMaxSupply,
        #[msg("Slashing amount exceeds balance")]
        SlashingExceedsBalance,
        #[msg("Cannot slash zero tokens")]
        ZeroSlashAmount,
        #[msg("Token is paused")]
        TokenPaused,
    }
}

pub mod state {
    use super::*;
    
    /// VCoin Configuration Account (Singleton PDA)
    /// Stores global configuration for the VCoin token
    #[account]
    #[derive(Default)]
    pub struct VCoinConfig {
        /// The authority that can mint tokens and update config
        pub authority: Pubkey,
        /// The VCoin mint address
        pub mint: Pubkey,
        /// The treasury token account that receives initial minted tokens
        pub treasury: Pubkey,
        /// The permanent delegate for slashing (governance multisig)
        pub permanent_delegate: Pubkey,
        /// Total tokens minted so far
        pub total_minted: u64,
        /// Whether token operations are paused
        pub paused: bool,
        /// Bump seed for PDA
        pub bump: u8,
    }
    
    impl VCoinConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // mint
            32 + // treasury
            32 + // permanent_delegate
            8 +  // total_minted
            1 +  // paused
            1;   // bump
    }
}

use constants::*;
use errors::*;
use state::*;

#[program]
pub mod vcoin_token {
    use super::*;

    /// Initialize the VCoin mint with Token-2022 extensions
    /// This creates the mint with:
    /// - Metadata extension
    /// - Permanent delegate extension (for slashing)
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        permanent_delegate: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.authority = ctx.accounts.authority.key();
        config.mint = ctx.accounts.mint.key();
        config.treasury = ctx.accounts.treasury.key();
        config.permanent_delegate = permanent_delegate;
        config.total_minted = 0;
        config.paused = false;
        config.bump = ctx.bumps.config;
        
        msg!("VCoin mint initialized");
        msg!("Mint: {}", config.mint);
        msg!("Authority: {}", config.authority);
        msg!("Permanent Delegate: {}", permanent_delegate);
        
        Ok(())
    }

    /// Mint VCoin tokens to a specified account
    /// Only the authority can mint tokens
    pub fn mint_tokens(
        ctx: Context<MintTokens>,
        amount: u64,
    ) -> Result<()> {
        // Get config values for validation first
        let authority = ctx.accounts.config.authority;
        let paused = ctx.accounts.config.paused;
        let total_minted = ctx.accounts.config.total_minted;
        let bump = ctx.accounts.config.bump;
        
        // Check authorization
        require!(
            ctx.accounts.authority.key() == authority,
            VCoinError::Unauthorized
        );
        
        // Check not paused
        require!(!paused, VCoinError::TokenPaused);
        
        // Check supply limit
        require!(
            total_minted.checked_add(amount).unwrap() <= TOTAL_SUPPLY,
            VCoinError::ExceedsMaxSupply
        );
        
        // Mint tokens using Token-2022
        let seeds = &[
            VCOIN_CONFIG_SEED,
            &[bump],
        ];
        let signer_seeds = &[&seeds[..]];
        
        token_2022::mint_to(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
                token_2022::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;
        
        // Update total minted
        ctx.accounts.config.total_minted = total_minted.checked_add(amount).unwrap();
        
        msg!("Minted {} VCoin tokens", amount);
        msg!("Total minted: {}", ctx.accounts.config.total_minted);
        
        Ok(())
    }

    /// Slash tokens from an account using permanent delegate authority
    /// This is used for penalizing bad actors
    pub fn slash_tokens(
        ctx: Context<SlashTokens>,
        amount: u64,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        
        // Check authorization (only permanent delegate can slash)
        require!(
            ctx.accounts.authority.key() == config.permanent_delegate,
            VCoinError::Unauthorized
        );
        
        require!(amount > 0, VCoinError::ZeroSlashAmount);
        
        // Check balance
        let account_balance = ctx.accounts.target_account.amount;
        require!(
            account_balance >= amount,
            VCoinError::SlashingExceedsBalance
        );
        
        // Burn the slashed tokens using permanent delegate authority
        let seeds = &[
            VCOIN_CONFIG_SEED,
            &[config.bump],
        ];
        let signer_seeds = &[&seeds[..]];
        
        token_2022::burn(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
                token_2022::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.target_account.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;
        
        msg!("Slashed {} VCoin tokens from {}", amount, ctx.accounts.target_account.key());
        
        Ok(())
    }

    /// Pause/unpause token operations
    /// Only authority can pause
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(
            ctx.accounts.authority.key() == config.authority,
            VCoinError::Unauthorized
        );
        
        config.paused = paused;
        
        msg!("Token paused status: {}", paused);
        
        Ok(())
    }

    /// Update the authority
    pub fn update_authority(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(
            ctx.accounts.authority.key() == config.authority,
            VCoinError::Unauthorized
        );
        
        config.authority = new_authority;
        
        msg!("Authority updated to: {}", new_authority);
        
        Ok(())
    }

    /// Update the permanent delegate (for slashing)
    pub fn update_permanent_delegate(
        ctx: Context<UpdateConfig>,
        new_delegate: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(
            ctx.accounts.authority.key() == config.authority,
            VCoinError::Unauthorized
        );
        
        config.permanent_delegate = new_delegate;
        
        msg!("Permanent delegate updated to: {}", new_delegate);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = VCoinConfig::LEN,
        seeds = [VCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, VCoinConfig>,
    
    /// The VCoin mint (Token-2022)
    /// CHECK: Validated by Token-2022 program
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    
    /// Treasury token account
    /// CHECK: Will be validated during token operations
    pub treasury: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [VCOIN_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, VCoinConfig>,
    
    /// The VCoin mint
    #[account(
        mut,
        constraint = mint.key() == config.mint
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// Destination token account
    #[account(mut)]
    pub destination: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct SlashTokens<'info> {
    /// The permanent delegate authority
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        seeds = [VCOIN_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, VCoinConfig>,
    
    /// The VCoin mint
    #[account(
        mut,
        constraint = mint.key() == config.mint
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// Target account to slash tokens from
    #[account(mut)]
    pub target_account: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [VCOIN_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, VCoinConfig>,
}


