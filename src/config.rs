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
        #[serde(default, rename = "type")]
        logo_type: Option<String>,
        #[serde(default)]
        color: Option<serde_json::Value>,
        #[serde(default)]
        width: Option<usize>,
        #[serde(default)]
        height: Option<usize>,
        #[serde(default)]
        padding: Option<Padding>,
    },
}

/// Logo selection inside a theme: a built-in logo name, a path to an ASCII
/// art file (fastfetch style), or a built-in art shipped by `src/art.rs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogoDef {
    Name(String),
    Object {
        source: Option<String>,
        #[serde(default, rename = "type")]
        logo_type: Option<String>,
        #[serde(default)]
        art: Option<String>,
        #[serde(default)]
        width: Option<usize>,
        #[serde(default)]
        height: Option<usize>,
        #[serde(default)]
        padding: Option<Padding>,
    },
}

impl LogoDef {
    /// Logo name, ASCII-art path, or `art:<name>` reference; `None` when the
    /// theme hides the logo.
    pub fn source(&self) -> Option<&str> {
        match self {
            LogoDef::Name(s) => Some(s.as_str()),
            LogoDef::Object { source, art, .. } => source.as_deref().or(art.as_deref()),
        }
    }

    pub fn logo_type(&self) -> Option<&str> {
        match self {
            LogoDef::Name(_) => None,
            LogoDef::Object { logo_type, .. } => logo_type.as_deref(),
        }
    }

    pub fn width(&self) -> Option<usize> {
        match self {
            LogoDef::Name(_) => None,
            LogoDef::Object { width, .. } => *width,
        }
    }

    pub fn height(&self) -> Option<usize> {
        match self {
            LogoDef::Name(_) => None,
            LogoDef::Object { height, .. } => *height,
        }
    }

    pub fn padding(&self) -> Padding {
        match self {
            LogoDef::Name(_) => Padding::default(),
            LogoDef::Object { padding, .. } => (*padding).unwrap_or_default(),
        }
    }

    /// `true` when this definition asks explicitly for "no logo".
    pub fn is_hidden(&self) -> bool {
        matches!(self.source(), Some("none") | Some(""))
    }
}

/// Vertical/horizontal breathing room around the logo, in cells.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Padding {
    #[serde(default)]
    pub top: usize,
    #[serde(default)]
    pub left: usize,
    #[serde(default)]
    pub right: usize,
}

/// A module entry in a theme or config layout. Accepts the fastfetch forms:
///
/// ```jsonc
/// "modules": [
///   "break",
///   "os",
///   { "type": "kernel", "key": "├ ", "keyColor": "33", "keyWidth": 6 }
/// ]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModuleSetting {
    Name(String),
    Object {
        #[serde(rename = "type")]
        module_type: String,
        #[serde(default)]
        key: Option<String>,
        #[serde(default, rename = "keyColor", alias = "key_color")]
        key_color: Option<String>,
        #[serde(default)]
        format: Option<String>,
        #[serde(default, rename = "keyWidth", alias = "key_width")]
        key_width: Option<usize>,
        #[serde(default)]
        command: Option<String>,
        #[serde(default)]
        text: Option<String>,
        #[serde(default)]
        shell: Option<bool>,
        #[serde(default, rename = "timeout", alias = "timeoutMs", alias = "timeout_ms")]
        timeout_ms: Option<u64>,
        #[serde(default, rename = "showFailure", alias = "show_failure")]
        show_failure: Option<bool>,
    },
}

impl ModuleSetting {
    /// Canonical, snake_case module kind (`wmtheme` -> `wm_theme`).
    #[allow(dead_code)]
    pub fn kind(&self) -> &str {
        match self {
            ModuleSetting::Name(s) => s,
            ModuleSetting::Object { module_type, .. } => module_type,
        }
    }
}

/// Resolved, render-ready module entry.
#[derive(Debug, Clone, PartialEq)]
pub struct ModuleSpec {
    pub kind: String,
    pub key: Option<String>,
    pub key_color: Option<String>,
    pub format: Option<String>,
    pub key_width: Option<usize>,
    pub command: Option<String>,
    pub text: Option<String>,
    pub shell: bool,
    pub timeout_ms: u64,
    pub show_failure: bool,
}

impl Default for ModuleSpec {
    fn default() -> Self {
        Self {
            kind: String::new(),
            key: None,
            key_color: None,
            format: None,
            key_width: None,
            command: None,
            text: None,
            shell: false,
            timeout_ms: crate::info::command::DEFAULT_TIMEOUT_MS,
            show_failure: false,
        }
    }
}

