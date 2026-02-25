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
}
