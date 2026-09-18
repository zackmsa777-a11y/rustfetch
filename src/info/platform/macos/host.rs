use super::sysctl;
use crate::utils::clean;

pub fn detect_host() -> Option<String> {
    sysctl::sysctl_string("hw.model").map(|m| clean(&m))
}
