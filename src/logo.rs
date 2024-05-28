pub fn fetch_by_os(os: &String) -> String {
    match os.as_str() {
        "Arch Linux" => include_str!("../logos/arch.txt").to_string(),
        "Debian" => include_str!("../logos/debian.txt").to_string(),
        "Fedora" => include_str!("../logos/fedora.txt").to_string(),
        "Gentoo" => include_str!("../logos/gentoo.txt").to_string(),
        "Linux Mint" => include_str!("../logos/mint.txt").to_string(),
        "openSUSE" => include_str!("../logos/opensuse.txt").to_string(),
        "Ubuntu" => include_str!("../logos/ubuntu.txt").to_string(),
        "Darwin" => include_str!("../logos/macos.txt").to_string(),
        "Windows" => include_str!("../logos/windows.txt").to_string(),
        _ => include_str!("../logos/unknown.txt").to_string(),
    }
}
