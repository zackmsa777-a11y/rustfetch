pub mod cpu;
pub mod disk;
pub mod ffi;
pub mod host;
pub mod hostname;
pub mod kernel;
pub mod memory;
pub mod os;
pub mod shell;
pub mod swap;
pub mod terminal;
pub mod uptime;

#[cfg(test)]
mod tests {
    #[test]
    fn windows_modules_link_symbols() {
        let _ = super::os::detect_os();
        let _ = super::kernel::detect_kernel();
        let _ = super::uptime::detect_uptime();
        let _ = super::cpu::detect_cpu();
        let _ = super::memory::detect_memory();
        let _ = super::swap::detect_swap();
        let _ = super::host::detect_host();
        let _ = super::shell::detect_shell();
        let _ = super::terminal::detect_terminal();
        let _ = super::disk::detect_disks(None);
        let _ = super::hostname::detect_hostname();
    }
}
