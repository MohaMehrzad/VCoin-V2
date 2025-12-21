use anchor_lang::prelude::*;

declare_id!("CnxKPyRgU3HZDUvbAFPAddYJVWM2rWhLVq9QoEnBgJdB");

/// ViWoApp Identity Protocol
/// 
/// Minimal on-chain DID anchor with Solana Attestation Service (SAS) integration.
/// Provides portable identity verification across all Solana dApps.
/// 
/// Verification Levels:
/// - Level 0 (None): Wallet connected only
/// - Level 1 (Basic): Email + phone verified  
/// - Level 2 (KYC): Identity documents verified
/// - Level 3 (Full): KYC + biometric verification
/// - Level 4 (Enhanced): Full + UniqueHuman attestation

pub mod constants {
    /// Seeds
    pub const IDENTITY_CONFIG_SEED: &[u8] = b"identity-config";
    pub const IDENTITY_SEED: &[u8] = b"identity";
    pub const SAS_ATTESTATION_SEED: &[u8] = b"sas-attestation";
    pub const SUBSCRIPTION_SEED: &[u8] = b"subscription";
    
    /// Subscription prices in USDC (6 decimals)
    pub const SUBSCRIPTION_FREE: u64 = 0;
    pub const SUBSCRIPTION_VERIFIED: u64 = 4_000_000;   // $4 USDC
    pub const SUBSCRIPTION_PREMIUM: u64 = 12_000_000;   // $12 USDC
    pub const SUBSCRIPTION_ENTERPRISE: u64 = 59_000_000; // $59 USDC
    
    /// Subscription duration (30 days in seconds)
    pub const SUBSCRIPTION_DURATION: i64 = 30 * 24 * 60 * 60;
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum IdentityError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("Identity protocol is paused")]
        ProtocolPaused,
        #[msg("Identity already exists for this wallet")]
        IdentityAlreadyExists,
        #[msg("Identity does not exist")]
        IdentityNotFound,
        #[msg("Invalid verification level")]
        InvalidVerificationLevel,
        #[msg("Verification level cannot be downgraded")]
        CannotDowngradeVerification,
        #[msg("SAS attestation required for this verification level")]
        SASAttestationRequired,
        #[msg("SAS attestation has expired")]
        SASAttestationExpired,
        #[msg("Invalid subscription tier")]
        InvalidSubscriptionTier,
        #[msg("Subscription has expired")]
        SubscriptionExpired,
        #[msg("Insufficient payment for subscription")]
        InsufficientPayment,
        #[msg("Attestation not from trusted attester")]
        UntrustedAttester,
        #[msg("Arithmetic overflow")]
        Overflow,
    }
}

pub mod state {
    use super::*;
    
    /// Verification levels
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum VerificationLevel {
        #[default]
        None = 0,
        Basic = 1,      // Email + phone
        KYC = 2,        // Identity documents
        Full = 3,       // KYC + biometric
        Enhanced = 4,   // Full + UniqueHuman
    }
    
