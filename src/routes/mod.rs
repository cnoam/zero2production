//! src/routes/mod.rs
pub(crate) mod health_check;
pub(crate) mod subscriptions;
pub use subscriptions::*;
pub mod subscriptions_confirm;
mod newsletters;



//pub use health_check::*;
// pub use home::*;
// pub use login::*;
pub use newsletters::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;
