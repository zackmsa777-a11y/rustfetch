use crate::utils::clean;
use std::env;
use std::path::Path;
use std::process::Command;

pub fn detect_shell() -> Option<String> {
    let shell_path = env::var("SHELL").ok()?;
    let shell_name = Path::new(&shell_path)
        .file_name()
        .and_then(|f| f.to_str())
        .unwrap_or(&shell_path)
        .to_string();

    let version = Command::new(&shell_path)
        .arg("--version")
        .output()
        .ok()
        .and_then(|out| {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for raw_word in stdout.split_whitespace() {
                let clean_word = raw_word
                    .split(&['(', '-', '+', '~', ','][..])
                    .next()
                    .unwrap_or(raw_word);
                let trimmed = clean_word.trim_matches(|c: char| !c.is_ascii_digit() && c != '.');
                let parts: Vec<&str> = trimmed.split('.').collect();
                if parts.len() >= 2
                    && parts[0].chars().all(|c| c.is_ascii_digit())
                    && parts[1].chars().all(|c| c.is_ascii_digit())
                    && parts
                        .iter()
                        .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
                {
                    return Some(trimmed.to_string());
                }
            }
            None
        });

    if let Some(ver) = version {
        Some(format!("{shell_name} {ver}"))
    } else {
        Some(clean(&shell_name))
    }
}