    impl VerificationLevel {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(VerificationLevel::None),
                1 => Some(VerificationLevel::Basic),
                2 => Some(VerificationLevel::KYC),
                3 => Some(VerificationLevel::Full),
                4 => Some(VerificationLevel::Enhanced),
                _ => None,
            }
        }
    }
    
    /// Subscription tiers
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum SubscriptionTier {
        #[default]
        Free = 0,
        Verified = 1,   // $4/month
        Premium = 2,    // $12/month
        Enterprise = 3, // $59/month
    }
    
    impl SubscriptionTier {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(SubscriptionTier::Free),
                1 => Some(SubscriptionTier::Verified),
                2 => Some(SubscriptionTier::Premium),
                3 => Some(SubscriptionTier::Enterprise),
                _ => None,
            }
        }
        
        pub fn price(&self) -> u64 {
            use super::constants::*;
            match self {
                SubscriptionTier::Free => SUBSCRIPTION_FREE,
                SubscriptionTier::Verified => SUBSCRIPTION_VERIFIED,
                SubscriptionTier::Premium => SUBSCRIPTION_PREMIUM,
                SubscriptionTier::Enterprise => SUBSCRIPTION_ENTERPRISE,
            }
        }
    }
    
    /// Global identity protocol configuration
    #[account]
    #[derive(Default)]
    pub struct IdentityConfig {
        /// Admin authority
        pub authority: Pubkey,
        /// SAS program ID (Solana Attestation Service)
        pub sas_program: Pubkey,
        /// USDC mint for subscriptions
        pub usdc_mint: Pubkey,
        /// Treasury for subscription payments
        pub treasury: Pubkey,
        /// Trusted attesters (max 10)
        pub trusted_attesters: [Pubkey; 10],
        /// Number of active trusted attesters
        pub attester_count: u8,
        /// Total registered identities
        pub total_identities: u64,
        /// Total verified identities (Level 1+)
        pub verified_identities: u64,
        /// Whether protocol is paused
        pub paused: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl IdentityConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // sas_program
            32 + // usdc_mint
            32 + // treasury
            (32 * 10) + // trusted_attesters
            1 +  // attester_count
            8 +  // total_identities
            8 +  // verified_identities
            1 +  // paused
            1;   // bump
    }
    
    /// Individual user identity (on-chain DID anchor)
    #[account]
    #[derive(Default)]
    pub struct Identity {
        /// Owner wallet
        pub owner: Pubkey,
        /// SHA256 hash of full DID document (stored off-chain)
        pub did_hash: [u8; 32],
        /// Current verification level
        pub verification_level: u8,
        /// Hash of verification proof
        pub verification_hash: [u8; 32],
        /// Username (max 32 chars)
        pub username: [u8; 32],
        /// Username length
        pub username_len: u8,
        /// Account creation timestamp
        pub created_at: i64,
        /// Last update timestamp
        pub updated_at: i64,
        /// Whether identity is active
        pub is_active: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl Identity {
        pub const LEN: usize = 8 + // discriminator
            32 + // owner
            32 + // did_hash
            1 +  // verification_level
            32 + // verification_hash
            32 + // username
            1 +  // username_len
            8 +  // created_at
            8 +  // updated_at
            1 +  // is_active
            1;   // bump
    }
    
    /// User's SAS attestation link
    #[account]
    #[derive(Default)]
    pub struct UserSASAttestation {
        /// User wallet
        pub user: Pubkey,
        /// SAS attestation account PDA
        pub sas_attestation_id: Pubkey,
        /// Attestation type (0=Email, 1=Phone, 2=KYC, 3=Biometric, 4=UniqueHuman)
        pub attestation_type: u8,
        /// Who issued the attestation
        pub attester: Pubkey,
        /// Bitmap of verified claims
        pub verified_claims: u16,
        /// Derived verification level from claims
        pub verification_level: u8,
        /// First verification timestamp
        pub first_verified_at: i64,
        /// Last verification timestamp
        pub last_verified_at: i64,
        /// Attestation expiry
        pub expires_at: i64,
        /// Portable score from other dApps (0-10000)
        pub portable_score: u16,
        /// PDA bump
        pub bump: u8,
    }
    
    impl UserSASAttestation {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            32 + // sas_attestation_id
            1 +  // attestation_type
            32 + // attester
            2 +  // verified_claims
            1 +  // verification_level
            8 +  // first_verified_at
            8 +  // last_verified_at
            8 +  // expires_at
            2 +  // portable_score
            1;   // bump
    }
    
    /// User subscription account
    #[account]
    #[derive(Default)]
    pub struct Subscription {
        /// User wallet
        pub user: Pubkey,
        /// Current subscription tier
        pub tier: u8,
        /// Subscription start timestamp
        pub started_at: i64,
        /// Subscription expiry timestamp
        pub expires_at: i64,
        /// Auto-renew enabled
        pub auto_renew: bool,
        /// Total payments made (USDC)
        pub total_paid: u64,
        /// PDA bump
        pub bump: u8,
    }
    
    impl Subscription {
        pub const LEN: usize = 8 + // discriminator
            32 + // user
            1 +  // tier
            8 +  // started_at
            8 +  // expires_at
            1 +  // auto_renew
            8 +  // total_paid
            1;   // bump
    }
}

