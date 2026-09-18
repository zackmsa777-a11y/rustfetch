#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use std::env;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use std::fs;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use std::path::Path;
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
use std::process::Command;

#[cfg(target_os = "macos")]
pub fn detect_packages() -> Option<String> {
    crate::info::platform::macos::packages::detect_packages()
}

#[cfg(target_os = "windows")]
pub fn detect_packages() -> Option<String> {
    None
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn detect_packages() -> Option<String> {
    let mut counts = Vec::new();

    let roots: Vec<std::path::PathBuf> = if let Ok(entries) = fs::read_dir("/bedrock/strata") {
        let dirs: Vec<_> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        if dirs.is_empty() {
            vec![std::path::PathBuf::from("/")]
        } else {
            dirs
        }
    } else {
        vec![std::path::PathBuf::from("/")]
    };

    let mut dpkg_count = 0;
    for root in &roots {
        if let Ok(content) = fs::read_to_string(root.join("var/lib/dpkg/status")) {
            dpkg_count += content
                .lines()
                .filter(|l| l.starts_with("Status: install ok installed"))
                .count();
        }
    }
    if dpkg_count > 0 {
        counts.push(format!("{dpkg_count} (dpkg)"));
    }

    let mut pacman_count = 0;
    for root in &roots {
        if let Ok(entries) = fs::read_dir(root.join("var/lib/pacman/local")) {
            pacman_count += entries.flatten().filter(|e| e.path().is_dir()).count();
        }
    }
    if pacman_count > 0 {
        counts.push(format!("{pacman_count} (pacman)"));
    }

    let has_rpm = roots.iter().any(|r| r.join("var/lib/rpm").exists());
    if has_rpm
        && let Ok(output) = Command::new("rpm")
            .args(["-qa", "--nodigest", "--nosignature"])
            .output()
    {
        let count = String::from_utf8_lossy(&output.stdout).lines().count();
        if count > 0 {
            counts.push(format!("{count} (rpm)"));
        }
    }

    let mut flatpak_count = 0;
    if let Ok(entries) = fs::read_dir("/var/lib/flatpak/app") {
        flatpak_count += entries.flatten().filter(|e| e.path().is_dir()).count();
    }
    if let Ok(home) = env::var("HOME")
        && let Ok(entries) = fs::read_dir(Path::new(&home).join(".local/share/flatpak/app"))
    {
        flatpak_count += entries.flatten().filter(|e| e.path().is_dir()).count();
    }
    if flatpak_count > 0 {
        counts.push(format!("{flatpak_count} (flatpak)"));
    }

    if let Ok(entries) = fs::read_dir("/snap") {
        let count = entries
            .flatten()
            .filter(|e| {
                let name = e.file_name();
                let s = name.to_string_lossy();
                s != "bin" && s != "README" && !s.starts_with('.')
            })
            .count();
        if count > 0 {
            counts.push(format!("{count} (snap)"));
        }
    }

    if Path::new("/nix/var/nix/profiles/default").exists()
        && let Ok(output) = Command::new("nix-store")
            .args(["--query", "--requisites", "/nix/var/nix/profiles/default"])
            .output()
    {
        let text = String::from_utf8_lossy(&output.stdout);
        let count = text
            .lines()
            .filter(|l| {
                if !Path::new(l).is_dir() {
                    return false;
                }
                let base = Path::new(l)
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("");
                if base.starts_with("nixos-system-nixos-")
                    || base.ends_with("-doc")
                    || base.ends_with("-man")
                    || base.ends_with("-info")
                    || base.ends_with("-dev")
                    || base.ends_with("-bin")
                {
                    return false;
                }
                let bytes = base.as_bytes();
                for i in 1..bytes.len().saturating_sub(1) {
                    if bytes[i] == b'.'
                        && bytes[i - 1].is_ascii_digit()
                        && bytes[i + 1].is_ascii_digit()
                    {
                        return true;
                    }
                }
                false
            })
            .count();
        if count > 0 {
            counts.push(format!("{count} (nix-default)"));
        }
    }

    let mut apk_count = 0;
    for root in &roots {
        if let Ok(content) = fs::read_to_string(root.join("lib/apk/db/installed")) {
            apk_count += content.lines().filter(|l| l.starts_with("P:")).count();
        }
    }
    if apk_count > 0 {
        counts.push(format!("{apk_count} (apk)"));
    }

    let mut emerge_count = 0;
    for root in &roots {
        if let Ok(entries) = fs::read_dir(root.join("var/db/pkg")) {
            for cat in entries.flatten() {
                if cat.path().is_dir()
                    && let Ok(pkgs) = fs::read_dir(cat.path())
                {
                    emerge_count += pkgs.flatten().filter(|p| p.path().is_dir()).count();
                }
            }
        }
    }
    if emerge_count > 0 {
        counts.push(format!("{emerge_count} (emerge)"));
    }

    let mut xbps_count = 0;
    for root in &roots {
        if let Ok(entries) = fs::read_dir(root.join("var/db/xbps")) {
            xbps_count += entries
                .flatten()
                .filter(|e| e.file_name().to_string_lossy().starts_with("pkg-"))
                .count();
        }
    }
    if xbps_count > 0 {
        counts.push(format!("{xbps_count} (xbps)"));
    }

    let brew_dirs = [
        "/opt/homebrew/Cellar",
        "/usr/local/Cellar",
        "/home/linuxbrew/.linuxbrew/Cellar",
    ];
    for b in &brew_dirs {
        if let Ok(entries) = fs::read_dir(b) {
            let count = entries.flatten().filter(|e| e.path().is_dir()).count();
            if count > 0 {
                counts.push(format!("{count} (brew)"));
                break;
            }
        }
    }

    if counts.is_empty() {
        None
    } else {
        Some(counts.join(", "))
    }
}
