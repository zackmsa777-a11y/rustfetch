use std::fs;
use std::path::Path;
use std::process::Command;

pub fn clean(input: &str) -> String {
    input.chars().filter(|c| !c.is_control()).collect()
}

pub fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let mut in_escape = false;
    for c in s.chars() {
        if in_escape {
            if c.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else if c == '\x1b' {
            in_escape = true;
        } else {
            out.push(c);
        }
    }
    out
}

pub fn visible_width(s: &str) -> usize {
    strip_ansi(s).chars().count()
}

pub fn value<'a>(input: &'a str, key: &str, delimiter: char) -> Option<&'a str> {
    for line in input.lines() {
        if let Some((k, v)) = line.split_once(delimiter) {
            if k.trim() != key {
                continue;
            }
            let v = v.trim();
            let v = if (v.starts_with('"') && v.ends_with('"') && v.len() >= 2)
                || (v.starts_with('\'') && v.ends_with('\'') && v.len() >= 2)
            {
                &v[1..v.len() - 1]
            } else {
                v
            };
            return Some(v);
        }
    }
    None
}

pub fn read_first_line<P: AsRef<Path>>(path: P) -> Option<String> {
    fs::read_to_string(path).ok().and_then(|content| {
        content
            .lines()
            .next()
            .map(|l| clean(l.trim()))
            .filter(|l| !l.is_empty())
    })
}

pub fn run_cmd(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd).args(args).output().ok().and_then(|o| {
        if o.status.success() {
            let out = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if out.is_empty() {
                None
            } else {
                Some(clean(&out))
            }
        } else {
            None
        }
    })
}
