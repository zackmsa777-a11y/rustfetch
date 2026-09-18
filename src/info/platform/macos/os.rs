use super::sysctl;
use crate::utils::clean;
use std::env;
use std::fs;

pub fn detect_os() -> (Option<String>, String, String) {
    let arch = env::consts::ARCH;
    let version = sysctl::sysctl_string("kern.osproductversion")
        .or_else(read_product_version)
        .unwrap_or_else(|| "Unknown".into());
    let name = read_product_name().unwrap_or_else(|| "macOS".into());
    let distro_name = name.clone();
    let os_str = clean(&format!("{name} {version} {arch}"));
    (Some(os_str), "macos".into(), distro_name)
}

fn read_product_version() -> Option<String> {
    let plist = fs::read_to_string("/System/Library/CoreServices/SystemVersion.plist").ok()?;
    plist_string_value(&plist, "ProductVersion")
}

fn read_product_name() -> Option<String> {
    let plist = fs::read_to_string("/System/Library/CoreServices/SystemVersion.plist").ok()?;
    plist_string_value(&plist, "ProductName").map(|n| {
        if n.eq_ignore_ascii_case("Mac OS X") || n.eq_ignore_ascii_case("macOS") {
            "macOS".into()
        } else {
            n
        }
    })
}

fn plist_string_value(plist: &str, key: &str) -> Option<String> {
    let key_tag = format!("<key>{key}</key>");
    let idx = plist.find(&key_tag)?;
    let after = &plist[idx + key_tag.len()..];
    let start = after.find("<string>")? + "<string>".len();
    let rest = &after[start..];
    let end = rest.find("</string>")?;
    let val = rest[..end].trim();
    if val.is_empty() { None } else { Some(val.to_string()) }
}
