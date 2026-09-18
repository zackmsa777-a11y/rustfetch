use super::ffi;
use crate::info::platform::format_usage_gib;

pub fn detect_swap() -> Option<String> {
    unsafe {
        let mut status = ffi::MEMORYSTATUSEX::zeroed();
        if ffi::GlobalMemoryStatusEx(&mut status) == 0 {
            return None;
        }
        let total = status.ullTotalPageFile.saturating_sub(status.ullTotalPhys);
        let avail = status.ullAvailPageFile.saturating_sub(status.ullAvailPhys);
        if total == 0 {
            return Some("Disabled".into());
        }
        let used = total.saturating_sub(avail.min(total));
        format_usage_gib(used, total)
    }
}
