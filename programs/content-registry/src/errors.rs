use anchor_lang::prelude::*;

#[error_code]
pub enum ContentError {
    #[msg("Unauthorized: Only the authority can perform this action")]
    Unauthorized,
    #[msg("Content registry is paused")]
    RegistryPaused,
    #[msg("Content not found")]
    ContentNotFound,
    #[msg("Content already deleted")]
    ContentAlreadyDeleted,
    #[msg("Cannot edit deleted content")]
    CannotEditDeleted,
    #[msg("Insufficient energy for this action")]
    InsufficientEnergy,
    #[msg("Daily post cap exceeded")]
    DailyCapExceeded,
    #[msg("Cooldown period not elapsed")]
    CooldownNotElapsed,
    #[msg("Invalid content type")]
    InvalidContentType,
    #[msg("Content URI too long (max 128 chars)")]
    ContentURITooLong,
    #[msg("Energy refund already claimed")]
    RefundAlreadyClaimed,
    #[msg("Engagement check period not elapsed")]
    RefundNotReady,
    #[msg("Arithmetic overflow")]
    Overflow,
}
