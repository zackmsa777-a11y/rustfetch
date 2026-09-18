use crate::utils::value;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use std::fs;

#[cfg_attr(any(target_os = "macos", target_os = "windows"), allow(dead_code))]
pub fn format_swap(input: &str) -> Option<String> {
    let total_str = value(input, "SwapTotal", ':')?;
    let free_str = value(input, "SwapFree", ':')?;

    let parse_kb = |s: &str| -> Option<u64> { s.split_whitespace().next()?.parse::<u64>().ok() };

    let total = parse_kb(total_str)?;
    let free = parse_kb(free_str)?;

    if total == 0 {
        return Some("Disabled".into());
    }
    if free > total {
        return None;
    }

    let used = total - free;
    let total_gib = total as f64 / 1048576.0;
    let used_gib = used as f64 / 1048576.0;
    let pct = ((used as f64 / total as f64) * 100.0).round() as u64;

    Some(format!("{used_gib:.2} GiB / {total_gib:.2} GiB ({pct}%)"))
}

#[cfg(target_os = "macos")]
pub fn detect_swap() -> Option<String> {
    crate::info::platform::macos::swap::detect_swap()
}

#[cfg(target_os = "windows")]
pub fn detect_swap() -> Option<String> {
    crate::info::platform::windows::swap::detect_swap()
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn detect_swap() -> Option<String> {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    format_swap(&meminfo)
}
