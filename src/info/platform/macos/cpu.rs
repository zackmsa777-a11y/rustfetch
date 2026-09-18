use super::sysctl;
use crate::utils::clean;

pub fn detect_cpu() -> Option<String> {
    let brand = sysctl::sysctl_string("machdep.cpu.brand_string")
        .or_else(|| sysctl::sysctl_string("hw.model"))?;
    let mut name = clean(&brand);
    if let Some(idx) = name.find(" CPU @") {
        name.truncate(idx);
    } else if let Some(idx) = name.find(" @") {
        name.truncate(idx);
    }
    let name = name.trim().to_string();

    let count = sysctl::sysctl_i32("hw.logicalcpu")
        .or_else(|| sysctl::sysctl_i32("hw.ncpu"))
        .unwrap_or(0);

    let mhz = sysctl::sysctl_u64("hw.cpufrequency").map(|hz| hz as f64 / 1_000_000.0);

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
