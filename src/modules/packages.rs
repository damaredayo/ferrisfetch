use crate::State;

const PACMAN_DB_PATH: &str = "/var/lib/pacman/local";
const APT_STATUS_PATH: &str = "/var/lib/dpkg/status";
const RPM_DB_PATH: &str = "/var/lib/rpm";
const HOMEBREW_CELLAR_PATH: &str = "/usr/local/Cellar";
const NIX_STORE_PATH: &str = "/nix/store";
const FLATPAK_INSTALLATIONS_PATH: &str = "/var/lib/flatpak/exports/share/flatpak/installations";
const SNAP_COMMON_PATH: &str = "/var/lib/snapd/snaps";

impl State {
    pub fn fetch_packages(&self) -> Vec<String> {
        let mut packages = Vec::new();

        if let Ok(package_managers) = get_package_manager_info() {
            for (manager, count) in package_managers {
                packages.push(format!("{} ({})", count, manager));
            }
        }

        packages
    }
}
fn get_package_manager_info() -> Result<Vec<(String, usize)>, String> {
    let mut package_managers = Vec::new();

    let apt_count = count_apt_packages();
    if apt_count > 0 {
        package_managers.push(("apt".to_string(), apt_count));
    }

    let pacman_count = count_pacman_packages();
    if pacman_count > 0 {
        package_managers.push(("pacman".to_string(), pacman_count));
    }

    let rpm_count = count_rpm_packages();
    if rpm_count > 0 {
        package_managers.push(("RPM".to_string(), rpm_count));
    }

    let homebrew_count = count_homebrew_packages();
    if homebrew_count > 0 {
        package_managers.push(("Homebrew".to_string(), homebrew_count));
    }

    let nix_count = count_nix_packages();
    if nix_count > 0 {
        package_managers.push(("nix".to_string(), nix_count));
    }

    let flatpak_count = count_flatpak_packages();
    if flatpak_count > 0 {
        package_managers.push(("flatpak".to_string(), flatpak_count));
    }

    let snap_count = count_snap_packages();
    if snap_count > 0 {
        package_managers.push(("snap".to_string(), snap_count));
    }

    if !package_managers.is_empty() {
        return Ok(package_managers);
    }

    Err("No packages found for supported package managers".to_string())
}

fn count_apt_packages() -> usize {
    let mut package_count = 0;
    if let Ok(status_file) = std::fs::read_to_string(APT_STATUS_PATH) {
        let packages: Vec<&str> = status_file.split("\n\n").collect();
        package_count = packages.len();
    }

    package_count
}

fn count_pacman_packages() -> usize {
    let mut package_count = 0;

    if let Ok(entries) = std::fs::read_dir(PACMAN_DB_PATH) {
        package_count = entries.count();
    }

    package_count
}

fn count_rpm_packages() -> usize {
    let mut package_count = 0;

    if let Ok(entries) = std::fs::read_dir(RPM_DB_PATH) {
        package_count = entries.count();
    }

    package_count
}

fn count_homebrew_packages() -> usize {
    let mut package_count = 0;

    if let Ok(entries) = std::fs::read_dir(HOMEBREW_CELLAR_PATH) {
        package_count = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .count();
    }

    package_count
}

fn count_nix_packages() -> usize {
    let mut package_count = 0;

    if let Ok(entries) = std::fs::read_dir(NIX_STORE_PATH) {
        package_count = entries.count();
    }

    package_count
}

fn count_flatpak_packages() -> usize {
    let mut package_count = 0;

    if let Ok(entries) = std::fs::read_dir(FLATPAK_INSTALLATIONS_PATH) {
        package_count = entries.count();
    }

    package_count
}

fn count_snap_packages() -> usize {
    let mut package_count = 0;

    if let Ok(entries) = std::fs::read_dir(SNAP_COMMON_PATH) {
        package_count = entries.count();
    }

    package_count
}
