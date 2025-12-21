use anchor_lang::prelude::*;
use anchor_spl::token_2022::{self, Token2022};
use anchor_spl::token_interface::{Mint, TokenAccount};

declare_id!("VEVCnmRk9hYxBGhH3JY8bQbqjUWCNEAVCiqUGmPjBqQ");

/// veVCoin Token - Vote-Escrowed VCoin (Soulbound)
/// 
/// Token-2022 with Non-Transferable extension making it truly soulbound.
/// Users receive veVCoin when they stake VCoin, representing their voting power.
/// 
/// Key Properties:
/// - Non-Transferable: Cannot be traded on secondary markets
/// - Mint Authority: Only Staking Protocol can mint
/// - Burn Authority: Only Staking Protocol can burn (on unstake)
/// - Prevents governance power markets

pub mod constants {
    pub const VEVCOIN_DECIMALS: u8 = 9;
    pub const TOKEN_NAME: &str = "veVCoin";
    pub const TOKEN_SYMBOL: &str = "veVIWO";
    pub const TOKEN_URI: &str = "https://viwoapp.com/vevcoin-metadata.json";
    
    // Seed for veVCoin Config PDA
    pub const VEVCOIN_CONFIG_SEED: &[u8] = b"vevcoin-config";
    
    // 4 years in seconds (for veVCoin calculation)
    pub const FOUR_YEARS_SECONDS: i64 = 4 * 365 * 24 * 60 * 60;
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum VeVCoinError {
        #[msg("Unauthorized: Only the staking protocol can perform this action")]
        Unauthorized,
        #[msg("Mint is already initialized")]
        MintAlreadyInitialized,
        #[msg("Cannot transfer soulbound tokens")]
        TransferNotAllowed,
        #[msg("Invalid staking protocol")]
        InvalidStakingProtocol,
        #[msg("Cannot burn more tokens than balance")]
        InsufficientBalance,
        #[msg("Cannot mint/burn zero tokens")]
        ZeroAmount,
        #[msg("Invalid token account owner")]
        InvalidTokenAccount,
        #[msg("Invalid token mint")]
        InvalidMint,
    }
}

pub mod state {
    use super::*;
    
    /// veVCoin Configuration Account (Singleton PDA)
    #[account]
    #[derive(Default)]
    pub struct VeVCoinConfig {
        /// The admin authority (can update staking protocol address)
        pub authority: Pubkey,
        /// The veVCoin mint address
        pub mint: Pubkey,
        /// The authorized staking protocol that can mint/burn
        pub staking_protocol: Pubkey,
        /// Total veVCoin currently in circulation
        pub total_supply: u64,
        /// Total unique holders
        pub total_holders: u64,
        /// Bump seed for PDA
        pub bump: u8,
    }
    
    impl VeVCoinConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // mint
            32 + // staking_protocol
            8 +  // total_supply
            8 +  // total_holders
            1;   // bump
    }
    
    /// User veVCoin Account (PDA per user)
    /// Tracks individual veVCoin balance and metadata
    #[account]
    #[derive(Default)]
    pub struct UserVeVCoin {
        /// The user's wallet address
        pub owner: Pubkey,
        /// Current veVCoin balance
        pub balance: u64,
        /// When veVCoin was first minted to this user
        pub first_mint_at: i64,
        /// When veVCoin was last updated
        pub last_update_at: i64,
        /// Bump seed for PDA
        pub bump: u8,
    }
    
    impl UserVeVCoin {
        pub const LEN: usize = 8 + // discriminator
            32 + // owner
            8 +  // balance
            8 +  // first_mint_at
            8 +  // last_update_at
            1;   // bump
            
        pub const SEED: &'static [u8] = b"user-vevcoin";
    }
}

use constants::*;
use errors::*;
use state::*;

#[program]
pub mod vevcoin_token {
    use super::*;

