use super::ffi;
use crate::info::platform::format_usage_gib;

pub fn detect_memory() -> Option<String> {
    unsafe {
        let mut status = ffi::MEMORYSTATUSEX::zeroed();
        if ffi::GlobalMemoryStatusEx(&mut status) == 0 {
            return None;
        }
        let total = status.ullTotalPhys;
        let avail = status.ullAvailPhys;
        if avail > total {
            return None;
        }
        format_usage_gib(total - avail, total)
    }
}
