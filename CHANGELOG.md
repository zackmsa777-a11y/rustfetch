# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-17

### Added
- Complete fastfetch-parity system information gathering engine in pure Rust.
- Real hardware and system metrics detection:
  - OS release parsing (`/etc/os-release`, arch, bitness).
  - Motherboard, vendor, product, and virtualization detection (`/sys/devices/virtual/dmi/id`, `systemd-detect-virt`, ARM device-tree).
  - Linux kernel release and architecture.
  - Formatted uptime (days, hours, minutes, seconds).
  - Multi-package manager detection (dpkg, pacman, rpm, flatpak, snap, nix, apk, emerge, xbps, brew).
  - Shell detection and version querying (bash, zsh, fish, etc.).
  - Display connector and resolution detection (DRM modes, X11 xrandr, Wayland hyprctl).
  - Desktop Environment (GNOME, KDE Plasma, XFCE, Cinnamon, MATE, LXQt, etc.).
  - Window Manager detection (Wayland compositors, X11 WMs).
  - GTK/Qt theme, icons, fonts, cursor settings detection.
  - Terminal emulator and terminal font detection (via process ancestry, term env, ttyname).
  - CPU model name, physical cores, threads, and dynamic clock frequency.
  - GPU detection across NVIDIA, AMD Radeon, Intel, VirtIO, and QEMU.
  - Memory and Swap usage statistics with percentage calculation.
  - Multi-mount disk partition inspection via `statvfs` with filesystem types.
  - Battery charge level, charge status, and AC power adapter connection status.
  - Audio hardware detection via ALSA card procfs.
  - Network interface detection and local IPv4 CIDR resolution via `getifaddrs`.
  - Locale detection (`LC_ALL`, `LANG`, `LC_MESSAGES`).
- 44+ authentic ASCII distro and OS logos with signatures ANSI colors:
  - Ubuntu, Debian, Arch Linux, Fedora, NixOS, Alpine, Void, Gentoo, Linux Mint, Manjaro, Pop!_OS, openSUSE, Kali Linux, Red Hat, Rocky Linux, AlmaLinux, CentOS, Oracle Linux, Slackware, EndeavourOS, FreeBSD, OpenBSD, NetBSD, DragonFly BSD, Haiku, macOS, Windows, Android, Solus, Devuan, Artix, Garuda, Parrot, Mageia, Zorin, Guix, SteamOS, Tails, Raspberry Pi, generic Linux, Rust.
- Rust 2024 Edition migration with stabilized let chains and zero-overhead match ergonomics.
- Structured JSON output mode (`--json`) for automated pipelines and integrations.
- Configurable module layout and ordering (`--structure <list>`).
- JSON and JSONC configuration file loader (`~/.config/rustfetch/config.jsonc`).
- Automatic CLI generator (`--gen-config`).
- ANSI terminal color palette blocks (8 standard + 8 bright).
- Multi-threaded concurrent probe pipeline using `std::thread::scope`.
- Strict zero-comment standard across entire Rust source tree.
- Comprehensive CI/CD with automated cross-platform release builds.
