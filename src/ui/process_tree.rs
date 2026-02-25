use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};

use crate::app::App;
use crate::theme::Theme;
use crate::theme::colors::{ACCENT, TEXT_DIM, TEXT_PRIMARY, TEXT_SECONDARY};

/// Draws the full-screen process tree overlay with selectable nodes.
pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    // Clear the entire area
    frame.render_widget(Clear, area);

    let title = format!(
        " Process Tree — {} (PID {}) ",
        app.tree_origin_name, app.tree_origin_pid
    );

    let block = Block::default()
        .title(Span::styled(title, Theme::title()))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT))
        .padding(Padding::new(2, 2, 1, 1));

    if app.tree_nodes.is_empty() {
        let content = Paragraph::new(Line::from(Span::styled(
            "No tree data available.",
            Style::default().fg(TEXT_DIM),
        )))
        .block(block);
        frame.render_widget(content, area);
        return;
    }

    // Calculate visible area (borders=2, top pad=1, bottom pad=1, footer=1)
    let inner_height = area.height.saturating_sub(6) as usize;
    let total_nodes = app.tree_nodes.len();

    // Auto-scroll to keep selection visible
    let scroll = if inner_height == 0 {
        0
    } else if app.tree_selected < app.tree_scroll {
        app.tree_selected
    } else if app.tree_selected >= app.tree_scroll + inner_height {
        app.tree_selected - inner_height + 1
    } else {
        app.tree_scroll
    };

    let mut lines: Vec<Line> = Vec::with_capacity(inner_height + 2);

    // Render visible tree nodes
    let visible_end = (scroll + inner_height).min(total_nodes);
    for i in scroll..visible_end {
        let node = &app.tree_nodes[i];
        let indent = "  ".repeat(node.depth as usize);
        let is_selected = i == app.tree_selected;

        let (marker, style) = if is_selected {
            // Selected node — highlighted
            (
                "▸",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            )
        } else if node.is_target {
            // Original target process — subtle highlight
            ("●", Style::default().fg(ACCENT))
        } else {
            ("├", Style::default().fg(TEXT_PRIMARY))
        };

        lines.push(Line::from(Span::styled(
            format!("{indent}{marker} {} ({})", node.name, node.pid),
            style,
        )));
    }

    // Pad remaining space
    while lines.len() < inner_height {
        lines.push(Line::from(""));
    }

    // Footer hints
    lines.push(Line::from(""));
    let mut hints = vec![
        Span::styled("↑/↓", Style::default().fg(ACCENT)),
        Span::styled(" navigate  ", Style::default().fg(TEXT_SECONDARY)),
        Span::styled("Enter", Style::default().fg(ACCENT)),
        Span::styled(" actions  ", Style::default().fg(TEXT_SECONDARY)),
        Span::styled("Esc", Style::default().fg(ACCENT)),
        Span::styled(" close", Style::default().fg(TEXT_SECONDARY)),
    ];

    if total_nodes > inner_height {
        hints.push(Span::styled(
            format!("  [{}/{}]", app.tree_selected + 1, total_nodes),
            Style::default().fg(TEXT_DIM),
        ));
    }

    lines.push(Line::from(hints));

    let content = Paragraph::new(lines).block(block);
    frame.render_widget(content, area);
}
