/// Contexts module - Account validation structures
mod initialize;
mod register_oracle;
mod submit_score;
mod create_snapshot;
mod vouch_for_user;
mod evaluate_vouch;
mod update_user_score;
mod update_config;
mod update_authority;
mod accept_authority;
mod get_score;

pub use initialize::*;
pub use register_oracle::*;
pub use submit_score::*;
pub use create_snapshot::*;
pub use vouch_for_user::*;
pub use evaluate_vouch::*;
pub use update_user_score::*;
pub use update_config::*;
pub use update_authority::*;
pub use accept_authority::*;
pub use get_score::*;

