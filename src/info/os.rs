use crate::utils::{clean, value};
use std::env;
use std::fs;

pub fn detect_os() -> (Option<String>, String, String) {
    let os_release = fs::read_to_string("/etc/os-release")
        .or_else(|_| fs::read_to_string("/usr/lib/os-release"))
        .unwrap_or_default();

    let id = value(&os_release, "ID", '=')
        .unwrap_or("linux")
        .to_lowercase();
    let name = value(&os_release, "NAME", '=');
    let version = value(&os_release, "VERSION", '=');
    let pretty_name = value(&os_release, "PRETTY_NAME", '=');

    let distro_name = name.or(pretty_name).unwrap_or("Linux").to_string();

    let arch = env::consts::ARCH;

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
                    "Linux".to_string()
                }
            } else {
                "Linux".to_string()
            }
        }
    };

    let cleaned_base = clean(&base);
    let os_str = if cleaned_base.contains(arch) {
        cleaned_base
    } else {
        format!("{cleaned_base} {arch}")
    };

    (Some(os_str), id, distro_name)
}
