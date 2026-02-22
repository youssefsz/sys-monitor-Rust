use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding};

use crate::system::host::HostInfo;
use crate::theme::Theme;
use crate::theme::colors;
use crate::util::format::format_uptime;

/// Draws the top header bar with hostname, OS, uptime, and load average.
pub fn draw(frame: &mut Frame, host: &HostInfo, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .border_style(Theme::border())
        .title(Span::styled(" sys-monitor ", Theme::title()))
        .padding(Padding::horizontal(1));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Split inner area into four columns
    let cols = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(inner);

    // Hostname
    let hostname_line = Line::from(vec![
        Span::styled("◉ ", Theme::healthy()),
        Span::styled(&host.hostname, Theme::value()),
    ]);
    frame.render_widget(hostname_line, cols[0]);

    // OS
    let os_line = Line::from(vec![
        Span::styled(&host.os_name, Theme::label()),
        Span::styled(" ", Theme::dim()),
        Span::styled(&host.os_version, Theme::value()),
    ]);
    frame.render_widget(os_line, cols[1]);

    // Uptime
    let uptime_str = format_uptime(host.uptime_secs);
    let uptime_line = Line::from(vec![
        Span::styled("↑ ", Theme::label()),
        Span::styled(uptime_str, Theme::value()),
    ]);
    frame.render_widget(uptime_line, cols[2]);

    // Load average
    let load_color = colors::severity_color(host.load_avg_one * 25.0); // rough heuristic
    let load_line = Line::from(vec![
        Span::styled("⎔ ", Theme::label()),
        Span::styled(
            format!(
                "{:.2}  {:.2}  {:.2}",
                host.load_avg_one, host.load_avg_five, host.load_avg_fifteen
            ),
            ratatui::style::Style::default().fg(load_color),
        ),
    ]);
    frame.render_widget(load_line, cols[3]);
}
