use super::sysctl;
use crate::info::uptime::format_uptime;

pub fn detect_uptime() -> Option<String> {
    let boot: libc::timeval = sysctl::sysctl_struct("kern.boottime")?;
    let mut now: libc::timeval = unsafe { std::mem::zeroed() };
    if unsafe { libc::gettimeofday(&mut now, std::ptr::null_mut()) } != 0 {
        return None;
    }
    let secs = (now.tv_sec as i64).saturating_sub(boot.tv_sec as i64);
    if secs < 0 {
        return None;
    }
    Some(format_uptime(secs as u64))
}
