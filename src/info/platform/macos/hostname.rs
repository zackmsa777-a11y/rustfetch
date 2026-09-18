use crate::utils::clean;
use std::ffi::CStr;

pub fn detect_hostname() -> Option<String> {
    let mut buf = [0i8; 256];
    let rc = unsafe { libc::gethostname(buf.as_mut_ptr(), buf.len()) };
    if rc != 0 {
        return None;
    }
    let name = unsafe { CStr::from_ptr(buf.as_ptr()) }
        .to_string_lossy()
        .trim()
        .to_string();
    let cleaned = clean(&name);
    if cleaned.is_empty() { None } else { Some(cleaned) }
}
