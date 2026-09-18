//! Built-in theme library.
//!
//! Themes here are **layouts**, not just palettes: each one may set its own
//! logo (distro logo, ASCII art file, or bundled art), logo padding, key/value
//! separator, per-module key text, per-module key colours and module order —
//! the same surface a fastfetch preset config exposes.
//!
//! Definitions are written as JSONC strings and parsed once per lookup, which
//! keeps the data readable and keeps the schema in exactly one place
//! ([`crate::config::ThemeDef`]). `presets::tests` parses every preset so a
//! typo cannot silently degrade a theme.

use crate::config::{ThemeDef, strip_jsonc_comments};

pub struct Builtin {
    pub name: &'static str,
    pub def: ThemeDef,
}

/// Ordered gallery shown by `rustfetch --setup`.
const PRESETS: &[(&str, &str)] = &[
    (
        "default",
        r##"{
  "description": "classic rustfetch - distro logo, colon separator",
  "separator": ": "
}"##,
    ),
    (
        "minimal",
        r##"{
  "description": "no logo, four-letter keys, rainbow palette",
  "logo": "none",
  "separator": " \u203a  ",
  "title": "magenta",
  "value": "white",
  "modules": [
    "break",
    { "type": "os",       "key": "OS  ", "keyColor": "31" },
    { "type": "kernel",   "key": "KER ", "keyColor": "32" },
    { "type": "packages", "key": "PKG ", "keyColor": "33" },
    { "type": "shell",    "key": "SH  ", "keyColor": "34" },
    { "type": "terminal", "key": "TER ", "keyColor": "35" },
    { "type": "wm",       "key": "WM  ", "keyColor": "36" },
    { "type": "cpu",      "key": "CPU ", "keyColor": "36" },
    { "type": "memory",   "key": "MEM ", "keyColor": "36" },
    "break"
  ]
}"##,
    ),
    (
        "os",
        r##"{
  "description": "lowercase aligned keys beside the distro logo",
  "logo": { "source": "auto", "padding": { "top": 1, "right": 4 } },
  "separator": "  ",
  "title": "blue",
  "keys": "34",
  "value": "white",
  "modules": [
    "break",
    "title",
    "break",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "host",     "key": "host    " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "pkgs    " },
    { "type": "shell",    "key": "shell   " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    { "type": "disk",     "key": "disk    " },
    "break"
  ]
}"##,
    ),
    (
        "groups",
        r##"{
  "description": "grouped tree with branch glyphs, fastfetch preset style",
  "logo": { "source": "auto", "padding": { "top": 2, "right": 6 } },
  "separator": " \u279c  ",
  "keys": "208",
  "value": "white",
  "logo_color": "208",
  "modules": [
    "break",
    "break",
    { "type": "os",       "key": "SYSTEM   ", "keyColor": "33" },
    { "type": "kernel",   "key": " \u251c\u2500 kernel  ", "keyColor": "33" },
    { "type": "uptime",   "key": " \u251c\u2500 uptime  ", "keyColor": "33" },
    { "type": "packages", "key": " \u2514\u2500 packages", "keyColor": "33" },
    "break",
    { "type": "wm",       "key": "DESKTOP  ", "keyColor": "35" },
    { "type": "theme",    "key": " \u251c\u2500 theme   ", "keyColor": "35" },
    { "type": "icons",    "key": " \u251c\u2500 icons   ", "keyColor": "35" },
    { "type": "cursor",   "key": " \u251c\u2500 cursor  ", "keyColor": "35" },
    { "type": "terminal", "key": " \u2514\u2500 terminal", "keyColor": "35" },
    "break",
    { "type": "host",     "key": "HARDWARE ", "keyColor": "36" },
    { "type": "cpu",      "key": " \u251c\u2500 cpu     ", "keyColor": "36" },
    { "type": "gpu",      "key": " \u251c\u2500 gpu     ", "keyColor": "36" },
    { "type": "memory",   "key": " \u251c\u2500 memory  ", "keyColor": "36" },
    { "type": "swap",     "key": " \u251c\u2500 swap    ", "keyColor": "36" },
    { "type": "disk",     "key": " \u2514\u2500 disk    ", "keyColor": "36" },
    "break",
    "break"
  ]
}"##,
    ),
    (
        "hypr",
        r##"{
  "description": "colour-dot bar framing a compact block layout",
  "logo": "none",
  "separator": " ",
  "title": "cyan",
  "keys": "cyan",
  "value": "white",
  "modules": [
    "break",
    { "type": "custom", "format": "\u001b[90m\u25cf  \u001b[31m\u25cf  \u001b[32m\u25cf  \u001b[33m\u25cf  \u001b[34m\u25cf  \u001b[35m\u25cf  \u001b[36m\u25cf  \u001b[37m\u25cf" },
    "break",
    "title",
    "break",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "packages", "key": "pkgs    " },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "term    " },
    { "type": "wm",       "key": "wm      " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    { "type": "localip",  "key": "ip      " },
    "break",
    { "type": "custom", "format": "\u001b[90m\u25cf  \u001b[31m\u25cf  \u001b[32m\u25cf  \u001b[33m\u25cf  \u001b[34m\u25cf  \u001b[35m\u25cf  \u001b[36m\u25cf  \u001b[37m\u25cf" },
    "break"
  ]
}"##,
    ),
    (
        "nyarch",
        r##"{
  "description": "cat art, tilde keys and a tilde divider",
  "logo": { "art": "cat", "padding": { "top": 1, "right": 6 } },
  "separator": " ",
  "title": "magenta",
  "keys": "34",
  "value": "white",
  "logo_color": "magenta",
  "modules": [
    "break",
    "break",
    "title",
    { "type": "custom", "format": "~~~~~~~~~~~~~~~~~~~~~~~~~~" },
    { "type": "os",       "key": "~ " },
    { "type": "kernel",   "key": "~ " },
    { "type": "packages", "key": "~ " },
    { "type": "shell",    "key": "~ " },
    { "type": "terminal", "key": "~ " },
    { "type": "wm",       "key": "~ " },
    { "type": "uptime",   "key": "~ " },
    { "type": "cpu",      "key": "~ " },
    { "type": "memory",   "key": "~ " },
    { "type": "disk",     "key": "~ " },
    { "type": "custom", "format": "~~~~~~~~~~~~~~~~~~~~~~~~~~" },
    "break"
  ]
}"##,
    ),
    (
        "rose",
        r##"{
  "description": "rose art with petal separators and a soft pink palette",
  "logo": { "art": "rose", "padding": { "top": 1, "right": 5 } },
  "separator": " \u00b7 ",
  "title": "#f5bde6",
  "keys": "#f5a9d0",
  "value": "#e0def4",
  "logo_color": "#eb6f92",
  "modules": [
    "break",
    "break",
    "title",
    "break",
    { "type": "os",       "key": "\u2740 os       " },
    { "type": "kernel",   "key": "\u2740 kernel   " },
    { "type": "host",     "key": "\u2740 host     " },
    { "type": "packages", "key": "\u2740 packages " },
    { "type": "shell",    "key": "\u2740 shell    " },
    { "type": "terminal", "key": "\u2740 terminal " },
    { "type": "wm",       "key": "\u2740 wm       " },
    { "type": "uptime",   "key": "\u2740 uptime   " },
    { "type": "cpu",      "key": "\u2740 cpu      " },
    { "type": "memory",   "key": "\u2740 memory   " },
    { "type": "disk",     "key": "\u2740 disk     " },
    "break"
  ]
}"##,
    ),
    (
        "matrix",
        r##"{
  "description": "green terminal rain: triangle art, :: keys, palette blocks",
  "logo": { "art": "triangle", "padding": { "top": 1, "right": 4 } },
  "separator": " :: ",
  "title": "82",
  "keys": "40",
  "value": "46",
  "logo_color": "40",
  "modules": [
    "break",
    "break",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "pkgs    " },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    { "type": "disk",     "key": "disk    " },
    { "type": "localip",  "key": "ip      " },
    "break",
    "colors",
    "break"
  ]
}"##,
    ),
    (
        "terminal",
        r##"{
  "description": "no logo, shell-prompt keys, every module in one column",
  "logo": "none",
  "separator": " : ",
  "title": "green",
  "keys": "green",
  "value": "white",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",            "key": "os       " },
    { "type": "host",          "key": "host     " },
    { "type": "kernel",        "key": "kernel   " },
    { "type": "uptime",        "key": "uptime   " },
    { "type": "packages",      "key": "packages " },
    { "type": "shell",         "key": "shell    " },
    { "type": "terminal",      "key": "terminal " },
    { "type": "terminalfont",  "key": "termfont " },
    { "type": "cpu",           "key": "cpu      " },
    { "type": "gpu",           "key": "gpu      " },
    { "type": "memory",        "key": "memory   " },
    { "type": "swap",          "key": "swap     " },
    { "type": "disk",          "key": "disk     " },
    { "type": "localip",       "key": "ip       " },
    { "type": "locale",        "key": "locale   " },
    "break",
    "colors"
  ]
}"##,
    ),
    (
        "rice",
        r##"{
  "description": "card look: rule lines, bracketed group labels, piped values",
  "logo": { "source": "auto", "padding": { "top": 3, "right": 5 } },
  "separator": "  ",
  "title": "208",
  "keys": "214",
  "value": "#e8e6df",
  "logo_color": "214",
  "modules": [
    "break",
    "break",
    { "type": "custom", "format": "\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500" },
    { "type": "custom", "format": "[ system ]" },
    { "type": "os",       "key": "\u2502 os       " },
    { "type": "kernel",   "key": "\u2502 kernel   " },
    { "type": "uptime",   "key": "\u2502 uptime   " },
    { "type": "packages", "key": "\u2502 packages " },
    { "type": "custom", "format": "[ hardware ]" },
    { "type": "cpu",      "key": "\u2502 cpu      " },
    { "type": "gpu",      "key": "\u2502 gpu      " },
    { "type": "memory",   "key": "\u2502 memory   " },
    { "type": "disk",     "key": "\u2502 disk     " },
    { "type": "custom", "format": "[ session ]" },
    { "type": "wm",       "key": "\u2502 wm       " },
    { "type": "terminal", "key": "\u2502 terminal " },
    { "type": "shell",    "key": "\u2502 shell    " },
    { "type": "custom", "format": "\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500" },
    "break"
  ]
}"##,
    ),
    (
        "cyber",
        r##"{
  "description": "neon cyan on magenta, arrow keys and a glowing block mark",
  "logo": { "art": "blocks", "padding": { "top": 2, "right": 5 } },
  "separator": " \u27e9 ",
  "title": "#f7768e",
  "keys": "51",
  "value": "#c0caf5",
  "logo_color": "201",
  "modules": [
    "break",
    "break",
    { "type": "os",       "key": "\u25b8 os       " },
    { "type": "kernel",   "key": "\u25b8 kernel   " },
    { "type": "host",     "key": "\u25b8 host     " },
    { "type": "uptime",   "key": "\u25b8 uptime   " },
    { "type": "packages", "key": "\u25b8 packages " },
    { "type": "shell",    "key": "\u25b8 shell    " },
    { "type": "terminal", "key": "\u25b8 terminal " },
    { "type": "cpu",      "key": "\u25b8 cpu      " },
    { "type": "gpu",      "key": "\u25b8 gpu      " },
    { "type": "memory",   "key": "\u25b8 memory   " },
    { "type": "disk",     "key": "\u25b8 disk     " },
    { "type": "localip",  "key": "\u25b8 ip       " },
    "break"
  ]
}"##,
    ),
    (
        "retro",
        r##"{
  "description": "amber CRT: boxed wordmark, :: keys, prompt art",
  "logo": { "art": "prompt", "padding": { "top": 1, "right": 4 } },
  "separator": " \u00bb ",
  "title": "214",
  "keys": "208",
  "value": "222",
  "logo_color": "208",
  "modules": [
    "break",
    "break",
    { "type": "os",       "key": "os     " },
    { "type": "kernel",   "key": "kernel " },
    { "type": "uptime",   "key": "uptime " },
    { "type": "shell",    "key": "shell  " },
    { "type": "packages", "key": "pkgs   " },
    { "type": "cpu",      "key": "cpu    " },
    { "type": "memory",   "key": "memory " },
    { "type": "disk",     "key": "disk   " },
    "break",
    "colors",
    "break"
  ]
}"##,
    ),
    (
        "full-info",
        r##"{
  "description": "no logo, every module, wide keys",
  "logo": "none",
  "separator": ": ",
  "keys": "cyan",
  "value": "white",
  "modules": [
    "title",
    "separator",
    { "type": "os",            "key": "OS" },
    { "type": "host",          "key": "Host" },
    { "type": "kernel",        "key": "Kernel" },
    { "type": "uptime",        "key": "Uptime" },
    { "type": "packages",      "key": "Packages" },
    { "type": "shell",         "key": "Shell" },
    { "type": "display",       "key": "Display" },
    { "type": "de",            "key": "DE" },
    { "type": "wm",            "key": "WM" },
    { "type": "wmtheme",       "key": "WM Theme" },
    { "type": "theme",         "key": "Theme" },
    { "type": "icons",         "key": "Icons" },
    { "type": "font",          "key": "Font" },
    { "type": "cursor",        "key": "Cursor" },
    { "type": "terminal",      "key": "Terminal" },
    { "type": "terminalfont",  "key": "Terminal Font" },
    { "type": "cpu",           "key": "CPU" },
    { "type": "gpu",           "key": "GPU" },
    { "type": "memory",        "key": "Memory" },
    { "type": "swap",          "key": "Swap" },
    { "type": "disk",          "key": "Disk" },
    { "type": "battery",       "key": "Battery" },
    { "type": "poweradapter",  "key": "Power Adapter" },
    { "type": "audio",         "key": "Audio" },
    { "type": "localip",       "key": "Local IP" },
    { "type": "locale",        "key": "Locale" },
    "break",
    "colors"
  ]
}"##,
    ),
    (
        "paper",
        r##"{
  "description": "quiet monochrome: grey keys, middot separators, no logo",
  "logo": "none",
  "separator": " \u00b7 ",
  "title": "245",
  "keys": "245",
  "value": "252",
  "modules": [
    "break",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "host",     "key": "host    " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "pkgs    " },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "term    " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    { "type": "disk",     "key": "disk    " },
    "break"
  ]
}"##,
    ),
    (
        "neon",
        r##"{
  "description": "bright magenta title, cyan keys, white values",
  "title": "magenta",
  "keys": "cyan",
  "value": "white"
}"##,
    ),
    (
        "gruvbox",
        r##"{
  "description": "warm retro palette",
  "title": "yellow",
  "keys": "208",
  "value": "green"
}"##,
    ),
    (
        "nord",
        r##"{
  "description": "cool arctic palette",
  "title": "blue",
  "keys": "110",
  "value": "white"
}"##,
    ),
    (
        "dracula",
        r##"{
  "description": "dark purple palette",
  "title": "magenta",
  "keys": "141",
  "value": "220"
}"##,
    ),
    (
        "tokyo-night",
        r##"{
  "description": "night city palette",
  "title": "magenta",
  "keys": "151",
  "value": "white"
}"##,
    ),
    (
        "everforest",
        r##"{
  "description": "muted forest palette",
  "title": "green",
  "keys": "151",
  "value": "yellow"
}"##,
    ),
];

