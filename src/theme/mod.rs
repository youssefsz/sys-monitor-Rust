pub mod colors;

use ratatui::style::{Modifier, Style};

use self::colors::*;

/// Centralized style factory so every widget stays visually consistent.
pub struct Theme;

#[allow(dead_code)]
impl Theme {
    // ── Block borders ───────────────────────────────────────────────────
    pub fn border() -> Style {
        Style::default().fg(BORDER)
    }

    pub fn border_focused() -> Style {
        Style::default().fg(ACCENT)
    }

    // ── Title ───────────────────────────────────────────────────────────
    pub fn title() -> Style {
        Style::default()
            .fg(TEXT_PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    // ── Text ────────────────────────────────────────────────────────────
    pub fn label() -> Style {
        Style::default().fg(TEXT_SECONDARY)
    }

    pub fn value() -> Style {
        Style::default()
            .fg(TEXT_PRIMARY)
            .add_modifier(Modifier::BOLD)
    }

    pub fn dim() -> Style {
        Style::default().fg(TEXT_DIM)
    }

    // ── Status ──────────────────────────────────────────────────────────
    pub fn healthy() -> Style {
        Style::default().fg(HEALTHY)
    }

    pub fn info() -> Style {
        Style::default().fg(INFO)
    }

    pub fn warning() -> Style {
        Style::default().fg(WARNING)
    }

    pub fn critical() -> Style {
        Style::default().fg(CRITICAL)
    }

    /// Returns a style whose foreground matches the severity of `percent`.
    pub fn severity(percent: f64) -> Style {
        Style::default().fg(colors::severity_color(percent))
    }

    // ── Table ───────────────────────────────────────────────────────────
    pub fn table_header() -> Style {
        Style::default()
            .fg(TEXT_PRIMARY)
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    }

    pub fn table_row() -> Style {
        Style::default().fg(TEXT_PRIMARY)
    }

    pub fn table_selected() -> Style {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    }
}
