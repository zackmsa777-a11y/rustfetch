use super::ffi;
use crate::info::uptime::format_uptime;

pub fn detect_uptime() -> Option<String> {
    unsafe {
        let ms = ffi::GetTickCount64();
        Some(format_uptime(ms / 1000))
    }
}