/// Parse a preset definition; `None` when the JSONC is malformed.
pub fn lookup(name: &str) -> Option<ThemeDef> {
    let (_, raw) = PRESETS.iter().find(|(n, _)| n.eq_ignore_ascii_case(name))?;
    serde_json::from_str::<ThemeDef>(&strip_jsonc_comments(raw)).ok()
}

/// All built-in presets, in gallery order.
pub fn builtin() -> Vec<Builtin> {
    PRESETS
        .iter()
        .map(|(name, raw)| Builtin {
            name,
            def: serde_json::from_str::<ThemeDef>(&strip_jsonc_comments(raw)).unwrap_or_default(),
        })
        .collect()
}

#[allow(dead_code)]
pub fn names() -> Vec<&'static str> {
    PRESETS.iter().map(|(n, _)| *n).collect()
}

#[allow(dead_code)]
pub fn count() -> usize {
    PRESETS.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_preset_parses_cleanly() {
        for (name, raw) in PRESETS {
            let parsed: Result<ThemeDef, _> = serde_json::from_str(&strip_jsonc_comments(raw));
            assert!(
                parsed.is_ok(),
                "Preset '{name}' failed to parse JSONC: {:?}",
                parsed.err()
            );
            let def = parsed.unwrap();
            let looked_up = lookup(name);
            assert!(looked_up.is_some(), "lookup failed for preset '{name}'");
            let _ = def.summary();
        }
    }

    #[test]
    fn presets_count_matches() {
        assert_eq!(builtin().len(), PRESETS.len());
        assert_eq!(names().len(), PRESETS.len());
        assert_eq!(count(), PRESETS.len());
        assert!(count() >= 15);
    }
}
