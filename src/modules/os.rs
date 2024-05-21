use crate::State;

impl State {
    pub fn fetch_os(&self) -> String {
        sysinfo::System::name().unwrap_or(String::from("Unknown"))
    }
}
