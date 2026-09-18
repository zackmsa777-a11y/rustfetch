use super::ffi;
use crate::info::platform::format_usage_gib;
use std::env;

pub fn detect_disks(custom_paths: Option<&[String]>) -> Option<Vec<String>> {
    let paths: Vec<String> = if let Some(paths) = custom_paths {
        paths.to_vec()
    } else {
        let mut list = Vec::new();
        if let Ok(home) = env::var("SystemDrive") {
            let root = if home.ends_with('\\') || home.ends_with('/') {
                home
            } else {
                format!("{home}\\")
            };
            list.push(root);
        } else {
            list.push("C:\\".into());
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
    unsafe {
        let path = ffi::wide(mount_path);
        let mut avail: ffi::ULONGLONG = 0;
        let mut total: ffi::ULONGLONG = 0;
        let mut free: ffi::ULONGLONG = 0;
        if ffi::GetDiskFreeSpaceExW(path.as_ptr(), &mut avail, &mut total, &mut free) == 0 {
            return None;
        }
        if total == 0 || avail > total {
            return None;
        }
        let used = total - avail;
        let usage = format_usage_gib(used, total)?;
        Some(format!("{mount_path}: {usage} - ntfs"))
    }
}
