use sysinfo::System;

/// Aggregated CPU metrics.
#[derive(Debug, Clone)]
pub struct CpuData {
    /// Overall CPU usage as a percentage (0–100).
    pub overall_percent: f64,
    /// Per-core usage percentages.
    pub per_core: Vec<f64>,
    /// Physical core count.
    pub physical_cores: usize,
    /// Total number of running processes.
    pub process_count: usize,
}

impl CpuData {
    /// Samples CPU data from a pre-refreshed [`System`].
    pub fn collect(sys: &System) -> Self {
        let cpus = sys.cpus();
        let per_core: Vec<f64> = cpus.iter().map(|c| c.cpu_usage() as f64).collect();

        let overall = if per_core.is_empty() {
            0.0
        } else {
            per_core.iter().sum::<f64>() / per_core.len() as f64
        };

        Self {
            overall_percent: overall,
            per_core,
            physical_cores: System::physical_core_count().unwrap_or(0),
            process_count: sys.processes().len(),
        }
    }
}
