use super::sysctl;
use crate::info::platform::format_usage_gib;

pub fn detect_swap() -> Option<String> {
    let usage: libc::xsw_usage = sysctl::sysctl_struct("vm.swapusage")?;
    if usage.xsu_total == 0 {
        return Some("Disabled".into());
    }
    format_usage_gib(usage.xsu_used, usage.xsu_total)
}
