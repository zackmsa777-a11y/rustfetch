#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use std::fs;

pub fn format_uptime(total_seconds: u64) -> String {
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
    }
    if hours > 0 || days > 0 {
        parts.push(format!(
            "{} hour{}",
            hours,
            if hours == 1 { "" } else { "s" }
        ));
    }
    if minutes > 0 || (hours > 0 || days > 0) {
        parts.push(format!(
            "{} min{}",
            minutes,
            if minutes == 1 { "" } else { "s" }
        ));
    } else {
        parts.push(format!(
            "{} sec{}",
            seconds,
            if seconds == 1 { "" } else { "s" }
        ));
    }

    parts.join(", ")
}

#[cfg_attr(any(target_os = "macos", target_os = "windows"), allow(dead_code))]
pub fn parse_uptime_str(input: &str) -> Option<String> {
    let first = input.split_whitespace().next()?;
    let secs: f64 = first.parse().ok()?;
    if secs < 0.0 || !secs.is_finite() {
        return None;
    }
    Some(format_uptime(secs as u64))
}

#[cfg(target_os = "macos")]
pub fn detect_uptime() -> Option<String> {
    crate::info::platform::macos::uptime::detect_uptime()
}

#[cfg(target_os = "windows")]
pub fn detect_uptime() -> Option<String> {
    crate::info::platform::windows::uptime::detect_uptime()
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn detect_uptime() -> Option<String> {
    if let Ok(content) = fs::read_to_string("/proc/uptime")
        && let Some(parsed) = parse_uptime_str(&content)
    {
        return Some(parsed);
    }

    #[cfg(target_os = "linux")]
    {
        let mut sys: libc::sysinfo = unsafe { std::mem::zeroed() };
        if unsafe { libc::sysinfo(&mut sys) } == 0 && sys.uptime >= 0 {
            return Some(format_uptime(sys.uptime as u64));
        }
    }
    None
}
