use super::{Platform, detect_platform};

pub struct SystemInfo {
    pub platform: Platform,
    pub os: String,
    pub os_version: String,
    pub architecture: String,
    pub cpu_cores: usize,
    pub total_memory: u64,
    pub app_name: String,
    pub app_version: String,
}

impl SystemInfo {
    pub fn new() -> Self {
        Self {
            platform: detect_platform(),
            os: std::env::consts::OS.to_string(),
            os_version: Self::get_os_version(),
            architecture: std::env::consts::ARCH.to_string(),
            cpu_cores: num_cpus::get(),
            total_memory: Self::get_total_memory(),
            app_name: "Optima".to_string(),
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    fn get_os_version() -> String {
        #[cfg(target_os = "android")]
        {
            "Android".to_string()
        }
        #[cfg(not(target_os = "android"))]
        {
            std::env::consts::OS.to_string()
        }
    }

    fn get_total_memory() -> u64 {
        #[cfg(target_os = "android")]
        {
            use std::fs;
            if let Ok(content) = fs::read_to_string("/proc/meminfo") {
                for line in content.lines() {
                    if line.starts_with("MemTotal:") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            if let Ok(kb) = parts[1].parse::<u64>() {
                                return kb * 1024;
                            }
                        }
                    }
                }
            }
            0
        }
        #[cfg(not(target_os = "android"))]
        {
            sys_info::mem_info().map(|m| m.total * 1024).unwrap_or(0)
        }
    }

    pub fn is_android(&self) -> bool {
        self.platform == Platform::Android
    }

    pub fn is_desktop(&self) -> bool {
        self.platform == Platform::Desktop
    }
}
