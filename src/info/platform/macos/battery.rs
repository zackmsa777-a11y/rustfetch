use crate::info::command::DEFAULT_TIMEOUT_MS;
use crate::utils::clean;
use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub fn detect_battery_and_power() -> (Option<String>, Option<String>) {
    if let Some(out) = run_pmset() {
        return parse_pmset(&out);
    }
    (None, None)
}

fn run_pmset() -> Option<String> {
    let timeout = Duration::from_millis(DEFAULT_TIMEOUT_MS);
    let mut child = Command::new("pmset")
        .args(["-g", "batt"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut buf = String::new();
                if let Some(mut out) = child.stdout.take() {
                    let _ = out.read_to_string(&mut buf);
                }
                if status.success() && !buf.trim().is_empty() {
                    return Some(buf);
                }
                return None;
            }
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
            Ok(None) => thread::sleep(Duration::from_millis(15)),
            Err(_) => return None,
        }
    }
}

fn parse_pmset(out: &str) -> (Option<String>, Option<String>) {
    let mut battery = None;
    let mut adapter = None;

    let joined = out.replace('\n', " ");
    if let Some(idx) = joined.find('%') {
        let before = &joined[..idx];
        let pct = before
            .rsplit(|c: char| !c.is_ascii_digit())
            .next()
            .and_then(|s| s.parse::<u32>().ok());
        let status = if joined.to_lowercase().contains("charging")
            && !joined.to_lowercase().contains("discharging")
        {
            "Charging"
        } else if joined.to_lowercase().contains("discharging") {
            "Discharging"
        } else if joined.to_lowercase().contains("charged") {
            "Full"
        } else {
            "Unknown"
        };
        if let Some(p) = pct {
            battery = Some(format!("{p}% [{}]", clean(status)));
        }
    }

    if joined.to_lowercase().contains("ac power") {
        adapter = Some("Connected".into());
    } else if joined.to_lowercase().contains("battery power") {
        adapter = Some("Disconnected".into());
    }

    (battery, adapter)
}

#[cfg(test)]
mod parse_tests {
    use super::parse_pmset;

    #[test]
    fn parses_sample_pmset() {
        let sample = "Now drawing from 'Battery Power'\n -InternalBattery-0 (id=123) 78%; discharging; 3:21 remaining present: true\n";
        let (bat, adp) = parse_pmset(sample);
        assert!(bat.unwrap().starts_with("78%"));
        assert_eq!(adp.as_deref(), Some("Disconnected"));
    }
}
