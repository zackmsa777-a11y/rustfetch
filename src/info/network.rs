#[cfg(target_os = "linux")]
use std::ffi::CStr;
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::net::Ipv4Addr;

#[cfg(target_os = "linux")]
pub fn detect_local_ip() -> Option<String> {
    let route = fs::read_to_string("/proc/net/route").ok()?;
    let mut default_iface = None;
    for line in route.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "00000000" {
            default_iface = Some(parts[0].to_string());
            break;
        }
    }

    let iface_name = default_iface.unwrap_or_else(|| "eth0".into());

    unsafe {
        let mut ifap: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifap) != 0 || ifap.is_null() {
            return None;
        }

        let mut result = None;
        let mut curr = ifap;
        while !curr.is_null() {
            let ifa = &*curr;
            if !ifa.ifa_addr.is_null() && (*ifa.ifa_addr).sa_family == libc::AF_INET as u16 {
                let name = CStr::from_ptr(ifa.ifa_name).to_string_lossy();
                if name == iface_name {
                    let sin = &*(ifa.ifa_addr as *const libc::sockaddr_in);
                    let ip = Ipv4Addr::from(u32::from_be(sin.sin_addr.s_addr));

                    let mask_cidr = if !ifa.ifa_netmask.is_null() {
                        let smask = &*(ifa.ifa_netmask as *const libc::sockaddr_in);
                        u32::from_be(smask.sin_addr.s_addr).count_ones()
                    } else {
                        24
                    };

                    result = Some(format!("{ip}/{mask_cidr}"));
                    break;
                }
            }
            curr = (*curr).ifa_next;
        }

        libc::freeifaddrs(ifap);

        result.map(|ip| format!("{iface_name}: {ip}"))
    }
}


#[cfg(not(target_os = "linux"))]
pub fn detect_local_ip() -> Option<String> {
    None
}
