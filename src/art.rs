//! Extra ASCII art shipped for layout themes.
//!
//! Distro logos live in [`crate::logos`]; these are the *decorative* pieces
//! layout themes reference with `"logo": { "art": "cat" }` (or
//! `"logo": "art:cat"`). The first four are the shapes used by the reference
//! fastfetch preset collection (`ascii/*.txt`), so a rustfetch theme can look
//! like those presets without any external file.

pub const CAT: &[&str] = &[
    "      /\\_____/\\",
    "      )     (",
    "     =\\     /=",
    "       )   (",
    "      /     \\",
    "      )     (",
    "     /       \\",
    "     \\       /",
    "      \\__ __/",
    "         ))",
    "        //",
    "       ((",
    "        \\)",
];

pub const ROSE: &[&str] = &[
    "        _,--._.-,",
    "       /\\_/-,\\_ )",
    "    .-.) _;=='_/ (.;",
    "     \\ \\'      \\/= )",
    "      -\\.'-. __.'|-'",
    "     <_`-'\\'__.'/",
    "       `'-._( \\",
    "        ___   \\\\\\,      ___",
    "        \\ .'-. \\\\\\   .-'_. /",
    "         '._' '.\\\\\\/.-'_.'",
    "            '--``\\\\('--'",
    "                  \\\\\\",
    "                  `\\\\\\",
    "                    \\\\|",
];

pub const TRIANGLE: &[&str] = &[
    "            ___",
    "           /\\  \\",
    "          /  \\  \\",
    "         /    \\  \\",
    "        /  /\\  \\  \\",
    "       /  /  \\  \\  \\",
    "      /  /  / \\  \\  \\",
    "     /  /  /   \\  \\  \\",
    "    /  /  /     \\  \\  \\",
    "   /  /  /       \\  \\  \\",
    "  /  /  /_________\\__\\  \\",
    " /  /  /_________________\\",
    " \\ /_____________________/",
];

pub const ARCH: &[&str] = &[
    "           .",
    "          / \\",
    "         /   \\",
    "        /\\    \\",
    "       /       \\",
    "      /         \\",
    "     /    .-.    \\",
    "    /    |   |   _\\",
    "   /   _.'   '._   \\",
    "  /_.-'         '-._\\",
];

/// A soft block gradient, good for colour-forward themes.
pub const BLOCKS: &[&str] = &[
    "   ░░░░░░░░░░░░  ",
    "  ░▓▓▓▓▓▓▓▓▓▓▓▓░ ",
    " ░▓▓▒▒▒▒▒▒▒▒▒▒▓▓░",
    "░▓▓▒▒░░░░░░░░▒▒▓▓",
    "░▓▓▒▒░░░░░░░░▒▒▓▓",
    " ░▓▓▒▒▒▒▒▒▒▒▒▒▓▓░",
    "  ░▓▓▓▓▓▓▓▓▓▓▓▓░ ",
    "   ░░░░░░░░░░░░  ",
];

/// A minimal terminal-ish glyph cluster.
pub const PROMPT: &[&str] = &[
    "  ╭───────────╮",
    "  │ ▸ rust   │",
    "  │   fetch  │",
    "  ╰───────────╯",
    "      ▔▔▔▔▔",
    "   ▸▸▸▸▸▸▸▸▸▸",
];

/// Names accepted by `"logo": { "art": "<name>" }`.
#[allow(dead_code)]
pub const ART_NAMES: &[&str] = &["cat", "rose", "triangle", "arch", "blocks", "prompt"];

pub fn art(name: &str) -> Option<&'static [&'static str]> {
    match name.to_lowercase().as_str() {
        "cat" => Some(CAT),
        "rose" => Some(ROSE),
        "triangle" => Some(TRIANGLE),
        "arch" => Some(ARCH),
        "blocks" => Some(BLOCKS),
        "prompt" => Some(PROMPT),
        _ => None,
    }
}
