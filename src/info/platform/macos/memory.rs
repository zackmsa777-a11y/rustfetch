use super::sysctl;
use crate::info::platform::format_usage_gib;

pub fn detect_memory() -> Option<String> {
    let total = sysctl::sysctl_u64("hw.memsize")?;
    let page_size = sysctl::sysctl_u64("hw.pagesize").unwrap_or(4096);
    let used = used_bytes(page_size).unwrap_or(0);
    format_usage_gib(used.min(total), total)
}

#[allow(deprecated)]
fn used_bytes(page_size: u64) -> Option<u64> {
    unsafe {
        let host = libc::mach_host_self();
        let mut count = libc::HOST_VM_INFO64_COUNT;
        let mut stats: libc::vm_statistics64 = std::mem::zeroed();
        let rc = libc::host_statistics64(
            host,
            libc::HOST_VM_INFO64,
            &mut stats as *mut _ as *mut libc::integer_t,
            &mut count,
        );
        if rc != libc::KERN_SUCCESS {
            return None;
        }
        let used_pages = (stats.active_count as u64)
            .saturating_add(stats.wire_count as u64)
            .saturating_add(stats.compressor_page_count as u64);
        Some(used_pages.saturating_mul(page_size))
    }
}
