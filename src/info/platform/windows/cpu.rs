use super::ffi;
use crate::utils::clean;
use std::env;

pub fn detect_cpu() -> Option<String> {
    let brand = ffi::reg_sz(
        r"HARDWARE\DESCRIPTION\System\CentralProcessor\0",
        "ProcessorNameString",
    )
    .or_else(|| env::var("PROCESSOR_IDENTIFIER").ok())?;

    let mut name = clean(&brand);
    if let Some(idx) = name.find(" CPU @") {
        name.truncate(idx);
    } else if let Some(idx) = name.find(" @") {
        name.truncate(idx);
    }
    let name = name.trim().to_string();

    let count = unsafe {
        let mut info: ffi::SYSTEM_INFO = std::mem::zeroed();
        ffi::GetSystemInfo(&mut info);
        info.dwNumberOfProcessors
    };
    let count = if count == 0 {
        env::var("NUMBER_OF_PROCESSORS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    } else {
        count
    };

    let mut result = name;
    if count > 0 {
        result.push_str(&format!(" ({count})"));
    }
    Some(result)
}