pub mod events {
    use super::*;
    
    #[event]
    pub struct IdentityCreated {
        pub owner: Pubkey,
        pub username: String,
        pub timestamp: i64,
    }
    
    #[event]
    pub struct VerificationUpdated {
        pub owner: Pubkey,
        pub old_level: u8,
        pub new_level: u8,
        pub timestamp: i64,
    }
    
    #[event]
    pub struct SASAttestationLinked {
        pub user: Pubkey,
        pub attestation_id: Pubkey,
        pub attester: Pubkey,
        pub verification_level: u8,
    }
    
    #[event]
    pub struct SubscriptionUpdated {
        pub user: Pubkey,
        pub tier: u8,
        pub expires_at: i64,
    }
}

use constants::*;
use errors::*;
use state::*;
use events::*;

#[program]
pub mod identity_protocol {
    use super::*;

    /// Initialize the identity protocol
    pub fn initialize(
        ctx: Context<Initialize>,
        sas_program: Pubkey,
        usdc_mint: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.identity_config;
        
        config.authority = ctx.accounts.authority.key();
        config.sas_program = sas_program;
        config.usdc_mint = usdc_mint;
        config.treasury = ctx.accounts.treasury.key();
        config.trusted_attesters = [Pubkey::default(); 10];
        config.attester_count = 0;
        config.total_identities = 0;
        config.verified_identities = 0;
        config.paused = false;
        config.bump = ctx.bumps.identity_config;
        
        msg!("Identity protocol initialized");
        Ok(())
    }
    
