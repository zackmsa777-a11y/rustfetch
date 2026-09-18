<div align="center">

# rustfetch

**A blazingly fast, highly extensible system information fetch tool written in Rust.**
*Full fastfetch parity • 45+ authentic distro logos • Real hardware probes • Zero runtime bloat*

[![CI](https://github.com/zackmsa777-a11y/rustfetch/actions/workflows/ci.yml/badge.svg)](https://github.com/zackmsa777-a11y/rustfetch/actions/workflows/ci.yml)
[![Release](https://github.com/zackmsa777-a11y/rustfetch/actions/workflows/release.yml/badge.svg)](https://github.com/zackmsa777-a11y/rustfetch/releases)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust: 2024 Edition](https://img.shields.io/badge/Rust-2024%20Edition-red.svg)](https://www.rust-lang.org/)
[![Author](https://img.shields.io/badge/author-zackmsa777--a11y-purple.svg)](https://github.com/zackmsa777-a11y)
[![Performance](https://img.shields.io/badge/startup-<5ms-brightgreen.svg)](#benchmarks)

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
           .':loooooo;  ,oooooooooc           [#] [#] [#] [#] [#] [#] [#] [#]
               ..';::c'  .;loooo:'            [#] [#] [#] [#] [#] [#] [#] [#]
```

</div>

---

## Highlights

- **Blazingly Fast**: Executes in **< 5ms** using native Linux `procfs`, `sysfs`, and `libc` calls. No shell forks, no slow subcommands.
- **Interactive Full-Screen Setup TUI (`--setup`)**: Visually browse, preview, and apply 50+ themes with real-time hardware metrics. Features split-screen live preview, full mouse click & scroll wheel support, and zero-latency unbuffered raw input.
- **Distro-Aware Rice Intelligence**: Automatically inspects `/etc/os-release` and surfaces recommended rice presets tailored to your distribution (Ubuntu, Arch, CachyOS, Gentoo, Bedrock, Fedora, Debian, NixOS, Mint, and more) flagged with `[rec]`.
- **50+ Built-in Themes & Layouts**: Tree branch graphs, compact card frames, retro amber CRT, neon cyberpunk, matrix rain, quiet monochrome paper, and iconic palettes (Catppuccin Mocha/Macchiato, Tokyo Night, Dracula, Nord, Gruvbox, Everforest, Synthwave '84, Monokai Pro, One Dark).
- **Authentic Fastfetch ASCII Logos**: 45+ pixel-perfect logos matching Fastfetch, compiled directly into Rust with accurate brand colors and automatic runtime fallback. Clean terminal aesthetics with zero emojis.
- **Concurrent Multi-threaded Probes**: Hardware and environment modules run in parallel with scoped threads (`std::thread::scope`).
- **Real System Detection**: Real hardware metrics—not hardcoded strings or mock prints. Accurately queries CPU, GPU, memory, swap, mount points, battery, displays, desktop environments, window managers, themes, fonts, and network interfaces.
- **Structured JSON & Config Export**: Full `--json` export for scripts and monitoring, plus instant `[e]` export from the TUI to save any theme as custom JSONC to `~/.config/rustfetch/config.jsonc`.
- **Kitty Graphics Protocol & Image Logos**: High-resolution image rendering using the native Kitty Graphics Protocol (`t=f` file mode and `t=d` chunked direct transfer) supported in Kitty, Ghostty, and WezTerm. Includes header dimension probing (PNG, JPEG, GIF, WebP), automatic aspect ratio preservation, and customizable cell width/height and padding.
- **Pure Self-Documenting Architecture**: Built with zero source-code comments for maximum cleanliness and maintainability.

---

## Benchmarks

Measured on a standard Linux workstation (average of 50 runs):

| Tool | Language | Execution Time | Memory Footprint | External Subprocesses |
| :--- | :--- | :---: | :---: | :---: |
| **`rustfetch`** | **Rust** | **~4.8 ms** | **~3.2 MB** | **0 (Direct sysfs/procfs/libc)** |
| `fastfetch` | C | ~4.5 ms | ~4.1 MB | 0 |
| `neofetch` | Bash | ~280 ms | ~18.5 MB | 25+ subshells |
| `screenfetch` | Bash | ~410 ms | ~24.0 MB | 35+ subshells |

---

## Installation

### Requirements (source builds)

- **Rust 1.88+** (edition 2024 + let-chains).
- Recommended: install via [rustup](https://rustup.rs/) and keep `stable` updated.

### Prebuilt Binary (recommended)

```bash
curl -sSL https://raw.githubusercontent.com/zackmsa777-a11y/rustfetch/master/install.sh | bash
```

The installer picks the latest GitHub Release asset for your OS/arch (musl, gnu, or darwin), validates the download archive, and falls back to `cargo install --git` if no matching asset exists.

You can also grab release archives directly from [GitHub Releases](https://github.com/zackmsa777-a11y/rustfetch/releases/latest).

### Cargo via Git

```bash
cargo install --git https://github.com/zackmsa777-a11y/rustfetch.git
```

### Build from Source

```bash
git clone https://github.com/zackmsa777-a11y/rustfetch.git
cd rustfetch
cargo build --release
cp target/release/rustfetch ~/.cargo/bin/
```

---

## Interactive Setup TUI (`--setup`)

Launch the visual theme and layout configurator:

```bash
rustfetch --setup
# or
rustfetch --themes
```

The TUI provides an interactive split-pane interface:
- **Left Pane**: Scrollable catalog of available themes. Distro-specific themes matching your current operating system are pinned at the top with a `rec` tag.
- **Right Pane**: Instant real-time live preview of how your terminal will look with the selected theme, rendered using your actual hardware metrics.

### Controls & Mouse Navigation

| Input | Action |
| :--- | :--- |
| **`Mouse Click`** | Click any theme in the list to select and preview it; click footer buttons to Apply, Export, or Quit |
| **`Mouse Scroll`** | Scroll wheel up / down through the theme catalog |
| **`↑` / `k`** or **`↓` / `j`** | Navigate up / down one theme |
| **`PageUp`** / **`PageDown`** | Jump 5 themes up / down |
| **`Home` / `g`** or **`End` / `G`** | Jump to top / bottom of catalog |
| **`Enter`** | **Apply & Save** selected theme to `~/.config/rustfetch/config.jsonc` |
| **`e`** | **Export** theme to `~/.config/rustfetch/config.jsonc` as a custom theme |
| **`q`** / **`Esc`** / **`Ctrl+C`** | Exit without saving changes |

> **Non-Interactive Fallback**: When stdout/stdin is redirected or not a TTY (e.g. piped or automated), `rustfetch --setup` gracefully falls back to a clean numbered command-line prompt.

---

## Usage & CLI Options

```text
USAGE:
    rustfetch [OPTIONS]

OPTIONS:
    --setup, --themes      Interactive full-screen theme & layout setup TUI
    --theme <NAME>         Use a theme once without saving
    --set-theme <NAME>     Save a theme as default in the config file
    --list-themes          List all available color themes and layouts
    --preview-themes       Print a live preview of every theme
    --logo <NAME>          Specify a custom distro or OS logo
    --logo-color <COLOR>   Override the logo primary ANSI color
    --logo-type <TYPE>     Logo type: kitty, kitty-direct, kitty-icat, file, builtin, auto
    --kitty <PATH>         Display an image logo using the Kitty graphics protocol
    --kitty-direct <PATH>  Display an image using direct Kitty file transfer
    --kitty-icat <PATH>    Display an image via kitten icat tool
    --logo-width <NUM>     Target width in terminal character cells for image logo
    --logo-height <NUM>    Target height in terminal character cells for image logo
    --logo-padding <NUM>   Horizontal gap between image logo and module lines
    --logo-padding-left <NUM>  Left padding spaces before the image
    --logo-padding-top <NUM>   Top padding empty lines before image and text
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

# Launch interactive visual setup TUI
rustfetch --setup

# Display high-resolution image logo via Kitty Graphics Protocol
rustfetch --kitty ~/Pictures/avatar.png
rustfetch --kitty ~/Pictures/wallpaper.png --logo-width 36 --logo-padding 4

# Direct transmission mode (works over SSH or inside containers without shared filesystem)
rustfetch --kitty-direct ~/Pictures/logo.png

# Try a theme once without modifying configuration
rustfetch --theme groups
rustfetch --theme catppuccin-mocha
rustfetch --theme cachyos-speed

# Permanently set default theme in ~/.config/rustfetch/config.jsonc
rustfetch --set-theme tokyo-night

# List all available built-in and user themes
rustfetch --list-themes

# Force a specific distro logo (e.g. Arch, CachyOS, Gentoo, Bedrock, or Ferris)
rustfetch --logo arch
rustfetch --logo cachyos
rustfetch --logo gentoo
rustfetch --logo bedrock
rustfetch --logo rust

# Select custom modules and ordering
rustfetch --structure title:os:kernel:cpu:gpu:memory:disk:colors

# Output raw JSON for scripting or monitoring
rustfetch --json | jq .cpu

# Minimal fetch with no logo
rustfetch --no-logo
```

---

## Themes & Distro Ricing

`rustfetch` comes with 50+ built-in presets covering major distributions, community desktop rices, and legendary colorways.

### Distro-Specific Themes
- **Ubuntu**: `ubuntu-classic`, `ubuntu-modern`, `ubuntu-tree`, `ubuntu-minimal`, `ubuntu-mini`
- **Arch Linux**: `arch-clean`
- **CachyOS**: `cachyos-speed` (x86-64-v3/v4 teal & cyan layout)
- **Bedrock Linux**: `bedrock-strata` (multi-distro meta layout with silver accents)
- **Gentoo**: `gentoo-purple` (compiled-from-source purple & lavender aesthetic)
- **Debian**: `debian-swirl` (classic ruby swirl & soft white)
- **Fedora**: `fedora-blue` (navy & ocean blue with double-colon keys)
- **NixOS**: `nixos-snowflake` (glacial cyan snowflake motif)
- **Linux Mint**: `mint-fresh` (fresh mint green & silver leaf elegance)
- **openSUSE**: `opensuse-geek` (gecko green & rolling chameleon speed)
- **Pop!_OS**: `pop-cosmic` (cosmic teal & amber accents)
- **Void Linux**: `void-xbps` (runit fast, xbps green & graphite precision)
- **Alpine Linux**: `alpine-peak` (ultra-lightweight alpine blue & summit white)
- **Manjaro**: `manjaro-teal` (signature dark teal & neon emerald flow)
- **Kali Linux**: `kali-dragon` (security dragon deep cobalt & obsidian cyan)
- **EndeavourOS**: `endeavour-space` (interstellar violet, magenta & cosmic orange)
- **RHEL**: `redhat-shadow` (corporate crimson shadow)

### Layout Styles & Terminal Rices
- **`groups`**: Grouped tree branches with glyph connectors (`├─`, `└─`).
- **`hypr`**: Colour-dot bar framing a compact block layout.
- **`rice`**: Card frame with rule lines, bracketed labels, and piped values.
- **`cyber`**: Neon cyan on magenta with arrow keys and glowing block marks.
- **`retro`**: Amber CRT terminal aesthetics with prompt art and double-colon keys.
- **`matrix`**: Terminal green digital rain motif.
- **`paper`**: Quiet monochrome with soft grey keys and middle-dot separators.
- **`minimal`**: Single-column four-letter abbreviations with rainbow palette.
- **`ferris-crab`**: Rustacean mascot ASCII art with fiery rust tones.
- **`tux-penguin`**: Classic Linux Tux art in clean monochrome.

### Community Color Palettes
- **Catppuccin**: `catppuccin-mocha`, `catppuccin-macchiato`
- **Tokyo Night**: `tokyo-night`
- **Dracula**: `dracula`
- **Nord**: `nord`
- **Gruvbox**: `gruvbox`
- **Everforest**: `everforest`
- **Solarized**: `solarized-dark`
- **Synthwave '84**: `synthwave-84`
- **Monokai Pro**: `monokai-pro`
- **One Dark**: `one-dark`

### Custom Theme Files
Drop any `.jsonc` theme into `~/.config/rustfetch/themes/<name>.jsonc`:

```jsonc
{
  "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json",
  "display": {
    "separator": " -> ",
    "color": {
      "keys": "magenta",
      "title": "bright_cyan"
    }
  },
  "modules": [
    "title",
    "separator",
    { "type": "os", "key": "os" },
    { "type": "cpu", "key": "cpu" },
    { "type": "memory", "key": "ram" },
    "colors"
  ]
}
```

Run your custom theme instantly:
```bash
rustfetch --theme <name>
```

---

## Supported Distros & Logos

Run `rustfetch --list-logos` to see all supported identifiers:

| Distro / OS | Logo Flag | Signature Color |
| :--- | :--- | :--- |
| **AlmaLinux** | `--logo almalinux` | Alma Blue |
| **Alpine Linux** | `--logo alpine` | Alpine Blue |
| **Android** | `--logo android` | Android Green |
| **Arch Linux** | `--logo arch` | Cyan |
| **Artix Linux** | `--logo artix` | Artix Cyan |
| **Bedrock Linux** | `--logo bedrock` | Bedrock White |
| **CachyOS** | `--logo cachyos` | Cyan / Emerald |
| **CentOS** | `--logo centos` | CentOS Purple |
| **Debian** | `--logo debian` | Debian Crimson |
| **Devuan** | `--logo devuan` | Devuan Purple |
| **DragonFly BSD** | `--logo dragonfly` | Red |
| **elementary OS** | `--logo elementary` | Blue |
| **EndeavourOS** | `--logo endeavouros` | Magenta |
| **Fedora** | `--logo fedora` | Fedora Blue |
| **FreeBSD** | `--logo freebsd` | FreeBSD Red |
| **Garuda Linux** | `--logo garuda` | Garuda Cyan |
| **Gentoo** | `--logo gentoo` | Gentoo Purple |
| **GNU Guix** | `--logo guix` | Guix Yellow |
| **Haiku** | `--logo haiku` | Haiku Yellow |
| **Kali Linux** | `--logo kali` | Kali Blue |
| **Linux Mint** | `--logo mint` | Mint Green |
| **Mageia** | `--logo mageia` | Mageia Blue |
| **Manjaro** | `--logo manjaro` | Forest Green |
| **macOS** | `--logo macos` | Apple White |
| **MX Linux** | `--logo mx` | White |
| **NetBSD** | `--logo netbsd` | NetBSD Orange |
| **NixOS** | `--logo nixos` | Light Cyan |
| **OpenBSD** | `--logo openbsd` | OpenBSD Yellow |
| **openSUSE** | `--logo opensuse` | Gecko Green |
| **Oracle Linux** | `--logo oracle` | Oracle Red |
| **Parrot OS** | `--logo parrot` | Parrot Cyan |
| **Pop!_OS** | `--logo pop` | Cyan |
| **Raspberry Pi OS** | `--logo raspberry` | Raspberry Red |
| **Red Hat / RHEL** | `--logo redhat` | Red Hat Red |
| **Rocky Linux** | `--logo rocky` | Rocky Green |
| **Rust (Ferris)** | `--logo rust` | Rust Orange |
| **Slackware** | `--logo slackware` | Dark Blue |
| **Solaris / Illumos** | `--logo solaris` | Sun Yellow |
| **Solus** | `--logo solus` | Solus Blue |
| **SteamOS** | `--logo steamos` | Steam Blue |
| **Tails** | `--logo tails` | Tails Purple |
| **Ubuntu** | `--logo ubuntu` | Bright Red / Orange |
| **Void Linux** | `--logo void` | Void Green |
| **Windows** | `--logo windows` | Windows Cyan |
| **Zorin OS** | `--logo zorin` | Zorin Blue |
| **Generic Linux (Tux)** | `--logo linux` | Tux Yellow |

---

## Available Modules

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

## Configuration (`config.jsonc`)

`rustfetch` uses the standard fastfetch JSON/JSONC configuration schema and automatically looks for configs in:
1. `--config <path>`
2. `$XDG_CONFIG_HOME/rustfetch/config.jsonc`
3. `~/.config/rustfetch/config.jsonc`
4. `~/.config/fastfetch/config.jsonc` (fastfetch fallback)

Generate a template configuration with:

```bash
rustfetch --gen-config > ~/.config/rustfetch/config.jsonc
```

### Example `config.jsonc`

```jsonc
{
  "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json",
  "logo": {
    "source": "auto",
    "color": {
      "1": "auto"
    }
  },
  "display": {
    "separator": ": ",
    "color": {
      "keys": "auto"
    }
  },
  "modules": [
    "title",
    "separator",
    { "type": "os", "key": "OS" },
    { "type": "host", "key": "Host" },
    { "type": "kernel", "key": "Kernel" },
    { "type": "uptime", "key": "Uptime" },
    { "type": "packages", "key": "Packages" },
    { "type": "shell", "key": "Shell" },
    { "type": "display", "key": "Display" },
    { "type": "de", "key": "DE" },
    { "type": "wm", "key": "WM" },
    { "type": "theme", "key": "Theme" },
    { "type": "icons", "key": "Icons" },
    { "type": "font", "key": "Font" },
    { "type": "cursor", "key": "Cursor" },
    { "type": "terminal", "key": "Terminal" },
    { "type": "terminalfont", "key": "Terminal Font" },
    { "type": "cpu", "key": "CPU" },
    { "type": "gpu", "key": "GPU" },
    { "type": "memory", "key": "Memory" },
    { "type": "swap", "key": "Swap" },
    { "type": "disk", "key": "Disk" },
    { "type": "battery", "key": "Battery" },
    { "type": "poweradapter", "key": "Power Adapter" },
    { "type": "localip", "key": "Local IP" },
    { "type": "locale", "key": "Locale" },
    "break",
    "colors"
  ],
  "disk_paths": ["/"]
}
```

### Kitty Image Logo Configuration

To render a high-resolution image logo in Kitty, Ghostty, or WezTerm via `config.jsonc`:

```jsonc
{
  "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json",
  "logo": {
    "source": "~/Pictures/avatar.png",
    "type": "kitty",
    "width": 32,
    "height": 16,
    "padding": {
      "top": 1,
      "left": 1,
      "right": 3
    }
  }
}
```

---

## Author & Maintainer

Created, designed, and maintained by **Zack** ([@zackmsa777-a11y](https://github.com/zackmsa777-a11y)).

---

## Contributing

Contributions, bug reports, and logo submissions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on the zero-comment code standard, test requirements, and workflow.

---

## License

This project is dual-licensed under either:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
