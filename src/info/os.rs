use crate::utils::{clean, value};
use std::env;
use std::ffi::CStr;
use std::fs;
use std::path::Path;

pub fn detect_os() -> (Option<String>, String, String) {
    let bedrock_restricted = env::var("BEDROCK_RESTRICT")
        .map(|v| v == "1")
        .unwrap_or(false);
    let os_release =
        if !bedrock_restricted && Path::new("/bedrock/strata/bedrock/etc/os-release").exists() {
            fs::read_to_string("/bedrock/strata/bedrock/etc/os-release").unwrap_or_default()
        } else {
            fs::read_to_string("/etc/os-release")
                .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
                .unwrap_or_default()
        };

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
            "bedrock" | "bedrocklinux" => "bedrock",
            "cachyos" | "cachy" => "cachyos",
            known => known,
        }
        .to_string()
    } else if !bedrock_restricted && Path::new("/bedrock/etc/bedrock-release").exists() {
        "bedrock".to_string()
    } else if Path::new("/etc/cachyos-release").exists() {
        "cachyos".to_string()
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

    let bedrock_release = if !bedrock_restricted
        && (normalized_id == "bedrock" || Path::new("/bedrock/etc/bedrock-release").exists())
    {
        fs::read_to_string("/bedrock/etc/bedrock-release").ok()
    } else {
        None
    };

    let distro_name = name
        .or(pretty_name)
        .or_else(|| {
            bedrock_release
                .as_deref()
                .and_then(|c| c.lines().next().map(|l| l.trim()))
        })
        .unwrap_or(if normalized_id == "bedrock" {
            "Bedrock Linux"
        } else if normalized_id == "cachyos" {
            "CachyOS"
        } else if normalized_id == "gentoo" {
            "Gentoo Linux"
        } else {
            normalized_id.as_str()
        })
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
            if let Some(ref rel) = bedrock_release {
                let first = rel.lines().next().unwrap_or("").trim();
                if !first.is_empty() {
                    first.to_string()
                } else {
                    distro_name.clone()
                }
            } else if let Ok(issue) = fs::read_to_string("/etc/issue") {
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
