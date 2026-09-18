use super::ffi;
use crate::utils::clean;
use std::env;

pub fn detect_os() -> (Option<String>, String, String) {
    let arch = env::consts::ARCH;
    let (major, minor, build) = version_numbers();
    let product = ffi::reg_sz(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "ProductName",
    )
    .unwrap_or_else(|| {
        if major >= 10 {
            "Windows".into()
        } else {
            format!("Windows {major}.{minor}")
        }
    });
    let display = ffi::reg_sz(
        r"SOFTWARE\Microsoft\Windows NT\CurrentVersion",
        "DisplayVersion",
    );
    let mut base = product;
    if let Some(d) = display {
        base.push(' ');
        base.push_str(&d);
    } else {
        base.push_str(&format!(" ({build})"));
    }
    let os_str = clean(&format!("{base} {arch}"));
    (Some(os_str), "windows".into(), "Windows".into())
}

fn version_numbers() -> (u32, u32, u32) {
    unsafe {
        let mut info = ffi::OSVERSIONINFOW::zeroed();
        if ffi::RtlGetVersion(&mut info) == 0 {
            (info.dwMajorVersion, info.dwMinorVersion, info.dwBuildNumber)
        } else {
            (10, 0, 0)
        }
    }
}
