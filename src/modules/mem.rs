use crate::State;

impl State {
    pub fn fetch_memory(&mut self) -> String {
        let (used, total) = self.get_memory_info();
        let used_gb = used as f64 / 1024.0 / 1024.0 / 1024.0;
        let total_gb = total as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_percent = used_gb / total_gb * 100.0;

        format!(
            "{:.2}GiB / {:.2}GiB ({:.2}%)",
            used_gb, total_gb, used_percent
        )
    }

    fn get_memory_info(&mut self) -> (u64, u64) {
        self.sys.refresh_memory();

        return (self.sys.used_memory(), self.sys.total_memory());
    }
}