impl From<&ModuleSetting> for ModuleSpec {
    fn from(m: &ModuleSetting) -> Self {
        match m {
            ModuleSetting::Name(s) => ModuleSpec {
                kind: normalize_module_name(s),
                ..Default::default()
            },
            ModuleSetting::Object {
                module_type,
                key,
                key_color,
                format,
                key_width,
                command,
                text,
                shell,
                timeout_ms,
                show_failure,
            } => ModuleSpec {
                kind: normalize_module_name(module_type),
                key: key.clone(),
                key_color: key_color.clone(),
                format: format.clone(),
                key_width: *key_width,
                command: command.clone(),
                text: text.clone(),
                shell: shell.unwrap_or(false),
                timeout_ms: timeout_ms.unwrap_or(crate::info::command::DEFAULT_TIMEOUT_MS),
                show_failure: show_failure.unwrap_or(false),
            },
        }
    }
}

/// fastfetch spelling -> rustfetch module kind.
pub fn normalize_module_name(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "wmtheme" => "wm_theme".into(),
        "terminalfont" => "terminal_font".into(),
        "poweradapter" => "power_adapter".into(),
        "localip" => "local_ip".into(),
        "publicip" => "public_ip".into(),
        "exec" => "command".into(),
        "static" => "custom".into(),
        other => other.to_string(),
    }
}

pub fn modules_to_specs(modules: &[ModuleSetting]) -> Vec<ModuleSpec> {
    modules.iter().map(ModuleSpec::from).collect()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisplaySetting {
    pub color: Option<serde_json::Value>,
    pub separator: Option<String>,
    pub key: Option<serde_json::Value>,
    /// When true, print info modules with empty/blank values.
    /// Default false (omit empties in human output). JSON output is unaffected.
    #[serde(default, rename = "showEmpty", alias = "show_empty")]
    pub show_empty: Option<bool>,
}

/// A complete theme: colours *and* layout.
///
/// Every field is optional, so a theme can be a pure palette (as in the first
/// release) or a full fastfetch-style layout preset. Unknown keys are ignored,
/// which keeps real fastfetch presets (`logo.type`, `logo.height`, module
/// `format` fields, ...) loadable.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThemeDef {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub distro: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub keys: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub separator: Option<String>,
    #[serde(default, rename = "logo_color", alias = "logoColor")]
    pub logo_color: Option<String>,
    #[serde(default)]
    pub logo: Option<LogoDef>,
    #[serde(default)]
    pub padding: Option<Padding>,
    #[serde(default)]
    pub modules: Option<Vec<ModuleSetting>>,
}

impl ThemeDef {
    /// Overlay `other` on top of `self`; fields set in `other` win.
    pub fn overlay(&self, other: &ThemeDef) -> ThemeDef {
        ThemeDef {
            description: other
                .description
                .clone()
                .or_else(|| self.description.clone()),
            distro: other.distro.clone().or_else(|| self.distro.clone()),
            title: other.title.clone().or_else(|| self.title.clone()),
            keys: other.keys.clone().or_else(|| self.keys.clone()),
            value: other.value.clone().or_else(|| self.value.clone()),
            separator: other.separator.clone().or_else(|| self.separator.clone()),
            logo_color: other.logo_color.clone().or_else(|| self.logo_color.clone()),
            logo: other.logo.clone().or_else(|| self.logo.clone()),
            padding: other.padding.or(self.padding),
            modules: other.modules.clone().or_else(|| self.modules.clone()),
        }
    }

    /// One-line summary used by listings and the setup TUI.
    pub fn summary(&self) -> String {
        let mut bits = Vec::new();
        for (label, v) in [
            ("title", &self.title),
            ("keys", &self.keys),
            ("value", &self.value),
        ] {
            if let Some(v) = v {
                bits.push(format!("{label}={v}"));
            }
        }
        if let Some(s) = &self.separator {
            bits.push(format!("sep={s:?}"));
        }
        if let Some(l) = &self.logo_color {
            bits.push(format!("logo={l}"));
        }
        if let Some(m) = &self.modules {
            bits.push(format!("{} modules", m.len()));
        }
        bits.join(", ")
    }

    /// `true` when the theme changes structure, not only colours.
    pub fn has_layout(&self) -> bool {
        self.modules.is_some() || self.logo.is_some() || self.padding.is_some()
    }

    pub fn logo_type(&self) -> Option<&str> {
        self.logo.as_ref().and_then(|l| l.logo_type())
    }

    pub fn logo_source(&self) -> Option<&str> {
        self.logo.as_ref().and_then(|l| l.source())
    }

    pub fn width(&self) -> Option<usize> {
        self.logo.as_ref().and_then(|l| l.width())
    }

    pub fn height(&self) -> Option<usize> {
        self.logo.as_ref().and_then(|l| l.height())
    }
}

/// Accepts `"theme": "gruvbox"` shorthand or the full object form.
#[allow(clippy::large_enum_variant)]
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

    /// The override half of this setting (everything except the base name).
    pub fn overrides(&self) -> ThemeDef {
        match self {
            ThemeArg::Name(_) => ThemeDef::default(),
            ThemeArg::Full(t) => t.overrides(),
        }
    }
}

