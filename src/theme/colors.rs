use ratatui::style::Color;

// ── Base palette ────────────────────────────────────────────────────────────
pub const BORDER: Color = Color::Rgb(85, 85, 85);
pub const TEXT_PRIMARY: Color = Color::Rgb(224, 224, 224);
pub const TEXT_SECONDARY: Color = Color::Rgb(136, 136, 136);
pub const TEXT_DIM: Color = Color::Rgb(85, 85, 85);

// ── Semantic colors ─────────────────────────────────────────────────────────
pub const HEALTHY: Color = Color::Rgb(90, 238, 160);
pub const INFO: Color = Color::Rgb(100, 181, 246);
pub const WARNING: Color = Color::Rgb(245, 200, 66);
pub const CRITICAL: Color = Color::Rgb(255, 107, 107);
pub const ACCENT: Color = Color::Rgb(187, 134, 252);

// ── Gauge colors ────────────────────────────────────────────────────────────
pub const GAUGE_EMPTY: Color = Color::Rgb(51, 51, 51);

/// Returns a color based on a percentage (0.0–100.0) using a smooth gradient:
///   0–50  → Healthy
///  50–75  → Info
///  75–90  → Warning
///  90–100 → Critical
pub fn severity_color(percent: f64) -> Color {
    match percent {
        p if p < 50.0 => HEALTHY,
        p if p < 75.0 => INFO,
        p if p < 90.0 => WARNING,
        _ => CRITICAL,
    }
}

/// Blends between two RGB colors by a ratio (0.0 = `from`, 1.0 = `to`).
pub fn lerp_color(from: Color, to: Color, ratio: f64) -> Color {
    let ratio = ratio.clamp(0.0, 1.0);
    if let (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) = (from, to) {
        Color::Rgb(
            (r1 as f64 + (r2 as f64 - r1 as f64) * ratio) as u8,
            (g1 as f64 + (g2 as f64 - g1 as f64) * ratio) as u8,
            (b1 as f64 + (b2 as f64 - b1 as f64) * ratio) as u8,
        )
    } else {
        to
    }
}

/// Returns a smoothly interpolated severity color for gauges.
pub fn gauge_color(percent: f64) -> Color {
    match percent {
        p if p < 50.0 => {
            let ratio = p / 50.0;
            lerp_color(HEALTHY, INFO, ratio)
        }
        p if p < 75.0 => {
            let ratio = (p - 50.0) / 25.0;
            lerp_color(INFO, WARNING, ratio)
        }
        p if p < 90.0 => {
            let ratio = (p - 75.0) / 15.0;
            lerp_color(WARNING, CRITICAL, ratio)
        }
        _ => CRITICAL,
    }
}
