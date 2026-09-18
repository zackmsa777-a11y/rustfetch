//! ASCII-art banner used by the `--setup` TUI.
//!
//! Artwork generated with figlet ("ANSI Shadow", "Small", "Calvin S" fonts).
//! Three widths let the setup screen keep a proper wordmark on narrow
//! terminals instead of dropping it entirely.

/// ANSI Shadow wordmark, 75 columns x 6 rows (80-column terminals and wider).
pub const BANNER_FULL: &[&str] = &[
    "██████╗ ██╗   ██╗███████╗████████╗███████╗███████╗████████╗ ██████╗██╗  ██╗",
    "██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔════╝██╔════╝╚══██╔══╝██╔════╝██║  ██║",
    "██████╔╝██║   ██║███████╗   ██║   █████╗  █████╗     ██║   ██║     ███████║",
    "██╔══██╗██║   ██║╚════██║   ██║   ██╔══╝  ██╔══╝     ██║   ██║     ██╔══██║",
    "██║  ██║╚██████╔╝███████║   ██║   ██║     ███████╗   ██║   ╚██████╗██║  ██║",
    "╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝   ╚═╝     ╚══════╝   ╚═╝    ╚═════╝╚═╝  ╚═╝",
];

/// Small wordmark, 44 columns x 4 rows.
pub const BANNER_MEDIUM: &[&str] = &[
    " ___ _   _ ___ _____ ___ ___ _____ ___ _  _",
    "| _ \\ | | / __|_   _| __| __|_   _/ __| || |",
    "|   / |_| \\__ \\ | | | _|| _|  | || (__| __ |",
    "|_|_\\\\___/|___/ |_| |_| |___| |_| \\___|_||_|",
];

/// Calvin S wordmark, 27 columns x 3 rows.
pub const BANNER_COMPACT: &[&str] = &[
    "╦═╗╦ ╦╔═╗╔╦╗╔═╗╔═╗╔╦╗╔═╗╦ ╦",
    "╠╦╝║ ║╚═╗ ║ ╠╣ ║╣  ║ ║  ╠═╣",
    "╩╚═╚═╝╚═╝ ╩ ╚  ╚═╝ ╩ ╚═╝╩ ╩",
];

/// Width of [`BANNER_FULL`] in columns.
pub const BANNER_FULL_WIDTH: usize = 75;

/// Width of [`BANNER_MEDIUM`] in columns.
pub const BANNER_MEDIUM_WIDTH: usize = 44;

/// Width of [`BANNER_COMPACT`] in columns.
pub const BANNER_COMPACT_WIDTH: usize = 27;

pub const TAGLINE: &str = "system information, beautifully";

/// 256-colour ramp used to tint the wordmark line by line (rust -> ember).
pub const BANNER_RAMP: &[u8] = &[166, 202, 208, 214, 220, 227];

/// Pick the widest wordmark that fits `cols` (with one column of breathing
/// room on each side).
pub fn for_width(cols: usize) -> Option<&'static [&'static str]> {
    if cols >= BANNER_FULL_WIDTH + 2 {
        Some(BANNER_FULL)
    } else if cols >= BANNER_MEDIUM_WIDTH + 2 {
        Some(BANNER_MEDIUM)
    } else if cols >= BANNER_COMPACT_WIDTH + 2 {
        Some(BANNER_COMPACT)
    } else {
        None
    }
}

/// Wrap one art line in its ramp colour. Returns the line untouched when
/// `plain` is set or the line is blank.
pub fn colorize(line: &str, index: usize, plain: bool) -> String {
    if plain || line.trim().is_empty() {
        return line.to_string();
    }
    let c = BANNER_RAMP[index % BANNER_RAMP.len()];
    format!("\x1b[38;5;{c}m{line}\x1b[0m")
}
