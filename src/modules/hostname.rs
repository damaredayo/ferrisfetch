use crate::State;

impl State {
    pub fn fetch_hostname(&self) -> String {
        whoami::fallible::hostname().unwrap_or(String::from("hostname"))
    }

    pub fn fetch_username(&self) -> String {
        whoami::username()
    }

    pub fn fetch_username_and_hostname(&self) -> String {
        format!("{}@{}", self.fetch_username(), self.fetch_hostname())
    }
}
