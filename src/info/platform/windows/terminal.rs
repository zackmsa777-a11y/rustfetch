use crate::utils::clean;
use std::env;

pub fn detect_terminal() -> (Option<String>, Option<String>) {
    if let Ok(prog) = env::var("TERM_PROGRAM") {
        let clean_prog = clean(&prog);
        if !clean_prog.is_empty() {
            let ver = env::var("TERM_PROGRAM_VERSION").ok().map(|v| clean(&v));
            let name = if let Some(v) = ver {
                format!("{clean_prog} {v}")
            } else {
                clean_prog
            };
            return (Some(name), None);
        }
    }
    if env::var("WT_SESSION").is_ok() {
        return (Some("Windows Terminal".into()), None);
    }
    if env::var("ConEmuPID").is_ok() || env::var("ConEmuANSI").is_ok() {
        return (Some("ConEmu".into()), None);
    }
    if let Ok(term) = env::var("TERM") {
        let t = clean(&term);
        if !t.is_empty() {
            return (Some(t), None);
        }
    }
    (Some("Console".into()), None)
}
