use crate::State;

impl State {
    pub fn fetch_wm(&self) -> String {
        detect_window_manager()
            .map(|(wm, backend)| format!("{} ({})", wm, backend))
            .unwrap_or_else(|| String::new())
    }
}

const X11: &str = "X11";
const WAYLAND: &str = "Wayland";

fn detect_window_manager() -> Option<(String, &'static str)> {
    if let Ok(xdg_current_desktop) = std::env::var("XDG_CURRENT_DESKTOP") {
        return Some((xdg_current_desktop, X11));
    }

    None
}
