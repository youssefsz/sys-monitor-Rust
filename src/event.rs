use std::time::Duration;

use crossterm::event::{self, Event as CtEvent, KeyCode, KeyEvent, KeyModifiers};

/// Application-level events produced by the event poller.
#[allow(dead_code)]
pub enum AppEvent {
    /// A keyboard key was pressed.
    Key(KeyEvent),
    /// A periodic tick fired (time to refresh data).
    Tick,
    /// The terminal was resized.
    Resize(u16, u16),
}

/// Polls the terminal for events with a given tick rate.
///
/// Returns `Some(event)` if an event occurred within the tick window,
/// or `AppEvent::Tick` when the timeout expires with no input.
pub fn poll(tick_rate: Duration) -> std::io::Result<AppEvent> {
    if event::poll(tick_rate)? {
        match event::read()? {
            CtEvent::Key(key) => Ok(AppEvent::Key(key)),
            CtEvent::Resize(w, h) => Ok(AppEvent::Resize(w, h)),
            // Ignore mouse / focus / paste events
            _ => Ok(AppEvent::Tick),
        }
    } else {
        Ok(AppEvent::Tick)
    }
}

/// Returns `true` if the key event represents a quit action.
pub fn is_quit(key: &KeyEvent) -> bool {
    matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
}
