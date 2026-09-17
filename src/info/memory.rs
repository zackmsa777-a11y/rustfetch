use crate::utils::value;
use std::fs;

pub fn format_memory(input: &str) -> Option<String> {
    let total_str = value(input, "MemTotal", ':')?;
    let avail_str = value(input, "MemAvailable", ':').or_else(|| value(input, "MemFree", ':'))?;

    let parse_kb = |s: &str| -> Option<u64> { s.split_whitespace().next()?.parse::<u64>().ok() };

    let total = parse_kb(total_str)?;
    let available = parse_kb(avail_str)?;

    if total == 0 || available > total {
        return None;
    }

    let used = total - available;
    let total_gib = total as f64 / 1048576.0;
    let used_gib = used as f64 / 1048576.0;
    let pct = ((used as f64 / total as f64) * 100.0).round() as u64;

    Some(format!("{used_gib:.2} GiB / {total_gib:.2} GiB ({pct}%)"))
}

pub fn detect_memory() -> Option<String> {
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap_or_default();
    format_memory(&meminfo)
}