    /// Initialize the veVCoin mint with Token-2022 Non-Transferable extension
    pub fn initialize_mint(
        ctx: Context<InitializeMint>,
        staking_protocol: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        config.authority = ctx.accounts.authority.key();
        config.mint = ctx.accounts.mint.key();
        config.staking_protocol = staking_protocol;
        config.total_supply = 0;
        config.total_holders = 0;
        config.bump = ctx.bumps.config;
        
        msg!("veVCoin mint initialized");
        msg!("Mint: {}", config.mint);
        msg!("Staking Protocol: {}", staking_protocol);
        
        Ok(())
    }

    /// Mint veVCoin to a user (only callable by staking protocol)
    /// Called when user stakes VCoin
    pub fn mint_vevcoin(
        ctx: Context<MintVeVCoin>,
        amount: u64,
    ) -> Result<()> {
        // Get bump from context
        let config_bump = ctx.bumps.config;
        let user_account_bump = ctx.bumps.user_account;
        
        // Only staking protocol can mint
        require!(
            ctx.accounts.staking_protocol.key() == ctx.accounts.config.staking_protocol,
            VeVCoinError::Unauthorized
        );
        
        require!(amount > 0, VeVCoinError::ZeroAmount);
        
        let clock = Clock::get()?;
        let now = clock.unix_timestamp;
        
        // Check if first time user
        let is_new_user = ctx.accounts.user_account.balance == 0;
        let current_balance = ctx.accounts.user_account.balance;
        let current_total_supply = ctx.accounts.config.total_supply;
        let current_total_holders = ctx.accounts.config.total_holders;
        
        // Mint tokens using Token-2022 first
        let seeds = &[
            VEVCOIN_CONFIG_SEED,
            &[config_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        
        token_2022::mint_to(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
                token_2022::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;
        
        // Now update state
        let user_account = &mut ctx.accounts.user_account;
        let config = &mut ctx.accounts.config;
        
        // Initialize user account if first time
        if is_new_user {
            user_account.owner = ctx.accounts.user.key();
            user_account.first_mint_at = now;
            user_account.bump = user_account_bump;
            config.total_holders = current_total_holders.checked_add(1).unwrap();
        }
        
        // Update balances
        user_account.balance = current_balance.checked_add(amount).unwrap();
        user_account.last_update_at = now;
        config.total_supply = current_total_supply.checked_add(amount).unwrap();
        
        msg!("Minted {} veVCoin to {}", amount, ctx.accounts.user.key());
        msg!("New balance: {}", user_account.balance);
        
        Ok(())
    }

    /// Burn veVCoin from a user (only callable by staking protocol)
    /// Called when user unstakes VCoin
    pub fn burn_vevcoin(
        ctx: Context<BurnVeVCoin>,
        amount: u64,
    ) -> Result<()> {
        // Get bumps from context
        let config_bump = ctx.bumps.config;
        
        // Only staking protocol can burn
        require!(
            ctx.accounts.staking_protocol.key() == ctx.accounts.config.staking_protocol,
            VeVCoinError::Unauthorized
        );
        
        require!(amount > 0, VeVCoinError::ZeroAmount);
        require!(ctx.accounts.user_account.balance >= amount, VeVCoinError::InsufficientBalance);
        
        let clock = Clock::get()?;
        let current_balance = ctx.accounts.user_account.balance;
        let current_total_supply = ctx.accounts.config.total_supply;
        let current_total_holders = ctx.accounts.config.total_holders;
        let new_balance = current_balance.checked_sub(amount).unwrap();
        
        // Burn tokens using Token-2022 first
        let seeds = &[
            VEVCOIN_CONFIG_SEED,
            &[config_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        
        token_2022::burn(
            CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
                token_2022::Burn {
                    mint: ctx.accounts.mint.to_account_info(),
                    from: ctx.accounts.user_token_account.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;
        
        // Now update state
        let user_account = &mut ctx.accounts.user_account;
        let config = &mut ctx.accounts.config;
        
        // Update balances
        user_account.balance = new_balance;
        user_account.last_update_at = clock.unix_timestamp;
        config.total_supply = current_total_supply.checked_sub(amount).unwrap();
        
        // Update holder count if balance is now zero
        if new_balance == 0 {
            config.total_holders = current_total_holders.checked_sub(1).unwrap();
        }
        
        msg!("Burned {} veVCoin from {}", amount, ctx.accounts.user.key());
        msg!("New balance: {}", user_account.balance);
        
        Ok(())
    }

    /// Update the staking protocol address (only authority)
    pub fn update_staking_protocol(
        ctx: Context<UpdateConfig>,
        new_staking_protocol: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(
            ctx.accounts.authority.key() == config.authority,
            VeVCoinError::Unauthorized
        );
        
        config.staking_protocol = new_staking_protocol;
        
        msg!("Staking protocol updated to: {}", new_staking_protocol);
        
        Ok(())
    }

    /// Update the authority
    pub fn update_authority(
        ctx: Context<UpdateConfig>,
        new_authority: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        
        require!(
            ctx.accounts.authority.key() == config.authority,
            VeVCoinError::Unauthorized
        );
        
        config.authority = new_authority;
        
        msg!("Authority updated to: {}", new_authority);
        
        Ok(())
    }

    /// Get user's veVCoin balance (view function)
    pub fn get_balance(ctx: Context<GetBalance>) -> Result<u64> {
        Ok(ctx.accounts.user_account.balance)
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = VeVCoinConfig::LEN,
        seeds = [VEVCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, VeVCoinConfig>,
    
    /// The veVCoin mint (Token-2022 with Non-Transferable extension)
    /// CHECK: Validated by Token-2022 program
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct MintVeVCoin<'info> {
    /// The staking protocol (must match config)
    pub staking_protocol: Signer<'info>,
    
    /// The user receiving veVCoin
    /// CHECK: Just a pubkey for PDA derivation
    pub user: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [VEVCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, VeVCoinConfig>,
    
    #[account(
        init_if_needed,
        payer = payer,
        space = UserVeVCoin::LEN,
        seeds = [UserVeVCoin::SEED, user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserVeVCoin>,
    
    /// The veVCoin mint
    #[account(
        mut,
        constraint = mint.key() == config.mint @ VeVCoinError::InvalidMint
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// User's token account for veVCoin - MUST be owned by user and use veVCoin mint
    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ VeVCoinError::InvalidTokenAccount,
        constraint = user_token_account.mint == config.mint @ VeVCoinError::InvalidMint
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct BurnVeVCoin<'info> {
    /// The staking protocol (must match config)
    pub staking_protocol: Signer<'info>,
    
    /// The user whose veVCoin is being burned
    /// CHECK: Just a pubkey for PDA derivation
    pub user: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [VEVCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, VeVCoinConfig>,
    
    #[account(
        mut,
        seeds = [UserVeVCoin::SEED, user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserVeVCoin>,
    
    /// The veVCoin mint
    #[account(
        mut,
        constraint = mint.key() == config.mint @ VeVCoinError::InvalidMint
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    
    /// User's token account for veVCoin - MUST be owned by user and use veVCoin mint
    #[account(
        mut,
        constraint = user_token_account.owner == user.key() @ VeVCoinError::InvalidTokenAccount,
        constraint = user_token_account.mint == config.mint @ VeVCoinError::InvalidMint
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token2022>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [VEVCOIN_CONFIG_SEED],
        bump
    )]
    pub config: Account<'info, VeVCoinConfig>,
}

#[derive(Accounts)]
pub struct GetBalance<'info> {
    /// CHECK: Just a pubkey for PDA derivation
    pub user: UncheckedAccount<'info>,
    
    #[account(
        seeds = [UserVeVCoin::SEED, user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserVeVCoin>,
}


