use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};

use crate::app::{App, ProcessAction};
use crate::theme::Theme;
use crate::theme::colors::{ACCENT, CRITICAL, HEALTHY, TEXT_DIM, TEXT_PRIMARY, TEXT_SECONDARY};

/// Draws the process action popup menu centered on the screen.
pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let menu = &app.process_menu;

    // ── Popup dimensions ────────────────────────────────────────────────
    let width = 44u16.min(area.width.saturating_sub(4));
    let height = if menu.feedback.is_some() {
        8
    } else if menu.confirm_mode {
        9
    } else {
        10
    }
    .min(area.height.saturating_sub(4));

    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    let popup = Rect::new(x, y, width, height);

    // Clear background behind popup
    frame.render_widget(Clear, popup);

    // ── Title ────────────────────────────────────────────────────────────
    let title = format!(" {} (PID {}) ", menu.target_name, menu.target_pid);

    let block = Block::default()
        .title(Span::styled(title, Theme::title()))
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Style::default().fg(ACCENT))
        .padding(Padding::new(2, 2, 1, 0));

    // ── Content ──────────────────────────────────────────────────────────
    if let Some(ref feedback) = menu.feedback {
        // Show feedback message
        let color = if feedback.starts_with('✓') {
            HEALTHY
        } else {
            CRITICAL
        };
        let lines = vec![
            Line::from(Span::styled(feedback.as_str(), Style::default().fg(color))),
            Line::from(""),
            Line::from(Span::styled(
                "Press any key to close",
                Style::default().fg(TEXT_DIM),
            )),
        ];
        let content = Paragraph::new(lines).block(block);
        frame.render_widget(content, popup);
        return;
    }

    if menu.confirm_mode {
        // Show confirmation prompt
        let action_label = match menu.current_action() {
            ProcessAction::Kill => "Kill",
            ProcessAction::ForceKill => "Force kill",
            ProcessAction::CopyPid => unreachable!(),
        };

        let lines = vec![
            Line::from(Span::styled(
                format!("{action_label} \"{}\"?", menu.target_name),
                Style::default().fg(CRITICAL).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Enter/y", Style::default().fg(ACCENT)),
                Span::styled(" confirm   ", Style::default().fg(TEXT_SECONDARY)),
                Span::styled("Esc/n", Style::default().fg(ACCENT)),
                Span::styled(" cancel", Style::default().fg(TEXT_SECONDARY)),
            ]),
        ];
        let content = Paragraph::new(lines).block(block);
        frame.render_widget(content, popup);
        return;
    }

    // ── Action list ──────────────────────────────────────────────────────
    let mut lines: Vec<Line> = Vec::with_capacity(ProcessAction::ALL.len() + 2);

    for (i, action) in ProcessAction::ALL.iter().enumerate() {
        let is_selected = i == menu.selected;
        let (prefix, style) = if is_selected {
            (
                "▸ ",
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            )
        } else {
            ("  ", Style::default().fg(TEXT_PRIMARY))
        };

        lines.push(Line::from(Span::styled(
            format!("{prefix}{}", action.label()),
            style,
        )));
    }

    // Footer hints
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("↑/↓", Style::default().fg(ACCENT)),
        Span::styled(" navigate  ", Style::default().fg(TEXT_SECONDARY)),
        Span::styled("Enter", Style::default().fg(ACCENT)),
        Span::styled(" select  ", Style::default().fg(TEXT_SECONDARY)),
        Span::styled("Esc", Style::default().fg(ACCENT)),
        Span::styled(" close", Style::default().fg(TEXT_SECONDARY)),
    ]));

    let content = Paragraph::new(lines).block(block);
    frame.render_widget(content, popup);
}
