/// Token instructions
pub mod mint;
pub mod slash;
pub mod propose_slash;
pub mod approve_slash;
pub mod execute_slash;

pub use mint::*;
pub use slash::*;
pub use propose_slash::*;
pub use approve_slash::*;
pub use execute_slash::*;
