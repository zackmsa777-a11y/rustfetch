use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

pub const DEFAULT_TIMEOUT_MS: u64 = 1500;

pub fn run_module_command(
    cmdline: Option<&str>,
    shell: bool,
    timeout_ms: u64,
    show_failure: bool,
) -> Option<String> {
    let cmdline = cmdline?.trim();
    if cmdline.is_empty() {
        return if show_failure {
            Some("empty command".into())
        } else {
            None
        };
    }

    let timeout = Duration::from_millis(if timeout_ms == 0 {
        DEFAULT_TIMEOUT_MS
    } else {
        timeout_ms
    });

    let result = if shell {
        spawn_timed("/bin/sh", &["-c", cmdline], timeout)
    } else {
        let parts = split_cmdline(cmdline);
        if parts.is_empty() {
            return if show_failure {
                Some("empty command".into())
            } else {
                None
            };
        }
        let (program, args) = parts.split_first().unwrap();
        let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        spawn_timed(program, &arg_refs, timeout)
    };

    match result {
        Ok(stdout) => {
            let trimmed = trim_trailing_newlines(&stdout);
            if trimmed.is_empty() {
                if show_failure {
                    Some(String::new())
                } else {
                    None
                }
            } else {
                Some(sanitize_output(&trimmed))
            }
        }
        Err(reason) => {
            if show_failure {
                Some(reason)
            } else {
                None
            }
        }
    }
}

fn spawn_timed(program: &str, args: &[&str], timeout: Duration) -> Result<String, String> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("spawn failed: {e}"))?;

    let start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let mut buf = String::new();
                if let Some(mut out) = child.stdout.take() {
                    let _ = out.read_to_string(&mut buf);
                }
                if status.success() {
                    return Ok(buf);
                }
                return Err(format!("exit {}", status.code().unwrap_or(-1)));
            }
            Ok(None) if start.elapsed() >= timeout => {
                let _ = child.kill();
                let _ = child.wait();
                return Err("timeout".into());
            }
            Ok(None) => thread::sleep(Duration::from_millis(15)),
            Err(e) => return Err(format!("wait failed: {e}")),
        }
    }
}

fn trim_trailing_newlines(s: &str) -> String {
    s.trim_end_matches(['\r', '\n']).to_string()
}

fn sanitize_output(s: &str) -> String {
    s.chars()
        .filter(|&c| c == '\n' || c == '\t' || !c.is_control())
        .collect()
}

fn split_cmdline(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut cur = String::new();
    let mut chars = input.chars().peekable();
    let mut in_single = false;
    let mut in_double = false;

    while let Some(c) = chars.next() {
        match c {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '\\' if !in_single => {
                if let Some(next) = chars.next() {
                    cur.push(next);
                }
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if !cur.is_empty() {
                    parts.push(std::mem::take(&mut cur));
                }
            }
            _ => cur.push(c),
        }
    }
    if !cur.is_empty() {
        parts.push(cur);
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_hello_succeeds() {
        let out = run_module_command(Some("echo hello"), false, 1500, false);
        assert_eq!(out.as_deref(), Some("hello"));
    }

    #[test]
    fn shell_pipeline_with_shell_true() {
        let out = run_module_command(Some("echo hello | tr a-z A-Z"), true, 1500, false);
        assert_eq!(out.as_deref(), Some("HELLO"));
    }

    #[test]
    fn failure_is_empty_by_default() {
        let out = run_module_command(Some("false"), false, 1500, false);
        assert_eq!(out, None);
    }

    #[test]
    fn failure_shows_when_requested() {
        let out = run_module_command(Some("false"), false, 1500, true);
        assert!(out.is_some());
        assert!(out.unwrap().contains("exit"));
    }

    #[test]
    fn timeout_yields_empty() {
        let out = run_module_command(Some("sleep 5"), false, 100, false);
        assert_eq!(out, None);
    }

    #[test]
    fn timeout_show_failure() {
        let out = run_module_command(Some("sleep 5"), false, 100, true);
        assert_eq!(out.as_deref(), Some("timeout"));
    }

    #[test]
    fn splits_quoted_args() {
        let parts = split_cmdline(r#"echo "hello world""#);
        assert_eq!(parts, vec!["echo", "hello world"]);
    }
}