    /// Create a new identity for a user
    pub fn create_identity(
        ctx: Context<CreateIdentity>,
        did_hash: [u8; 32],
        username: String,
    ) -> Result<()> {
        require!(!ctx.accounts.identity_config.paused, IdentityError::ProtocolPaused);
        require!(username.len() <= 32, IdentityError::InvalidVerificationLevel);
        
        let clock = Clock::get()?;
        let identity = &mut ctx.accounts.identity;
        
        identity.owner = ctx.accounts.owner.key();
        identity.did_hash = did_hash;
        identity.verification_level = VerificationLevel::None as u8;
        identity.verification_hash = [0u8; 32];
        
        // Store username
        let username_bytes = username.as_bytes();
        identity.username[..username_bytes.len()].copy_from_slice(username_bytes);
        identity.username_len = username_bytes.len() as u8;
        
        identity.created_at = clock.unix_timestamp;
        identity.updated_at = clock.unix_timestamp;
        identity.is_active = true;
        identity.bump = ctx.bumps.identity;
        
        // Update global stats
        let config = &mut ctx.accounts.identity_config;
        config.total_identities = config.total_identities.saturating_add(1);
        
        emit!(IdentityCreated {
            owner: identity.owner,
            username,
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Identity created for: {}", identity.owner);
        Ok(())
    }
    
    /// Update DID document hash
    pub fn update_did_hash(
        ctx: Context<UpdateIdentity>,
        new_did_hash: [u8; 32],
    ) -> Result<()> {
        let clock = Clock::get()?;
        let identity = &mut ctx.accounts.identity;
        
        identity.did_hash = new_did_hash;
        identity.updated_at = clock.unix_timestamp;
        
        msg!("DID hash updated for: {}", identity.owner);
        Ok(())
    }
    
    /// Update verification level (admin only)
    pub fn update_verification(
        ctx: Context<AdminUpdateIdentity>,
        new_level: u8,
        verification_hash: [u8; 32],
    ) -> Result<()> {
        let level = VerificationLevel::from_u8(new_level)
            .ok_or(IdentityError::InvalidVerificationLevel)?;
        
        let clock = Clock::get()?;
        let identity = &mut ctx.accounts.identity;
        let old_level = identity.verification_level;
        
        // Cannot downgrade verification
        require!(
            new_level >= old_level,
            IdentityError::CannotDowngradeVerification
        );
        
        identity.verification_level = new_level;
        identity.verification_hash = verification_hash;
        identity.updated_at = clock.unix_timestamp;
        
        // Update verified count if upgrading from None
        if old_level == 0 && new_level > 0 {
            let config = &mut ctx.accounts.identity_config;
            config.verified_identities = config.verified_identities.saturating_add(1);
        }
        
        emit!(VerificationUpdated {
            owner: identity.owner,
            old_level,
            new_level,
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Verification updated: {} -> {}", old_level, new_level);
        Ok(())
    }
    
    /// Link SAS attestation to identity
    pub fn link_sas_attestation(
        ctx: Context<LinkSASAttestation>,
        sas_attestation_id: Pubkey,
        attestation_type: u8,
        verified_claims: u16,
        expires_at: i64,
        portable_score: u16,
    ) -> Result<()> {
        let config = &ctx.accounts.identity_config;
        
        // Verify attester is trusted
        let attester = ctx.accounts.attester.key();
        let is_trusted = config.trusted_attesters[..config.attester_count as usize]
            .contains(&attester);
        require!(is_trusted, IdentityError::UntrustedAttester);
        
        let clock = Clock::get()?;
        let sas = &mut ctx.accounts.sas_attestation;
        
        sas.user = ctx.accounts.user.key();
        sas.sas_attestation_id = sas_attestation_id;
        sas.attestation_type = attestation_type;
        sas.attester = attester;
        sas.verified_claims = verified_claims;
        
        // Derive verification level from claims
        sas.verification_level = derive_verification_level(verified_claims);
        
        sas.first_verified_at = clock.unix_timestamp;
        sas.last_verified_at = clock.unix_timestamp;
        sas.expires_at = expires_at;
        sas.portable_score = portable_score;
        sas.bump = ctx.bumps.sas_attestation;
        
        // Update identity verification level if higher
        let identity = &mut ctx.accounts.identity;
        if sas.verification_level > identity.verification_level {
            identity.verification_level = sas.verification_level;
            identity.updated_at = clock.unix_timestamp;
        }
        
        emit!(SASAttestationLinked {
            user: sas.user,
            attestation_id: sas_attestation_id,
            attester,
            verification_level: sas.verification_level,
        });
        
        msg!("SAS attestation linked for: {}", sas.user);
        Ok(())
    }
    
    /// Subscribe to a tier
    pub fn subscribe(
        ctx: Context<Subscribe>,
        tier: u8,
    ) -> Result<()> {
        let tier_enum = SubscriptionTier::from_u8(tier)
            .ok_or(IdentityError::InvalidSubscriptionTier)?;
        
        let clock = Clock::get()?;
        let subscription = &mut ctx.accounts.subscription;
        
        subscription.user = ctx.accounts.user.key();
        subscription.tier = tier;
        subscription.started_at = clock.unix_timestamp;
        subscription.expires_at = clock.unix_timestamp + SUBSCRIPTION_DURATION;
        subscription.auto_renew = false;
        subscription.total_paid = subscription.total_paid.saturating_add(tier_enum.price());
        subscription.bump = ctx.bumps.subscription;
        
        emit!(SubscriptionUpdated {
            user: subscription.user,
            tier,
            expires_at: subscription.expires_at,
        });
        
        msg!("Subscription activated: tier {}", tier);
        Ok(())
    }
    
    /// Add trusted attester (admin only)
    pub fn add_trusted_attester(
        ctx: Context<UpdateConfig>,
        attester: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.identity_config;
        
        require!(config.attester_count < 10, IdentityError::Overflow);
        
        let idx = config.attester_count as usize;
        config.trusted_attesters[idx] = attester;
        config.attester_count += 1;
        
        msg!("Trusted attester added: {}", attester);
        Ok(())
    }
    
    /// Remove trusted attester (admin only)
    pub fn remove_trusted_attester(
        ctx: Context<UpdateConfig>,
        attester: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.identity_config;
        
        // Find and remove the attester
        let mut found = false;
        let count = config.attester_count as usize;
        for i in 0..count {
            if config.trusted_attesters[i] == attester {
                // Shift remaining attesters
                let last_idx = (config.attester_count - 1) as usize;
                for j in i..last_idx {
                    config.trusted_attesters[j] = config.trusted_attesters[j + 1];
                }
                config.trusted_attesters[last_idx] = Pubkey::default();
                config.attester_count -= 1;
                found = true;
                break;
            }
        }
        
        require!(found, IdentityError::UntrustedAttester);
        
        msg!("Trusted attester removed: {}", attester);
        Ok(())
    }
    
    /// Pause/unpause protocol
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        ctx.accounts.identity_config.paused = paused;
        msg!("Identity protocol paused: {}", paused);
        Ok(())
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.identity_config.authority = new_authority;
        msg!("Authority updated to: {}", new_authority);
        Ok(())
    }
    
    /// Get identity info
    pub fn get_identity(ctx: Context<GetIdentity>) -> Result<()> {
        let identity = &ctx.accounts.identity;
        msg!("Owner: {}", identity.owner);
        msg!("Verification level: {}", identity.verification_level);
        msg!("Active: {}", identity.is_active);
        Ok(())
    }
}

// Helper functions

fn derive_verification_level(claims: u16) -> u8 {
    // Claims bitmap:
    // bit 0: Email verified
    // bit 1: Phone verified
    // bit 2: Social verified
    // bit 3: KYC verified
    // bit 4: Biometric verified
    // bit 5: UniqueHuman verified
    
    let has_email = claims & 0x01 != 0;
    let has_phone = claims & 0x02 != 0;
    let has_social = claims & 0x04 != 0;
    let has_kyc = claims & 0x08 != 0;
    let has_biometric = claims & 0x10 != 0;
    let has_unique_human = claims & 0x20 != 0;
    
    if has_kyc && has_biometric && has_unique_human {
        VerificationLevel::Enhanced as u8
    } else if has_kyc && has_biometric {
        VerificationLevel::Full as u8
    } else if has_kyc || (has_email && has_phone && has_social) {
        VerificationLevel::KYC as u8
    } else if has_email && has_phone {
        VerificationLevel::Basic as u8
    } else {
        VerificationLevel::None as u8
    }
}

// Account contexts

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = IdentityConfig::LEN,
        seeds = [IDENTITY_CONFIG_SEED],
        bump
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    /// CHECK: Treasury account for subscription payments
    pub treasury: UncheckedAccount<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateIdentity<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    #[account(
        init,
        payer = owner,
        space = Identity::LEN,
        seeds = [IDENTITY_SEED, owner.key().as_ref()],
        bump
    )]
    pub identity: Account<'info, Identity>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateIdentity<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_SEED, owner.key().as_ref()],
        bump = identity.bump,
        has_one = owner
    )]
    pub identity: Account<'info, Identity>,
    
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdminUpdateIdentity<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump,
        has_one = authority @ IdentityError::Unauthorized
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    #[account(
        mut,
        seeds = [IDENTITY_SEED, identity.owner.as_ref()],
        bump = identity.bump
    )]
    pub identity: Account<'info, Identity>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct LinkSASAttestation<'info> {
    #[account(
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    #[account(
        mut,
        seeds = [IDENTITY_SEED, user.key().as_ref()],
        bump = identity.bump
    )]
    pub identity: Account<'info, Identity>,
    
    #[account(
        init,
        payer = user,
        space = UserSASAttestation::LEN,
        seeds = [SAS_ATTESTATION_SEED, user.key().as_ref()],
        bump
    )]
    pub sas_attestation: Account<'info, UserSASAttestation>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// The attester signing this attestation
    pub attester: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = Subscription::LEN,
        seeds = [SUBSCRIPTION_SEED, user.key().as_ref()],
        bump
    )]
    pub subscription: Account<'info, Subscription>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump,
        has_one = authority @ IdentityError::Unauthorized
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [IDENTITY_CONFIG_SEED],
        bump = identity_config.bump,
        has_one = authority @ IdentityError::Unauthorized
    )]
    pub identity_config: Account<'info, IdentityConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetIdentity<'info> {
    #[account(
        seeds = [IDENTITY_SEED, identity.owner.as_ref()],
        bump = identity.bump
    )]
    pub identity: Account<'info, Identity>,
}


