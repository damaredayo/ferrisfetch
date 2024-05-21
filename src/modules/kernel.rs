use crate::State;

impl State {
    pub fn fetch_kernel(&self) -> String {
        #[cfg(target_os = "linux")]
        return format!(
            "Linux {} {}",
            sysinfo::System::kernel_version().unwrap_or(String::from("Unknown")),
            std::env::consts::ARCH
        );

        #[cfg(not(target_os = "linux"))]
        return format!(
            "{} {}",
            sysinfo::System::kernel_version().unwrap_or(String::from("Unknown")),
            std::env::consts::ARCH
        );
    }
}
