use crate::utils::clean;
use std::env;

pub fn detect_locale() -> Option<String> {
    env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .or_else(|_| env::var("LC_MESSAGES"))
        .ok()
        .map(|s| clean(&s))
}
