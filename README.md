# ferrisfetch

ferrisfetch is a [neofetch](https://github.com/dylanaraps/neofetch)-like tool for fetching system information, written in Rust.

> 🛈 Please note that this project is still in very early development and may not be stable and certainly not feature complete. If you encounter any issues or would like a feature added, please create an issue.

## Installation

### From Pre-built Binaries

Pre-built binaries are available for Linux and Windows on the [releases page](https://github.com/damaredayo/ferrisfetch/releases). Download the binary for your platform and place it in a directory in your PATH. You can then run the binary from the command line.

### From Source

To install ferrisfetch from source, you will need to have Rust installed. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and build the project using the following commands:

```sh
git clone https://github.com/damaredayo/ferrisfetch.git
cd ferrisfetch
cargo build --release
```

The compiled binary will be located at `target/release/ferrisfetch`.

## Features and Roadmap

- [x] Display system information
- [ ] Full support for all operating systems and architectures
- [ ] Customizable output
- [ ] Support for themes
- [ ] Support for plugins (e.g. fetching additional information)
- [ ] Full Desktop/Window Manager support (linux)
- [ ] Ricing detection

## Modules

| Module name | Source File | Description | Example | Implemented |
| --------- | ---------- | ----------- | ---- | ------- |
| OS | os.rs | Fetches the operating system name and version | `OS: Arch Linux` | ✅ |
| Host | host.rs | Fetches the host information | `Host: MS-7D91 (3.0)` | ✅ |
| Kernel | kernel.rs | Fetches the kernel version | `Kernel: Linux 6.8.9-arch1-2` | ✅ |
| Uptime | uptime.rs | Fetches the system uptime | `Uptime: 2 hours, 30 minutes` | ✅ |
| Packages | packages.rs | Fetches the number of installed packages | `Packages: 900 (pacman)` | ✅ |
| Shell | shell.rs | Fetches the shell name | `Shell: zsh` | ✅ |
| Display | display.rs | Fetches the screen resolution | `Display (ZOWIE XL LCD): 1920x1080 @ 240Hz` | ✅ |
| DE | de.rs | Fetches the desktop environment or window manager | `DE: Xfce` | ❌ |
| WM | wm.rs | Fetches the window manager | `WM: LeftWM (X11)` | ✅ (Only X11) |
| Cursor | cursor.rs | Fetches the current cursor theme | `Cursor: Adwaita` | ❌ |
| Theme | theme.rs | Fetches the current GTK theme | `Theme: Adwaita` | ❌ |
| Icons | icons.rs | Fetches the current icon theme | `Icons: Papirus` | ❌ |
| Terminal | terminal.rs | Fetches the terminal emulator name | `Terminal: alacritty` | ❌ |
| Font | font.rs | Fetches the current font | `Font: alacritty (11px)` | ❌ |
| CPU | cpu.rs | Fetches the CPU model and number of cores | `CPU: Intel(R) Core(TM) i9-14900KF (24) @ 5.70 GHz` | ✅ |
| GPU | gpu.rs | Fetches the GPU model | `GPU: NVIDIA GeForce RTX 4080` | ✅ |
| Memory | memory.rs | Fetches the total and used memory | `Memory: 11.11GiB / 31.18GiB (35.63%)` | ✅ |
| Swap | swap.rs | Fetches the total and used swap space | `Swap: 0.00GiB / 0.00GiB (0.00%)` | ❌ |
| Disk | disk.rs | Fetches the total and used disk space | `Disk (/): 44.96 GiB / 425.39 GiB (11%) - ext4` | ✅ |
| Battery | battery.rs | Fetches the battery status | `Battery: 100%` | ❌ |
| Network | network.rs | Fetches the network interfaces | `Network (enp5s0): 192.168.0.45/24` | ❌ |
| Locale | locale.rs | Fetches the system locale | `Locale: en_GB.UTF-8` | ✅ |
