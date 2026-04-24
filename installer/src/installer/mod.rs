//! Installer logic — side-effecting modules that actually touch the host.

pub mod config;
pub mod database;
pub mod deps;
pub mod executor;
pub mod preflight;
pub mod services;
pub mod source;
pub mod verify;
pub mod webui;
