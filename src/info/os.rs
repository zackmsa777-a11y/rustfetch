use crate::utils::{clean, value};
use std::env;
use std::ffi::CStr;
use std::fs;
use std::path::Path;

pub fn detect_os() -> (Option<String>, String, String) {
    let os_release = fs::read_to_string("/etc/os-release")
        .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
        .unwrap_or_default();

    let raw_id = value(&os_release, "ID", '=').unwrap_or("").to_lowercase();
    let id_like = value(&os_release, "ID_LIKE", '=')
        .unwrap_or("")
        .to_lowercase();
    let name = value(&os_release, "NAME", '=');
    let version = value(&os_release, "VERSION", '=');
    let pretty_name = value(&os_release, "PRETTY_NAME", '=');

    let arch = env::consts::ARCH;

    let normalized_id = if !raw_id.is_empty() {
        match raw_id.as_str() {
            "ol" => "oracle",
            "rhel" => "redhat",
            "rocky" | "rockylinux" => "rocky",
            "almalinux" | "alma" => "almalinux",
            "archarm" => "arch",
            "linuxmint" => "mint",
            "pop" | "pop_os" => "pop",
            "opensuse-tumbleweed" | "opensuse-leap" | "suse" => "opensuse",
            "raspbian" => "raspberry",
            "parrotos" => "parrot",
            "elementaryos" => "elementary",
            "mxlinux" => "mx",
            known => known,
        }
        .to_string()
    } else if Path::new("/etc/arch-release").exists() {
        "arch".to_string()
    } else if Path::new("/etc/debian_version").exists() {
        "debian".to_string()
    } else if Path::new("/etc/alpine-release").exists() {
        "alpine".to_string()
    } else if Path::new("/etc/gentoo-release").exists() {
        "gentoo".to_string()
    } else if Path::new("/etc/redhat-release").exists() {
        "redhat".to_string()
    } else if Path::new("/etc/void-release").exists() {
        "void".to_string()
    } else {
        let mut uts: libc::utsname = unsafe { std::mem::zeroed() };
        if unsafe { libc::uname(&mut uts) } == 0 {
            let sysname = unsafe { CStr::from_ptr(uts.sysname.as_ptr()) }
                .to_string_lossy()
                .to_lowercase();
            match sysname.as_str() {
                "darwin" => "macos".to_string(),
                "freebsd" => "freebsd".to_string(),
                "openbsd" => "openbsd".to_string(),
                "netbsd" => "netbsd".to_string(),
                "dragonfly" => "dragonfly".to_string(),
                "haiku" => "haiku".to_string(),
                "sunos" => "solaris".to_string(),
                _ => {
                    if !id_like.is_empty() {
                        if id_like.contains("arch") {
                            "arch".to_string()
                        } else if id_like.contains("debian") {
                            "debian".to_string()
                        } else if id_like.contains("fedora") || id_like.contains("rhel") {
                            "fedora".to_string()
                        } else if id_like.contains("suse") {
                            "opensuse".to_string()
                        } else {
                            "linux".to_string()
                        }
                    } else {
                        "linux".to_string()
                    }
                }
            }
        } else {
            "linux".to_string()
        }
    };

    let distro_name = name
        .or(pretty_name)
        .unwrap_or(normalized_id.as_str())
        .to_string();

    let base = match (name, version, pretty_name) {
        (Some(n), Some(v), _) => {
            if v.starts_with(n) {
                v.to_string()
            } else {
                format!("{n} {v}")
            }
        }
        (_, _, Some(p)) => p.to_string(),
        (Some(n), None, None) => n.to_string(),
        _ => {
            if let Ok(issue) = fs::read_to_string("/etc/issue") {
                let first = issue.lines().next().unwrap_or("").trim();
                let stripped = first
                    .replace("\\n", "")
                    .replace("\\l", "")
                    .trim()
                    .to_string();
                if !stripped.is_empty() {
                    stripped
                } else {
                    distro_name.clone()
                }
            } else {
                distro_name.clone()
            }
        }
    };

    let cleaned_base = clean(&base);
    let os_str = if cleaned_base.contains(arch) {
        cleaned_base
    } else {
        format!("{cleaned_base} {arch}")
    };

    (Some(os_str), normalized_id, distro_name)
}
