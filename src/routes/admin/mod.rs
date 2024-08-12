//!src/routes/admin/mod.rs
mod dashboard;
mod password;
mod logout;

pub use password::*;
pub use dashboard::admin_dashboard;
pub use logout::log_out;