use crate::utils::{clean, read_first_line, value};
use std::fs;

pub fn format_cpu(cpuinfo: &str) -> Option<String> {
    let model = value(cpuinfo, "model name", ':')
        .or_else(|| value(cpuinfo, "Hardware", ':'))
        .or_else(|| value(cpuinfo, "Processor", ':'))?;

    let cleaned_model = clean(model);

    let count = cpuinfo
        .lines()
        .filter(|line| line.starts_with("processor\t") || line.starts_with("processor:"))
        .count();

    let mhz = read_first_line("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq")
        .and_then(|f| f.parse::<f64>().ok())
        .map(|khz| khz / 1000.0)
        .or_else(|| value(cpuinfo, "cpu MHz", ':').and_then(|m| m.parse::<f64>().ok()));

    let mut name = cleaned_model;
    if let Some(idx) = name.find(" CPU @") {
        name.truncate(idx);
    } else if let Some(idx) = name.find(" @") {
        name.truncate(idx);
    }
    let name = name.trim().to_string();

    let mut result = name;
    if count > 0 {
        result.push_str(&format!(" ({count})"));
    }
    if let Some(m) = mhz {
        let ghz = m / 1000.0;
        if ghz > 0.1 {
            result.push_str(&format!(" @ {ghz:.2} GHz"));
        }
    }

    Some(result)
}

pub fn detect_cpu() -> Option<String> {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    format_cpu(&cpuinfo)
}
