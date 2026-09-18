use crate::utils::clean;
use std::env;
use std::ffi::CStr;
use std::fs;
use std::path::Path;

pub fn detect_terminal() -> (Option<String>, Option<String>) {
    let mut term_name = None;
    let mut term_font = None;

    if let Ok(prog) = env::var("TERM_PROGRAM") {
        let clean_prog = clean(&prog);
        if !clean_prog.is_empty() {
            let ver = env::var("TERM_PROGRAM_VERSION").ok().map(|v| clean(&v));
            term_name = Some(if let Some(v) = ver {
                format!("{clean_prog} {v}")
            } else {
                clean_prog
            });
        }
    }

    if term_name.is_none()
        && let Ok(term_env) = env::var("TERMINAL")
    {
        let clean_t = clean(&term_env);
        if !clean_t.is_empty() {
            term_name = Some(clean_t);
        }
    }

    if term_name.is_none() {
        let pts = unsafe {
            let ptr = libc::ttyname(0);
            if !ptr.is_null() {
                Some(CStr::from_ptr(ptr).to_string_lossy().to_string())
            } else {
                None
            }
        };
        if let Some(p) = pts {
            term_name = Some(p);
        } else if let Ok(t) = env::var("TERM") {
            term_name = Some(t);
        }
    }

    if let Ok(home) = env::var("HOME") {
        let kitty_conf = Path::new(&home).join(".config/kitty/kitty.conf");
        if kitty_conf.exists()
            && let Ok(content) = fs::read_to_string(kitty_conf)
        {
            for line in content.lines() {
                if line.starts_with("font_family")
                    && let Some((_, f)) = line.split_once(' ')
                {
                    term_font = Some(clean(f.trim()));
                    break;
                }
            }
        }
    }

    (term_name, term_font)
}
