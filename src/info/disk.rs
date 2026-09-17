use std::ffi::CString;
use std::fs;

pub fn format_disk_mount(mount_path: &str) -> Option<String> {
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let c_path = CString::new(mount_path).ok()?;

    if unsafe { libc::statvfs(c_path.as_ptr(), &mut stat) } != 0 {
        return None;
    }

    let total = stat.f_blocks as u64 * stat.f_frsize as u64;
    let avail = stat.f_bavail as u64 * stat.f_frsize as u64;
    if total == 0 || avail > total {
        return None;
    }

    let used = total - avail;
    let total_gib = total as f64 / 1073741824.0;
    let used_gib = used as f64 / 1073741824.0;
    let pct = ((used as f64 / total as f64) * 100.0).round() as u64;

    let fs_type = fs::read_to_string("/proc/mounts")
        .ok()
        .and_then(|mounts| {
            for line in mounts.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 && parts[1] == mount_path {
                    return Some(parts[2].to_string());
                }
            }
            None
        })
        .unwrap_or_else(|| "ext4".into());

    Some(format!(
        "{mount_path}: {used_gib:.2} GiB / {total_gib:.2} GiB ({pct}%) - {fs_type}"
    ))
}

pub fn detect_disks(custom_paths: Option<&[String]>) -> Option<Vec<String>> {
    let paths_to_check: Vec<String> = if let Some(paths) = custom_paths {
        paths.to_vec()
    } else {
        let mut list = vec!["/".to_string()];
        if let Ok(mounts) = fs::read_to_string("/proc/mounts") {
            for line in mounts.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let mount = parts[1];
                    let fstype = parts[2];
                    if (mount == "/home"
                        || mount.starts_with("/mnt/")
                        || mount.starts_with("/media/"))
                        && fstype != "tmpfs"
                        && fstype != "devtmpfs"
                        && fstype != "squashfs"
                        && !list.contains(&mount.to_string())
                    {
                        list.push(mount.to_string());
                    }
                }
            }
        }
        list
    };

    let mut results = Vec::new();
    for p in paths_to_check {
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
