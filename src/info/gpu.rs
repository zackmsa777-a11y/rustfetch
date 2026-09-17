use crate::utils::{clean, read_first_line, run_cmd};
use std::fs;

pub fn detect_gpu() -> Option<String> {
    if let Some(nvidia) = run_cmd("nvidia-smi", &["--query-gpu=name", "--format=csv,noheader"]) {
        let first = nvidia.lines().next().unwrap_or("").trim();
        if !first.is_empty() {
            return Some(clean(first));
        }
    }

    if let Ok(entries) = fs::read_dir("/sys/bus/pci/devices") {
        for entry in entries.flatten() {
            let path = entry.path();
            let class = read_first_line(path.join("class")).unwrap_or_default();
            if class.trim().starts_with("0x03") {
                let vendor = read_first_line(path.join("vendor")).unwrap_or_default();
                let device = read_first_line(path.join("device")).unwrap_or_default();
                let v = vendor.trim().trim_start_matches("0x");
                let d = device.trim().trim_start_matches("0x");

                if v.eq_ignore_ascii_case("1234") && d.eq_ignore_ascii_case("1111") {
                    return Some("QEMU Virtual Video Controller".into());
                } else if v.eq_ignore_ascii_case("1af4") {
                    return Some("Red Hat VirtIO GPU".into());
                } else if v.eq_ignore_ascii_case("15ad") {
                    return Some("VMware SVGA II Adapter".into());
                } else if v.eq_ignore_ascii_case("80ee") {
                    return Some("VirtualBox Graphics Adapter".into());
                }
            }
        }
    }

    if let Some(lspci_out) = run_cmd("lspci", &[]) {
        for line in lspci_out.lines() {
            if (line.contains("VGA compatible controller")
                || line.contains("3D controller")
                || line.contains("Display controller"))
                && let Some((_, rest)) = line.split_once(": ")
            {
                let cleaned = rest.trim();
                if cleaned.contains("1234:1111") {
                    return Some("QEMU Virtual Video Controller".into());
                }
                return Some(clean(cleaned));
            }
        }
    }

    None
}
