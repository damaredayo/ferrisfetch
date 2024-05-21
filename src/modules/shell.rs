use crate::State;

impl State {
    pub fn fetch_shell(&self) -> String {
        get_shell_info().unwrap_or_else(|e| e)
    }
}

#[cfg(target_os = "linux")]
fn get_shell_info() -> Result<String, String> {
    if let Ok(shell) = std::env::var("SHELL") {
        return Ok(shell.split('/').last().unwrap_or(&shell).to_string());
    }

    Err("Unable to detect shell on Linux".to_string())
}
