pub mod audio;
pub mod battery;
pub mod command;
pub mod cpu;
pub mod de_wm;
pub mod disk;
pub mod display;
pub mod gpu;
pub mod host;
pub mod kernel;
pub mod locale;
pub mod memory;
pub mod network;
pub mod os;
pub mod platform;
pub mod packages;
pub mod shell;
pub mod swap;
pub mod terminal;
pub mod types;
pub mod uptime;

use std::thread;
use types::SystemInfo;

fn wants(only: Option<&[String]>, name: &str) -> bool {
    match only {
        None => true,
        Some(list) => list.iter().any(|m| {
            let n = m.as_str();
            n == name
                || (name == "wm_theme" && (n == "wmtheme" || n == "wm-theme"))
                || (name == "terminal_font" && (n == "terminalfont" || n == "terminal-font"))
                || (name == "local_ip" && (n == "localip" || n == "local-ip"))
                || (name == "power_adapter" && (n == "poweradapter" || n == "power-adapter"))
        }),
    }
}

pub fn gather_info(custom_disks: Option<&[String]>, only: Option<&[String]>) -> SystemInfo {
    let need_title = wants(only, "title");
    let need_os = only.is_none() || wants(only, "os");
    let need_host = wants(only, "host");
    let need_kernel = wants(only, "kernel");
    let need_uptime = wants(only, "uptime");
    let need_packages = wants(only, "packages");
    let need_shell = wants(only, "shell");
    let need_display = wants(only, "display");
    let need_de = wants(only, "de");
    let need_wm = wants(only, "wm");
    let need_wm_theme = wants(only, "wm_theme");
    let need_theme = wants(only, "theme");
    let need_icons = wants(only, "icons");
    let need_font = wants(only, "font");
    let need_cursor = wants(only, "cursor");
    let need_terminal = wants(only, "terminal");
    let need_terminal_font = wants(only, "terminal_font");
    let need_cpu = wants(only, "cpu");
    let need_gpu = wants(only, "gpu");
    let need_memory = wants(only, "memory");
    let need_swap = wants(only, "swap");
    let need_disk = wants(only, "disk");
    let need_battery = wants(only, "battery");
    let need_power = wants(only, "power_adapter");
    let need_audio = wants(only, "audio");
    let need_ip = wants(only, "local_ip");
    let need_locale = wants(only, "locale");

    let need_desktop_full =
        need_de || need_wm_theme || need_theme || need_icons || need_font || need_cursor;
    let need_desktop = need_desktop_full || need_wm;
    let need_term = need_terminal || need_terminal_font;
    let need_bat = need_battery || need_power;

    let user = if need_title || only.is_none() {
        platform::detect_user()
    } else {
        String::new()
    };
    let hostname = if need_title || only.is_none() {
        platform::detect_hostname()
    } else {
        String::new()
    };

    let (os_val, distro_id, distro_name) = if need_os || only.is_some() {
        os::detect_os()
    } else {
        (None, String::new(), String::new())
    };
    let os_val = if need_os { os_val } else { None };

    let kernel_val = if need_kernel {
        kernel::detect_kernel()
    } else {
        None
    };
    let uptime_val = if need_uptime {
        uptime::detect_uptime()
    } else {
        None
    };
    let memory_val = if need_memory {
        memory::detect_memory()
    } else {
        None
    };
    let swap_val = if need_swap { swap::detect_swap() } else { None };
    let locale_val = if need_locale {
        locale::detect_locale()
    } else {
        None
    };

    let (
        packages_val,
        shell_val,
        desktop_info,
        term_info,
        cpu_val,
        gpu_val,
        disk_val,
        battery_pair,
        display_val,
        local_ip_val,
        host_val,
        audio_val,
    ) = thread::scope(|s| {
        let h_pkg = need_packages.then(|| s.spawn(packages::detect_packages));
        let h_shell = need_shell.then(|| s.spawn(shell::detect_shell));
        let h_desktop = need_desktop.then(|| {
            s.spawn(move || {
                if need_desktop_full {
                    de_wm::detect_de_wm()
                } else {
                    de_wm::DesktopInfo {
                        de: None,
                        wm: de_wm::detect_wm(),
                        wm_theme: None,
                        theme: None,
                        icons: None,
                        font: None,
                        cursor: None,
                    }
                }
            })
        });
        let h_term = need_term.then(|| s.spawn(terminal::detect_terminal));
        let h_cpu = need_cpu.then(|| s.spawn(cpu::detect_cpu));
        let h_gpu = need_gpu.then(|| s.spawn(gpu::detect_gpu));
        let h_disk = need_disk.then(|| s.spawn(|| disk::detect_disks(custom_disks)));
        let h_bat = need_bat.then(|| s.spawn(battery::detect_battery_and_power));
        let h_disp = need_display.then(|| s.spawn(display::detect_display));
        let h_ip = need_ip.then(|| s.spawn(network::detect_local_ip));
        let h_host = need_host.then(|| s.spawn(host::detect_host));
        let h_audio = need_audio.then(|| s.spawn(audio::detect_audio));

        (
            h_pkg.map(|h| h.join().unwrap_or(None)).unwrap_or(None),
            h_shell.map(|h| h.join().unwrap_or(None)).unwrap_or(None),
            h_desktop
                .map(|h| {
                    h.join().unwrap_or(de_wm::DesktopInfo {
                        de: None,
                        wm: None,
                        wm_theme: None,
                        theme: None,
                        icons: None,
                        font: None,
                        cursor: None,
                    })
                })
                .unwrap_or(de_wm::DesktopInfo {
                    de: None,
                    wm: None,
                    wm_theme: None,
                    theme: None,
                    icons: None,
                    font: None,
                    cursor: None,
                }),
            h_term
                .map(|h| h.join().unwrap_or((None, None)))
                .unwrap_or((None, None)),
            h_cpu.map(|h| h.join().unwrap_or(None)).unwrap_or(None),
            h_gpu.map(|h| h.join().unwrap_or(None)).unwrap_or(None),
            h_disk.map(|h| h.join().unwrap_or(None)).unwrap_or(None),
            h_bat
                .map(|h| h.join().unwrap_or((None, None)))
                .unwrap_or((None, None)),
            h_disp.map(|h| h.join().unwrap_or(None)).unwrap_or(None),
            h_ip.map(|h| h.join().unwrap_or(None)).unwrap_or(None),
            h_host.map(|h| h.join().unwrap_or(None)).unwrap_or(None),
            h_audio.map(|h| h.join().unwrap_or(None)).unwrap_or(None),
        )
    });

    let (terminal_val, terminal_font_val) = term_info;
    let (battery_val, power_adapter_val) = battery_pair;

    SystemInfo {
        user,
        hostname,
        os: os_val,
        host: host_val,
        kernel: kernel_val,
        uptime: uptime_val,
        packages: packages_val,
        shell: shell_val,
        display: display_val,
        de: desktop_info.de,
        wm: desktop_info.wm,
        wm_theme: desktop_info.wm_theme,
        theme: desktop_info.theme,
        icons: desktop_info.icons,
        font: desktop_info.font,
        cursor: desktop_info.cursor,
        terminal: if need_terminal { terminal_val } else { None },
        terminal_font: if need_terminal_font {
            terminal_font_val
        } else {
            None
        },
        cpu: cpu_val,
        gpu: gpu_val,
        memory: memory_val,
        swap: swap_val,
        disk: disk_val,
        battery: if need_battery { battery_val } else { None },
        power_adapter: if need_power { power_adapter_val } else { None },
        audio: audio_val,
        local_ip: local_ip_val,
        public_ip: None,
        locale: locale_val,
        distro_id,
        distro_name,
    }
}
