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
        "kitty-modern",
        r##"{
  "description": "kitty graphics image logo - sleek two-column modern card",
  "logo": {
    "source": "kitty:example",
    "type": "kitty",
    "width": 30,
    "padding": { "top": 1, "left": 1, "right": 4 }
  },
  "separator": " \u2500 ",
  "keys": "bright_cyan",
  "value": "white",
  "modules": [
    "title",
    "separator",
    { "type": "os",       "key": "OS      ", "keyColor": "cyan" },
    { "type": "host",     "key": "Host    ", "keyColor": "cyan" },
    { "type": "kernel",   "key": "Kernel  ", "keyColor": "cyan" },
    { "type": "uptime",   "key": "Uptime  ", "keyColor": "cyan" },
    { "type": "packages", "key": "Packages", "keyColor": "cyan" },
    { "type": "shell",    "key": "Shell   ", "keyColor": "cyan" },
    { "type": "terminal", "key": "Terminal", "keyColor": "cyan" },
    { "type": "cpu",      "key": "CPU     ", "keyColor": "cyan" },
    { "type": "memory",   "key": "Memory  ", "keyColor": "cyan" },
    { "type": "disk",     "key": "Disk    ", "keyColor": "cyan" },
    "break",
    "colors"
  ]
}"##,
    ),
    (
        "kitty-card",
        r##"{
  "description": "kitty graphics image logo - boxed card frame with green accents",
  "logo": {
    "source": "kitty:example",
    "type": "kitty",
    "width": 32,
    "padding": { "top": 2, "left": 1, "right": 5 }
  },
  "separator": " \u2502 ",
  "keys": "bright_green",
  "value": "white",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "system  ", "keyColor": "green" },
    { "type": "kernel",   "key": "kernel  ", "keyColor": "green" },
    { "type": "uptime",   "key": "uptime  ", "keyColor": "green" },
    { "type": "packages", "key": "packages", "keyColor": "green" },
    "break",
    { "type": "wm",       "key": "desktop ", "keyColor": "bright_green" },
    { "type": "terminal", "key": "terminal", "keyColor": "bright_green" },
    { "type": "shell",    "key": "shell   ", "keyColor": "bright_green" },
    "break",
    { "type": "cpu",      "key": "cpu     ", "keyColor": "yellow" },
    { "type": "memory",   "key": "memory  ", "keyColor": "yellow" },
    { "type": "disk",     "key": "disk    ", "keyColor": "yellow" },
    "break",
    "colors"
  ]
}"##,
    ),
    (
        "kitty-minimal",
        r##"{
  "description": "kitty graphics image logo - compact 4-letter rainbow metrics",
  "logo": {
    "source": "kitty:example",
    "type": "kitty",
    "width": 26,
    "padding": { "top": 1, "left": 1, "right": 4 }
  },
  "separator": " \u203a ",
  "title": "magenta",
  "value": "white",
  "modules": [
    "break",
    { "type": "os",       "key": "OS  ", "keyColor": "31" },
    { "type": "kernel",   "key": "KER ", "keyColor": "32" },
    { "type": "packages", "key": "PKG ", "keyColor": "33" },
    { "type": "shell",    "key": "SH  ", "keyColor": "34" },
    { "type": "terminal", "key": "TER ", "keyColor": "35" },
    { "type": "cpu",      "key": "CPU ", "keyColor": "36" },
    { "type": "memory",   "key": "MEM ", "keyColor": "36" },
    { "type": "disk",     "key": "DSK ", "keyColor": "37" },
    "break",
    "colors"
  ]
}"##,
    ),
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
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "host",     "key": "host    " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "wm",       "key": "wm      " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    { "type": "disk",     "key": "disk    " },
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
    (
        "ubuntu-classic",
        r##"{
  "description": "canonical orange and aubergine: official Ubuntu layout",
  "distro": "ubuntu",
  "logo": "ubuntu",
  "separator": " \u203a ",
  "title": "#e95420",
  "keys": "208",
  "value": "white",
  "logo_color": "208",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "OS       " },
    { "type": "host",     "key": "Host     " },
    { "type": "kernel",   "key": "Kernel   " },
    { "type": "uptime",   "key": "Uptime   " },
    { "type": "packages", "key": "Packages " },
    { "type": "shell",    "key": "Shell    " },
    { "type": "de",       "key": "DE       " },
    { "type": "wm",       "key": "WM       " },
    { "type": "terminal", "key": "Terminal " },
    { "type": "cpu",      "key": "CPU      " },
    { "type": "memory",   "key": "Memory   " },
    { "type": "disk",     "key": "Disk     " },
    "break",
    "colors"
  ]
}"##,
    ),
    (
        "ubuntu-modern",
        r##"{
  "description": "sleek modern Ubuntu: orange accents, color badges, clean hierarchy",
  "distro": "ubuntu",
  "logo": { "source": "ubuntu", "padding": { "top": 1, "right": 4 } },
  "separator": "  ",
  "title": "#e95420",
  "keys": "#ff7f50",
  "value": "white",
  "logo_color": "#e95420",
  "modules": [
    "break",
    { "type": "custom", "format": "\u001b[38;2;233;84;32m\u25cf  \u001b[38;2;119;33;111m\u25cf  \u001b[38;2;255;127;80m\u25cf  \u001b[37m\u25cf\u001b[0m" },
    "title",
    { "type": "custom", "format": "\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500" },
    { "type": "os",       "key": "os       ", "keyColor": "208" },
    { "type": "host",     "key": "host     ", "keyColor": "208" },
    { "type": "kernel",   "key": "kernel   ", "keyColor": "208" },
    { "type": "uptime",   "key": "uptime   ", "keyColor": "208" },
    { "type": "packages", "key": "packages ", "keyColor": "208" },
    { "type": "shell",    "key": "shell    ", "keyColor": "208" },
    { "type": "terminal", "key": "terminal ", "keyColor": "208" },
    { "type": "cpu",      "key": "cpu      ", "keyColor": "208" },
    { "type": "memory",   "key": "memory   ", "keyColor": "208" },
    { "type": "disk",     "key": "disk     ", "keyColor": "208" },
    { "type": "custom", "format": "\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500" },
    "break"
  ]
}"##,
    ),
    (
        "ubuntu-tree",
        r##"{
  "description": "grouped tree branches in Ubuntu orange with distro logo",
  "distro": "ubuntu",
  "logo": { "source": "ubuntu", "padding": { "top": 2, "right": 5 } },
  "separator": " \u279c ",
  "title": "#e95420",
  "keys": "208",
  "value": "white",
  "logo_color": "208",
  "modules": [
    "break",
    "break",
    { "type": "os",       "key": "SYSTEM   ", "keyColor": "208" },
    { "type": "kernel",   "key": " \u251c\u2500 kernel  ", "keyColor": "208" },
    { "type": "uptime",   "key": " \u251c\u2500 uptime  ", "keyColor": "208" },
    { "type": "packages", "key": " \u2514\u2500 packages", "keyColor": "208" },
    "break",
    { "type": "de",       "key": "DESKTOP  ", "keyColor": "133" },
    { "type": "wm",       "key": " \u251c\u2500 wm      ", "keyColor": "133" },
    { "type": "terminal", "key": " \u251c\u2500 terminal", "keyColor": "133" },
    { "type": "shell",    "key": " \u2514\u2500 shell   ", "keyColor": "133" },
    "break",
    { "type": "cpu",      "key": "HARDWARE ", "keyColor": "214" },
    { "type": "memory",   "key": " \u251c\u2500 memory  ", "keyColor": "214" },
    { "type": "disk",     "key": " \u2514\u2500 disk    ", "keyColor": "214" },
    "break"
  ]
}"##,
    ),
    (
        "ubuntu-minimal",
        r##"{
  "description": "compact Ubuntu: aubergine title, orange keys, fast single-column",
  "distro": "ubuntu",
  "logo": "none",
  "separator": " \u00bb ",
  "title": "#77216f",
  "keys": "#e95420",
  "value": "white",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break",
    "colors"
  ]
}"##,
    ),
    (
        "ubuntu-mini",
        r##"{
  "description": "compact mini Ubuntu circle-of-friends ASCII art",
  "distro": "ubuntu",
  "logo": { "art": "ubuntu-mini", "padding": { "top": 1, "right": 4 } },
  "separator": " \u2022 ",
  "title": "208",
  "keys": "214",
  "value": "white",
  "logo_color": "208",
  "modules": [
    "break",
    "title",
    { "type": "os",       "key": "os    " },
    { "type": "kernel",   "key": "kernel" },
    { "type": "uptime",   "key": "uptime" },
    { "type": "packages", "key": "pkgs  " },
    { "type": "shell",    "key": "shell " },
    { "type": "cpu",      "key": "cpu   " },
    { "type": "memory",   "key": "memory" },
    "break"
  ]
}"##,
    ),
    (
        "arch-clean",
        r##"{
  "description": "Arch Linux minimalism: cyan & cold blue arrows",
  "distro": "arch",
  "logo": { "source": "arch", "padding": { "top": 1, "right": 4 } },
  "separator": " \u279c ",
  "title": "39",
  "keys": "33",
  "value": "white",
  "logo_color": "39",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "debian-swirl",
        r##"{
  "description": "Debian crimson swirl: classic red ruby & soft white",
  "distro": "debian",
  "logo": { "source": "debian", "padding": { "top": 1, "right": 4 } },
  "separator": " \u2237 ",
  "title": "197",
  "keys": "161",
  "value": "white",
  "logo_color": "197",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "fedora-blue",
        r##"{
  "description": "Fedora navy & ocean blue with clean double-colon keys",
  "distro": "fedora",
  "logo": { "source": "fedora", "padding": { "top": 1, "right": 4 } },
  "separator": " :: ",
  "title": "33",
  "keys": "75",
  "value": "white",
  "logo_color": "33",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "nixos-snowflake",
        r##"{
  "description": "NixOS glacial cyan & snowflake motif",
  "distro": "nixos",
  "logo": { "source": "nixos", "padding": { "top": 1, "right": 4 } },
  "separator": " \u00b7 ",
  "title": "81",
  "keys": "123",
  "value": "white",
  "logo_color": "81",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "bedrock-strata",
        r##"{
  "description": "Bedrock Linux: stratified multi-distro meta layout with silver accents",
  "distro": "bedrock",
  "logo": { "source": "bedrock", "padding": { "top": 1, "right": 4 } },
  "separator": " :: ",
  "title": "white",
  "keys": "250",
  "value": "white",
  "logo_color": "white",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "gentoo-purple",
        r##"{
  "description": "Gentoo Linux: compiled from source purple & lavender aesthetic",
  "distro": "gentoo",
  "logo": { "source": "gentoo", "padding": { "top": 1, "right": 4 } },
  "separator": " \u00bb ",
  "title": "141",
  "keys": "177",
  "value": "white",
  "logo_color": "141",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "cachyos-speed",
        r##"{
  "description": "CachyOS: blazing fast x86-64-v3/v4 optimized cyan & teal layout",
  "distro": "cachyos",
  "logo": { "source": "cachyos", "padding": { "top": 1, "right": 4 } },
  "separator": " \u203a ",
  "title": "51",
  "keys": "44",
  "value": "white",
  "logo_color": "51",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "mint-fresh",
        r##"{
  "description": "Linux Mint: fresh mint green & silver leaf elegance",
  "distro": "mint",
  "logo": { "source": "mint", "padding": { "top": 1, "right": 4 } },
  "separator": " \u2022 ",
  "title": "112",
  "keys": "120",
  "value": "white",
  "logo_color": "112",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "opensuse-geek",
        r##"{
  "description": "openSUSE: gecko green & chameleon rolling speed",
  "distro": "opensuse",
  "logo": { "source": "opensuse", "padding": { "top": 1, "right": 4 } },
  "separator": " \u203a ",
  "title": "112",
  "keys": "148",
  "value": "white",
  "logo_color": "112",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "pop-cosmic",
        r##"{
  "description": "Pop!_OS: cosmic teal & energetic amber accents",
  "distro": "pop",
  "logo": { "source": "pop", "padding": { "top": 1, "right": 4 } },
  "separator": " \u276f ",
  "title": "37",
  "keys": "214",
  "value": "white",
  "logo_color": "37",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "void-xbps",
        r##"{
  "description": "Void Linux: runit fast, xbps green & graphite precision",
  "distro": "void",
  "logo": { "source": "void", "padding": { "top": 1, "right": 4 } },
  "separator": " :: ",
  "title": "35",
  "keys": "71",
  "value": "white",
  "logo_color": "35",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "alpine-peak",
        r##"{
  "description": "Alpine Linux: ultra-lightweight alpine blue & summit white",
  "distro": "alpine",
  "logo": { "source": "alpine", "padding": { "top": 1, "right": 4 } },
  "separator": " \u2227 ",
  "title": "32",
  "keys": "75",
  "value": "white",
  "logo_color": "32",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "manjaro-teal",
        r##"{
  "description": "Manjaro: signature dark teal & neon emerald flow",
  "distro": "manjaro",
  "logo": { "source": "manjaro", "padding": { "top": 1, "right": 4 } },
  "separator": " \u279c ",
  "title": "36",
  "keys": "42",
  "value": "white",
  "logo_color": "36",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "kali-dragon",
        r##"{
  "description": "Kali Linux: security dragon deep cobalt & obsidian cyan",
  "distro": "kali",
  "logo": { "source": "kali", "padding": { "top": 1, "right": 4 } },
  "separator": " \u00bb ",
  "title": "33",
  "keys": "69",
  "value": "white",
  "logo_color": "33",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "endeavour-space",
        r##"{
  "description": "EndeavourOS: interstellar violet, magenta & cosmic orange",
  "distro": "endeavouros",
  "logo": { "source": "endeavouros", "padding": { "top": 1, "right": 4 } },
  "separator": " \u203a ",
  "title": "135",
  "keys": "171",
  "value": "white",
  "logo_color": "135",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "wm",       "key": "wm      " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "redhat-shadow",
        r##"{
  "description": "Red Hat Enterprise Linux: corporate crimson shadow",
  "distro": "redhat",
  "logo": { "source": "redhat", "padding": { "top": 1, "right": 4 } },
  "separator": " \u25b8 ",
  "title": "196",
  "keys": "203",
  "value": "white",
  "logo_color": "196",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "catppuccin-mocha",
        r##"{
  "description": "Catppuccin Mocha: cozy pastel palette (lavender, blue, pink, mauve)",
  "logo": { "art": "cat", "padding": { "top": 1, "right": 5 } },
  "separator": " \u2022 ",
  "title": "#cdd6f4",
  "keys": "#b4befe",
  "value": "#cdd6f4",
  "logo_color": "#f5c2e7",
  "modules": [
    "break",
    { "type": "custom", "format": "\u001b[38;2;243;139;168m\u25cf  \u001b[38;2;250;179;135m\u25cf  \u001b[38;2;249;226;175m\u25cf  \u001b[38;2;166;227;161m\u25cf  \u001b[38;2;137;220;235m\u25cf  \u001b[38;2;137;180;250m\u25cf  \u001b[38;2;180;190;254m\u25cf" },
    "title",
    "break",
    { "type": "os",       "key": "os      ", "keyColor": "#f38ba8" },
    { "type": "kernel",   "key": "kernel  ", "keyColor": "#fab387" },
    { "type": "uptime",   "key": "uptime  ", "keyColor": "#f9e2af" },
    { "type": "packages", "key": "pkgs    ", "keyColor": "#a6e3a1" },
    { "type": "shell",    "key": "shell   ", "keyColor": "#89dceb" },
    { "type": "terminal", "key": "terminal", "keyColor": "#89b4fa" },
    { "type": "cpu",      "key": "cpu     ", "keyColor": "#b4befe" },
    { "type": "memory",   "key": "memory  ", "keyColor": "#cba6f7" },
    "break"
  ]
}"##,
    ),
    (
        "catppuccin-macchiato",
        r##"{
  "description": "Catppuccin Macchiato: warm dark pastel palette",
  "logo": { "source": "auto", "padding": { "top": 1, "right": 4 } },
  "separator": " \u203a ",
  "title": "#8aadf4",
  "keys": "#eed49f",
  "value": "#cad3f5",
  "logo_color": "#f5bde6",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "pkgs    " },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "solarized-dark",
        r##"{
  "description": "Solarized Dark: precision color scheme by Ethan Schoonover",
  "logo": { "source": "auto", "padding": { "top": 1, "right": 4 } },
  "separator": " : ",
  "title": "#268bd2",
  "keys": "#b58900",
  "value": "#839496",
  "logo_color": "#2aa198",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    { "type": "disk",     "key": "disk    " },
    "break"
  ]
}"##,
    ),
    (
        "synthwave-84",
        r##"{
  "description": "Synthwave '84: vibrant neon sunset (hot pink, electric cyan, yellow)",
  "logo": { "art": "blocks", "padding": { "top": 1, "right": 4 } },
  "separator": " \u203a ",
  "title": "#ff7edb",
  "keys": "#36f9f6",
  "value": "#fede5d",
  "logo_color": "#fe4450",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "gpu",      "key": "gpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "monokai-pro",
        r##"{
  "description": "Monokai Pro: modern filtered palette (spectrum green, magenta, amber)",
  "logo": { "source": "auto", "padding": { "top": 1, "right": 4 } },
  "separator": " \u00bb ",
  "title": "#ff6188",
  "keys": "#a9dc76",
  "value": "#fcfcfa",
  "logo_color": "#ffd866",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "one-dark",
        r##"{
  "description": "Atom One Dark: iconic balanced developer palette",
  "logo": { "source": "auto", "padding": { "top": 1, "right": 4 } },
  "separator": " \u276f ",
  "title": "#61afef",
  "keys": "#98c379",
  "value": "#abb2bf",
  "logo_color": "#e06c75",
  "modules": [
    "break",
    "title",
    "separator",
    { "type": "os",       "key": "os      " },
    { "type": "kernel",   "key": "kernel  " },
    { "type": "uptime",   "key": "uptime  " },
    { "type": "packages", "key": "packages" },
    { "type": "shell",    "key": "shell   " },
    { "type": "terminal", "key": "terminal" },
    { "type": "cpu",      "key": "cpu     " },
    { "type": "memory",   "key": "memory  " },
    "break"
  ]
}"##,
    ),
    (
        "ferris-crab",
        r##"{
  "description": "Rustacean pride: cute Ferris crab ASCII art & fiery rust colors",
  "logo": { "art": "ferris", "padding": { "top": 1, "right": 4 } },
  "separator": " \u25b8 ",
  "title": "208",
  "keys": "214",
  "value": "white",
  "logo_color": "196",
  "modules": [
    "break",
    "title",
    { "type": "os",       "key": "os    " },
    { "type": "kernel",   "key": "kernel" },
    { "type": "uptime",   "key": "uptime" },
    { "type": "packages", "key": "pkgs  " },
    { "type": "shell",    "key": "shell " },
    { "type": "terminal", "key": "term  " },
    { "type": "cpu",      "key": "cpu   " },
    { "type": "memory",   "key": "memory" },
    "break"
  ]
}"##,
    ),
    (
        "tux-penguin",
        r##"{
  "description": "Linux mascot Tux with clean monochrome & gold terminal layout",
  "logo": { "art": "tux", "padding": { "top": 1, "right": 5 } },
  "separator": " \u25b8 ",
  "title": "yellow",
  "keys": "cyan",
  "value": "white",
  "logo_color": "white",
  "modules": [
    "break",
    "title",
    { "type": "os",       "key": "os    " },
    { "type": "kernel",   "key": "kernel" },
    { "type": "uptime",   "key": "uptime" },
    { "type": "packages", "key": "pkgs  " },
    { "type": "shell",    "key": "shell " },
    { "type": "terminal", "key": "term  " },
    { "type": "cpu",      "key": "cpu   " },
    { "type": "memory",   "key": "memory" },
    "break"
  ]
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
