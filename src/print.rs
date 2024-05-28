use crate::{logo, State};

macro_rules! add_string {
    ($output:expr, $prefix:expr, $func:expr) => {
        let result = $func();
        if !result.is_empty() {
            $output.push_str($prefix);
            $output.push_str(&result);
            $output.push('\n');
        }
    };
}

macro_rules! add_string_vec {
    ($output:expr, $prefix:expr, $func:expr) => {
        let result = $func();
        if !result.is_empty() {
            for item in result {
                $output.push_str($prefix);
                $output.push_str(&item);
                $output.push('\n');
            }
        }
    };
}

impl State {
    pub fn print(&mut self) {
        let mut output = String::new();

        add_string!(output, "OS: ", || self.fetch_os());
        add_string!(output, "Host: ", || self.fetch_host());
        add_string!(output, "Kernel: ", || self.fetch_kernel());
        add_string!(output, "Uptime: ", || self.fetch_uptime());
        add_string_vec!(output, "Packages: ", || self.fetch_packages());
        add_string!(output, "Shell: ", || self.fetch_shell());
        add_string_vec!(output, "Display: ", || self.fetch_display());
        add_string!(output, "WM: ", || self.fetch_wm());
        add_string!(output, "DE: ", || self.fetch_de());
        add_string!(output, "Terminal: ", || self.fetch_term());
        add_string!(output, "CPU: ", || self.fetch_cpu());
        add_string_vec!(output, "GPU: ", || self.fetch_gpu());
        add_string!(output, "Memory: ", || self.fetch_memory());
        add_string_vec!(output, "Disk: ", || self.fetch_disk());
        add_string!(output, "Locale: ", || self.fetch_locale());

        println!("{}", output);
    }

    pub fn print_with_logo(&mut self) {
        let os = self.fetch_os();

        let logo = logo::fetch_by_os(&os);

        let logo_width = logo.lines().map(str::len).max().unwrap_or(0);

        let mut logo_info = Vec::new();

        macro_rules! add_string_logo {
            ($prefix:expr, $func:expr) => {
                let result = $func();
                if !result.is_empty() {
                    logo_info.push(format!("{}{}", $prefix, result));
                }
            };
        }
        macro_rules! add_string_vec_logo {
            ($prefix:expr, $func:expr) => {
                let result = $func();
                if !result.is_empty() {
                    for item in result {
                        logo_info.push(format!("{}{}", $prefix, item));
                    }
                }
            };
        }

        let uname_hname = self.fetch_username_and_hostname();
        let uname_hname_len = uname_hname.len();
        add_string_logo!("", || uname_hname);
        add_string_logo!("", || "-".repeat(uname_hname_len));
        add_string_logo!("OS: ", || os);
        add_string_logo!("Host: ", || self.fetch_host());
        add_string_logo!("Kernel: ", || self.fetch_kernel());
        add_string_logo!("Uptime: ", || self.fetch_uptime());
        add_string_vec_logo!("Packages: ", || self.fetch_packages());
        add_string_logo!("Shell: ", || self.fetch_shell());
        add_string_vec_logo!("Display: ", || self.fetch_display());
        add_string_logo!("WM: ", || self.fetch_wm());
        add_string_logo!("DE: ", || self.fetch_de());
        add_string_logo!("Terminal: ", || self.fetch_term());
        add_string_logo!("CPU: ", || self.fetch_cpu());
        add_string_vec_logo!("GPU: ", || self.fetch_gpu());
        add_string_logo!("Memory: ", || self.fetch_memory());
        add_string_vec_logo!("Disk: ", || self.fetch_disk());
        add_string_logo!("Locale: ", || self.fetch_locale());

        let mut str_buf = String::new();

        for (i, line) in logo.lines().enumerate() {
            let padding = " ".repeat(logo_width - line.len() + 5);
            let new_line = format!(
                "{}{}{}\n",
                line,
                padding,
                logo_info.get(i).unwrap_or(&String::new())
            );

            str_buf.push_str(&new_line);
        }

        let logo_lines_len = logo.lines().count();

        for line in logo_info.iter().skip(logo_lines_len) {
            let padding = " ".repeat(logo_width + 5);

            let new_line = format!("{}{}\n", padding, line);

            str_buf.push_str(&new_line);
        }

        print!("{}", str_buf);
    }
}
