use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding};

use crate::system::cpu::CpuData;
use crate::theme::Theme;
use crate::theme::colors::{self, ACCENT, GAUGE_EMPTY, TEXT_SECONDARY};
use crate::util::format::format_percent;

/// Draws the CPU panel with overall + scrollable per-core gauge bars.
pub fn draw(
    frame: &mut Frame,
    cpu: &CpuData,
    show_per_core: bool,
    scroll_offset: &mut usize,
    area: Rect,
) {
    let block = Block::default()
        .title(Span::styled(" CPU ", Theme::title()))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Theme::border())
        .padding(Padding::new(1, 1, 0, 0));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let available_rows = inner.height as usize;
    if available_rows == 0 {
        return;
    }

    // Fixed rows: overall(1) + blank(1) + footer(2) = 4
    // When per-core is shown: + blank separator after cores(1) = 5
    let fixed_rows = if show_per_core { 5 } else { 4 };
    let core_slots = if show_per_core && available_rows > fixed_rows {
        available_rows - fixed_rows
    } else {
        0
    };

    let total_cores = cpu.per_core.len();

    // We may need 1 row for "▲ more" and 1 for "▼ more" indicators
    let need_scroll = show_per_core && total_cores > core_slots;

    let (show_up, show_down, visible_cores, actual_offset);
    if !show_per_core || core_slots == 0 {
        show_up = false;
        show_down = false;
        visible_cores = 0;
        actual_offset = 0;
        *scroll_offset = 0;
    } else if !need_scroll {
        // All cores fit — no scrolling needed
        show_up = false;
        show_down = false;
        visible_cores = total_cores;
        actual_offset = 0;
        *scroll_offset = 0;
    } else {
        // We need scrolling. Reserve rows for indicators as needed.
        // Clamp the scroll offset first
        let max_offset = total_cores.saturating_sub(1);
        if *scroll_offset > max_offset {
            *scroll_offset = max_offset;
        }
        actual_offset = *scroll_offset;

        let has_items_above = actual_offset > 0;
        let indicator_overhead = if has_items_above { 1 } else { 0 };
        let slots_for_cores = core_slots.saturating_sub(indicator_overhead);

        // Determine how many cores we can show after the offset
        let remaining = total_cores - actual_offset;
        let can_show = remaining.min(slots_for_cores);

        // Check if there are items below
        let has_items_below = actual_offset + can_show < total_cores;

        // If we have items below, we need a row for the ▼ indicator too
        let final_slots = if has_items_below {
            slots_for_cores.saturating_sub(1)
        } else {
            slots_for_cores
        };
        let final_can_show = remaining.min(final_slots).min(total_cores);

        // Re-check after adjustment
        let final_has_below = actual_offset + final_can_show < total_cores;

        // Re-clamp offset so we don't overshoot
        if final_can_show == 0 && total_cores > 0 {
            *scroll_offset = 0;
        }

        show_up = has_items_above;
        show_down = final_has_below;
        visible_cores = final_can_show;
    }

    // ── Build layout constraints ────────────────────────────────────────
    let mut constraints: Vec<Constraint> = Vec::new();

    // Overall bar + blank
    constraints.push(Constraint::Length(1));
    constraints.push(Constraint::Length(1));

    // Scroll-up indicator
    if show_up {
        constraints.push(Constraint::Length(1));
    }

    // Per-core bars
    for _ in 0..visible_cores {
        constraints.push(Constraint::Length(1));
    }

    // Scroll-down indicator
    if show_down {
        constraints.push(Constraint::Length(1));
    }

    // Blank separator after cores (only if we showed any)
    if show_per_core && (visible_cores > 0 || show_up || show_down) {
        constraints.push(Constraint::Length(1));
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

    // ── Scroll-up indicator ─────────────────────────────────────────────
    if show_up {
        let hidden_above = actual_offset;
        let indicator = Line::from(vec![
            Span::styled(
                format!("          ▲ {hidden_above} more above "),
                Style::default().fg(TEXT_SECONDARY),
            ),
            Span::styled(
                "press '[' to scroll up",
                Style::default().fg(ACCENT).add_modifier(Modifier::DIM),
            ),
        ]);
        frame.render_widget(indicator, rows[row_idx]);
        row_idx += 1;
    }

    // ── Per-core bars ───────────────────────────────────────────────────
    if visible_cores > 0 {
        let offset = if need_scroll { actual_offset } else { 0 };
        for (i, &usage) in cpu
            .per_core
            .iter()
            .skip(offset)
            .take(visible_cores)
            .enumerate()
        {
            let core_num = offset + i;
            let label = format!("Core {core_num}");
            let line = gauge_line(&label, usage, inner.width as usize);
            frame.render_widget(line, rows[row_idx]);
            row_idx += 1;
        }
    }

    // ── Scroll-down indicator ───────────────────────────────────────────
    if show_down {
        let shown_up_to = actual_offset + visible_cores;
        let hidden_below = total_cores - shown_up_to;
        let indicator = Line::from(vec![
            Span::styled(
                format!("          ▼ {hidden_below} more below "),
                Style::default().fg(TEXT_SECONDARY),
            ),
            Span::styled(
                "press ']' to scroll down",
                Style::default().fg(ACCENT).add_modifier(Modifier::DIM),
            ),
        ]);
        frame.render_widget(indicator, rows[row_idx]);
        row_idx += 1;
    }

    // Skip blank separator
    if show_per_core && (visible_cores > 0 || show_up || show_down) {
        row_idx += 1;
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
