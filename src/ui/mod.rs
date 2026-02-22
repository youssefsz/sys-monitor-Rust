use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};

use crate::app::App;

mod cpu;
mod header;
mod help;
mod memory;
mod processes;

/// Renders the entire dashboard for a single frame.
pub fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // ── Main layout: header, mid panels, process table ──────────────────
    let [header_area, mid_area, proc_area] = Layout::vertical([
        Constraint::Length(3),      // header bar
        Constraint::Percentage(40), // CPU + Memory panels
        Constraint::Min(10),        // process table fills remainder
    ])
    .areas(area);

    // ── Mid: CPU (left) | Memory (right) ────────────────────────────────
    let [cpu_area, mem_area] =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .areas(mid_area);

    // ── Draw widgets ────────────────────────────────────────────────────
    header::draw(frame, &app.data.host, header_area);
    cpu::draw(
        frame,
        &app.data.cpu,
        app.show_per_core,
        &mut app.core_scroll_offset,
        cpu_area,
    );
    memory::draw(frame, &app.data.memory, mem_area);
    processes::draw(frame, app, proc_area);

    // ── Help overlay (on top if active) ─────────────────────────────────
    if app.show_help {
        help::draw(frame, area);
    }
}
