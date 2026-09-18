use super::ffi;
use crate::utils::clean;

pub fn detect_kernel() -> Option<String> {
    unsafe {
        let mut info = ffi::OSVERSIONINFOW::zeroed();
        if ffi::RtlGetVersion(&mut info) != 0 {
            return None;
        }
        Some(clean(&format!(
            "WIN32_NT {}.{}.{}",
            info.dwMajorVersion, info.dwMinorVersion, info.dwBuildNumber
        )))
    }
}
