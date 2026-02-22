use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding};

use crate::system::cpu::CpuData;
use crate::theme::Theme;
use crate::theme::colors::{self, GAUGE_EMPTY};
use crate::util::format::format_percent;

/// Draws the CPU panel with overall + per-core gauge bars.
pub fn draw(frame: &mut Frame, cpu: &CpuData, show_per_core: bool, area: Rect) {
    let block = Block::default()
        .title(Span::styled(" CPU ", Theme::title()))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Theme::border())
        .padding(Padding::new(1, 1, 0, 0));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Calculate how many rows we can fit
    let available_rows = inner.height as usize;
    if available_rows == 0 {
        return;
    }

    // We need to allocate rows for: overall (1) + blank (1) + per-core (N) + blank (1) + footer (2)
    let footer_lines = 2;
    let overhead = if show_per_core {
        4 + footer_lines
    } else {
        2 + footer_lines
    };
    let max_cores = if show_per_core && available_rows > overhead {
        available_rows - overhead
    } else {
        0
    };

    // Build constraints
    let mut constraints: Vec<Constraint> = Vec::new();

    // Overall bar
    constraints.push(Constraint::Length(1));
    // Blank line
    constraints.push(Constraint::Length(1));

    // Per-core bars
    let cores_to_show = max_cores.min(cpu.per_core.len());
    if show_per_core && cores_to_show > 0 {
        for _ in 0..cores_to_show {
            constraints.push(Constraint::Length(1));
        }
        constraints.push(Constraint::Length(1)); // blank
    }

    // Footer
    constraints.push(Constraint::Length(1));
    constraints.push(Constraint::Length(1));
    constraints.push(Constraint::Min(0)); // absorb remainder

    let rows = Layout::vertical(constraints).split(inner);

    let mut row_idx = 0;

    // ── Overall bar ─────────────────────────────────────────────────────
    let overall_line = gauge_line("Overall", cpu.overall_percent, inner.width as usize);
    frame.render_widget(overall_line, rows[row_idx]);
    row_idx += 2; // skip blank

    // ── Per-core bars ───────────────────────────────────────────────────
    if show_per_core && cores_to_show > 0 {
        for (i, &usage) in cpu.per_core.iter().take(cores_to_show).enumerate() {
            let label = format!("Core {i}");
            let line = gauge_line(&label, usage, inner.width as usize);
            frame.render_widget(line, rows[row_idx]);
            row_idx += 1;
        }
        row_idx += 1; // blank
    }

    // ── Footer info ─────────────────────────────────────────────────────
    let cores_label = format!(
        "Cores: {} physical, {} logical",
        cpu.physical_cores,
        cpu.per_core.len()
    );
    frame.render_widget(
        Line::from(Span::styled(cores_label, Theme::label())),
        rows[row_idx],
    );
    row_idx += 1;

    let proc_label = format!("Processes: {}", cpu.process_count);
    frame.render_widget(
        Line::from(Span::styled(proc_label, Theme::label())),
        rows[row_idx],
    );
}

/// Renders a single gauge bar line:  `Label   ████████░░░░  67.2%`
fn gauge_line<'a>(label: &str, percent: f64, width: usize) -> Line<'a> {
    let pct_str = format_percent(percent);
    let label_width = 10;
    let pct_width = 7; // e.g. " 99.9%"
    let gap = 2; // spaces between bar and text
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
