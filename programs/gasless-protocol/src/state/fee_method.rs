use anchor_lang::prelude::*;

/// Fee deduction method
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum FeeMethod {
    #[default]
    PlatformSubsidized,
    VCoinDeduction,
    SSCREDeduction,
}

