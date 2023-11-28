//! src/routes/mod.rs
pub(crate) mod health_check;
pub(crate) mod subscriptions;
pub use subscriptions::*;
pub mod subscriptions_confirm;
mod newsletters;
pub use newsletters::*;