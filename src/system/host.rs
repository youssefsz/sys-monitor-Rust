use sysinfo::System;

/// Host-level information: hostname, OS, uptime.
#[derive(Debug, Clone)]
pub struct HostInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub uptime_secs: u64,
    pub load_avg_one: f64,
    pub load_avg_five: f64,
    pub load_avg_fifteen: f64,
}

impl HostInfo {
    pub fn collect() -> Self {
        let load = System::load_average();

        Self {
            hostname: System::host_name().unwrap_or_else(|| "unknown".into()),
            os_name: System::name().unwrap_or_else(|| "macOS".into()),
            os_version: System::os_version().unwrap_or_else(|| "?".into()),
            uptime_secs: System::uptime(),
            load_avg_one: load.one,
            load_avg_five: load.five,
            load_avg_fifteen: load.fifteen,
        }
    }
}
