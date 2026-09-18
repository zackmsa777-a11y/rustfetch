#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use crate::utils::{clean, read_first_line};
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use std::fs;

#[cfg(target_os = "macos")]
pub fn detect_battery_and_power() -> (Option<String>, Option<String>) {
    crate::info::platform::macos::battery::detect_battery_and_power()
}

#[cfg(target_os = "windows")]
pub fn detect_battery_and_power() -> (Option<String>, Option<String>) {
    (None, None)
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn detect_battery_and_power() -> (Option<String>, Option<String>) {
    let mut battery_str = None;
    let mut power_adapter_str = None;

    if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if (name.starts_with("BAT") || name.starts_with("battery")) && battery_str.is_none() {
                let cap =
                    read_first_line(path.join("capacity")).and_then(|c| c.parse::<u32>().ok());
                let status =
                    read_first_line(path.join("status")).unwrap_or_else(|| "Unknown".into());

                if let Some(c) = cap {
                    battery_str = Some(format!("{c}% [{}]", clean(&status)));
                }
            } else if (name.starts_with("AC")
                || name.starts_with("ADP")
                || name.starts_with("Mains"))
                && power_adapter_str.is_none()
            {
                let online =
                    read_first_line(path.join("online")).and_then(|o| o.parse::<u32>().ok());
                if let Some(o) = online {
                    power_adapter_str = Some(if o == 1 {
                        "Connected".to_string()
                    } else {
                        "Disconnected".to_string()
                    });
                }
            }
        }
    }

    (battery_str, power_adapter_str)
}
