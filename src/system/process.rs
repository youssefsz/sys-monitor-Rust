use sysinfo::{Process, System};

/// A single row in the process table.
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f64,
    pub mem_bytes: u64,
    pub mem_percent: f64,
    pub status: String,
}

/// Which column the process table is sorted by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Cpu,
    Memory,
    Pid,
    Name,
}

#[allow(dead_code)]
impl SortColumn {
    /// Cycle to the next sort column.
    pub fn next(self) -> Self {
        match self {
            Self::Cpu => Self::Memory,
            Self::Memory => Self::Pid,
            Self::Pid => Self::Name,
            Self::Name => Self::Cpu,
        }
    }

    /// Label shown in the table header.
    pub fn label(self) -> &'static str {
        match self {
            Self::Cpu => "CPU%",
            Self::Memory => "MEM%",
            Self::Pid => "PID",
            Self::Name => "Name",
        }
    }
}

/// Collects and sorts the process list from a pre-refreshed [`System`].
pub fn collect_processes(
    sys: &System,
    sort_col: SortColumn,
    sort_ascending: bool,
    filter: Option<&str>,
    max_rows: usize,
) -> Vec<ProcessInfo> {
    let total_mem = sys.total_memory() as f64;

    let mut procs: Vec<ProcessInfo> = sys
        .processes()
        .values()
        .filter(|p| process_is_visible(p))
        .filter(|p| {
            filter
                .map(|f| {
                    p.name()
                        .to_string_lossy()
                        .to_lowercase()
                        .contains(&f.to_lowercase())
                })
                .unwrap_or(true)
        })
        .map(|p| {
            let mem = p.memory();
            ProcessInfo {
                pid: p.pid().as_u32(),
                name: p.name().to_string_lossy().to_string(),
                cpu_percent: p.cpu_usage() as f64,
                mem_bytes: mem,
                mem_percent: if total_mem > 0.0 {
                    (mem as f64 / total_mem) * 100.0
                } else {
                    0.0
                },
                status: format!("{:?}", p.status()),
            }
        })
        .collect();

    // Sort
    procs.sort_by(|a, b| {
        let cmp = match sort_col {
            SortColumn::Cpu => b.cpu_percent.partial_cmp(&a.cpu_percent).unwrap(),
            SortColumn::Memory => b.mem_percent.partial_cmp(&a.mem_percent).unwrap(),
            SortColumn::Pid => a.pid.cmp(&b.pid),
            SortColumn::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        };
        if sort_ascending { cmp.reverse() } else { cmp }
    });

    procs.truncate(max_rows);
    procs
}

/// Filters out kernel/system idle processes with zero CPU and memory.
fn process_is_visible(p: &Process) -> bool {
    p.cpu_usage() > 0.0 || p.memory() > 0
}
