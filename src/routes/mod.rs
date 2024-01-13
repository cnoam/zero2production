//! src/routes/mod.rs
mod health_check;
//pub(crate) mod subscriptions;
//pub use subscriptions::*;
//pub mod subscriptions_confirm;
//mod newsletters;


mod home;
mod login;
mod newsletters;
mod subscriptions;
mod subscriptions_confirm;
mod admin;

pub use admin::*;
pub use health_check::*;
pub use home::*;
// book 10.6
pub use login::*; 
pub use newsletters::*;
pub use subscriptions::*;
pub use subscriptions_confirm::*;
