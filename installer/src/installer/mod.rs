//! Installer logic — side-effecting modules that actually touch the host.

pub mod config;
pub mod database;
pub mod deps;
pub mod executor;
pub mod preflight;
pub mod roundtrip;
pub mod services;
pub mod source;
pub mod tls;
pub mod verify;
pub mod webui;
