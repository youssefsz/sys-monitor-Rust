pub mod cpu;
pub mod host;
pub mod memory;
pub mod process;

use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, Pid, ProcessRefreshKind, RefreshKind, Signal, System,
};

use self::cpu::CpuData;
use self::host::HostInfo;
use self::memory::MemoryData;
use self::process::{ProcessInfo, SortColumn};

/// All system metrics the UI needs to render a single frame.
#[derive(Debug, Clone)]
pub struct SystemData {
    pub host: HostInfo,
    pub cpu: CpuData,
    pub memory: MemoryData,
    pub processes: Vec<ProcessInfo>,
}

/// Result of a process kill attempt.
pub enum KillResult {
    /// Signal was sent successfully.
    Success,
    /// The process was not found (may have already exited).
    NotFound,
    /// The signal is not supported on this platform.
    Unsupported,
    /// The signal could not be sent (e.g. permission denied).
    Failed,
}

/// Owns the `sysinfo::System` instance and provides a clean refresh API.
pub struct SystemCollector {
    sys: System,
}

impl SystemCollector {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
                .with_processes(ProcessRefreshKind::everything()),
        );
        Self { sys }
    }

    /// Refreshes all metrics and returns a snapshot.
    pub fn refresh(
        &mut self,
        sort_col: SortColumn,
        sort_ascending: bool,
        filter: Option<&str>,
        max_process_rows: usize,
    ) -> SystemData {
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys
            .refresh_processes(sysinfo::ProcessesToUpdate::All, true);

        SystemData {
            host: HostInfo::collect(),
            cpu: CpuData::collect(&self.sys),
            memory: MemoryData::collect(&self.sys),
            processes: process::collect_processes(
                &self.sys,
                sort_col,
                sort_ascending,
                filter,
                max_process_rows,
            ),
        }
    }

    /// Sends a graceful termination signal (`SIGTERM` on Unix, `TerminateProcess`
    /// on Windows) to the process with the given PID.
    pub fn terminate_process(&mut self, pid: u32) -> KillResult {
        let sysinfo_pid = Pid::from_u32(pid);
        match self.sys.process(sysinfo_pid) {
            Some(proc) => match proc.kill_with(Signal::Term) {
                Some(true) => KillResult::Success,
                Some(false) => KillResult::Failed,
                None => KillResult::Unsupported,
            },
            None => KillResult::NotFound,
        }
    }

    /// Sends a forceful kill signal (`SIGKILL` on Unix, `TerminateProcess` on
    /// Windows) to the process with the given PID. This is guaranteed to be
    /// supported on all platforms.
    pub fn force_kill_process(&mut self, pid: u32) -> KillResult {
        let sysinfo_pid = Pid::from_u32(pid);
        match self.sys.process(sysinfo_pid) {
            Some(proc) => {
                if proc.kill() {
                    KillResult::Success
                } else {
                    KillResult::Failed
                }
            }
            None => KillResult::NotFound,
        }
    }

    /// Builds a process tree (ancestors + children) for the given PID.
    ///
    /// Returns `(pid, name, depth)` tuples where depth 0 is the target process,
    /// negative depths are ancestors, and positive depths are children.
    pub fn process_tree(&self, target_pid: u32) -> Vec<(u32, String, i32)> {
        let mut result: Vec<(u32, String, i32)> = Vec::new();

        // ── Walk ancestors upward ───────────────────────────────────────
        let mut ancestors: Vec<(u32, String)> = Vec::new();
        let mut current = Pid::from_u32(target_pid);
        while let Some(proc) = self.sys.process(current) {
            if let Some(parent_pid) = proc.parent() {
                if let Some(parent) = self.sys.process(parent_pid) {
                    ancestors.push((
                        parent_pid.as_u32(),
                        parent.name().to_string_lossy().to_string(),
                    ));
                    current = parent_pid;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Add ancestors top-down (root first)
        let ancestor_count = ancestors.len() as i32;
        for (i, (pid, name)) in ancestors.into_iter().rev().enumerate() {
            result.push((pid, name, -(ancestor_count - i as i32)));
        }

        // ── Add target process at depth 0 ───────────────────────────────
        if let Some(proc) = self.sys.process(Pid::from_u32(target_pid)) {
            result.push((target_pid, proc.name().to_string_lossy().to_string(), 0));
        }

        // ── Walk children downward (BFS) ────────────────────────────────
        let mut queue: Vec<(u32, i32)> = vec![(target_pid, 0)];
        while let Some((parent, parent_depth)) = queue.pop() {
            let parent_sysinfo = Pid::from_u32(parent);
            for (child_pid, child_proc) in self.sys.processes() {
                if child_proc.parent() == Some(parent_sysinfo) {
                    let child_depth = parent_depth + 1;
                    result.push((
                        child_pid.as_u32(),
                        child_proc.name().to_string_lossy().to_string(),
                        child_depth,
                    ));
                    queue.push((child_pid.as_u32(), child_depth));
                }
            }
        }

        result
    }
}
