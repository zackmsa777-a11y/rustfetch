use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogoSetting {
    Name(String),
    Object {
        source: Option<String>,
        color: Option<serde_json::Value>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModuleSetting {
    Name(String),
    Object {
        #[serde(rename = "type")]
        module_type: String,
        key: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplaySetting {
    pub color: Option<serde_json::Value>,
    pub separator: Option<String>,
    pub key: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub logo: Option<LogoSetting>,
    pub logo_color: Option<String>,
    pub no_logo: Option<bool>,
    pub no_color: Option<bool>,
    pub display: Option<DisplaySetting>,
    pub modules: Option<Vec<ModuleSetting>>,
    pub disk_paths: Option<Vec<String>>,
    pub color_keys: Option<String>,
}

impl Config {
    pub fn get_logo_name(&self) -> Option<String> {
        match &self.logo {
            Some(LogoSetting::Name(n)) => Some(n.clone()),
            Some(LogoSetting::Object { source, .. }) => source.clone(),
            None => None,
        }
    }

    pub fn get_logo_color(&self) -> Option<String> {
        if self.logo_color.is_some() {
            return self.logo_color.clone();
        }
        if let Some(LogoSetting::Object {
            color: Some(color), ..
        }) = &self.logo
        {
            if let serde_json::Value::String(s) = color {
                return Some(s.clone());
            } else if let serde_json::Value::Object(map) = color
                && let Some(serde_json::Value::String(s)) = map.get("1")
            {
                return Some(s.clone());
            }
        }
        None
    }

    pub fn get_key_color(&self) -> Option<String> {
        if self.color_keys.is_some() {
            return self.color_keys.clone();
        }
        if let Some(disp) = &self.display
            && let Some(color) = &disp.color
        {
            if let serde_json::Value::Object(color_map) = color {
                if let Some(serde_json::Value::String(k)) = color_map.get("keys") {
                    return Some(k.clone());
                }
            } else if let serde_json::Value::String(s) = color {
                return Some(s.clone());
            }
        }
        None
    }

    pub fn get_normalized_modules(&self) -> Option<Vec<String>> {
        let raw_modules = self.modules.as_ref()?;
        let mut list = Vec::new();

        for m in raw_modules {
            let name = match m {
                ModuleSetting::Name(s) => s.as_str(),
                ModuleSetting::Object { module_type, .. } => module_type.as_str(),
            };

            let lower = name.to_lowercase();
            let normalized = match lower.as_str() {
                "wmtheme" => "wm_theme",
                "terminalfont" => "terminal_font",
                "poweradapter" => "power_adapter",
                "localip" => "local_ip",
                "publicip" => "public_ip",
                other => other,
            };

            list.push(normalized.to_string());
        }

        Some(list)
    }
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
                    if next_c == '*'
                        && let Some(&'/') = chars.peek()
                    {
                        chars.next();
                        break;
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
    let candidates = [
        ("XDG_CONFIG_HOME", "rustfetch/config.jsonc"),
        ("XDG_CONFIG_HOME", "rustfetch/config.json"),
        ("XDG_CONFIG_HOME", "fastfetch/config.jsonc"),
        ("XDG_CONFIG_HOME", "fastfetch/config.json"),
        ("HOME", ".config/rustfetch/config.jsonc"),
        ("HOME", ".config/rustfetch/config.json"),
        ("HOME", ".config/fastfetch/config.jsonc"),
        ("HOME", ".config/fastfetch/config.json"),
    ];

    for (var, rel) in &candidates {
        if let Ok(base) = env::var(var) {
            let p = Path::new(&base).join(rel);
            if p.exists() {
                return Some(p);
            }
        }
    }

    None
}

pub fn load_config(path: Option<&Path>) -> Config {
    let target = path
        .map(|p| p.to_path_buf())
        .or_else(find_default_config_path);

    if let Some(p) = target
        && let Ok(raw) = fs::read_to_string(p)
    {
        let stripped = strip_jsonc_comments(&raw);
        if let Ok(cfg) = serde_json::from_str::<Config>(&stripped) {
            return cfg;
        }
    }

    Config::default()
}

pub fn generate_default_config() -> String {
    let default_cfg = serde_json::json!({
        "$schema": "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json",
        "logo": {
            "source": "auto",
            "color": {
                "1": "auto"
            }
        },
        "display": {
            "separator": ": ",
            "color": {
                "keys": "auto"
            }
        },
        "modules": [
            "title",
            "separator",
            { "type": "os", "key": "OS" },
            { "type": "host", "key": "Host" },
            { "type": "kernel", "key": "Kernel" },
            { "type": "uptime", "key": "Uptime" },
            { "type": "packages", "key": "Packages" },
            { "type": "shell", "key": "Shell" },
            { "type": "display", "key": "Display" },
            { "type": "de", "key": "DE" },
            { "type": "wm", "key": "WM" },
            { "type": "theme", "key": "Theme" },
            { "type": "icons", "key": "Icons" },
            { "type": "font", "key": "Font" },
            { "type": "cursor", "key": "Cursor" },
            { "type": "terminal", "key": "Terminal" },
            { "type": "terminalfont", "key": "Terminal Font" },
            { "type": "cpu", "key": "CPU" },
            { "type": "gpu", "key": "GPU" },
            { "type": "memory", "key": "Memory" },
            { "type": "swap", "key": "Swap" },
            { "type": "disk", "key": "Disk" },
            { "type": "battery", "key": "Battery" },
            { "type": "poweradapter", "key": "Power Adapter" },
            { "type": "localip", "key": "Local IP" },
            { "type": "locale", "key": "Locale" },
            "break",
            "colors"
        ]
    });

    serde_json::to_string_pretty(&default_cfg).unwrap_or_default()
}
