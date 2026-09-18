use crate::utils::clean;
use std::ffi::CStr;

pub fn detect_kernel() -> Option<String> {
    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    if unsafe { libc::uname(&mut uts) } != 0 {
        return None;
    }
    let sysname = unsafe { CStr::from_ptr(uts.sysname.as_ptr()) }.to_string_lossy();
    let release = unsafe { CStr::from_ptr(uts.release.as_ptr()) }.to_string_lossy();
    Some(format!("{} {}", clean(&sysname), clean(&release)))
}
