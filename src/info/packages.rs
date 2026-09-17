use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn detect_packages() -> Option<String> {
    let mut counts = Vec::new();

    if let Ok(content) = fs::read_to_string("/var/lib/dpkg/status") {
        let count = content
            .lines()
            .filter(|l| l.starts_with("Status: install ok installed"))
            .count();
        if count > 0 {
            counts.push(format!("{count} (dpkg)"));
        }
    }

    if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
        let count = entries.flatten().filter(|e| e.path().is_dir()).count();
        if count > 0 {
            counts.push(format!("{count} (pacman)"));
        }
    }

    if Path::new("/var/lib/rpm").exists() {
        if let Ok(output) = Command::new("rpm")
            .args(["-qa", "--nodigest", "--nosignature"])
            .output()
        {
            let count = String::from_utf8_lossy(&output.stdout).lines().count();
            if count > 0 {
                counts.push(format!("{count} (rpm)"));
            }
        }
    }

    let mut flatpak_count = 0;
    if let Ok(entries) = fs::read_dir("/var/lib/flatpak/app") {
        flatpak_count += entries.flatten().filter(|e| e.path().is_dir()).count();
    }
    if let Ok(home) = env::var("HOME") {
        if let Ok(entries) = fs::read_dir(Path::new(&home).join(".local/share/flatpak/app")) {
            flatpak_count += entries.flatten().filter(|e| e.path().is_dir()).count();
        }
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

    if Path::new("/nix/var/nix/profiles/default").exists() {
        if let Ok(output) = Command::new("nix-store")
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
    }

    if let Ok(content) = fs::read_to_string("/lib/apk/db/installed") {
        let count = content.lines().filter(|l| l.starts_with("P:")).count();
        if count > 0 {
            counts.push(format!("{count} (apk)"));
        }
    }

    if let Ok(entries) = fs::read_dir("/var/db/pkg") {
        let mut count = 0;
        for cat in entries.flatten() {
            if cat.path().is_dir() {
                if let Ok(pkgs) = fs::read_dir(cat.path()) {
                    count += pkgs.flatten().filter(|p| p.path().is_dir()).count();
                }
            }
        }
        if count > 0 {
            counts.push(format!("{count} (emerge)"));
        }
    }

    if let Ok(entries) = fs::read_dir("/var/db/xbps") {
        let count = entries
            .flatten()
            .filter(|e| e.file_name().to_string_lossy().starts_with("pkg-"))
            .count();
        if count > 0 {
            counts.push(format!("{count} (xbps)"));
        }
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
