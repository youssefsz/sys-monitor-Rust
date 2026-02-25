use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::TableState;

use crate::system::process::SortColumn;
use crate::system::{KillResult, SystemCollector, SystemData};

/// How often system data is refreshed.
const DATA_REFRESH_INTERVAL: Duration = Duration::from_secs(1);

/// Maximum number of process rows to collect.
const MAX_PROCESS_ROWS: usize = 100;

// ── Process Action Menu ─────────────────────────────────────────────────

/// Actions available in the process context menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessAction {
    Kill,
    ForceKill,
    CopyPid,
    OpenFileLocation,
    ProcessTree,
}

impl ProcessAction {
    /// All actions in display order.
    pub const ALL: [Self; 5] = [
        Self::Kill,
        Self::ForceKill,
        Self::CopyPid,
        Self::OpenFileLocation,
        Self::ProcessTree,
    ];

    /// Human-readable label shown in the menu.
    pub fn label(self) -> &'static str {
        match self {
            Self::Kill => "Kill Process  (SIGTERM)",
            Self::ForceKill => "Force Kill    (SIGKILL)",
            Self::CopyPid => "Copy PID",
            Self::OpenFileLocation => "Open File Location",
            Self::ProcessTree => "Process Tree",
        }
    }
}

/// State of the process action popup menu.
pub struct ProcessMenu {
    /// Whether the menu is currently visible.
    pub visible: bool,
    /// Index into `ProcessAction::ALL` for the highlighted action.
    pub selected: usize,
    /// PID of the target process.
    pub target_pid: u32,
    /// Name of the target process (for display).
    pub target_name: String,
    /// Whether awaiting kill confirmation.
    pub confirm_mode: bool,
    /// Feedback message shown after an action (e.g. "✓ Killed").
    pub feedback: Option<String>,
}

impl ProcessMenu {
    fn new() -> Self {
        Self {
            visible: false,
            selected: 0,
            target_pid: 0,
            target_name: String::new(),
            confirm_mode: false,
            feedback: None,
        }
    }

    /// Opens the menu for a specific process.
    fn open(&mut self, pid: u32, name: String) {
        self.visible = true;
        self.selected = 0;
        self.target_pid = pid;
        self.target_name = name;
        self.confirm_mode = false;
        self.feedback = None;
    }

    /// Closes the menu and resets all state.
    fn close(&mut self) {
        self.visible = false;
        self.confirm_mode = false;
        self.feedback = None;
    }

    /// Returns the currently highlighted action.
    pub fn current_action(&self) -> ProcessAction {
        ProcessAction::ALL[self.selected]
    }
}

// ── Process Tree Node ───────────────────────────────────────────────────

/// A single node in the process tree view.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub pid: u32,
    pub name: String,
    pub depth: i32,
    pub is_target: bool,
}

// ── Central App State ───────────────────────────────────────────────────

/// Central application state — owns data, selection, and UI modes.
pub struct App {
    pub running: bool,
    pub data: SystemData,
    pub table_state: TableState,
    pub sort_column: SortColumn,
    pub sort_ascending: bool,
    pub show_help: bool,
    pub show_per_core: bool,
    pub core_scroll_offset: usize,
    pub filter_mode: bool,
    pub filter_text: String,
    pub process_menu: ProcessMenu,

    /// Whether the full-screen process tree view is active.
    pub show_process_tree: bool,
    /// Tree nodes for the full-screen tree view.
    pub tree_nodes: Vec<TreeNode>,
    /// Currently selected index in tree_nodes.
    pub tree_selected: usize,
    /// Scroll offset for the tree view.
    pub tree_scroll: usize,
    /// The original target PID that opened the tree.
    pub tree_origin_pid: u32,
    /// The original target name that opened the tree.
    pub tree_origin_name: String,

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
            core_scroll_offset: 0,
            filter_mode: false,
            filter_text: String::new(),
            process_menu: ProcessMenu::new(),
            show_process_tree: false,
            tree_nodes: Vec::new(),
            tree_selected: 0,
            tree_scroll: 0,
            tree_origin_pid: 0,
            tree_origin_name: String::new(),
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
        // Full-screen tree view takes top priority.
        if self.show_process_tree {
            // If the menu is visible on top of the tree, handle menu keys.
            if self.process_menu.visible {
                self.handle_menu_key(key);
            } else {
                self.handle_tree_key(key);
            }
            return;
        }

