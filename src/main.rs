mod app;
mod event;
mod system;
mod theme;
mod ui;
mod util;

use std::io;
use std::time::Duration;

use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::App;
use event::{AppEvent, is_quit};

/// Tick rate for the event loop (250ms = smooth UI updates).
const TICK_RATE: Duration = Duration::from_millis(250);

fn main() -> io::Result<()> {
    // ── Setup terminal ──────────────────────────────────────────────────
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // ── Run application ─────────────────────────────────────────────────
    let result = run(&mut terminal);

    // ── Restore terminal (always, even on error) ────────────────────────
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Propagate any error from the main loop
    result
}

/// Main event loop — polls events and redraws the UI.
fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut app = App::new();

    while app.running {
        // Draw
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        // Event handling
        match event::poll(TICK_RATE)? {
            AppEvent::Key(key) => {
                if is_quit(&key) && !app.filter_mode {
                    app.running = false;
                } else {
                    app.handle_key(key);
                }
            }
            AppEvent::Tick => {
                app.tick();
            }
            AppEvent::Resize(_, _) => {
                // Ratatui handles resize automatically on next draw
            }
        }
    }

    Ok(())
}
