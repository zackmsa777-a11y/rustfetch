use super::ffi;
use crate::utils::clean;

pub fn detect_hostname() -> Option<String> {
    unsafe {
        let mut buf = [0u16; 256];
        let mut size = buf.len() as ffi::DWORD;
        if ffi::GetComputerNameW(buf.as_mut_ptr(), &mut size) == 0 {
            return None;
        }
        let name = ffi::from_wide_nul(&buf);
        let cleaned = clean(name.trim());
        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned)
        }
    }
}
