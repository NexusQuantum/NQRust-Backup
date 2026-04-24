//! NQRustBackup brand theme.
//!
//! Color palette tuned to read clearly on dark terminals and match the
//! NQR-family look from the MicroVM installer.

#![allow(dead_code)]

use ratatui::style::{Color, Modifier, Style};

pub const PRODUCT_NAME: &str = "NQRustBackup";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// NQR brand orange
pub const PRIMARY: Color = Color::Rgb(255, 80, 1);
// NQRustBackup accent — backup/archive blue
pub const ACCENT: Color = Color::Rgb(0, 120, 214);

pub const SUCCESS: Color = Color::Rgb(34, 197, 94);
pub const WARNING: Color = Color::Rgb(234, 179, 8);
pub const ERROR: Color = Color::Rgb(239, 68, 68);
pub const INFO: Color = Color::Rgb(59, 130, 246);

pub const BACKGROUND: Color = Color::Rgb(26, 26, 26);
pub const FOREGROUND: Color = Color::Rgb(252, 252, 252);
pub const CARD: Color = Color::Rgb(53, 53, 53);
pub const BORDER: Color = Color::Rgb(74, 74, 74);
pub const MUTED: Color = Color::Rgb(107, 114, 128);
pub const SECONDARY: Color = Color::Rgb(156, 163, 175);

pub mod symbols {
    pub const CHECK: &str = "✓";
    pub const CROSS: &str = "✗";
    pub const PENDING: &str = "○";
    pub const IN_PROGRESS: &str = "◐";
    pub const ARROW_RIGHT: &str = "▶";
    pub const BULLET: &str = "•";
    pub const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
}

pub mod styles {
    use super::*;

    pub fn text() -> Style {
        Style::default().fg(FOREGROUND)
    }

    pub fn primary() -> Style {
        Style::default().fg(PRIMARY)
    }

    pub fn primary_bold() -> Style {
        Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)
    }

    pub fn accent() -> Style {
        Style::default().fg(ACCENT)
    }

    pub fn success() -> Style {
        Style::default().fg(SUCCESS)
    }

    pub fn warning() -> Style {
        Style::default().fg(WARNING)
    }

    pub fn error() -> Style {
        Style::default().fg(ERROR)
    }

    pub fn info() -> Style {
        Style::default().fg(INFO)
    }

    pub fn border() -> Style {
        Style::default().fg(BORDER)
    }

    pub fn muted() -> Style {
        Style::default().fg(MUTED)
    }

    pub fn secondary() -> Style {
        Style::default().fg(SECONDARY)
    }

    pub fn selected() -> Style {
        Style::default()
            .fg(FOREGROUND)
            .bg(CARD)
            .add_modifier(Modifier::BOLD)
    }
}
