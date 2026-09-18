use std::fs;
use std::path::Path;
use std::process::Command;

pub fn clean(input: &str) -> String {
    input.chars().filter(|c| !c.is_control()).collect()
}

pub fn strip_ansi(s: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == '\x1b' {
            if i + 1 < len && (chars[i + 1] == '_' || chars[i + 1] == ']' || chars[i + 1] == 'P') {
                i += 2;
                while i < len {
                    if chars[i] == '\x07' {
                        i += 1;
                        break;
                    }
                    if chars[i] == '\x1b' && i + 1 < len && chars[i + 1] == '\\' {
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            } else if i + 1 < len && chars[i + 1] == '[' {
                i += 2;
                while i < len {
                    let c = chars[i];
                    i += 1;
                    if (c as u32) >= 0x40 && (c as u32) <= 0x7E {
                        break;
                    }
                }
            } else {
                i += 2;
            }
        } else {
            out.push(chars[i]);
            i += 1;
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
                let cleaned: String = out
                    .chars()
                    .filter(|&c| c == '\n' || !c.is_control())
                    .collect();
                if cleaned.is_empty() {
                    None
                } else {
                    Some(cleaned)
                }
            }
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_cmd_preserves_newlines_in_multiline_output() {
        let dir = std::env::temp_dir().join(format!("rustfetch_test_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let script = dir.join("multiline_cmd");
        std::fs::write(&script, "#!/bin/sh\necho 'line one'\necho 'line two'\n").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
        }

        let out = run_cmd(script.to_str().unwrap(), &[]).unwrap();
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines, vec!["line one", "line two"]);

        std::fs::remove_file(&script).ok();
    }

    #[test]
    fn test_strip_ansi_comprehensive() {
        let plain = "Hello, World!";
        assert_eq!(strip_ansi(plain), "Hello, World!");

        let colored = "\x1b[1;32mGreen\x1b[0m and \x1b[38;5;208mOrange\x1b[0m";
        assert_eq!(strip_ansi(colored), "Green and Orange");

        let kitty_escape = "\x1b_Ga=T,f=100,t=f;L3BhdGgvdG8vaW1hZ2U=\x1b\\Text after image";
        assert_eq!(strip_ansi(kitty_escape), "Text after image");

        let mixed = "\x1b[10C\x1b_Ga=T;b64\x1b\\\x1b[31mRed Text\x1b[0m";
        assert_eq!(strip_ansi(mixed), "Red Text");
    }
}
