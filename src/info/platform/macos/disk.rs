use crate::info::platform::format_usage_gib;
use std::ffi::{CStr, CString};
use std::fs;

pub fn detect_disks(custom_paths: Option<&[String]>) -> Option<Vec<String>> {
    let paths: Vec<String> = if let Some(paths) = custom_paths {
        paths.to_vec()
    } else {
        let mut list = vec!["/".to_string()];
        if let Ok(entries) = fs::read_dir("/Volumes") {
            for e in entries.flatten() {
                let p = e.path();
                if p.is_dir() {
                    let s = p.to_string_lossy().to_string();
                    if s != "/Volumes/Macintosh HD" && !list.contains(&s) {
                        list.push(s);
                    }
                }
            }
        }
        list
    };

    let mut results = Vec::new();
    for p in paths {
        if let Some(info) = format_disk_mount(&p) {
            results.push(info);
        }
    }
    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

fn format_disk_mount(mount_path: &str) -> Option<String> {
    let c_path = CString::new(mount_path).ok()?;
    let mut st: libc::statfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statfs(c_path.as_ptr(), &mut st) } != 0 {
        return None;
    }
    let total = st.f_blocks.saturating_mul(st.f_bsize as u64);
    let avail = st.f_bavail.saturating_mul(st.f_bsize as u64);
    if total == 0 || avail > total {
        return None;
    }
    let used = total - avail;
    let usage = format_usage_gib(used, total)?;
    let fstype = unsafe { CStr::from_ptr(st.f_fstypename.as_ptr()) }
        .to_string_lossy()
        .to_string();
    let fstype = if fstype.is_empty() {
        "apfs".into()
    } else {
        fstype
    };
    Some(format!("{mount_path}: {usage} - {fstype}"))
}