/// Inline theme selection with per-field overrides:
///
/// ```jsonc
/// "theme": { "name": "groups", "keys": "208", "separator": " -> " }
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeSetting {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub distro: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub keys: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub separator: Option<String>,
    #[serde(default, rename = "logo_color", alias = "logoColor")]
    pub logo_color: Option<String>,
    #[serde(default)]
    pub logo: Option<LogoDef>,
    #[serde(default)]
    pub padding: Option<Padding>,
    #[serde(default)]
    pub modules: Option<Vec<ModuleSetting>>,
}

impl ThemeSetting {
    #[allow(dead_code)]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// The override half of this setting (everything except the base name).
    pub fn overrides(&self) -> ThemeDef {
        ThemeDef {
            description: self.description.clone(),
            distro: self.distro.clone(),
            title: self.title.clone(),
            keys: self.keys.clone(),
            value: self.value.clone(),
            separator: self.separator.clone(),
            logo_color: self.logo_color.clone(),
            logo: self.logo.clone(),
            padding: self.padding,
            modules: self.modules.clone(),
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
    /// User-defined themes, inline:
    /// `"themes": { "my-theme": { "keys": "cyan", "modules": [...] } }`.
    pub themes: Option<std::collections::HashMap<String, ThemeDef>>,
    /// Extra directory to load `*.jsonc` theme files from.
    pub themes_dir: Option<String>,
    pub modules: Option<Vec<ModuleSetting>>,
    pub disk_paths: Option<Vec<String>>,
    pub color_keys: Option<String>,
}

#[allow(dead_code)]
pub type ThemeColors = ThemeDef;

/// Where a theme definition came from.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ThemeSource {
    /// Shipped with the binary (`src/presets.rs`).
    #[default]
    Builtin,
    /// Inline in the config's `themes` map.
    Config,
    /// A `*.jsonc` file from the themes directory.
    File(PathBuf),
}

impl ThemeSource {
    /// Short tag shown by the setup TUI.
    pub fn tag(&self) -> &'static str {
        match self {
            ThemeSource::Builtin => "built-in",
            ThemeSource::Config => "config",
            ThemeSource::File(_) => "file",
        }
    }
}

/// A theme definition plus where it came from.
#[derive(Debug, Clone, Default)]
pub struct ThemeEntry {
    pub name: String,
    pub source: ThemeSource,
    pub def: ThemeDef,
}

impl ThemeEntry {
    /// Check if this theme matches or is tailored for the given Linux distro ID (e.g. "ubuntu").
    pub fn matches_distro(&self, distro_id: &str) -> bool {
        let id = distro_id.trim().to_lowercase();
        if id.is_empty() {
            return false;
        }
        if let Some(ref d) = self.def.distro
            && d.trim().eq_ignore_ascii_case(&id)
        {
            return true;
        }
        let n = self.name.to_lowercase();
        if n.starts_with(&format!("{id}-")) || n.starts_with(&format!("{id}_")) || n == id {
            return true;
        }
        false
    }
}

pub fn dirs_home() -> Option<PathBuf> {
    env::var("HOME").ok().map(PathBuf::from)
}

/// Expand a leading `~` or `$HOME` so themes can reference art files with the
/// same paths fastfetch presets use.
pub fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = dirs_home()
    {
        return home.join(rest);
    }
    if let Some(rest) = path.strip_prefix("$HOME/")
        && let Some(home) = dirs_home()
    {
        return home.join(rest);
    }
    PathBuf::from(path)
}

/// Directory scanned for `*.jsonc` / `*.json` theme files.
pub fn default_themes_dir(cfg: &Config) -> Option<PathBuf> {
    if let Some(dir) = cfg.themes_dir.as_deref() {
        return Some(expand_tilde(dir));
    }
    if let Ok(base) = env::var("XDG_CONFIG_HOME") {
        return Some(Path::new(&base).join("rustfetch/themes"));
    }
    dirs_home().map(|h| h.join(".config/rustfetch/themes"))
}

