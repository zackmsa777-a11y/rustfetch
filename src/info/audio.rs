use crate::utils::clean;
use std::fs;

pub fn detect_audio() -> Option<String> {
    if let Ok(cards) = fs::read_to_string("/proc/asound/cards") {
        for line in cards.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with(|c: char| c.is_ascii_digit()) && trimmed.contains(':') {
                if let Some((_, rest)) = trimmed.split_once(" - ") {
                    return Some(clean(rest.trim()));
                } else if let Some((_, rest)) = trimmed.split_once(':') {
                    return Some(clean(rest.trim()));
                }
            }
        }
    }

    None
}
