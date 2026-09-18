use std::ffi::CString;

pub fn sysctl_raw(name: &str, buf: &mut [u8]) -> Option<usize> {
    let c_name = CString::new(name).ok()?;
    let mut len = buf.len();
    let rc = unsafe {
        libc::sysctlbyname(
            c_name.as_ptr(),
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut len,
            std::ptr::null_mut(),
            0,
        )
    };
    if rc == 0 { Some(len) } else { None }
}

pub fn sysctl_string(name: &str) -> Option<String> {
    let mut buf = vec![0u8; 1024];
    let len = sysctl_raw(name, &mut buf)?;
    let end = buf[..len].iter().position(|&b| b == 0).unwrap_or(len);
    let s = String::from_utf8_lossy(&buf[..end]).trim().to_string();
    if s.is_empty() { None } else { Some(s) }
}

pub fn sysctl_u64(name: &str) -> Option<u64> {
    let mut val: u64 = 0;
    let mut len = std::mem::size_of::<u64>();
    let c_name = CString::new(name).ok()?;
    let rc = unsafe {
        libc::sysctlbyname(
            c_name.as_ptr(),
            &mut val as *mut u64 as *mut libc::c_void,
            &mut len,
            std::ptr::null_mut(),
            0,
        )
    };
    if rc == 0 { Some(val) } else { None }
}

pub fn sysctl_i32(name: &str) -> Option<i32> {
    let mut val: i32 = 0;
    let mut len = std::mem::size_of::<i32>();
    let c_name = CString::new(name).ok()?;
    let rc = unsafe {
        libc::sysctlbyname(
            c_name.as_ptr(),
            &mut val as *mut i32 as *mut libc::c_void,
            &mut len,
            std::ptr::null_mut(),
            0,
        )
    };
    if rc == 0 { Some(val) } else { None }
}

pub fn sysctl_struct<T: Copy>(name: &str) -> Option<T> {
    let mut val: T = unsafe { std::mem::zeroed() };
    let mut len = std::mem::size_of::<T>();
    let c_name = CString::new(name).ok()?;
    let rc = unsafe {
        libc::sysctlbyname(
            c_name.as_ptr(),
            &mut val as *mut T as *mut libc::c_void,
            &mut len,
            std::ptr::null_mut(),
            0,
        )
    };
    if rc == 0 { Some(val) } else { None }
}
