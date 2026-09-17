pub mod audio;
pub mod battery;
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
pub mod packages;
pub mod shell;
pub mod swap;
pub mod terminal;
pub mod types;
pub mod uptime;

use crate::utils::clean;
use std::env;
use std::fs;
use std::thread;
use types::SystemInfo;

pub fn gather_info(custom_disks: Option<&[String]>) -> SystemInfo {
    let user = clean(
        &env::var("USER")
            .or_else(|_| env::var("LOGNAME"))
            .unwrap_or_else(|_| "user".into()),
    );

    let hostname = clean(
        fs::read_to_string("/etc/hostname")
            .or_else(|_| fs::read_to_string("/proc/sys/kernel/hostname"))
            .or_else(|_| env::var("HOSTNAME"))
            .unwrap_or_else(|_| "localhost".into())
            .trim(),
    );

    let (os_val, distro_id, distro_name) = os::detect_os();
    let kernel_val = kernel::detect_kernel();
    let uptime_val = uptime::detect_uptime();
    let memory_val = memory::detect_memory();
    let swap_val = swap::detect_swap();
    let locale_val = locale::detect_locale();

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
        let h_pkg = s.spawn(packages::detect_packages);
        let h_shell = s.spawn(shell::detect_shell);
        let h_desktop = s.spawn(de_wm::detect_de_wm);
        let h_term = s.spawn(terminal::detect_terminal);
        let h_cpu = s.spawn(cpu::detect_cpu);
        let h_gpu = s.spawn(gpu::detect_gpu);
        let h_disk = s.spawn(|| disk::detect_disks(custom_disks));
        let h_bat = s.spawn(battery::detect_battery_and_power);
        let h_disp = s.spawn(display::detect_display);
        let h_ip = s.spawn(network::detect_local_ip);
        let h_host = s.spawn(host::detect_host);
        let h_audio = s.spawn(audio::detect_audio);

        (
            h_pkg.join().unwrap_or(None),
            h_shell.join().unwrap_or(None),
            h_desktop.join().unwrap_or(de_wm::DesktopInfo {
                de: None,
                wm: None,
                wm_theme: None,
                theme: None,
                icons: None,
                font: None,
                cursor: None,
            }),
            h_term.join().unwrap_or((None, None)),
            h_cpu.join().unwrap_or(None),
            h_gpu.join().unwrap_or(None),
            h_disk.join().unwrap_or(None),
            h_bat.join().unwrap_or((None, None)),
            h_disp.join().unwrap_or(None),
            h_ip.join().unwrap_or(None),
            h_host.join().unwrap_or(None),
            h_audio.join().unwrap_or(None),
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
        terminal: terminal_val,
        terminal_font: terminal_font_val,
        cpu: cpu_val,
        gpu: gpu_val,
        memory: memory_val,
        swap: swap_val,
        disk: disk_val,
        battery: battery_val,
        power_adapter: power_adapter_val,
        audio: audio_val,
        local_ip: local_ip_val,
        public_ip: None,
        locale: locale_val,
        distro_id,
        distro_name,
    }
}
