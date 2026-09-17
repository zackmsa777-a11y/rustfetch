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
pub struct ThemeColors {
    pub title: Option<String>,
    pub keys: Option<String>,
    pub value: Option<String>,
    pub separator: Option<String>,
    pub logo_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeSetting {
    pub name: Option<String>,
    pub title: Option<String>,
    pub keys: Option<String>,
    pub value: Option<String>,
    pub separator: Option<String>,
    pub logo_color: Option<String>,
}

/// Accepts `"theme": "gruvbox"` shorthand or the full object form.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThemeArg {
    Name(String),
    Full(ThemeSetting),
}

impl ThemeArg {
    pub fn name(&self) -> Option<&str> {
        match self {
            ThemeArg::Name(s) => Some(s.as_str()),
            ThemeArg::Full(t) => t.name.as_deref(),
        }
    }

    pub fn overrides(&self) -> ThemeSetting {
        match self {
            ThemeArg::Name(_) => ThemeSetting::default(),
            ThemeArg::Full(t) => ThemeSetting {
                name: None,
                title: t.title.clone(),
                keys: t.keys.clone(),
                value: t.value.clone(),
                separator: t.separator.clone(),
                logo_color: t.logo_color.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub logo: Option<LogoSetting>,
    pub logo_color: Option<String>,
    pub no_logo: Option<bool>,
    pub no_color: Option<bool>,
    pub display: Option<DisplaySetting>,
    pub theme: Option<ThemeArg>,
    /// User-defined themes: `"themes": { "my-theme": { "keys": "cyan", ... } }`.
    pub themes: Option<std::collections::HashMap<String, ThemeColors>>,
    pub modules: Option<Vec<ModuleSetting>>,
    pub disk_paths: Option<Vec<String>>,
    pub color_keys: Option<String>,
}

pub const BUILTIN_THEME_NAMES: &[&str] = &[
    "default",
    "neon",
    "gruvbox",
    "nord",
    "dracula",
    "tokyo-night",
    "everforest",
];

/// Built-in preset definitions. Custom `themes` from the config take
/// precedence over these when names collide.
pub fn resolve_theme(name: &str) -> Option<ThemeColors> {
    let mut out = ThemeColors::default();
    match name.to_lowercase().as_str() {
        "default" => Some(out),
        "neon" => {
            out.title = Some("magenta".into());
            out.keys = Some("cyan".into());
            out.value = Some("white".into());
            Some(out)
        }
        "gruvbox" => {
            out.title = Some("yellow".into());
            out.keys = Some("208".into());
            out.value = Some("green".into());
            Some(out)
        }
        "nord" => {
            out.title = Some("blue".into());
            out.keys = Some("110".into());
            out.value = Some("white".into());
            Some(out)
        }
        "dracula" => {
            out.title = Some("magenta".into());
            out.keys = Some("141".into());
            out.value = Some("220".into());
            Some(out)
        }
        "tokyo-night" => {
            out.title = Some("magenta".into());
            out.keys = Some("151".into());
            out.value = Some("white".into());
            Some(out)
        }
        "everforest" => {
            out.title = Some("green".into());
            out.keys = Some("151".into());
            out.value = Some("yellow".into());
            Some(out)
        }
        _ => None,
    }
}

/// Look up a theme by name: user `themes` first (case-insensitive), then built-ins.
pub fn lookup_theme(cfg: &Config, name: &str) -> Option<ThemeColors> {
    if let Some(custom) = cfg.themes.as_ref() {
        for (key, colors) in custom {
            if key.eq_ignore_ascii_case(name) {
                return Some(colors.clone());
            }
        }
    }
    resolve_theme(name)
}

/// All available theme names: built-ins first, then user-defined ones.
pub fn all_theme_names(cfg: &Config) -> Vec<String> {
    let mut names: Vec<String> = BUILTIN_THEME_NAMES.iter().map(|s| s.to_string()).collect();
    if let Some(custom) = cfg.themes.as_ref() {
        let mut extra: Vec<String> = custom
            .keys()
            .filter(|k| {
                !BUILTIN_THEME_NAMES
                    .iter()
                    .any(|b| b.eq_ignore_ascii_case(k))
            })
            .cloned()
            .collect();
        extra.sort();
        names.extend(extra);
    }
    names
}

/// Merge a base theme with per-field overrides (overrides win when set).
pub fn merge_theme(base: ThemeColors, over: &ThemeSetting) -> ThemeColors {
    ThemeColors {
        title: over.title.clone().or(base.title),
        keys: over.keys.clone().or(base.keys),
        value: over.value.clone().or(base.value),
        separator: over.separator.clone().or(base.separator),
        logo_color: over.logo_color.clone().or(base.logo_color),
    }
}

/// Active theme selection: returns (selected name if any, merged colors).
pub fn active_theme(cfg: &Config) -> (Option<String>, ThemeColors) {
    let arg = match cfg.theme.as_ref() {
        Some(a) => a,
        None => return (None, ThemeColors::default()),
    };
    let name = arg.name().map(|s| s.to_string());
    let base = name
        .as_deref()
        .and_then(|n| lookup_theme(cfg, n))
        .unwrap_or_default();
    let over = arg.overrides();
    (name, merge_theme(base, &over))
}

/// Resolve a color spec to an ANSI escape. Supports the 8 basic names,
/// 0-255 numbers (256-color), "#rrggbb" hex (truecolor), and "auto".
pub fn parse_color(spec: &str) -> Option<String> {
    let basic = match spec.to_lowercase().as_str() {
        "black" => "\x1b[1;30m",
        "red" => "\x1b[1;31m",
        "green" => "\x1b[1;32m",
        "yellow" => "\x1b[1;33m",
        "blue" => "\x1b[1;34m",
        "magenta" | "purple" => "\x1b[1;35m",
        "cyan" => "\x1b[1;36m",
        "white" => "\x1b[1;37m",
        _ => "",
    };
    if !basic.is_empty() {
        return Some(basic.to_string());
    }

    if let Ok(n) = spec.parse::<u8>() {
        return Some(format!("\x1b[38;5;{n}m"));
    }

    let hex = spec.strip_prefix('#').unwrap_or(spec);
    if hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
        return Some(format!("\x1b[38;2;{r};{g};{b}m"));
    }

    None
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

/// Writable default location for `rustfetch` configs (never a fastfetch path).
pub fn default_config_path_for_write() -> PathBuf {
    if let Ok(base) = env::var("XDG_CONFIG_HOME") {
        return Path::new(&base).join("rustfetch/config.jsonc");
    }
    if let Ok(home) = env::var("HOME") {
        return Path::new(&home).join(".config/rustfetch/config.jsonc");
    }
    PathBuf::from("rustfetch-config.jsonc")
}

/// Persist the selected theme name into the config file.
/// Keeps every other key untouched; creates the file if missing.
pub fn save_theme_name(path: Option<&Path>, name: &str) -> Result<PathBuf, String> {
    let target: PathBuf = match path {
        Some(p) => p.to_path_buf(),
        None => find_default_config_path().unwrap_or_else(default_config_path_for_write),
    };

    let mut value = if target.exists() {
        let raw = fs::read_to_string(&target).map_err(|e| e.to_string())?;
        let stripped = strip_jsonc_comments(&raw);
        serde_json::from_str::<serde_json::Value>(&stripped).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    let obj = value
        .as_object_mut()
        .ok_or("config root must be an object")?;
    obj.insert("theme".to_string(), serde_json::json!({ "name": name }));

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let pretty = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;
    fs::write(&target, pretty + "\n").map_err(|e| e.to_string())?;
    Ok(target)
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
        "theme": {
            "name": "default"
        },
        "themes": {
            "sunset": {
                "title": "#ff9e64",
                "keys": "208",
                "value": "#7aa2f7",
                "separator": " => ",
                "logo_color": "208"
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
