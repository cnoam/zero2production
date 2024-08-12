//! lib.rs

pub mod configuration;
pub mod domain;
pub mod email_client;
pub mod routes;
pub mod startup;
pub mod telemetry;
mod authentication;
pub mod session_state; // 10.7.5.3
pub mod utils; // 10.8.2.1 page 502