use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub logo: Option<String>,
    pub logo_color: Option<String>,
    pub no_logo: Option<bool>,
    pub no_color: Option<bool>,
    pub modules: Option<Vec<String>>,
    pub disk_paths: Option<Vec<String>>,
    pub color_keys: Option<String>,
    pub key_width: Option<usize>,
}

pub fn strip_jsonc_comments(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_quote = false;

    while let Some(c) = chars.next() {
        if c == '"' {
            in_quote = !in_quote;
            out.push(c);
        } else if !in_quote && c == '/' {
            if let Some(&'/') = chars.peek() {
                chars.next();
                for next_c in chars.by_ref() {
                    if next_c == '\n' {
                        out.push('\n');
                        break;
                    }
                }
            } else if let Some(&'*') = chars.peek() {
                chars.next();
                while let Some(next_c) = chars.next() {
                    if next_c == '*' {
                        if let Some(&'/') = chars.peek() {
                            chars.next();
                            break;
                        }
                    }
                }
            } else {
                out.push(c);
            }
        } else {
            out.push(c);
        }
    }

    out
}

pub fn find_default_config_path() -> Option<PathBuf> {
    if let Ok(config_home) = env::var("XDG_CONFIG_HOME") {
        let p = Path::new(&config_home).join("rustfetch/config.jsonc");
        if p.exists() {
            return Some(p);
        }
        let p2 = Path::new(&config_home).join("rustfetch/config.json");
        if p2.exists() {
            return Some(p2);
        }
    }

    if let Ok(home) = env::var("HOME") {
        let p = Path::new(&home).join(".config/rustfetch/config.jsonc");
        if p.exists() {
            return Some(p);
        }
        let p2 = Path::new(&home).join(".config/rustfetch/config.json");
        if p2.exists() {
            return Some(p2);
        }
    }

    None
}

pub fn load_config(path: Option<&Path>) -> Config {
    let target = path
        .map(|p| p.to_path_buf())
        .or_else(find_default_config_path);

    if let Some(p) = target {
        if let Ok(raw) = fs::read_to_string(p) {
            let stripped = strip_jsonc_comments(&raw);
            if let Ok(cfg) = serde_json::from_str::<Config>(&stripped) {
                return cfg;
            }
        }
    }

    Config::default()
}

pub fn generate_default_config() -> String {
    let default_cfg = serde_json::json!({
        "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json",
        "logo": "auto",
        "logo_color": "auto",
        "no_logo": false,
        "no_color": false,
        "modules": [
            "title",
            "separator",
            "os",
            "host",
            "kernel",
            "uptime",
            "packages",
            "shell",
            "display",
            "de",
            "wm",
            "wm_theme",
            "theme",
            "icons",
            "font",
            "cursor",
            "terminal",
            "terminal_font",
            "cpu",
            "gpu",
            "memory",
            "swap",
            "disk",
            "battery",
            "power_adapter",
            "audio",
            "local_ip",
            "locale",
            "break",
            "colors"
        ],
        "disk_paths": ["/"]
    });

    serde_json::to_string_pretty(&default_cfg).unwrap_or_default()
}
