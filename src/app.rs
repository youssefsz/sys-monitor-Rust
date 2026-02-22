use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::TableState;

use crate::system::process::SortColumn;
use crate::system::{SystemCollector, SystemData};

/// How often system data is refreshed.
const DATA_REFRESH_INTERVAL: Duration = Duration::from_secs(1);

/// Maximum number of process rows to collect.
const MAX_PROCESS_ROWS: usize = 100;

/// Central application state — owns data, selection, and UI modes.
pub struct App {
    pub running: bool,
    pub data: SystemData,
    pub table_state: TableState,
    pub sort_column: SortColumn,
    pub sort_ascending: bool,
    pub show_help: bool,
    pub show_per_core: bool,
    pub filter_mode: bool,
    pub filter_text: String,

    collector: SystemCollector,
    last_refresh: Instant,
}

impl App {
    /// Creates a new App and performs the initial data collection.
    pub fn new() -> Self {
        let mut collector = SystemCollector::new();

        // sysinfo needs two refreshes to get meaningful CPU data.
        let _ = collector.refresh(SortColumn::Cpu, false, None, MAX_PROCESS_ROWS);
        std::thread::sleep(Duration::from_millis(200));
        let data = collector.refresh(SortColumn::Cpu, false, None, MAX_PROCESS_ROWS);

        let mut table_state = TableState::default();
        if !data.processes.is_empty() {
            table_state.select(Some(0));
        }

        Self {
            running: true,
            data,
            table_state,
            sort_column: SortColumn::Cpu,
            sort_ascending: false,
            show_help: false,
            show_per_core: true,
            filter_mode: false,
            filter_text: String::new(),
            collector,
            last_refresh: Instant::now(),
        }
    }

    // ── Tick ─────────────────────────────────────────────────────────────

    /// Called on every tick — refreshes data if enough time has passed.
    pub fn tick(&mut self) {
        if self.last_refresh.elapsed() >= DATA_REFRESH_INTERVAL {
            let filter = if self.filter_text.is_empty() {
                None
            } else {
                Some(self.filter_text.as_str())
            };

            self.data = self.collector.refresh(
                self.sort_column,
                self.sort_ascending,
                filter,
                MAX_PROCESS_ROWS,
            );

            // Keep selection in bounds
            let len = self.data.processes.len();
            if let Some(i) = self.table_state.selected() {
                if i >= len && len > 0 {
                    self.table_state.select(Some(len - 1));
                }
            }

            self.last_refresh = Instant::now();
        }
    }

    // ── Input handling ──────────────────────────────────────────────────

    /// Processes a key event and updates app state.
    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.filter_mode {
            self.handle_filter_key(key);
            return;
        }

        if self.show_help {
            // Any key dismisses help
            self.show_help = false;
            return;
        }

        match key.code {
            // Navigation
            KeyCode::Down | KeyCode::Char('j') => self.select_next(),
            KeyCode::Up | KeyCode::Char('k') => self.select_prev(),
            KeyCode::Home | KeyCode::Char('g') => self.select_first(),
            KeyCode::End | KeyCode::Char('G') => self.select_last(),

            // Sort
            KeyCode::Char('s') => {
                self.sort_column = self.sort_column.next();
                self.force_refresh();
            }
            KeyCode::Char('S') => {
                self.sort_ascending = !self.sort_ascending;
                self.force_refresh();
            }

            // Toggle per-core view
            KeyCode::Char('1') => self.show_per_core = !self.show_per_core,

            // Filter
            KeyCode::Char('/') => {
                self.filter_mode = true;
            }

            // Help
            KeyCode::Char('?') => self.show_help = true,

            // Force refresh
            KeyCode::Char('r') => self.force_refresh(),

            _ => {}
        }
    }

    fn handle_filter_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.filter_mode = false;
                self.filter_text.clear();
                self.force_refresh();
            }
            KeyCode::Enter => {
                self.filter_mode = false;
                self.force_refresh();
            }
            KeyCode::Backspace => {
                self.filter_text.pop();
                self.force_refresh();
            }
            KeyCode::Char(c) => {
                self.filter_text.push(c);
                self.force_refresh();
            }
            _ => {}
        }
    }

    // ── Selection helpers ───────────────────────────────────────────────

    fn select_next(&mut self) {
        let len = self.data.processes.len();
        if len == 0 {
            return;
        }
        let i = self
            .table_state
            .selected()
            .map_or(0, |i| (i + 1).min(len - 1));
        self.table_state.select(Some(i));
    }

    fn select_prev(&mut self) {
        let len = self.data.processes.len();
        if len == 0 {
            return;
        }
        let i = self
            .table_state
            .selected()
            .map_or(0, |i| i.saturating_sub(1));
        self.table_state.select(Some(i));
    }

    fn select_first(&mut self) {
        if !self.data.processes.is_empty() {
            self.table_state.select(Some(0));
        }
    }

    fn select_last(&mut self) {
        let len = self.data.processes.len();
        if len > 0 {
            self.table_state.select(Some(len - 1));
        }
    }

    /// Forces an immediate data refresh on the next tick.
    fn force_refresh(&mut self) {
        self.last_refresh = Instant::now() - DATA_REFRESH_INTERVAL;
    }
}
