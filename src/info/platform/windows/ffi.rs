#![allow(non_snake_case, non_camel_case_types, dead_code)]

use std::mem;

pub type BOOL = i32;
pub type DWORD = u32;
pub type WORD = u16;
pub type ULONGLONG = u64;
pub type WCHAR = u16;
pub type HANDLE = *mut core::ffi::c_void;
pub type HKEY = *mut core::ffi::c_void;
pub type LONG = i32;
pub type LSTATUS = LONG;

pub const MAX_PATH: usize = 260;
pub const ERROR_SUCCESS: LSTATUS = 0;
pub const KEY_READ: DWORD = 0x20019;
pub const HKEY_LOCAL_MACHINE: HKEY = 0x80000002u32 as HKEY;
pub const REG_SZ: DWORD = 1;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORYSTATUSEX {
    pub dwLength: DWORD,
    pub dwMemoryLoad: DWORD,
    pub ullTotalPhys: ULONGLONG,
    pub ullAvailPhys: ULONGLONG,
    pub ullTotalPageFile: ULONGLONG,
    pub ullAvailPageFile: ULONGLONG,
    pub ullTotalVirtual: ULONGLONG,
    pub ullAvailVirtual: ULONGLONG,
    pub ullAvailExtendedVirtual: ULONGLONG,
}

impl MEMORYSTATUSEX {
    pub fn zeroed() -> Self {
        let mut s: Self = unsafe { mem::zeroed() };
        s.dwLength = mem::size_of::<Self>() as DWORD;
        s
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_INFO {
    pub wProcessorArchitecture: WORD,
    pub wReserved: WORD,
    pub dwPageSize: DWORD,
    pub lpMinimumApplicationAddress: *mut core::ffi::c_void,
    pub lpMaximumApplicationAddress: *mut core::ffi::c_void,
    pub dwActiveProcessorMask: usize,
    pub dwNumberOfProcessors: DWORD,
    pub dwProcessorType: DWORD,
    pub dwAllocationGranularity: DWORD,
    pub wProcessorLevel: WORD,
    pub wProcessorRevision: WORD,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: DWORD,
    pub dwMajorVersion: DWORD,
    pub dwMinorVersion: DWORD,
    pub dwBuildNumber: DWORD,
    pub dwPlatformId: DWORD,
    pub szCSDVersion: [WCHAR; 128],
}

impl OSVERSIONINFOW {
    pub fn zeroed() -> Self {
        let mut s: Self = unsafe { mem::zeroed() };
        s.dwOSVersionInfoSize = mem::size_of::<Self>() as DWORD;
        s
    }
}

#[link(name = "kernel32")]
unsafe extern "system" {
    pub fn GetTickCount64() -> ULONGLONG;
    pub fn GlobalMemoryStatusEx(lpBuffer: *mut MEMORYSTATUSEX) -> BOOL;
    pub fn GetComputerNameW(lpBuffer: *mut WCHAR, nSize: *mut DWORD) -> BOOL;
    pub fn GetSystemInfo(lpSystemInfo: *mut SYSTEM_INFO);
    pub fn GetDiskFreeSpaceExW(
        lpDirectoryName: *const WCHAR,
        lpFreeBytesAvailableToCaller: *mut ULONGLONG,
        lpTotalNumberOfBytes: *mut ULONGLONG,
        lpTotalNumberOfFreeBytes: *mut ULONGLONG,
    ) -> BOOL;
}

#[link(name = "ntdll")]
unsafe extern "system" {
    pub fn RtlGetVersion(lpVersionInformation: *mut OSVERSIONINFOW) -> LONG;
}

#[link(name = "advapi32")]
unsafe extern "system" {
    pub fn RegOpenKeyExW(
        hKey: HKEY,
        lpSubKey: *const WCHAR,
        ulOptions: DWORD,
        samDesired: DWORD,
        phkResult: *mut HKEY,
    ) -> LSTATUS;
    pub fn RegQueryValueExW(
        hKey: HKEY,
        lpValueName: *const WCHAR,
        lpReserved: *mut DWORD,
        lpType: *mut DWORD,
        lpData: *mut u8,
        lpcbData: *mut DWORD,
    ) -> LSTATUS;
    pub fn RegCloseKey(hKey: HKEY) -> LSTATUS;
}

pub fn wide(s: &str) -> Vec<WCHAR> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn from_wide_nul(buf: &[WCHAR]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

pub fn reg_sz(subkey: &str, value: &str) -> Option<String> {
    unsafe {
        let mut hkey: HKEY = std::ptr::null_mut();
        let sub = wide(subkey);
        let rc = RegOpenKeyExW(HKEY_LOCAL_MACHINE, sub.as_ptr(), 0, KEY_READ, &mut hkey);
        if rc != ERROR_SUCCESS || hkey.is_null() {
            return None;
        }
        let val = wide(value);
        let mut ty: DWORD = 0;
        let mut size: DWORD = 0;
        let q1 = RegQueryValueExW(
            hkey,
            val.as_ptr(),
            std::ptr::null_mut(),
            &mut ty,
            std::ptr::null_mut(),
            &mut size,
        );
        if q1 != ERROR_SUCCESS || size == 0 {
            RegCloseKey(hkey);
            return None;
        }
        let mut buf = vec![0u8; size as usize];
        let q2 = RegQueryValueExW(
            hkey,
            val.as_ptr(),
            std::ptr::null_mut(),
            &mut ty,
            buf.as_mut_ptr(),
            &mut size,
        );
        RegCloseKey(hkey);
        if q2 != ERROR_SUCCESS {
            return None;
        }
        if ty != REG_SZ {
            return None;
        }
        let usable = (size as usize).min(buf.len());
        let words: Vec<WCHAR> = buf[..usable]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        let s = from_wide_nul(&words).trim().to_string();
        if s.is_empty() { None } else { Some(s) }
    }
}
