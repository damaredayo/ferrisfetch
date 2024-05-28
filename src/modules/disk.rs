use crate::State;

impl State {
    pub fn fetch_disk(&self) -> Vec<String> {
        sysinfo::Disks::new_with_refreshed_list().iter().map(|disk| {
            format!(
                "{}: {:.2}GB / {:.2}GB ({})",
                disk.mount_point().to_str().unwrap_or(""),
                disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0,
                disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0,
                disk.file_system().to_str().unwrap_or(""),
            )
        }).collect()
    }
}
