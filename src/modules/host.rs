use std::fs;

use crate::State;

impl State {
    #[cfg(target_os = "linux")]
    pub fn fetch_host(&self) -> String {
        const BASE_PATH: &str = "/sys/class/dmi/id/";
        let name = fs::read_to_string(format!("{}{}", BASE_PATH, "product_name"))
            .unwrap_or_else(|_| "Unknown".to_string())
            .trim()
            .to_string();
        let version = fs::read_to_string(format!("{}{}", BASE_PATH, "board_version"))
            .unwrap_or_else(|_| "0.0".to_string())
            .trim()
            .to_string();

        format!("{} ({})", name, version)
    }

    #[cfg(not(target_os = "linux"))]
    pub fn fetch_host(&self) -> String {
        String::new()
    }
}
