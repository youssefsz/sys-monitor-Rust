use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};

use crate::theme::Theme;
use crate::theme::colors::{ACCENT, TEXT_PRIMARY, TEXT_SECONDARY};

/// Draws a centered help overlay listing all keyboard shortcuts.
pub fn draw(frame: &mut Frame, area: Rect) {
    // Size the popup: 50 wide, 18 tall, centered
    let width = 50u16.min(area.width.saturating_sub(4));
    let height = 20u16.min(area.height.saturating_sub(4));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let popup = Rect::new(x, y, width, height);

    // Clear the background behind the popup
    frame.render_widget(Clear, popup);

    let block = Block::default()
        .title(Span::styled(" Keyboard Shortcuts ", Theme::title()))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT))
        .padding(Padding::new(2, 2, 1, 1));

    let shortcuts = vec![
        ("q / Esc", "Quit"),
        ("↑ / k", "Move up"),
        ("↓ / j", "Move down"),
        ("g", "Go to first"),
        ("G", "Go to last"),
        ("s", "Cycle sort column"),
        ("S", "Reverse sort order"),
        ("/", "Filter processes"),
        ("1", "Toggle per-core view"),
        ("[ / ]", "Scroll cores"),
        ("r", "Force refresh"),
        ("?", "Toggle this help"),
    ];

    let lines: Vec<Line> = shortcuts
        .iter()
        .map(|(key, desc)| {
            Line::from(vec![
                Span::styled(
                    format!("{key:>12}"),
                    Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
                ),
                Span::styled("  │  ", Style::default().fg(TEXT_SECONDARY)),
                Span::styled(*desc, Style::default().fg(TEXT_PRIMARY)),
            ])
        })
        .collect();

    let help = Paragraph::new(lines).block(block);
    frame.render_widget(help, popup);
}
