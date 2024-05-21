use crate::State;

impl State {
    pub fn fetch_kernel(&self) -> String {
        format!(
            "Linux {}",
            sysinfo::System::kernel_version().unwrap_or(String::from("Unknown"))
        )
    }
}
