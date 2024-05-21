use crate::State;

impl State {
    pub fn fetch_hostname(&self) -> String {
        const HOSTNAME_PATH: &str = "/proc/sys/kernel/hostname";
        std::fs::read_to_string(HOSTNAME_PATH)
            .unwrap_or_else(|_| "Unknown".to_string())
            .trim()
            .to_string()
    }

    pub fn fetch_username(&self) -> String {
        whoami::username()
    }

    pub fn fetch_username_and_hostname(&self) -> String {
        format!("{}@{}", self.fetch_username(), self.fetch_hostname())
    }
}
