use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::State;

const CPUINFO_PATH: &str = "/proc/cpuinfo";
const FREQ_PATH: &str = "/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq";

impl State {
    pub fn fetch_cpu(&mut self) -> String {
        #[cfg(target_os = "linux")]
        return get_cpu_from_proc();

        #[cfg(not(target_os = "linux"))]
        return self.get_cpu_sysinfo();
    }

    #[cfg(not(target_os = "linux"))]
    fn get_cpu_sysinfo(&mut self) -> String {
        let cpus = self.sys.cpus();
        let cpu = cpus.first().unwrap();

        let freq = cpus
            .iter()
            .map(|cpu| cpu.frequency())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0) as f64
            / 1_000.0;

        format!("{} ({}) @ {:.2} GHz", cpu.brand(), cpus.len(), freq)
    }
}

#[cfg(target_os = "linux")]
fn get_cpu_from_proc() -> String {
    let file = File::open(CPUINFO_PATH).expect("Failed to open /proc/cpuinfo");
    let reader = io::BufReader::new(file);

    let mut model_name = String::new();
    let mut cpu_cores = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("model name") {
            if model_name.is_empty() {
                model_name = line.split(':').nth(1).unwrap_or("").trim().to_string();
            }
        } else if line.starts_with("cpu cores") {
            cpu_cores = line
                .split(':')
                .nth(1)
                .unwrap_or("0")
                .trim()
                .parse()
                .unwrap_or(0);
        }

        if !model_name.is_empty() && cpu_cores > 0 {
            break;
        }
    }

    let freq = std::fs::read_to_string(FREQ_PATH).unwrap_or_else(|_| "0".to_string());
    let cpu_freq = freq.trim().parse().unwrap_or(0) as f64 / 1_000_000.0;

    format!("{} ({}) @ {:.2} GHz", model_name, cpu_cores, cpu_freq)
}