        // Process menu takes priority when visible.
        if self.process_menu.visible {
            self.handle_menu_key(key);
            return;
        }

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

            // Open process action menu
            KeyCode::Enter => self.open_process_menu(),

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
            KeyCode::Char('1') => {
                self.show_per_core = !self.show_per_core;
                self.core_scroll_offset = 0;
            }

            // Scroll per-core view
            KeyCode::Char('[') => {
                if self.show_per_core && self.core_scroll_offset > 0 {
                    self.core_scroll_offset -= 1;
                }
            }
            KeyCode::Char(']') => {
                if self.show_per_core {
                    self.core_scroll_offset += 1;
                }
            }

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

    // ── Process menu ────────────────────────────────────────────────────

    /// Opens the process action menu for the currently selected process.
    fn open_process_menu(&mut self) {
        if let Some(idx) = self.table_state.selected() {
            if let Some(proc) = self.data.processes.get(idx) {
                self.process_menu.open(proc.pid, proc.name.clone());
            }
        }
    }

    /// Opens the process action menu for a specific PID and name.
    fn open_menu_for(&mut self, pid: u32, name: String) {
        self.process_menu.open(pid, name);
    }

    /// Handles key events while the process action menu is open.
    fn handle_menu_key(&mut self, key: KeyEvent) {
        // If showing feedback, any key closes the menu.
        if self.process_menu.feedback.is_some() {
            self.process_menu.close();
            if !self.show_process_tree {
                self.force_refresh();
            }
            return;
        }

        // Confirmation mode: waiting for y/Enter or n/Esc.
        if self.process_menu.confirm_mode {
            match key.code {
                KeyCode::Enter | KeyCode::Char('y') | KeyCode::Char('Y') => {
                    self.execute_kill();
                }
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('N') => {
                    self.process_menu.confirm_mode = false;
                }
                _ => {}
            }
            return;
        }

        // Normal menu navigation.
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                let count = ProcessAction::ALL.len();
                self.process_menu.selected = (self.process_menu.selected + 1) % count;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                let count = ProcessAction::ALL.len();
                self.process_menu.selected = (self.process_menu.selected + count - 1) % count;
            }
            KeyCode::Enter => self.execute_menu_action(),
            KeyCode::Esc => self.process_menu.close(),
            _ => {}
        }
    }

    /// Dispatches the currently selected menu action.
    fn execute_menu_action(&mut self) {
        match self.process_menu.current_action() {
            ProcessAction::Kill | ProcessAction::ForceKill => {
                self.process_menu.confirm_mode = true;
            }
            ProcessAction::CopyPid => {
                self.copy_pid_to_clipboard();
            }
            ProcessAction::OpenFileLocation => {
                self.open_file_location();
            }
            ProcessAction::ProcessTree => {
                self.enter_process_tree();
            }
        }
    }

    /// Performs the actual kill/force-kill after confirmation.
    fn execute_kill(&mut self) {
        let pid = self.process_menu.target_pid;
        let result = match self.process_menu.current_action() {
            ProcessAction::Kill => self.collector.terminate_process(pid),
            ProcessAction::ForceKill => self.collector.force_kill_process(pid),
            _ => unreachable!(),
        };

        let feedback = match result {
            KillResult::Success => "✓ Signal sent successfully".to_string(),
            KillResult::NotFound => "✗ Process not found (already exited?)".to_string(),
            KillResult::Unsupported => "✗ Signal not supported on this platform".to_string(),
            KillResult::Failed => "✗ Failed (permission denied?)".to_string(),
        };

        self.process_menu.confirm_mode = false;
        self.process_menu.feedback = Some(feedback);
    }

    /// Copies the target PID to the system clipboard.
    fn copy_pid_to_clipboard(&mut self) {
        let pid_str = self.process_menu.target_pid.to_string();
        let feedback = match arboard::Clipboard::new() {
            Ok(mut clipboard) => match clipboard.set_text(&pid_str) {
                Ok(()) => format!("✓ PID {} copied to clipboard", pid_str),
                Err(e) => format!("✗ Clipboard error: {e}"),
            },
            Err(e) => format!("✗ Clipboard unavailable: {e}"),
        };
        self.process_menu.feedback = Some(feedback);
    }

    /// Opens the file location of the target process in Finder (macOS).
    fn open_file_location(&mut self) {
        let pid = self.process_menu.target_pid;

        // Look up the exe_path from the current process list.
        let exe_path = self
            .data
            .processes
            .iter()
            .find(|p| p.pid == pid)
            .and_then(|p| p.exe_path.clone());

        let feedback = match exe_path {
            Some(path) => {
                match std::process::Command::new("open")
                    .arg("-R")
                    .arg(&path)
                    .spawn()
                {
                    Ok(_) => format!("✓ Revealed: {}", path),
                    Err(e) => format!("✗ Failed to open Finder: {e}"),
                }
            }
            None => "✗ Executable path not available".to_string(),
        };
        self.process_menu.feedback = Some(feedback);
    }

    // ── Process tree (full-screen) ──────────────────────────────────────

    /// Builds the process tree and enters the full-screen tree view.
    fn enter_process_tree(&mut self) {
        let pid = self.process_menu.target_pid;
        let name = self.process_menu.target_name.clone();
        let tree = self.collector.process_tree(pid);

        if tree.is_empty() {
            self.process_menu.feedback = Some("✗ Could not build process tree".to_string());
            return;
        }

        // Find the minimum depth to normalize indentation.
        let min_depth = tree.iter().map(|(_, _, d)| *d).min().unwrap_or(0);

        let nodes: Vec<TreeNode> = tree
            .iter()
            .map(|(p, n, depth)| TreeNode {
                pid: *p,
                name: n.clone(),
                depth: depth - min_depth,
                is_target: *depth == 0,
            })
            .collect();

        // Find the index of the target process to pre-select it.
        let target_idx = nodes.iter().position(|n| n.is_target).unwrap_or(0);

        // Close the popup menu and switch to full-screen tree.
        self.process_menu.close();
        self.tree_nodes = nodes;
        self.tree_selected = target_idx;
        self.tree_scroll = 0;
        self.tree_origin_pid = pid;
        self.tree_origin_name = name;
        self.show_process_tree = true;
    }

    /// Handles key events while the full-screen tree view is active.
    fn handle_tree_key(&mut self, key: KeyEvent) {
        match key.code {
            // Close tree view
            KeyCode::Esc | KeyCode::Char('q') => {
                self.show_process_tree = false;
                self.tree_nodes.clear();
                self.tree_selected = 0;
                self.tree_scroll = 0;
            }

            // Navigate selection
            KeyCode::Down | KeyCode::Char('j') => {
                if !self.tree_nodes.is_empty() {
                    self.tree_selected = (self.tree_selected + 1).min(self.tree_nodes.len() - 1);
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.tree_selected = self.tree_selected.saturating_sub(1);
            }
            KeyCode::Home | KeyCode::Char('g') => {
                self.tree_selected = 0;
            }
            KeyCode::End | KeyCode::Char('G') => {
                if !self.tree_nodes.is_empty() {
                    self.tree_selected = self.tree_nodes.len() - 1;
                }
            }

            // Open action menu for the selected tree node
            KeyCode::Enter => {
                if let Some(node) = self.tree_nodes.get(self.tree_selected) {
                    let pid = node.pid;
                    let name = node.name.clone();
                    self.open_menu_for(pid, name);
                }
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
