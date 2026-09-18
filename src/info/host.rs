#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use crate::utils::{clean, read_first_line, run_cmd};
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use std::path::Path;

#[cfg(target_os = "macos")]
pub fn detect_host() -> Option<String> {
    crate::info::platform::macos::host::detect_host()
}

#[cfg(target_os = "windows")]
pub fn detect_host() -> Option<String> {
    crate::info::platform::windows::host::detect_host()
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn detect_host() -> Option<String> {
    if Path::new("/.dockerenv").exists() {
        return Some("Docker Container".to_string());
    }
    if Path::new("/run/.containerenv").exists() {
        return Some("Podman Container".to_string());
    }

    if let Some(arm_model) = read_first_line("/proc/device-tree/model") {
        let trimmed = arm_model.trim_matches(char::from(0)).trim();
        if !trimmed.is_empty() {
            return Some(clean(trimmed));
        }
    }

    let vendor = read_first_line("/sys/devices/virtual/dmi/id/sys_vendor")
        .or_else(|| read_first_line("/sys/class/dmi/id/sys_vendor"));
    let product = read_first_line("/sys/devices/virtual/dmi/id/product_name")
        .or_else(|| read_first_line("/sys/class/dmi/id/product_name"));
    let version = read_first_line("/sys/devices/virtual/dmi/id/product_version")
        .or_else(|| read_first_line("/sys/class/dmi/id/product_version"));
    let board = read_first_line("/sys/devices/virtual/dmi/id/board_name")
        .or_else(|| read_first_line("/sys/class/dmi/id/board_name"));

    let virt = run_cmd("systemd-detect-virt", &[]);

    let filter_junk = |s: Option<String>| -> Option<String> {
        s.map(|v| clean(&v)).filter(|v| {
            let l = v.to_lowercase();
            !l.is_empty()
                && l != "none"
                && l != "to be filled by o.e.m."
                && l != "default string"
                && l != "system manufacturer"
                && l != "system product name"
                && l != "system version"
                && l != "type1productconfigid"
        })
    };

    let vendor = filter_junk(vendor);
    let product = filter_junk(product).or_else(|| filter_junk(board));
    let version = filter_junk(version);

    let mut parts = Vec::new();

    if let Some(ref v) = virt {
        let v_clean = clean(v);
        if v_clean.eq_ignore_ascii_case("kvm") || v_clean.eq_ignore_ascii_case("qemu") {
            parts.push("KVM/QEMU".to_string());
        } else if !v_clean.is_empty() && !v_clean.eq_ignore_ascii_case("none") {
            parts.push(v_clean.to_uppercase());
        }
    }

    match (vendor, product) {
        (Some(v), Some(p)) => {
            if parts.is_empty() && !p.to_lowercase().contains(&v.to_lowercase()) {
                parts.push(v);
            }
            parts.push(p);
        }
        (None, Some(p)) => {
            parts.push(p);
        }
        (Some(v), None) => {
            parts.push(v);
        }
        (None, None) => {
            if parts.is_empty() {
                return None;
            }
        }
    }

    let mut host_str = parts.join(" ");

    if let Some(ver) = version
        && !host_str.contains(&ver)
    {
        host_str.push_str(&format!(" ({ver})"));
    }

    Some(host_str)
}
