mod app;
mod event;
mod system;
mod theme;
mod ui;
mod util;

use std::io;
use std::process::Command;
use std::time::Duration;

use clap::{Parser, Subcommand};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::App;
use event::{AppEvent, is_quit};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTALL_URL: &str =
    "https://raw.githubusercontent.com/youssefsz/sys-monitor-Rust/master/install.sh";

// ── CLI definition ──────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "sys-monitor",
    about = "A modern, elegant terminal system monitor",
    version = VERSION,
    author,
)]
struct Cli {
    /// Refresh rate in milliseconds (default: 250)
    #[arg(short, long, default_value_t = 250)]
    refresh_rate: u64,

    /// Start with per-core CPU view disabled
    #[arg(long)]
    no_per_core: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Self-update to the latest version
    Upgrade,
}

// ── Main ────────────────────────────────────────────────────────────────

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // Handle subcommands before entering TUI mode
    if let Some(Commands::Upgrade) = cli.command {
        return self_upgrade();
    }

    let tick_rate = Duration::from_millis(cli.refresh_rate);

    // ── Setup terminal ──────────────────────────────────────────────────
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // ── Run application ─────────────────────────────────────────────────
    let result = run(&mut terminal, tick_rate, cli.no_per_core);

    // ── Restore terminal (always, even on error) ────────────────────────
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

/// Main event loop — polls events and redraws the UI.
fn run(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    tick_rate: Duration,
    no_per_core: bool,
) -> io::Result<()> {
    let mut app = App::new();

    if no_per_core {
        app.show_per_core = false;
    }

    while app.running {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        match event::poll(tick_rate)? {
            AppEvent::Key(key) => {
                if is_quit(&key) && !app.filter_mode && !app.process_menu.visible {
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

/// Downloads and runs the install script to self-update.
fn self_upgrade() -> io::Result<()> {
    println!("⬆ Upgrading sys-monitor...");
    println!("  Current version: v{VERSION}");
    println!();

    let status = Command::new("bash")
        .args(["-c", &format!("curl -sSL {INSTALL_URL} | bash")])
        .status()?;

    if status.success() {
        println!();
        println!("✓ Upgrade complete!");
    } else {
        eprintln!("✗ Upgrade failed. Try manually:");
        eprintln!("  curl -sSL {INSTALL_URL} | bash");
    }

    Ok(())
}
