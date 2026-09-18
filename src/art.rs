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

/// Ferris the Rust mascot crab.
pub const FERRIS: &[&str] = &[
    r#"      _~^~^~_      "#,
    r#"  \) /  o o  \ (/  "#,
    r#"    '_   -   _'    "#,
    r#"    / '-----' \    "#,
];

/// Classic Linux mascot Tux penguin.
pub const TUX: &[&str] = &[
    r#"   .--.   "#,
    r#"  |o_o |  "#,
    r#"  |:_/ |  "#,
    r#" //   \ \ "#,
    r#"(|     | )"#,
    r#"/'\_   _/`\"#,
    r#"\___)=(___/"#,
];

/// Compact Ubuntu circle-of-friends emblem.
pub const UBUNTU_MINI: &[&str] = &[
    r#"         .-.         "#,
    r#"    .-'   |   '-.    "#,
    r#"  .'   .-'-.   '.  "#,
    r#" /    (  o  )    \ "#,
    r#" :   . '-.-' .   : "#,
    r#"  '.  '-. | .-'  .'  "#,
    r#"    '-.   |   .-'    "#,
    r#"         '-'         "#,
];

/// Steaming hot cup of coffee.
pub const COFFEE: &[&str] = &[
    r#"      )  (      "#,
    r#"     (   ) )    "#,
    r#"      ) ( (     "#,
    r#"    _______)_   "#,
    r#" .-'---------|  "#,
    r#"( C|/\/\/\/\/|  "#,
    r#" '-._________|  "#,
    r#"   '--------'   "#,
];

/// Classic ASCII heart.
pub const HEART: &[&str] = &[
    r#"   .-''''-.     .-''''-.   "#,
    r#"  /        \   /        \  "#,
    r#" /          \ /          \ "#,
    r#" |           '           | "#,
    r#"  \                     /  "#,
    r#"   \                   /   "#,
    r#"    \                 /    "#,
    r#"     '.             .'     "#,
    r#"       '.         .'       "#,
    r#"         '..   ..'         "#,
    r#"            '.'            "#,
];

/// Retro arcade ghost.
pub const GHOST: &[&str] = &[
    r#"     .---.     "#,
    r#"    /     \    "#,
    r#"   | () () |   "#,
    r#"   |  ___  |   "#,
    r#"   |       |   "#,
    r#"   /\_/\_/\/\  "#,
];

/// Mini Arch Linux peak.
pub const ARCH_MINI: &[&str] = &[
    r#"      /\      "#,
    r#"     /  \     "#,
    r#"    /\   \    "#,
    r#"   /      \   "#,
    r#"  /   ,,   \  "#,
    r#" /   |  |  -\ "#,
    r#"/_-''    ''-_/"#,
];

/// Names accepted by `"logo": { "art": "<name>" }`.
#[allow(dead_code)]
pub const ART_NAMES: &[&str] = &[
    "cat",
    "rose",
    "triangle",
    "arch",
    "blocks",
    "prompt",
    "ferris",
    "tux",
    "ubuntu-mini",
    "coffee",
    "heart",
    "ghost",
    "arch-mini",
];

pub fn art(name: &str) -> Option<&'static [&'static str]> {
    let clean = name.to_lowercase().replace(['-', '_'], "");
    match clean.as_str() {
        "cat" => Some(CAT),
        "rose" => Some(ROSE),
        "triangle" => Some(TRIANGLE),
        "arch" => Some(ARCH),
        "blocks" => Some(BLOCKS),
        "prompt" => Some(PROMPT),
        "ferris" | "crab" | "rust" => Some(FERRIS),
        "tux" | "penguin" => Some(TUX),
        "ubuntumini" => Some(UBUNTU_MINI),
        "coffee" | "cup" => Some(COFFEE),
        "heart" | "love" => Some(HEART),
        "ghost" | "pacman" => Some(GHOST),
        "archmini" => Some(ARCH_MINI),
        _ => None,
    }
}
