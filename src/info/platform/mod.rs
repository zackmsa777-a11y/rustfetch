#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

use crate::utils::clean;
use std::env;

pub fn detect_user() -> String {
    #[cfg(target_os = "windows")]
    {
        return clean(
            &env::var("USERNAME")
                .or_else(|_| env::var("USER"))
                .unwrap_or_else(|_| "user".into()),
        );
    }
    #[cfg(not(target_os = "windows"))]
    {
        clean(
            &env::var("USER")
                .or_else(|_| env::var("LOGNAME"))
                .unwrap_or_else(|_| "user".into()),
        )
    }
}

pub fn detect_hostname() -> String {
    #[cfg(target_os = "windows")]
    {
        if let Ok(name) = env::var("COMPUTERNAME") {
            let c = clean(name.trim());
            if !c.is_empty() {
                return c;
            }
        }
        return windows::hostname::detect_hostname().unwrap_or_else(|| "localhost".into());
    }
    #[cfg(target_os = "macos")]
    {
        if let Some(name) = macos::hostname::detect_hostname() {
            return name;
        }
        return env::var("HOSTNAME")
            .ok()
            .map(|s| clean(s.trim()))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "localhost".into());
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        use std::fs;
        clean(
            fs::read_to_string("/etc/hostname")
                .or_else(|_| fs::read_to_string("/proc/sys/kernel/hostname"))
                .or_else(|_| env::var("HOSTNAME"))
                .unwrap_or_else(|_| "localhost".into())
                .trim(),
        )
    }
}

#[allow(dead_code)]
pub fn format_usage_gib(used: u64, total: u64) -> Option<String> {
    if total == 0 || used > total {
        return None;
    }
    let total_gib = total as f64 / 1073741824.0;
    let used_gib = used as f64 / 1073741824.0;
    let pct = ((used as f64 / total as f64) * 100.0).round() as u64;
    Some(format!("{used_gib:.2} GiB / {total_gib:.2} GiB ({pct}%)"))
}
