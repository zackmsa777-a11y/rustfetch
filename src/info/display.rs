use crate::utils::{clean, run_cmd};
use std::fs;

pub fn detect_display() -> Option<String> {
    if let Ok(entries) = fs::read_dir("/sys/class/drm") {
        for entry in entries.flatten() {
            let path = entry.path();
            let modes_file = path.join("modes");
            if modes_file.is_file() {
                if let Ok(content) = fs::read_to_string(modes_file) {
                    if let Some(mode) = content.lines().next() {
                        let connector = entry
                            .file_name()
                            .to_string_lossy()
                            .trim_start_matches("card0-")
                            .trim_start_matches("card1-")
                            .to_string();
                        return Some(format!("{connector}: {mode}"));
                    }
                }
            }
        }
    }

    if let Some(out) = run_cmd("xrandr", &["--current"]) {
        for line in out.lines() {
            if line.contains(" connected ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(name) = parts.first() {
                    for part in parts.iter().skip(1) {
                        if part.contains('x') && part.contains('+') {
                            if let Some((res, _)) = part.split_once('+') {
                                return Some(format!("{name}: {res}"));
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(out) = run_cmd("hyprctl", &["monitors"]) {
        for line in out.lines() {
            let l = line.trim();
            if l.starts_with("Monitor ") {
                if let Some(name) = l.split_whitespace().nth(1) {
                    return Some(clean(name));
                }
            }
        }
    }

    None
}