/// Load every theme file from the themes directory.
///
/// Each file is either a single theme definition (name taken from the file
/// stem) or `{ "themes": { name: def } }`.
pub fn load_theme_files(cfg: &Config) -> Vec<ThemeEntry> {
    let Some(dir) = default_themes_dir(cfg) else {
        return Vec::new();
    };
    let Ok(read) = fs::read_dir(&dir) else {
        return Vec::new();
    };

    let mut out: Vec<ThemeEntry> = Vec::new();
    for entry in read.flatten() {
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();
        if ext != "json" && ext != "jsonc" {
            continue;
        }
        let Ok(raw) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(value) = serde_json::from_str::<serde_json::Value>(&strip_jsonc_comments(&raw))
        else {
            continue;
        };

        if let Some(map) = value.get("themes").and_then(|v| v.as_object()) {
            for (name, def_value) in map {
                if let Ok(def) = serde_json::from_value::<ThemeDef>(def_value.clone()) {
                    out.push(ThemeEntry {
                        name: name.clone(),
                        source: ThemeSource::File(path.clone()),
                        def,
                    });
                }
            }
            continue;
        }

        if let Ok(def) = serde_json::from_value::<ThemeDef>(value) {
            out.push(ThemeEntry {
                name: path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unnamed")
                    .to_string(),
                source: ThemeSource::File(path),
                def,
            });
        }
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Every theme the user can pick: built-ins first, then theme files, then
/// inline config themes. Later sources win on name clashes.
pub fn all_themes(cfg: &Config) -> Vec<ThemeEntry> {
    let mut entries: Vec<ThemeEntry> = crate::presets::builtin()
        .into_iter()
        .map(|p| ThemeEntry {
            name: p.name.to_string(),
            source: ThemeSource::Builtin,
            def: p.def,
        })
        .collect();

    for entry in load_theme_files(cfg) {
        match entries
            .iter_mut()
            .find(|e| e.name.eq_ignore_ascii_case(&entry.name))
        {
            Some(existing) => *existing = entry,
            None => entries.push(entry),
        }
    }

    if let Some(map) = cfg.themes.as_ref() {
        let mut names: Vec<&String> = map.keys().collect();
        names.sort();
        for name in names {
            let entry = ThemeEntry {
                name: name.clone(),
                source: ThemeSource::Config,
                def: map[name].clone(),
            };
            match entries
                .iter_mut()
                .find(|e| e.name.eq_ignore_ascii_case(name))
            {
                Some(existing) => *existing = entry,
                None => entries.push(entry),
            }
        }
    }

    entries
}

/// All themes ordered with detected distro-specific themes at the top of the gallery.
pub fn all_themes_for_distro(cfg: &Config, distro_id: Option<&str>) -> Vec<ThemeEntry> {
    let all = all_themes(cfg);
    if let Some(distro) = distro_id {
        let d = distro.trim().to_lowercase();
        if !d.is_empty() {
            let mut matches = Vec::new();
            let mut rest = Vec::new();
            for entry in all {
                if entry.matches_distro(&d) {
                    matches.push(entry);
                } else {
                    rest.push(entry);
                }
            }
            matches.extend(rest);
            return matches;
        }
    }
    all
}

/// Case-insensitive lookup across built-ins, theme files and inline themes.
pub fn lookup_theme(cfg: &Config, name: &str) -> Option<ThemeEntry> {
    all_themes(cfg)
        .into_iter()
        .find(|e| e.name.eq_ignore_ascii_case(name))
}

/// Theme name selected by the config.
pub fn active_theme_name(cfg: &Config) -> Option<String> {
    cfg.theme
        .as_ref()
        .and_then(|t| t.name())
        .map(|s| s.to_string())
}

impl std::ops::Deref for ThemeEntry {
    type Target = ThemeDef;
    fn deref(&self) -> &Self::Target {
        &self.def
    }
}

/// Built-in preset lookup by name (used by CLI tests and preset resolution).
#[allow(dead_code)]
pub fn resolve_theme(name: &str) -> Option<ThemeDef> {
    crate::presets::lookup(name)
}

/// Fully resolved theme: base definition merged with the config's inline
/// overrides. A `--theme` value wins over the config's selection.
pub fn resolve_active_theme(cfg: &Config, cli_name: Option<&str>) -> ThemeEntry {
    let requested = cli_name
        .map(|s| s.to_string())
        .or_else(|| active_theme_name(cfg));

    let base = requested
        .as_deref()
        .and_then(|n| lookup_theme(cfg, n))
        .unwrap_or(ThemeEntry {
            name: "default".into(),
            source: ThemeSource::Builtin,
            def: crate::presets::lookup("default").unwrap_or_default(),
        });

    // Inline overrides belong to the config selection only; `--theme` picks a
    // theme as-is.
    let overrides = if cli_name.is_none() {
        cfg.theme
            .as_ref()
            .map(|t| t.overrides())
            .unwrap_or_default()
    } else {
        ThemeDef::default()
    };

    ThemeEntry {
        name: base.name,
        source: base.source,
        def: base.def.overlay(&overrides),
    }
}

/// All theme names, in display order.
#[allow(dead_code)]
pub fn all_theme_names(cfg: &Config) -> Vec<String> {
    all_themes(cfg).into_iter().map(|e| e.name).collect()
}

#[allow(dead_code)]
pub const BUILTIN_THEME_NAMES: &[&str] = &[
    "default",
    "minimal",
    "os",
    "groups",
    "hypr",
    "nyarch",
    "rose",
    "matrix",
    "cyber",
    "retro",
    "rice",
    "terminal",
    "paper",
    "full-info",
    "gruvbox",
    "dracula",
    "nord",
    "tokyo-night",
    "everforest",
    "neon",
];

/// Returns the currently active theme name (if configured) and its resolved definition.
#[allow(dead_code)]
pub fn active_theme(cfg: &Config) -> (Option<String>, ThemeDef) {
    let resolved = resolve_active_theme(cfg, None);
    (active_theme_name(cfg), resolved.def)
}

/// Resolve a color spec to an ANSI escape. Supports the 8 basic names,
/// 0-255 numbers (256-color), "#rrggbb" hex (truecolor), and "auto".
pub fn parse_color(spec: &str) -> Option<String> {
    if spec.starts_with("\x1b[") {
        return Some(spec.to_string());
    }

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

    pub fn get_logo_type(&self) -> Option<String> {
        match &self.logo {
            Some(LogoSetting::Object { logo_type, .. }) => logo_type.clone(),
            _ => None,
        }
    }

    pub fn get_logo_width(&self) -> Option<usize> {
        match &self.logo {
            Some(LogoSetting::Object { width, .. }) => *width,
            _ => None,
        }
    }

    pub fn get_logo_height(&self) -> Option<usize> {
        match &self.logo {
            Some(LogoSetting::Object { height, .. }) => *height,
            _ => None,
        }
    }

    pub fn get_logo_padding(&self) -> Option<Padding> {
        match &self.logo {
            Some(LogoSetting::Object { padding, .. }) => *padding,
            _ => None,
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

    /// Config-declared layout as normalized module names (legacy/backward compatibility).
    #[allow(dead_code)]
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
                "exec" => "command",
                "static" => "custom",
                other => other,
            };

            list.push(normalized.to_string());
        }

        Some(list)
    }

    /// Config-declared layout as render-ready specs.
    pub fn get_modules(&self) -> Option<Vec<ModuleSpec>> {
        self.modules.as_deref().map(modules_to_specs)
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

/// Resolve the config file a write should target.
pub fn write_target(path: Option<&Path>) -> PathBuf {
    match path {
        Some(p) => p.to_path_buf(),
        None => find_default_config_path().unwrap_or_else(default_config_path_for_write),
    }
}

/// Read a config file as a JSON value, tolerating JSONC comments. Returns an
/// empty object when the file is missing or unparsable.
fn read_config_value(target: &Path) -> Result<serde_json::Value, String> {
    if !target.exists() {
        return Ok(serde_json::json!({}));
    }
    let raw = fs::read_to_string(target).map_err(|e| e.to_string())?;
    let stripped = strip_jsonc_comments(&raw);
    Ok(serde_json::from_str::<serde_json::Value>(&stripped).unwrap_or(serde_json::json!({})))
}

fn write_config_value(target: &Path, value: &serde_json::Value) -> Result<PathBuf, String> {
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let pretty = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    fs::write(target, format!("{pretty}\n")).map_err(|e| e.to_string())?;
    Ok(target.to_path_buf())
}

/// Persist the selected theme name into the config file.
///
/// Every other key is preserved, but the file is rewritten as pretty JSON —
/// JSONC comments are dropped by this operation.
pub fn save_theme_name(path: Option<&Path>, name: &str) -> Result<PathBuf, String> {
    let target = write_target(path);
    let mut value = read_config_value(&target)?;

    let obj = value
        .as_object_mut()
        .ok_or("config root must be an object")?;
    obj.insert("theme".to_string(), serde_json::json!({ "name": name }));

    write_config_value(&target, &value)
}

/// Copy a theme definition into the config's `themes` map and make it active,
/// so the user gets an editable copy of a built-in preset.
pub fn export_theme(
    path: Option<&Path>,
    def: &ThemeDef,
    new_name: &str,
) -> Result<PathBuf, String> {
    let target = write_target(path);
    let mut value = read_config_value(&target)?;

    let obj = value
        .as_object_mut()
        .ok_or("config root must be an object")?;

    let themes = obj.entry("themes").or_insert_with(|| serde_json::json!({}));
    let map = themes.as_object_mut().ok_or("`themes` must be an object")?;
    map.insert(
        new_name.to_string(),
        serde_json::to_value(def).map_err(|e| e.to_string())?,
    );

    obj.insert("theme".to_string(), serde_json::json!({ "name": new_name }));

    write_config_value(&target, &value)
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
            },
            "showEmpty": false
        },
        // Active theme: built-in preset name, or the full object form below.
        // Themes carry layout (logo, padding, module order, per-module keys and
        // colours) as well as colours. Run `rustfetch --setup` for the gallery,
        // or `rustfetch --list-themes` for the plain list.
        "theme": {
            "name": "default"
        },
        // Your own themes. Drop files into ~/.config/rustfetch/themes/*.jsonc
        // instead if you prefer one file per theme.
        "themes": {
            "my-rice": {
                "description": "an example layout theme",
                "logo": {
                    "source": "auto",
                    "padding": { "top": 1, "right": 4 }
                },
                "separator": "  ",
                "title": "#b4befe",
                "keys": "183",
                "value": "#cdd6f4",
                "logo_color": "#f5c2e7",
                "modules": [
                    "break",
                    { "type": "os", "key": "os    ", "keyColor": "183" },
                    { "type": "kernel", "key": "kernel", "keyColor": "183" },
                    { "type": "host", "key": "host  ", "keyColor": "183" },
                    { "type": "cpu", "key": "cpu   ", "keyColor": "183" },
                    { "type": "memory", "key": "memory", "keyColor": "183" },
                    "break"
                ]
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

const SUPPORTED_MODULES: &[&str] = &[
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
    "public_ip",
    "locale",
    "command",
    "custom",
    "break",
    "colors",
];

const MAPPED_TOP_LEVEL: &[&str] = &[
    "$schema",
    "logo",
    "logo_color",
    "no_logo",
    "no_color",
    "display",
    "theme",
    "themes",
    "themes_dir",
    "modules",
    "disk_paths",
    "color_keys",
];

const MAPPED_LOGO_KEYS: &[&str] = &["source", "type", "color", "width", "height", "padding"];
const MAPPED_DISPLAY_KEYS: &[&str] = &["separator", "color", "key", "showEmpty", "show_empty"];
const MAPPED_MODULE_KEYS: &[&str] = &[
    "type",
    "key",
    "keyColor",
    "key_color",
    "format",
    "keyWidth",
    "key_width",
    "folders",
    "command",
    "text",
    "shell",
    "timeout",
    "timeoutMs",
    "timeout_ms",
    "showFailure",
    "show_failure",
];

fn is_supported_module(kind: &str) -> bool {
    SUPPORTED_MODULES.contains(&kind)
}

/// Default search paths for a fastfetch config to import.
pub fn find_fastfetch_config_path() -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(base) = env::var("XDG_CONFIG_HOME") {
        candidates.push(Path::new(&base).join("fastfetch/config.jsonc"));
        candidates.push(Path::new(&base).join("fastfetch/config.json"));
    }
    if let Ok(home) = env::var("HOME") {
        candidates.push(Path::new(&home).join(".config/fastfetch/config.jsonc"));
        candidates.push(Path::new(&home).join(".config/fastfetch/config.json"));
    }

    candidates.into_iter().find(|p| p.exists())
}

/// Result of mapping a fastfetch config into rustfetch form.
#[derive(Debug, Clone)]
pub struct FastfetchImport {
    pub config: serde_json::Value,
    pub mapped_modules: usize,
    pub warnings: Vec<String>,
    pub disk_paths: Option<Vec<String>>,
}

fn map_logo_value(logo: &serde_json::Value, warnings: &mut Vec<String>) -> serde_json::Value {
    match logo {
        serde_json::Value::String(_) => logo.clone(),
        serde_json::Value::Object(map) => {
            let mut out = serde_json::Map::new();
            for (k, v) in map {
                if MAPPED_LOGO_KEYS.contains(&k.as_str()) {
                    out.insert(k.clone(), v.clone());
                } else {
                    warnings.push(format!("unmapped logo key '{k}' (ignored)"));
                }
            }
            serde_json::Value::Object(out)
        }
        other => {
            warnings.push(format!("unsupported logo value type ({other}), ignored"));
            serde_json::Value::Null
        }
    }
}

fn map_display_value(display: &serde_json::Value, warnings: &mut Vec<String>) -> serde_json::Value {
    let Some(map) = display.as_object() else {
        warnings.push("unsupported display value (ignored)".into());
        return serde_json::Value::Null;
    };
    let mut out = serde_json::Map::new();
    for (k, v) in map {
        if MAPPED_DISPLAY_KEYS.contains(&k.as_str()) {
            out.insert(k.clone(), v.clone());
        } else {
            warnings.push(format!("unmapped display key '{k}' (ignored)"));
        }
    }
    serde_json::Value::Object(out)
}

fn map_module_entry(
    entry: &serde_json::Value,
    warnings: &mut Vec<String>,
    disk_paths: &mut Option<Vec<String>>,
) -> Option<serde_json::Value> {
    match entry {
        serde_json::Value::String(name) => {
            let kind = normalize_module_name(name);
            if is_supported_module(&kind) {
                Some(serde_json::Value::String(kind))
            } else {
                warnings.push(format!("unmapped module '{name}' (ignored)"));
                None
            }
        }
        serde_json::Value::Object(map) => {
            let raw_type = map
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if raw_type.is_empty() {
                warnings.push("module object missing 'type' (ignored)".into());
                return None;
            }
            let kind = normalize_module_name(&raw_type);
            if !is_supported_module(&kind) {
                warnings.push(format!("unmapped module '{raw_type}' (ignored)"));
                return None;
            }

            for (k, _) in map {
                if !MAPPED_MODULE_KEYS.contains(&k.as_str()) {
                    warnings.push(format!(
                        "unmapped module key '{k}' on '{raw_type}' (ignored)"
                    ));
                }
            }

            if let Some(folders) = map.get("folders").and_then(|v| v.as_array()) {
                let paths: Vec<String> = folders
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                if !paths.is_empty() {
                    match disk_paths {
                        Some(existing) => {
                            for p in paths {
                                if !existing.contains(&p) {
                                    existing.push(p);
                                }
                            }
                        }
                        None => *disk_paths = Some(paths),
                    }
                }
            }

            let mut out = serde_json::Map::new();
            out.insert("type".into(), serde_json::Value::String(kind.clone()));
            if let Some(key) = map.get("key") {
                out.insert("key".into(), key.clone());
            }
            if let Some(kc) = map.get("keyColor").or_else(|| map.get("key_color")) {
                out.insert("keyColor".into(), kc.clone());
            }
            if let Some(fmt) = map.get("format") {
                out.insert("format".into(), fmt.clone());
            }
            if let Some(kw) = map.get("keyWidth").or_else(|| map.get("key_width")) {
                out.insert("keyWidth".into(), kw.clone());
            }
            if kind == "command" {
                if let Some(cmd) = map.get("command") {
                    out.insert("command".into(), cmd.clone());
                } else if let Some(text) = map.get("text") {
                    out.insert("command".into(), text.clone());
                }
                if let Some(text) = map.get("text") {
                    out.insert("text".into(), text.clone());
                }
            } else if let Some(text) = map.get("text") {
                out.insert("text".into(), text.clone());
            }
            if let Some(cmd) = map.get("command")
                && kind != "command"
            {
                out.insert("command".into(), cmd.clone());
            }
            if let Some(shell) = map.get("shell") {
                out.insert("shell".into(), shell.clone());
            }
            if let Some(t) = map
                .get("timeout")
                .or_else(|| map.get("timeoutMs"))
                .or_else(|| map.get("timeout_ms"))
            {
                out.insert("timeout".into(), t.clone());
            }
            if let Some(sf) = map.get("showFailure").or_else(|| map.get("show_failure")) {
                out.insert("showFailure".into(), sf.clone());
            }
            Some(serde_json::Value::Object(out))
        }
        other => {
            warnings.push(format!("unsupported module entry ({other}), ignored"));
            None
        }
    }
}

/// Parse a fastfetch JSON/JSONC document and map supported keys into rustfetch form.
pub fn map_fastfetch_config(raw: &str) -> Result<FastfetchImport, String> {
    let stripped = strip_jsonc_comments(raw);
    let value: serde_json::Value =
        serde_json::from_str(&stripped).map_err(|e| format!("invalid JSON/JSONC: {e}"))?;
    let obj = value
        .as_object()
        .ok_or_else(|| "config root must be a JSON object".to_string())?;

    let mut warnings = Vec::new();
    let mut out = serde_json::Map::new();
    let mut disk_paths: Option<Vec<String>> = None;

    for key in obj.keys() {
        if !MAPPED_TOP_LEVEL.contains(&key.as_str()) {
            warnings.push(format!("unmapped key '{key}' (ignored)"));
        }
    }

    out.insert(
        "$schema".into(),
        serde_json::json!(
            "https://github.com/fastfetch-cli/fastfetch/raw/dev/doc/json_schema.json"
        ),
    );

    if let Some(logo) = obj.get("logo") {
        let mapped = map_logo_value(logo, &mut warnings);
        if !mapped.is_null() {
            out.insert("logo".into(), mapped);
        }
    }
    if let Some(v) = obj.get("logo_color") {
        out.insert("logo_color".into(), v.clone());
    }
    if let Some(v) = obj.get("no_logo") {
        out.insert("no_logo".into(), v.clone());
    }
    if let Some(v) = obj.get("no_color") {
        out.insert("no_color".into(), v.clone());
    }
    if let Some(v) = obj.get("color_keys") {
        out.insert("color_keys".into(), v.clone());
    }
    if let Some(display) = obj.get("display") {
        let mapped = map_display_value(display, &mut warnings);
        if !mapped.is_null() {
            out.insert("display".into(), mapped);
        }
    }
    if let Some(v) = obj.get("theme") {
        out.insert("theme".into(), v.clone());
    }
    if let Some(v) = obj.get("themes") {
        out.insert("themes".into(), v.clone());
    }
    if let Some(v) = obj.get("themes_dir") {
        out.insert("themes_dir".into(), v.clone());
    }
    if let Some(v) = obj.get("disk_paths") {
        if let Some(arr) = v.as_array() {
            disk_paths = Some(
                arr.iter()
                    .filter_map(|x| x.as_str().map(|s| s.to_string()))
                    .collect(),
            );
        } else {
            out.insert("disk_paths".into(), v.clone());
        }
    }

    let mut mapped_modules = 0usize;
    if let Some(modules) = obj.get("modules") {
        let Some(arr) = modules.as_array() else {
            return Err("`modules` must be an array".into());
        };
        let mut mapped = Vec::new();
        for entry in arr {
            if let Some(m) = map_module_entry(entry, &mut warnings, &mut disk_paths) {
                mapped.push(m);
                mapped_modules += 1;
            }
        }
        out.insert("modules".into(), serde_json::Value::Array(mapped));
    }

    if let Some(paths) = &disk_paths {
        out.insert("disk_paths".into(), serde_json::json!(paths));
    }

    Ok(FastfetchImport {
        config: serde_json::Value::Object(out),
        mapped_modules,
        warnings,
        disk_paths,
    })
}

/// Load a fastfetch config file from disk and map it.
pub fn import_fastfetch_from_path(path: &Path) -> Result<FastfetchImport, String> {
    let raw =
        fs::read_to_string(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    map_fastfetch_config(&raw)
}

/// Write an imported config to `dest`. Refuses to overwrite unless `force`.
pub fn write_imported_config(
    dest: &Path,
    value: &serde_json::Value,
    force: bool,
) -> Result<PathBuf, String> {
    if dest.exists() && !force {
        return Err(format!(
            "refusing to overwrite existing config at {} (use --force)",
            dest.display()
        ));
    }
    write_config_value(dest, value)
}

#[cfg(test)]
mod import_tests {
    use super::*;

    #[test]
    fn maps_fastfetch_fixture_modules() {
        let raw = include_str!("../tests/fixtures/fastfetch_sample.jsonc");
        let imported = map_fastfetch_config(raw).expect("fixture should parse");

        assert_eq!(imported.mapped_modules, 11);
        assert!(
            imported
                .warnings
                .iter()
                .any(|w| w.contains("unmapped key 'general'"))
        );
        assert!(
            imported
                .warnings
                .iter()
                .any(|w| w.contains("unmapped key 'stat'"))
        );
        assert!(
            imported
                .warnings
                .iter()
                .any(|w| w.contains("unmapped module 'weather'"))
        );
        assert!(
            imported
                .warnings
                .iter()
                .any(|w| w.contains("unmapped logo key 'preserveAspectRatio'"))
        );
        assert!(
            imported
                .warnings
                .iter()
                .any(|w| w.contains("unmapped display key 'brightColor'"))
        );

        let modules = imported.config["modules"].as_array().unwrap();
        let kinds: Vec<&str> = modules
            .iter()
            .map(|m| match m {
                serde_json::Value::String(s) => s.as_str(),
                serde_json::Value::Object(o) => o["type"].as_str().unwrap(),
                _ => panic!("unexpected module entry"),
            })
            .collect();

        assert_eq!(
            kinds,
            vec![
                "title",
                "separator",
                "os",
                "kernel",
                "terminal_font",
                "local_ip",
                "power_adapter",
                "wm_theme",
                "disk",
                "break",
                "colors",
            ]
        );

        assert_eq!(
            imported.disk_paths.as_deref(),
            Some(vec!["/".to_string()].as_slice())
        );
        assert_eq!(
            imported.config["display"]["separator"].as_str(),
            Some(" -> ")
        );
        assert_eq!(imported.config["logo"]["source"].as_str(), Some("arch"));

        let cfg: Config =
            serde_json::from_value(imported.config.clone()).expect("mapped config deserializes");
        let normalized = cfg.get_normalized_modules().unwrap();
        assert!(normalized.contains(&"terminal_font".to_string()));
        assert!(normalized.contains(&"local_ip".to_string()));
        assert!(!normalized.iter().any(|m| m == "weather"));
    }

    #[test]
    fn strips_jsonc_before_import() {
        let raw = "{\n  // comment\n  \"modules\": [\"os\" /* hi */, \"cpu\"]\n}\n";
        let imported = map_fastfetch_config(raw).unwrap();
        assert_eq!(imported.mapped_modules, 2);
    }

    #[test]
    fn maps_fastfetch_command_module() {
        let raw = r#"{
            "modules": [
                { "type": "command", "key": "Editor", "text": "echo vim" },
                { "type": "custom", "key": "Git", "text": "zack" }
            ]
        }"#;
        let imported = map_fastfetch_config(raw).unwrap();
        assert_eq!(imported.mapped_modules, 2);
        let modules = imported.config["modules"].as_array().unwrap();
        assert_eq!(modules[0]["type"], "command");
        assert_eq!(modules[0]["command"], "echo vim");
        assert_eq!(modules[0]["key"], "Editor");
        assert_eq!(modules[1]["type"], "custom");
        assert_eq!(modules[1]["text"], "zack");
    }
}
