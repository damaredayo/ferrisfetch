use crate::State;

impl State {
    pub fn fetch_locale(&self) -> String {
       std::env::var("LANG").unwrap_or(String::from(""))
    }
}