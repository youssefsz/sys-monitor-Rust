use ratatui::Frame;
use ratatui::layout::{Constraint, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Cell, Padding, Row, Table};

use crate::app::App;
use crate::system::process::SortColumn;
use crate::theme::Theme;
use crate::theme::colors;
use crate::util::format::{format_bytes, format_percent, truncate};

/// Draws the process table with sort indicators and a footer bar.
pub fn draw(frame: &mut Frame, app: &mut App, area: Rect) {
    let title = if app.filter_mode {
        format!(" PROCESSES ─ filter: {}▏", app.filter_text)
    } else if !app.filter_text.is_empty() {
        format!(" PROCESSES ─ \"{}\" ", app.filter_text)
    } else {
        " PROCESSES ".to_string()
    };

    let block = Block::default()
        .title(Span::styled(title, Theme::title()))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(if app.filter_mode {
            Theme::border_focused()
        } else {
            Theme::border()
        })
        .padding(Padding::new(1, 1, 0, 0));

    // ── Header row ──────────────────────────────────────────────────────
    let header_cells = [
        ("PID", SortColumn::Pid),
        ("Name", SortColumn::Name),
        ("CPU%", SortColumn::Cpu),
        ("MEM%", SortColumn::Memory),
        ("MEM", SortColumn::Memory),
        ("Status", SortColumn::Cpu), // not sortable, just placeholder
    ];

    let header = Row::new(header_cells.iter().map(|(label, col)| {
        let text = if *col == app.sort_column && *label != "MEM" && *label != "Status" {
            let arrow = if app.sort_ascending { "▲" } else { "▼" };
            format!("{label} {arrow}")
        } else {
            label.to_string()
        };
        Cell::from(text)
    }))
    .style(Theme::table_header())
    .height(1);

    // ── Data rows ───────────────────────────────────────────────────────
    let rows: Vec<Row> = app
        .data
        .processes
        .iter()
        .map(|p| {
            let cpu_color = colors::severity_color(p.cpu_percent);
            let mem_color = colors::severity_color(p.mem_percent);

            Row::new(vec![
                Cell::from(p.pid.to_string()).style(Theme::label()),
                Cell::from(truncate(&p.name, 25)).style(Theme::table_row()),
                Cell::from(format_percent(p.cpu_percent))
                    .style(Style::default().fg(cpu_color).add_modifier(Modifier::BOLD)),
                Cell::from(format_percent(p.mem_percent)).style(Style::default().fg(mem_color)),
                Cell::from(format_bytes(p.mem_bytes)).style(Theme::label()),
                Cell::from(p.status.clone()).style(Theme::label()),
            ])
        })
        .collect();

    // ── Column widths ───────────────────────────────────────────────────
    let widths = [
        Constraint::Length(8),  // PID
        Constraint::Min(15),    // Name (flexible)
        Constraint::Length(10), // CPU%
        Constraint::Length(10), // MEM%
        Constraint::Length(10), // MEM
        Constraint::Length(10), // Status
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(block)
        .row_highlight_style(Theme::table_selected());

    frame.render_stateful_widget(table, area, &mut app.table_state);

    // ── Footer hints (rendered over the bottom border) ───────────────────
    let hint = Line::from(vec![
        Span::styled(" ↑/↓", Theme::value()),
        Span::styled(" Navigate  ", Theme::label()),
        Span::styled("Enter", Theme::value()),
        Span::styled(" Actions  ", Theme::label()),
        Span::styled("s", Theme::value()),
        Span::styled(" Sort  ", Theme::label()),
        Span::styled("S", Theme::value()),
        Span::styled(" Reverse  ", Theme::label()),
        Span::styled("/", Theme::value()),
        Span::styled(" Filter  ", Theme::label()),
        Span::styled("?", Theme::value()),
        Span::styled(" Help  ", Theme::label()),
        Span::styled("q", Theme::value()),
        Span::styled(" Quit ", Theme::label()),
    ]);

    // Place hints on the last row of the area (the bottom border line)
    if area.height > 1 {
        let hint_area = Rect {
            x: area.x + 2,
            y: area.y + area.height - 1,
            width: area.width.saturating_sub(4),
            height: 1,
        };
        frame.render_widget(hint, hint_area);
    }
}
