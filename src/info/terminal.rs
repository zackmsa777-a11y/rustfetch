use crate::utils::{clean, run_cmd};
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

    if term_name.is_none() {
        if let Ok(term_env) = env::var("TERMINAL") {
            let clean_t = clean(&term_env);
            if !clean_t.is_empty() {
                term_name = Some(clean_t);
            }
        }
    }

    if term_name.is_none() {
        let mut curr_pid = unsafe { libc::getppid() };
        for _ in 0..6 {
            if curr_pid <= 1 {
                break;
            }
            let comm_path = format!("/proc/{curr_pid}/comm");
            let comm = fs::read_to_string(&comm_path)
                .map(|s| clean(s.trim()))
                .unwrap_or_default();

            let lcomm = comm.to_lowercase();
            if !lcomm.is_empty()
                && lcomm != "bash"
                && lcomm != "zsh"
                && lcomm != "sh"
                && lcomm != "fish"
                && lcomm != "sudo"
                && lcomm != "su"
                && lcomm != "login"
            {
                term_name = Some(comm);
                break;
            }

            let stat_path = format!("/proc/{curr_pid}/stat");
            if let Ok(stat_content) = fs::read_to_string(&stat_path) {
                if let Some(rparen) = stat_content.rfind(')') {
                    let rest = &stat_content[rparen + 1..];
                    let parts: Vec<&str> = rest.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(parent_pid) = parts[1].parse::<i32>() {
                            curr_pid = parent_pid;
                            continue;
                        }
                    }
                }
            }
            break;
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

    if env::var("SSH_CONNECTION").is_ok() || env::var("SSH_TTY").is_ok() {
        if let Some(ssh_out) = run_cmd("ssh", &["-V"]) {
            let ver = ssh_out.trim_start_matches("OpenSSH_");
            if let Some(ref mut name) = term_name {
                name.push_str(&format!(" ({ver})"));
            }
        }
    }

    if let Ok(home) = env::var("HOME") {
        let kitty_conf = Path::new(&home).join(".config/kitty/kitty.conf");
        if kitty_conf.exists() {
            if let Ok(content) = fs::read_to_string(kitty_conf) {
                for line in content.lines() {
                    if line.starts_with("font_family") {
                        if let Some((_, f)) = line.split_once(' ') {
                            term_font = Some(clean(f.trim()));
                            break;
                        }
                    }
                }
            }
        }
    }

    (term_name, term_font)
}
