use crate::State;

fn format_duration(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    let mut result = String::new();

    if days > 0 {
        result.push_str(&format!("{} days, ", days));
    }
    if hours > 0 {
        result.push_str(&format!("{} hours, ", hours));
    }
    if minutes > 0 {
        result.push_str(&format!("{} minutes", minutes));
    }

    if result.ends_with(", ") {
        result.truncate(result.len() - 2);
    }

    result
}

impl State {
    pub fn fetch_uptime(&self) -> String {
        let uptime = sysinfo::System::uptime();
        format_duration(uptime)
    }
}
