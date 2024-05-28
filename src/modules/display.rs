use x11rb::{connection::Connection, protocol::randr::ConnectionExt};

use crate::State;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl State {
    pub fn fetch_display(&self) -> Vec<String> {
        fetch_x11_display().unwrap_or_else(|_| Vec::new())
    }
}


fn fetch_x11_display() -> Result<Vec<String>> {
    let mut displays = Vec::new();

    let (conn, screen_num) = x11rb::rust_connection::RustConnection::connect(None).unwrap();

    let screen = &conn.setup().roots[screen_num];

    let screen_resources = conn.randr_get_screen_resources(screen.root)?.reply()?;
    if screen_resources.num_crtcs() == 0 {
        displays.push("Headless".to_string());
        return Ok(displays);
    }

    let modes_map = screen_resources.modes.iter().map(|mode| (mode.id, mode)).collect::<std::collections::HashMap<_, _>>();

    for crtc in screen_resources.crtcs {
        let crtc_info = conn.randr_get_crtc_info(crtc, 0)?.reply()?;

        if crtc_info.mode == 0 {
            continue;
        }

        let mode_info = modes_map.get(&crtc_info.mode).unwrap();

        displays.push(format!(
            "{}x{} @ {:.2}Hz",
            crtc_info.width,
            crtc_info.height,
            mode_info.dot_clock as f32 / (mode_info.htotal as f32 * mode_info.vtotal as f32)
        ));
    }

    Ok(displays)
}