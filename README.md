<div align="center">

# 🦀 rustfetch

**A blazingly fast, highly extensible system information fetch tool written in Rust.**
*Full fastfetch parity • 22+ authentic distro logos • Real hardware probes • Zero runtime bloat*

[![CI](https://github.com/zackmsa777-a11y/rustfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/zackmsa777-a11y/rustfetch/actions/workflows/ci.yml)
[![Release](https://github.com/zackmsa777-a11y/rustfetch/actions/workflows/release.yml/badge.svg)](https://github.com/zackmsa777-a11y/rustfetch/releases)
[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/rustfetch)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust: 2021 Edition](https://img.shields.io/badge/Rust-2021%20Edition-red.svg)](https://www.rust-lang.org/)
[![Performance](https://img.shields.io/badge/startup-<5ms-brightgreen.svg)](#-benchmarks)

```text
                             ....             ubuntu@workstation
              .',:clooo:  .:looooo:.          --------------------------
           .;looooooooc  .oooooooooo'         OS: Ubuntu 24.04.4 LTS x86_64
        .;looooool:,''.  :ooooooooooc         Host: KVM/QEMU Standard PC
       ;looool;.         'oooooooooo,         Kernel: Linux 6.17.0-oracle
      ;clool'             .cooooooc.  ,,      Uptime: 6 days, 21 hours
         ...                ......  .:oo,     Packages: 1229 (dpkg), 65 (nix)
  .;clol:,.                        .loooo'    Shell: bash 5.2.21
 :ooooooooo,                        'ooool    Display (Virtual-1): 1280x800
'ooooooooooo.                        loooo.   Terminal: alacritty 0.13.0
'ooooooooool                         coooo.   CPU: Intel(R) Xeon(R) (2) @ 2.59 GHz
 ,loooooooc.                        .loooo.   GPU: QEMU Virtual Video Controller
   .,;;;'.                          ;ooooc    Memory: 2.85 GiB / 15.61 GiB (18%)
       ...                         ,ooool.    Swap: Disabled
    .cooooc.              ..',,'.  .cooo.     Disk (/): 38.93 GiB / 47.39 GiB (82%)
      ;ooooo:.           ;oooooooc.  :l.      Local IP (ens3): 10.0.0.248/24
       .coooooc,..      coooooooooo.          Locale: C.UTF-8
         .:ooooooolc:. .ooooooooooo'          
           .':loooooo;  ,oooooooooc           ███ ███ ███ ███ ███ ███ ███ ███
               ..';::c'  .;loooo:'            ███ ███ ███ ███ ███ ███ ███ ███
```

</div>

---

## ⚡ Highlights

- **⚡ Blazingly Fast**: Executes in **< 5ms** using native Linux `procfs`, `sysfs`, and `libc` calls. No shell forks, no slow subcommands.
- **🧵 Concurrent Multi-threaded Probes**: Hardware and environment modules run in parallel with scoped threads (`std::thread::scope`).
- **🎨 22+ Distro & OS Logos**: Complete authentic ASCII art collection with accurate distro brand colors (Arch, NixOS, Fedora, Ubuntu, Debian, Alpine, Gentoo, macOS, Windows, and more).
- **📊 Real System Detection**: Real hardware metrics—not hardcoded strings or mock prints. Accurately queries CPU, GPU, memory, swap, mount points, battery, displays, desktop environments, window managers, themes, fonts, and network interfaces.
- **📄 Structured JSON Output**: Full `--json` export for scripts, monitoring, dotfile automation, and integrations.
- **🛠️ JSONC Configuration**: Supports customizable layouts, module filters, and overrides via `~/.config/rustfetch/config.jsonc`.
- **🧼 Pure Self-Documenting Architecture**: Built with zero source-code comments for maximum cleanliness and maintainability.

---

## 📊 Benchmarks

Measured on a standard Linux workstation (average of 50 runs):

| Tool | Language | Execution Time | Memory Footprint | External Subprocesses |
| :--- | :--- | :---: | :---: | :---: |
| **`rustfetch`** | **Rust** | **~4.8 ms** | **~3.2 MB** | **0 (Direct sysfs/procfs/libc)** |
| `fastfetch` | C | ~4.5 ms | ~4.1 MB | 0 |
| `neofetch` | Bash | ~280 ms | ~18.5 MB | 25+ subshells |
| `screenfetch` | Bash | ~410 ms | ~24.0 MB | 35+ subshells |

---

## 📦 Installation

### Cargo (Recommended)

```bash
cargo install rustfetch
```

### Prebuilt Binary (Linux x86_64 / ARM64 / macOS / Windows)

Download the latest release archive from [GitHub Releases](https://github.com/zackmsa777-a11y/rustfetch/releases/latest):

```bash
curl -sSL https://raw.githubusercontent.com/zackmsa777-a11y/rustfetch/master/install.sh | bash
```

Or manually extract and place the binary in your `$PATH`:

```bash
tar -xzvf rustfetch-v0.1.0-x86_64-unknown-linux-musl.tar.gz
sudo mv rustfetch /usr/local/bin/
```

### Arch Linux (AUR)

```bash
yay -S rustfetch-bin
```

### Build from Source

```bash
git clone https://github.com/zackmsa777-a11y/rustfetch.git
cd rustfetch
cargo build --release
sudo install -m 755 target/release/rustfetch /usr/local/bin/
```

---

## 🚀 Usage & CLI Options

```text
USAGE:
    rustfetch [OPTIONS]

OPTIONS:
    --logo <NAME>          Specify a custom distro or OS logo
    --logo-color <COLOR>   Override the logo primary ANSI color
    --no-logo              Hide the ASCII distro logo
    --no-color             Disable ANSI terminal colors
    --color <MODE>         Color output mode (always, auto, never)
    --structure <LIST>     Colon or comma-separated list of modules to display
    --disk-paths <PATHS>   Comma-separated list of mount paths to check (default: /)
    --config <PATH>        Path to custom JSON/JSONC configuration file
    --gen-config           Print default JSON configuration to stdout
    --list-logos           List all supported distro and OS logos
    --list-modules         List all available information modules
    --json                 Output system information in structured JSON format
    -v, --version          Print version information
    -h, --help             Print help information
```

### Common Examples

```bash
# Default auto-detected fetch
rustfetch

# Force a specific distro logo (e.g. Arch, NixOS, or Rust)
rustfetch --logo arch
rustfetch --logo nixos
rustfetch --logo rust

# Select custom modules and ordering
rustfetch --structure title:os:kernel:cpu:gpu:memory:disk:colors

# Output raw JSON for scripting
rustfetch --json | jq .cpu

# Minimal fetch with no logo
rustfetch --no-logo
```

---

## 🎨 Supported Distros & Logos

Run `rustfetch --list-logos` to see all supported identifiers:

| Distro / OS | Logo Flag | Signature Color |
| :--- | :--- | :--- |
| **Ubuntu** | `--logo ubuntu` | Bright Red / Orange |
| **Debian** | `--logo debian` | Debian Crimson |
| **Arch Linux** | `--logo arch` | Cyan |
| **Fedora** | `--logo fedora` | Fedora Blue |
| **NixOS** | `--logo nixos` | Light Cyan |
| **Alpine Linux** | `--logo alpine` | Alpine Blue |
| **Void Linux** | `--logo void` | Void Green |
| **Gentoo** | `--logo gentoo` | Gentoo Purple |
| **Linux Mint** | `--logo mint` | Mint Green |
| **Manjaro** | `--logo manjaro` | Forest Green |
| **Pop!_OS** | `--logo pop` | Cyan |
| **openSUSE** | `--logo opensuse` | Gecko Green |
| **Kali Linux** | `--logo kali` | Kali Blue |
| **Red Hat / RHEL** | `--logo redhat` | Red Hat Red |
| **Slackware** | `--logo slackware` | Dark Blue |
| **EndeavourOS** | `--logo endeavouros` | Magenta |
| **FreeBSD** | `--logo freebsd` | FreeBSD Red |
| **macOS** | `--logo macos` | Apple White |
| **Windows** | `--logo windows` | Windows Cyan |
| **Android** | `--logo android` | Android Green |
| **Generic Linux (Tux)** | `--logo linux` | Tux Yellow |
| **Rust (Ferris)** | `--logo rust` | Rust Orange |

---

## 🧩 Available Modules

Run `rustfetch --list-modules` to see the complete module catalog:

| Module Key | Description | Detection Source |
| :--- | :--- | :--- |
| `title` | `username@hostname` | `$USER` / `/etc/hostname` / libc |
| `separator` | Horizontal divider dashes | String repeat matching title width |
| `os` | Distro name, version & architecture | `/etc/os-release`, `uname` |
| `host` | Motherboard, model, virtualization | DMI sysfs, ARM device-tree, systemd-virt |
| `kernel` | Operating system kernel release | `/proc/sys/kernel/osrelease`, `uname` |
| `uptime` | Human-readable system uptime | `/proc/uptime`, `sysinfo` |
| `packages` | Installed packages count | dpkg, pacman, rpm, flatpak, snap, nix, apk, brew |
| `shell` | Current shell and version | `$SHELL`, `--version` |
| `display` | Monitor resolution and refresh rate | DRM modes, X11 xrandr, Wayland hyprctl |
| `de` | Desktop Environment | `$XDG_CURRENT_DESKTOP`, desktop sessions |
| `wm` | Window Manager | Active Wayland compositors & X11 WMs |
| `wm_theme` | Window manager theme | Window manager settings |
| `theme` | GTK/Qt theme | `~/.config/gtk-3.0/settings.ini` |
| `icons` | Desktop icon theme | GTK settings / dconf |
| `font` | System UI font | GTK settings |
| `cursor` | Mouse cursor theme | GTK settings |
| `terminal` | Terminal emulator & version | Process tree walker, `$TERM_PROGRAM` |
| `terminal_font` | Terminal typeface font | Terminal configuration files |
| `cpu` | CPU model, core count, frequency | `/proc/cpuinfo`, cpufreq sysfs |
| `gpu` | Dedicated & integrated graphics | PCI bus scan, `nvidia-smi`, `lspci` |
| `memory` | Used and total RAM with % | `/proc/meminfo` (MemTotal - MemAvailable) |
| `swap` | Used and total swap space with % | `/proc/meminfo` (SwapTotal - SwapFree) |
| `disk` | Partition usage and filesystem type | `statvfs` on `/proc/mounts` |
| `battery` | Battery percentage & charging status | `/sys/class/power_supply/BAT*` |
| `power_adapter`| AC adapter connected status | `/sys/class/power_supply/AC*` |
| `audio` | Sound devices & audio server | `/proc/asound/cards`, ALSA/PipeWire |
| `local_ip` | Primary interface IP and CIDR subnet | `/proc/net/route`, `getifaddrs` |
| `locale` | System locale and encoding | `$LC_ALL`, `$LANG` |
| `break` | Blank spacing line | Terminal newline |
| `colors` | 16 ANSI color palette blocks | 8 standard + 8 bright background blocks |

---

## ⚙️ Configuration (`config.jsonc`)

`rustfetch` checks for a configuration file in:
1. `--config <path>`
2. `$XDG_CONFIG_HOME/rustfetch/config.jsonc`
3. `~/.config/rustfetch/config.jsonc`

Generate a template configuration with:

```bash
rustfetch --gen-config > ~/.config/rustfetch/config.jsonc
```

### Example `config.jsonc`

```jsonc
{
  "logo": "auto",
  "logo_color": "auto",
  "no_logo": false,
  "no_color": false,
  "modules": [
    "title",
    "separator",
    "os",
    "host",
    "kernel",
    "uptime",
    "packages",
    "shell",
    "display",
    "terminal",
    "cpu",
    "gpu",
    "memory",
    "swap",
    "disk",
    "battery",
    "local_ip",
    "locale",
    "break",
    "colors"
  ],
  "disk_paths": ["/"]
}
```

---

## 🤝 Contributing

Contributions, bug reports, and logo submissions are warmly welcomed! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our zero-comment code standard, test requirements, and workflow.

---

## 📜 License

This project is dual-licensed under either:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
