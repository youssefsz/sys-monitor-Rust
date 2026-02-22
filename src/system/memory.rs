use sysinfo::System;

/// Memory metrics — RAM and swap.
#[derive(Debug, Clone)]
pub struct MemoryData {
    pub total_mem: u64,
    pub used_mem: u64,
    pub available_mem: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

impl MemoryData {
    /// Samples memory data from a pre-refreshed [`System`].
    pub fn collect(sys: &System) -> Self {
        Self {
            total_mem: sys.total_memory(),
            used_mem: sys.used_memory(),
            available_mem: sys.available_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
        }
    }

    /// RAM usage as a percentage (0–100).
    pub fn ram_percent(&self) -> f64 {
        if self.total_mem == 0 {
            return 0.0;
        }
        (self.used_mem as f64 / self.total_mem as f64) * 100.0
    }

    /// Swap usage as a percentage (0–100).
    pub fn swap_percent(&self) -> f64 {
        if self.total_swap == 0 {
            return 0.0;
        }
        (self.used_swap as f64 / self.total_swap as f64) * 100.0
    }
}
