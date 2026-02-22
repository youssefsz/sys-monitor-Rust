pub mod cpu;
pub mod host;
pub mod memory;
pub mod process;

use sysinfo::{CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, RefreshKind, System};

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
}
