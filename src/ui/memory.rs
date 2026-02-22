use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding};

use crate::system::memory::MemoryData;
use crate::theme::Theme;
use crate::theme::colors::{self, CRITICAL, GAUGE_EMPTY, HEALTHY, WARNING};
use crate::util::format::{format_bytes, format_percent};

/// Draws the memory panel with RAM bar, swap bar, and breakdown.
pub fn draw(frame: &mut Frame, mem: &MemoryData, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" MEMORY ", Theme::title()))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Theme::border())
        .padding(Padding::new(1, 1, 0, 0));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let constraints = [
        Constraint::Length(1), // RAM bar
        Constraint::Length(1), // RAM detail
        Constraint::Length(1), // blank
        Constraint::Length(1), // Swap bar
        Constraint::Length(1), // Swap detail
        Constraint::Length(1), // blank
        Constraint::Length(1), // separator
        Constraint::Length(1), // blank
        Constraint::Length(1), // Available
        Constraint::Length(1), // blank
        Constraint::Length(1), // pressure indicator
        Constraint::Min(0),
    ];
    let rows = Layout::vertical(constraints).split(inner);
    let w = inner.width as usize;

    // ── RAM bar ─────────────────────────────────────────────────────────
    let ram_pct = mem.ram_percent();
    frame.render_widget(gauge_line("RAM", ram_pct, w), rows[0]);

    let ram_detail = format!(
        "          {} / {}",
        format_bytes(mem.used_mem),
        format_bytes(mem.total_mem),
    );
    frame.render_widget(
        Line::from(Span::styled(ram_detail, Theme::label())),
        rows[1],
    );

    // ── Swap bar ────────────────────────────────────────────────────────
    let swap_pct = mem.swap_percent();
    frame.render_widget(gauge_line("Swap", swap_pct, w), rows[3]);

    let swap_detail = format!(
        "          {} / {}",
        format_bytes(mem.used_swap),
        format_bytes(mem.total_swap),
    );
    frame.render_widget(
        Line::from(Span::styled(swap_detail, Theme::label())),
        rows[4],
    );

    // ── Separator ───────────────────────────────────────────────────────
    let sep = "┄".repeat(w.saturating_sub(2));
    frame.render_widget(Line::from(Span::styled(sep, Theme::dim())), rows[6]);

    // ── Available memory ────────────────────────────────────────────────
    let avail_line = Line::from(vec![
        Span::styled("Available     ", Theme::label()),
        Span::styled(format_bytes(mem.available_mem), Theme::value()),
    ]);
    frame.render_widget(avail_line, rows[8]);

    // ── Pressure indicator ──────────────────────────────────────────────
    let (pressure_label, pressure_color) = if ram_pct < 70.0 {
        ("● Normal", HEALTHY)
    } else if ram_pct < 85.0 {
        ("● Elevated", WARNING)
    } else {
        ("● Critical", CRITICAL)
    };

    let pressure_line = Line::from(vec![
        Span::styled("Pressure:  ", Theme::label()),
        Span::styled(pressure_label, Style::default().fg(pressure_color)),
    ]);
    frame.render_widget(pressure_line, rows[10]);
}

/// Renders a gauge bar line similar to the CPU panel.
fn gauge_line<'a>(label: &str, percent: f64, width: usize) -> Line<'a> {
    let pct_str = format_percent(percent);
    let label_width = 10;
    let pct_width = 7;
    let gap = 2;
    let bar_width = width
        .saturating_sub(label_width)
        .saturating_sub(pct_width)
        .saturating_sub(gap);

    if bar_width == 0 {
        return Line::from(vec![
            Span::styled(format!("{label:<label_width$}"), Theme::label()),
            Span::styled(pct_str, Theme::severity(percent)),
        ]);
    }

    let filled = ((percent / 100.0) * bar_width as f64).round() as usize;
    let empty = bar_width.saturating_sub(filled);
    let bar_color = colors::gauge_color(percent);

    Line::from(vec![
        Span::styled(format!("{label:<label_width$}"), Theme::label()),
        Span::styled("█".repeat(filled), Style::default().fg(bar_color)),
        Span::styled("░".repeat(empty), Style::default().fg(GAUGE_EMPTY)),
        Span::styled(format!("  {pct_str:>6}"), Theme::severity(percent)),
    ])
}
