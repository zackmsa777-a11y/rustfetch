use crate::utils::{clean, read_first_line};
use std::ffi::CStr;

pub fn detect_kernel() -> Option<String> {
    if let Some(release) = read_first_line("/proc/sys/kernel/osrelease") {
        let ostype = read_first_line("/proc/sys/kernel/ostype").unwrap_or_else(|| "Linux".into());
        return Some(format!("{ostype} {}", clean(&release)));
    }

    let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
    if unsafe { libc::uname(&mut uts) } == 0 {
        let sysname = unsafe { CStr::from_ptr(uts.sysname.as_ptr()) }.to_string_lossy();
        let release = unsafe { CStr::from_ptr(uts.release.as_ptr()) }.to_string_lossy();
        Some(format!("{sysname} {release}"))
    } else {
        None
    }
}
