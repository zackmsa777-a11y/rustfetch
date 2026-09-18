use crate::banner;
use crate::config::{self, Config, ThemeDef, ThemeEntry, ThemeSource, parse_color};
use crate::info::types::SystemInfo;
use crate::printer::{render_lines, style_from_theme};
use crate::utils::visible_width;
use std::io::{self, Write};
use std::os::unix::io::AsRawFd;
use std::path::Path;

/// Short summary description for a theme.
pub fn describe(name: &str, theme: &ThemeDef) -> String {
    if let Some(ref desc) = theme.description {
        format!("{name} - {desc}")
    } else {
        let summary = theme.summary();
        if summary.is_empty() {
            format!("{name} (default)")
        } else {
            format!("{name} ({summary})")
        }
    }
}

/// Print a live preview of every available theme to stdout.
pub fn preview_all(
    info: &SystemInfo,
    cfg: &Config,
    no_color: bool,
    no_logo: bool,
    logo_override: Option<&str>,
) {
    let themes = config::all_themes_for_distro(cfg, Some(&info.distro_id));
    let detected_hdr = if !info.distro_name.is_empty() {
        format!(" (detected: {})", info.distro_name)
    } else {
        String::new()
    };
    println!(
        "\x1b[1;36m=== rustfetch theme gallery ({} available in organized sections{}) ===\x1b[0m\n",
        themes.len(),
        detected_hdr
    );

    let items = build_tui_items(&themes, &info.distro_id, TabFilter::All);
    for item in &items {
        match item {
            TuiItem::SectionHeader {
                title,
                count,
                category,
            } => {
                let cat_col = category.color_code();
                println!(
                    "\n{cat_col}═══════════════════════ {title} ({count}) ═══════════════════════\x1b[0m\n"
                );
            }
            TuiItem::Theme { theme_idx } => {
                let entry = &themes[*theme_idx];
                let tag = if entry.matches_distro(&info.distro_id) {
                    "rec"
                } else {
                    entry.source.tag()
                };
                let layout_tag = if entry.def.has_layout() {
                    " [layout]"
                } else {
                    ""
                };
                println!(
                    "\x1b[1;33m--- Theme: {} [{tag}]{layout_tag} ---\x1b[0m",
                    entry.name
                );
                if let Some(ref desc) = entry.def.description {
                    println!("    \x1b[90m{desc}\x1b[0m");
                }

                let mut style = style_from_theme(&entry.def);
                if no_color {
                    style.no_color = true;
                }
                if no_logo {
                    style.no_logo = true;
                }
                if let Some(logo) = logo_override {
                    style.logo = Some(logo.to_string());
                }

                for line in render_lines(info, &style) {
                    println!("  {line}");
                }
                println!();
            }
        }
    }

    println!(
        "\x1b[1;32mUse `rustfetch --setup` to choose interactively with live full-screen preview & sections,\x1b[0m"
    );
    println!("or `rustfetch --theme <NAME>` to try one once.");
}

// ---------------- Terminal Helpers & Raw Mode ----------------

#[cfg(unix)]
pub fn terminal_dimensions() -> (usize, usize) {
    let mut ws: libc::winsize = unsafe { std::mem::zeroed() };
    let res = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &mut ws) };
    if res == 0 && ws.ws_col > 0 && ws.ws_row > 0 {
        (ws.ws_col as usize, ws.ws_row as usize)
    } else {
        (80, 24)
    }
}

#[cfg(not(unix))]
pub fn terminal_dimensions() -> (usize, usize) {
    (80, 24)
}

fn stdin_is_tty() -> bool {
    unsafe { libc::isatty(io::stdin().as_raw_fd()) != 0 }
}

#[cfg(unix)]
struct TerminalGuard {
    fd: i32,
    orig: libc::termios,
}

#[cfg(unix)]
impl TerminalGuard {
    fn enter() -> Result<Self, String> {
        let fd = io::stdin().as_raw_fd();
        unsafe {
            let mut orig: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(fd, &mut orig) != 0 {
                return Err("tcgetattr failed".into());
            }
            let mut raw = orig;
            // Disable canonical mode, echo, and ISIG (Ctrl-C received as byte 0x03)
            raw.c_lflag &= !(libc::ICANON | libc::ECHO | libc::ISIG);
            raw.c_cc[libc::VMIN] = 1;
            raw.c_cc[libc::VTIME] = 0;
            if libc::tcsetattr(fd, libc::TCSADRAIN, &raw) != 0 {
                return Err("tcsetattr failed".into());
            }

            let mut out = io::stdout();
            // Alternate screen buffer, hide cursor, enable mouse click tracking (normal + SGR mode), clear screen
            let _ = write!(
                out,
                "\x1b[?1049h\x1b[?25l\x1b[?1000h\x1b[?1006h\x1b[2J\x1b[H"
            );
            let _ = out.flush();

            Ok(Self { fd, orig })
        }
    }
}

#[cfg(unix)]
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut out = io::stdout();
        // Disable mouse tracking, restore cursor, leave alternate screen buffer
        let _ = write!(out, "\x1b[?1006l\x1b[?1000l\x1b[?25h\x1b[?1049l");
        let _ = out.flush();
        unsafe {
            libc::tcsetattr(self.fd, libc::TCSADRAIN, &self.orig);
        }
    }
}

enum Key {
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Enter,
    Export,
    Quit,
    Tab,
    BackTab,
    Digit(usize),
    ScrollUp,
    ScrollDown,
    MouseClick { x: usize, y: usize },
    Other,
}

#[cfg(unix)]
struct RawInput {
    fd: i32,
}

#[cfg(unix)]
impl RawInput {
    fn new(fd: i32) -> Self {
        Self { fd }
    }

    /// Read a single byte directly from the file descriptor.
    fn read_byte(&self) -> Option<u8> {
        let mut b = 0u8;
        let n = unsafe { libc::read(self.fd, &mut b as *mut u8 as *mut libc::c_void, 1) };
        if n == 1 { Some(b) } else { None }
    }

    /// Poll if `fd` is readable within `timeout_ms`.
    fn poll_readable(&self, timeout_ms: i32) -> bool {
        let mut pfd = libc::pollfd {
            fd: self.fd,
            events: libc::POLLIN,
            revents: 0,
        };
        let ret = unsafe { libc::poll(&mut pfd, 1, timeout_ms) };
        ret > 0 && (pfd.revents & libc::POLLIN) != 0
    }

    /// Read next byte if available within `timeout_ms`.
    fn read_byte_timeout(&self, timeout_ms: i32) -> Option<u8> {
        if self.poll_readable(timeout_ms) {
            self.read_byte()
        } else {
            None
        }
    }

    /// Parse next user key / mouse event.
    fn next_key(&self) -> Key {
        let b = match self.read_byte() {
            Some(b) => b,
            None => return Key::Other,
        };

        match b {
            b'\r' | b'\n' => Key::Enter,
            b'q' | b'Q' | 0x03 | 0x04 => Key::Quit, // q, Q, Ctrl-C, Ctrl-D
            b'e' | b'E' => Key::Export,
            b'k' | b'K' => Key::Up,
            b'j' | b'J' => Key::Down,
            b'g' => Key::Home,
            b'G' => Key::End,
            b'\t' => Key::Tab,
            d @ b'1'..=b'7' => Key::Digit((d - b'0') as usize),
            0x1b => {
                // If no following byte arrives within 60ms, it is a standalone Escape key!
                let b2 = match self.read_byte_timeout(60) {
                    Some(b2) => b2,
                    None => return Key::Quit,
                };

                if b2 == b'[' {
                    self.parse_csi()
                } else if b2 == b'O' {
                    // SS3 sequence used by some terminals for cursor / keypad
                    let b3 = match self.read_byte_timeout(50) {
                        Some(b3) => b3,
                        None => return Key::Other,
                    };
                    match b3 {
                        b'A' => Key::Up,
                        b'B' => Key::Down,
                        b'H' => Key::Home,
                        b'F' => Key::End,
                        _ => Key::Other,
                    }
                } else {
                    Key::Other
                }
            }
            _ => Key::Other,
        }
    }

    fn parse_csi(&self) -> Key {
        let first = match self.read_byte_timeout(50) {
            Some(f) => f,
            None => return Key::Other,
        };

        match first {
            b'A' => Key::Up,
            b'B' => Key::Down,
            b'H' => Key::Home,
            b'F' => Key::End,
            b'Z' => Key::BackTab,
            b'<' => self.parse_sgr_mouse(),
            b'M' => self.parse_x10_mouse(),
            d @ b'0'..=b'9' => {
                // Numbered sequence: e.g. 5~ (PageUp), 6~ (PageDown), 1~ (Home), 4~ (End)
                let mut num = (d - b'0') as usize;
                loop {
                    match self.read_byte_timeout(50) {
                        Some(b'~') => break,
                        Some(next_d @ b'0'..=b'9') => {
                            num = num * 10 + (next_d - b'0') as usize;
                        }
                        Some(_) | None => return Key::Other,
                    }
                }
                match num {
                    5 => Key::PageUp,
                    6 => Key::PageDown,
                    1 | 7 => Key::Home,
                    4 | 8 => Key::End,
                    _ => Key::Other,
                }
            }
            _ => Key::Other,
        }
    }

    fn parse_sgr_mouse(&self) -> Key {
        // SGR format: <btn;x;y(M|m)
        let mut s = String::new();
        let mut final_char = 'M';
        for _ in 0..40 {
            let ch = match self.read_byte_timeout(50) {
                Some(ch) => ch,
                None => return Key::Other,
            };
            if ch == b'M' || ch == b'm' {
                final_char = ch as char;
                break;
            }
            s.push(ch as char);
        }
        let parts: Vec<&str> = s.split(';').collect();
        if parts.len() == 3 {
            let btn: u32 = parts[0].parse().unwrap_or(0);
            let x: usize = parts[1].parse().unwrap_or(1);
            let y: usize = parts[2].parse().unwrap_or(1);
            if btn == 64 {
                return Key::ScrollUp;
            } else if btn == 65 {
                return Key::ScrollDown;
            } else if final_char == 'M' && (btn & 3) == 0 && (btn & 32) == 0 {
                return Key::MouseClick { x, y };
            }
        }
        Key::Other
    }

    fn parse_x10_mouse(&self) -> Key {
        let cb = match self.read_byte_timeout(50) {
            Some(b) => b.saturating_sub(32),
            None => return Key::Other,
        };
        let cx = match self.read_byte_timeout(50) {
            Some(b) => (b.saturating_sub(32)) as usize,
            None => return Key::Other,
        };
        let cy = match self.read_byte_timeout(50) {
            Some(b) => (b.saturating_sub(32)) as usize,
            None => return Key::Other,
        };
        if cb == 64 {
            Key::ScrollUp
        } else if cb == 65 {
            Key::ScrollDown
        } else if (cb & 3) == 0 && (cb & 32) == 0 {
            Key::MouseClick { x: cx, y: cy }
        } else {
            Key::Other
        }
    }
}

// ---------------- Full Screen TUI Renderer ----------------

fn truncate_visible(s: &str, max_width: usize) -> String {
    if visible_width(s) <= max_width {
        return s.to_string();
    }
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut vis = 0;

    while i < len {
        if chars[i] == '\x1b' {
            if i + 1 < len && (chars[i + 1] == '_' || chars[i + 1] == ']' || chars[i + 1] == 'P') {
                out.push(chars[i]);
                out.push(chars[i + 1]);
                i += 2;
                while i < len {
                    out.push(chars[i]);
                    if chars[i] == '\x07' {
                        i += 1;
                        break;
                    }
                    if chars[i] == '\x1b' && i + 1 < len && chars[i + 1] == '\\' {
                        out.push(chars[i + 1]);
                        i += 2;
                        break;
                    }
                    i += 1;
                }
            } else if i + 1 < len && chars[i + 1] == '[' {
                out.push(chars[i]);
                out.push(chars[i + 1]);
                i += 2;
                while i < len {
                    let c = chars[i];
                    out.push(c);
                    i += 1;
                    if (c as u32) >= 0x40 && (c as u32) <= 0x7E {
                        break;
                    }
                }
            } else {
                out.push(chars[i]);
                if i + 1 < len {
                    out.push(chars[i + 1]);
                    i += 2;
                } else {
                    i += 1;
                }
            }
        } else {
            if vis >= max_width {
                break;
            }
            out.push(chars[i]);
            vis += 1;
            i += 1;
        }
    }
    out.push_str("\x1b[0m");
    out
}

fn pad_right_visible(s: &str, target_width: usize) -> String {
    let w = visible_width(s);
    if w >= target_width {
        truncate_visible(s, target_width)
    } else {
        format!("{s}{}", " ".repeat(target_width - w))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeCategory {
    Recommended,
    Kitty,
    Layouts,
    Distros,
    Palettes,
    Mascots,
    Custom,
}

impl ThemeCategory {
    pub fn title(&self) -> &'static str {
        match self {
            ThemeCategory::Recommended => "Recommended for your OS",
            ThemeCategory::Kitty => "Kitty & Image Logos",
            ThemeCategory::Layouts => "Modern Layouts",
            ThemeCategory::Distros => "Distro Rices",
            ThemeCategory::Palettes => "Color Palettes",
            ThemeCategory::Mascots => "Mascot Art",
            ThemeCategory::Custom => "Custom Themes",
        }
    }

    pub fn color_code(&self) -> &'static str {
        match self {
            ThemeCategory::Recommended => "\x1b[1;32m",
            ThemeCategory::Kitty => "\x1b[1;36m",
            ThemeCategory::Layouts => "\x1b[1;35m",
            ThemeCategory::Distros => "\x1b[1;33m",
            ThemeCategory::Palettes => "\x1b[1;34m",
            ThemeCategory::Mascots => "\x1b[1;31m",
            ThemeCategory::Custom => "\x1b[1;37m",
        }
    }

    pub fn tab_label(&self) -> &'static str {
        match self {
            ThemeCategory::Recommended => "Recommended",
            ThemeCategory::Kitty => "Kitty/Images",
            ThemeCategory::Layouts => "Layouts",
            ThemeCategory::Distros => "Distros",
            ThemeCategory::Palettes => "Palettes",
            ThemeCategory::Mascots => "Mascot Art",
            ThemeCategory::Custom => "Custom",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabFilter {
    All,
    Category(ThemeCategory),
}

pub const TAB_ORDER: &[TabFilter] = &[
    TabFilter::All,
    TabFilter::Category(ThemeCategory::Recommended),
    TabFilter::Category(ThemeCategory::Kitty),
    TabFilter::Category(ThemeCategory::Layouts),
    TabFilter::Category(ThemeCategory::Distros),
    TabFilter::Category(ThemeCategory::Palettes),
    TabFilter::Category(ThemeCategory::Mascots),
];

#[derive(Debug, Clone)]
pub enum TuiItem {
    SectionHeader {
        title: String,
        category: ThemeCategory,
        count: usize,
    },
    Theme {
        theme_idx: usize,
    },
}

pub fn classify_theme(entry: &ThemeEntry) -> ThemeCategory {
    if matches!(entry.source, ThemeSource::File(_) | ThemeSource::Config) {
        return ThemeCategory::Custom;
    }
    let name = entry.name.as_str();
    if name.starts_with("kitty-")
        || entry.def.logo_type() == Some("kitty")
        || entry.def.logo_type() == Some("kitty-direct")
        || entry.def.logo_type() == Some("kitty-icat")
        || entry
            .def
            .logo_source()
            .map(|s| s.starts_with("kitty:"))
            .unwrap_or(false)
    {
        return ThemeCategory::Kitty;
    }
    if name == "ferris-crab" || name == "tux-penguin" {
        return ThemeCategory::Mascots;
    }
    if entry.def.distro.is_some()
        || name.starts_with("ubuntu-")
        || name.starts_with("arch-")
        || name.starts_with("debian-")
        || name.starts_with("fedora-")
        || name.starts_with("nixos-")
        || name.starts_with("bedrock-")
        || name.starts_with("gentoo-")
        || name.starts_with("cachyos-")
        || name.starts_with("mint-")
        || name.starts_with("opensuse-")
        || name.starts_with("popos-")
        || name.starts_with("void-")
        || name.starts_with("alpine-")
        || name.starts_with("manjaro-")
        || name.starts_with("kali-")
        || name.starts_with("endeavouros-")
        || name.starts_with("rhel-")
    {
        return ThemeCategory::Distros;
    }
    if name.starts_with("catppuccin-")
        || matches!(
            name,
            "tokyo-night"
                | "dracula"
                | "nord"
                | "gruvbox"
                | "everforest"
                | "solarized-dark"
                | "synthwave-84"
                | "monokai-pro"
                | "one-dark"
        )
    {
        return ThemeCategory::Palettes;
    }
    if entry.def.has_layout()
        || matches!(
            name,
            "default"
                | "minimal"
                | "os"
                | "groups"
                | "hypr"
                | "rice"
                | "cyber"
                | "retro"
                | "matrix"
                | "paper"
                | "card"
                | "neon"
                | "crt"
                | "dense"
                | "mute"
        )
    {
        return ThemeCategory::Layouts;
    }
    ThemeCategory::Palettes
}

pub fn build_tui_items(themes: &[ThemeEntry], distro_id: &str, tab: TabFilter) -> Vec<TuiItem> {
    let mut rec_indices = Vec::new();
    let mut kitty_indices = Vec::new();
    let mut layout_indices = Vec::new();
    let mut distro_indices = Vec::new();
    let mut palette_indices = Vec::new();
    let mut mascot_indices = Vec::new();
    let mut custom_indices = Vec::new();

    for (idx, t) in themes.iter().enumerate() {
        if t.matches_distro(distro_id) {
            rec_indices.push(idx);
        }
        match classify_theme(t) {
            ThemeCategory::Kitty => kitty_indices.push(idx),
            ThemeCategory::Layouts => layout_indices.push(idx),
            ThemeCategory::Distros => distro_indices.push(idx),
            ThemeCategory::Palettes => palette_indices.push(idx),
            ThemeCategory::Mascots => mascot_indices.push(idx),
            ThemeCategory::Custom => custom_indices.push(idx),
            ThemeCategory::Recommended => {}
        }
    }

    let mut items = Vec::new();
    match tab {
        TabFilter::All => {
            if !rec_indices.is_empty() {
                items.push(TuiItem::SectionHeader {
                    title: "Recommended for your OS".into(),
                    category: ThemeCategory::Recommended,
                    count: rec_indices.len(),
                });
                for idx in rec_indices {
                    items.push(TuiItem::Theme { theme_idx: idx });
                }
            }
            if !kitty_indices.is_empty() {
                items.push(TuiItem::SectionHeader {
                    title: "Kitty & Image Logos".into(),
                    category: ThemeCategory::Kitty,
                    count: kitty_indices.len(),
                });
                for idx in kitty_indices {
                    items.push(TuiItem::Theme { theme_idx: idx });
                }
            }
            if !layout_indices.is_empty() {
                items.push(TuiItem::SectionHeader {
                    title: "Modern Layouts".into(),
                    category: ThemeCategory::Layouts,
                    count: layout_indices.len(),
                });
                for idx in layout_indices {
                    items.push(TuiItem::Theme { theme_idx: idx });
                }
            }
            let non_rec_distros: Vec<usize> = distro_indices
                .into_iter()
                .filter(|idx| !themes[*idx].matches_distro(distro_id))
                .collect();
            if !non_rec_distros.is_empty() {
                items.push(TuiItem::SectionHeader {
                    title: "Distro Rices".into(),
                    category: ThemeCategory::Distros,
                    count: non_rec_distros.len(),
                });
                for idx in non_rec_distros {
                    items.push(TuiItem::Theme { theme_idx: idx });
                }
            }
            if !palette_indices.is_empty() {
                items.push(TuiItem::SectionHeader {
                    title: "Color Palettes".into(),
                    category: ThemeCategory::Palettes,
                    count: palette_indices.len(),
                });
                for idx in palette_indices {
                    items.push(TuiItem::Theme { theme_idx: idx });
                }
            }
            if !mascot_indices.is_empty() {
                items.push(TuiItem::SectionHeader {
                    title: "Mascots & Art".into(),
                    category: ThemeCategory::Mascots,
                    count: mascot_indices.len(),
                });
                for idx in mascot_indices {
                    items.push(TuiItem::Theme { theme_idx: idx });
                }
            }
            if !custom_indices.is_empty() {
                items.push(TuiItem::SectionHeader {
                    title: "Custom Themes".into(),
                    category: ThemeCategory::Custom,
                    count: custom_indices.len(),
                });
                for idx in custom_indices {
                    items.push(TuiItem::Theme { theme_idx: idx });
                }
            }
        }
        TabFilter::Category(ThemeCategory::Recommended) => {
            for idx in rec_indices {
                items.push(TuiItem::Theme { theme_idx: idx });
            }
        }
        TabFilter::Category(ThemeCategory::Kitty) => {
            for idx in kitty_indices {
                items.push(TuiItem::Theme { theme_idx: idx });
            }
        }
        TabFilter::Category(ThemeCategory::Layouts) => {
            for idx in layout_indices {
                items.push(TuiItem::Theme { theme_idx: idx });
            }
        }
        TabFilter::Category(ThemeCategory::Distros) => {
            for idx in distro_indices {
                items.push(TuiItem::Theme { theme_idx: idx });
            }
        }
        TabFilter::Category(ThemeCategory::Palettes) => {
            for idx in palette_indices {
                items.push(TuiItem::Theme { theme_idx: idx });
            }
        }
        TabFilter::Category(ThemeCategory::Mascots) => {
            for idx in mascot_indices {
                items.push(TuiItem::Theme { theme_idx: idx });
            }
        }
        TabFilter::Category(ThemeCategory::Custom) => {
            for idx in custom_indices {
                items.push(TuiItem::Theme { theme_idx: idx });
            }
        }
    }
    items
}

fn render_kitty_preview_mockup(
    info: &SystemInfo,
    style: &crate::printer::RenderStyle,
    img_path: &Path,
    cell_w: usize,
    cell_h: usize,
) -> Vec<String> {
    let module_lines = crate::printer::format_styled_module_lines(info, style);
    let path_display = img_path.file_name().unwrap_or_default().to_string_lossy();
    let dims_str = if let Some((pw, ph)) = crate::kitty::probe_image_dimensions(img_path) {
        format!("{pw}x{ph} px")
    } else {
        "Image".to_string()
    };

    let inner_w = cell_w.saturating_sub(4).max(18);
    let border_top = format!("┌{}┐", "─".repeat(inner_w + 2));
    let border_bot = format!("└{}┘", "─".repeat(inner_w + 2));

    let mut box_lines = Vec::new();
    box_lines.push(format!("\x1b[1;36m{border_top}\x1b[0m"));
    box_lines.push(format!(
        "\x1b[1;36m│\x1b[0m \x1b[1;37m{:<inner_w$}\x1b[0m \x1b[1;36m│\x1b[0m",
        "[KITTY GRAPHICS]"
    ));
    box_lines.push(format!(
        "\x1b[1;36m│\x1b[0m {:<inner_w$} \x1b[1;36m│\x1b[0m",
        ""
    ));
    box_lines.push(format!(
        "\x1b[1;36m│\x1b[0m \x1b[1;32m[IMG] {:<inner_w2$}\x1b[0m \x1b[1;36m│\x1b[0m",
        "Image Logo",
        inner_w2 = inner_w.saturating_sub(6)
    ));
    let file_label = if path_display.len() > inner_w {
        format!("{}...", &path_display[..inner_w.saturating_sub(3)])
    } else {
        path_display.to_string()
    };
    box_lines.push(format!(
        "\x1b[1;36m│\x1b[0m \x1b[33m{:<inner_w$}\x1b[0m \x1b[1;36m│\x1b[0m",
        file_label
    ));
    box_lines.push(format!(
        "\x1b[1;36m│\x1b[0m {:<inner_w$} \x1b[1;36m│\x1b[0m",
        ""
    ));
    box_lines.push(format!(
        "\x1b[1;36m│\x1b[0m \x1b[90mProtocol: Kitty (t=f)\x1b[0m{:<pad$} \x1b[1;36m│\x1b[0m",
        "",
        pad = inner_w.saturating_sub(21)
    ));
    box_lines.push(format!(
        "\x1b[1;36m│\x1b[0m \x1b[90mTarget  : {cell_w}x{cell_h} cells\x1b[0m{:<pad$} \x1b[1;36m│\x1b[0m",
        "",
        pad = inner_w.saturating_sub(18 + format!("{cell_w}x{cell_h}").len())
    ));
    box_lines.push(format!(
        "\x1b[1;36m│\x1b[0m \x1b[90mSource  : {dims_str}\x1b[0m{:<pad$} \x1b[1;36m│\x1b[0m",
        "",
        pad = inner_w.saturating_sub(10 + dims_str.len())
    ));
    box_lines.push(format!(
        "\x1b[1;36m│\x1b[0m {:<inner_w$} \x1b[1;36m│\x1b[0m",
        ""
    ));
    box_lines.push(format!("\x1b[1;36m{border_bot}\x1b[0m"));

    let gap = " ".repeat(style.padding_right.max(3));
    let left_pad = " ".repeat(style.padding_left);
    let total = box_lines.len().max(module_lines.len());
    let mut out = Vec::with_capacity(total);

    let max_box_w = inner_w + 4;
    for i in 0..total {
        let box_part = match box_lines.get(i) {
            Some(line) => {
                let pad = max_box_w.saturating_sub(visible_width(line));
                format!("{left_pad}{line}{}{gap}", " ".repeat(pad))
            }
            None => format!("{left_pad}{}", " ".repeat(max_box_w + gap.len())),
        };
        match module_lines.get(i) {
            Some(line) => out.push(format!("{box_part}{line}")),
            None => out.push(box_part.trim_end().to_string()),
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
fn render_tui(
    info: &SystemInfo,
    _cfg: &Config,
    themes: &[ThemeEntry],
    items: &[TuiItem],
    selected_item_idx: usize,
    active_tab: TabFilter,
    active_theme_name: Option<&str>,
    status_msg: Option<&str>,
    no_logo: bool,
    logo_override: Option<&str>,
) {
    let (cols, rows) = terminal_dimensions();
    let mut out = io::stdout();
    let mut screen = String::with_capacity(16384);

    screen.push_str("\x1b[H");

    let mut header_rows = 0;
    if let Some(art_lines) = banner::for_width(cols) {
        for (i, line) in art_lines.iter().enumerate() {
            let colorized = banner::colorize(line, i, false);
            let pad = (cols.saturating_sub(visible_width(line))) / 2;
            screen.push_str(&" ".repeat(pad));
            screen.push_str(&colorized);
            screen.push_str("\x1b[K\r\n");
            header_rows += 1;
        }
    }

    let detected_pc = if !info.distro_name.is_empty() {
        format!("PC: {}", info.distro_name)
    } else if !info.distro_id.is_empty() {
        format!("PC: {}", info.distro_id)
    } else {
        banner::TAGLINE.to_string()
    };
    let subtitle = format!("rustfetch v{} ─ {}", env!("CARGO_PKG_VERSION"), detected_pc);
    let sub_pad = (cols.saturating_sub(visible_width(&subtitle))) / 2;
    screen.push_str(&" ".repeat(sub_pad));
    screen.push_str("\x1b[1;33m");
    screen.push_str(&subtitle);
    screen.push_str("\x1b[0m\x1b[K\r\n");
    header_rows += 1;

    let divider = "─".repeat(cols);
    screen.push_str("\x1b[90m");
    screen.push_str(&divider);
    screen.push_str("\x1b[0m\x1b[K\r\n");
    header_rows += 1;

    let footer_rows = 3;
    let main_rows = rows.saturating_sub(header_rows + footer_rows).max(6);

    let selected_theme_idx = match items.get(selected_item_idx) {
        Some(TuiItem::Theme { theme_idx }) => *theme_idx,
        _ => items
            .iter()
            .find_map(|it| match it {
                TuiItem::Theme { theme_idx } => Some(*theme_idx),
                _ => None,
            })
            .unwrap_or(0),
    };
    let sel_entry = &themes[selected_theme_idx];
    let is_split = cols >= 88;

    if is_split {
        let left_width = 44.min(cols / 2 - 2).max(38);
        let right_width = cols.saturating_sub(left_width + 3);

        let is_kitty_theme = sel_entry.name.starts_with("kitty-")
            || sel_entry.def.logo_type() == Some("kitty")
            || sel_entry.def.logo_type() == Some("kitty-direct")
            || sel_entry
                .def
                .logo_source()
                .map(|s| s.starts_with("kitty:"))
                .unwrap_or(false);

        let preview_lines = if is_kitty_theme && !no_logo {
            let logo_src = sel_entry.def.logo_source().unwrap_or("kitty:example");
            let resolved_img = crate::kitty::resolve_image_path(logo_src)
                .unwrap_or_else(|| std::path::PathBuf::from("assets/example-kitty.png"));
            let style = style_from_theme(&sel_entry.def);
            let cell_w = sel_entry.def.width().unwrap_or(30);
            let cell_h = sel_entry.def.height().unwrap_or(12);
            render_kitty_preview_mockup(info, &style, &resolved_img, cell_w, cell_h)
        } else {
            let mut style = style_from_theme(&sel_entry.def);
            if no_logo {
                style.no_logo = true;
            }
            if let Some(logo) = logo_override {
                style.logo = Some(logo.to_string());
            }
            render_lines(info, &style)
        };

        // Row 0: Column header
        let cat_title = match active_tab {
            TabFilter::All => format!("All Themes ({})", themes.len()),
            TabFilter::Category(cat) => {
                let count = items
                    .iter()
                    .filter(|it| matches!(it, TuiItem::Theme { .. }))
                    .count();
                format!("{} ({count})", cat.title())
            }
        };
        let list_header = format!(
            "┌─ Section: \x1b[1;37m{}\x1b[1;36m ─{}",
            cat_title,
            "─".repeat(left_width.saturating_sub(visible_width(&cat_title) + 14))
        );
        let preview_title = format!(
            " Live Preview: {} [{}] ",
            sel_entry.name,
            sel_entry.source.tag()
        );
        let preview_header = format!(
            "┌─{}─{}",
            preview_title,
            "─".repeat(right_width.saturating_sub(visible_width(&preview_title) + 2))
        );

        screen.push_str("\x1b[1;36m");
        screen.push_str(&pad_right_visible(&list_header, left_width));
        screen.push_str(" │ ");
        screen.push_str(&pad_right_visible(&preview_header, right_width));
        screen.push_str("\x1b[0m\x1b[K\r\n");

        // Row 1: Pill bar on left, description/summary on right
        let pill_labels = [
            ("1:All", TabFilter::All),
            ("2:Rec", TabFilter::Category(ThemeCategory::Recommended)),
            ("3:Kitty", TabFilter::Category(ThemeCategory::Kitty)),
            ("4:Lay", TabFilter::Category(ThemeCategory::Layouts)),
            ("5:Dist", TabFilter::Category(ThemeCategory::Distros)),
            ("6:Pal", TabFilter::Category(ThemeCategory::Palettes)),
            ("7:Art", TabFilter::Category(ThemeCategory::Mascots)),
        ];
        let mut pill_str = String::from(" ");
        for (label, tab) in pill_labels {
            if active_tab == tab {
                pill_str.push_str(&format!("\x1b[1;30;46m {label} \x1b[0m "));
            } else {
                pill_str.push_str(&format!("\x1b[90m[{label}]\x1b[0m "));
            }
        }

        let desc_cell = if let Some(ref desc) = sel_entry.def.description {
            format!("\x1b[3m\x1b[90m// {desc}\x1b[0m")
        } else {
            format!("\x1b[90m// {}\x1b[0m", sel_entry.def.summary())
        };

        screen.push_str(&pad_right_visible(&pill_str, left_width));
        screen.push_str("\x1b[90m │ \x1b[0m");
        screen.push_str(&pad_right_visible(&desc_cell, right_width));
        screen.push_str("\x1b[K\r\n");

        // Scroll window for items list (rows 2..main_rows)
        let max_list_items = main_rows.saturating_sub(2);
        let scroll_offset = if selected_item_idx < max_list_items / 2 {
            0
        } else if selected_item_idx + max_list_items / 2 >= items.len() {
            items.len().saturating_sub(max_list_items)
        } else {
            selected_item_idx.saturating_sub(max_list_items / 2)
        };

        for row in 0..max_list_items {
            let item_idx = scroll_offset + row;
            let left_cell = if item_idx < items.len() {
                match &items[item_idx] {
                    TuiItem::SectionHeader {
                        title,
                        count,
                        category,
                    } => {
                        let col = category.color_code();
                        format!("{col}── {title} ({count}) ──\x1b[0m")
                    }
                    TuiItem::Theme { theme_idx } => {
                        let t = &themes[*theme_idx];
                        let is_sel = item_idx == selected_item_idx;
                        let is_active =
                            active_theme_name.is_some_and(|a| a.eq_ignore_ascii_case(&t.name));

                        let dot_col = t
                            .def
                            .keys
                            .as_deref()
                            .or(t.def.title.as_deref())
                            .and_then(parse_color)
                            .unwrap_or_else(|| "\x1b[37m".to_string());

                        let prefix = if is_sel { "\x1b[1;32m▸ " } else { "  " };
                        let dot = format!("{dot_col}●\x1b[0m");

                        let tag_str = if t.name.starts_with("kitty-") {
                            "\x1b[1;36mkitty\x1b[0m"
                        } else if t.matches_distro(&info.distro_id) {
                            "\x1b[1;32mrec\x1b[0m"
                        } else {
                            match t.source {
                                ThemeSource::Builtin => "\x1b[90mpreset\x1b[0m",
                                ThemeSource::File(_) => "\x1b[33mfile\x1b[0m",
                                ThemeSource::Config => "\x1b[35mcfg\x1b[0m",
                            }
                        };

                        let active_marker = if is_active {
                            "\x1b[1;32m✓\x1b[0m"
                        } else {
                            " "
                        };
                        let layout_flag = if t.def.has_layout() {
                            "\x1b[36m*\x1b[0m"
                        } else {
                            " "
                        };

                        let name_display = if is_sel {
                            format!("\x1b[1;37;44m {:<13}\x1b[0m", t.name)
                        } else {
                            format!("{:<14}", t.name)
                        };

                        format!(
                            "{prefix}{active_marker}{dot} {name_display} {tag_str}{layout_flag}"
                        )
                    }
                }
            } else {
                String::new()
            };

            let right_cell = preview_lines.get(row).cloned().unwrap_or_default();

            let left_formatted = pad_right_visible(&left_cell, left_width);
            let right_formatted = pad_right_visible(&right_cell, right_width);

            screen.push_str(&left_formatted);
            screen.push_str("\x1b[90m │ \x1b[0m");
            screen.push_str(&right_formatted);
            screen.push_str("\x1b[K\r\n");
        }
    } else {
        // Narrow Terminal (< 88 cols)
        let cat_title = match active_tab {
            TabFilter::All => format!("All ({})", themes.len()),
            TabFilter::Category(cat) => cat.tab_label().to_string(),
        };
        screen.push_str(&format!(
            "\x1b[1;36m── Category: {cat_title} [Tab/1-7] ─────────────────────\x1b[0m\x1b[K\r\n"
        ));

        let max_list = (main_rows / 2).max(4);
        let scroll_offset = if selected_item_idx < max_list / 2 {
            0
        } else if selected_item_idx + max_list / 2 >= items.len() {
            items.len().saturating_sub(max_list)
        } else {
            selected_item_idx.saturating_sub(max_list / 2)
        };

        for row in 0..max_list {
            let item_idx = scroll_offset + row;
            if item_idx < items.len() {
                match &items[item_idx] {
                    TuiItem::SectionHeader {
                        title,
                        count,
                        category,
                    } => {
                        let col = category.color_code();
                        screen.push_str(&format!("{col}── {title} ({count}) ──\x1b[0m\x1b[K\r\n"));
                    }
                    TuiItem::Theme { theme_idx } => {
                        let t = &themes[*theme_idx];
                        let is_sel = item_idx == selected_item_idx;
                        let dot_col = t
                            .def
                            .keys
                            .as_deref()
                            .and_then(parse_color)
                            .unwrap_or_default();
                        let prefix = if is_sel {
                            "\x1b[1;32m▸ \x1b[1;37m"
                        } else {
                            "  \x1b[0m"
                        };
                        let tag_str = if t.name.starts_with("kitty-") {
                            "\x1b[1;36mkitty\x1b[0m"
                        } else if t.matches_distro(&info.distro_id) {
                            "\x1b[1;32mrec\x1b[0m"
                        } else {
                            match t.source {
                                ThemeSource::Builtin => "\x1b[90mpreset\x1b[0m",
                                ThemeSource::File(_) => "\x1b[33mfile\x1b[0m",
                                ThemeSource::Config => "\x1b[35mcfg\x1b[0m",
                            }
                        };
                        screen.push_str(&format!(
                            "{prefix}{dot_col}●\x1b[0m {:<14} [{tag_str}]\x1b[0m\x1b[K\r\n",
                            t.name,
                        ));
                    }
                }
            }
        }

        screen.push_str("\x1b[1;36m── Preview ───────────────────────────────\x1b[0m\x1b[K\r\n");
        let mut style = style_from_theme(&sel_entry.def);
        if no_logo {
            style.no_logo = true;
        }
        let preview_lines = render_lines(info, &style);
        let rem_rows = main_rows.saturating_sub(max_list + 2);
        for i in 0..rem_rows {
            if let Some(line) = preview_lines.get(i) {
                screen.push_str(&truncate_visible(line, cols));
            }
            screen.push_str("\x1b[K\r\n");
        }
    }

    // Footer
    screen.push_str("\x1b[90m");
    screen.push_str(&"─".repeat(cols));
    screen.push_str("\x1b[0m\x1b[K\r\n");

    let keys_hint = "\x1b[1;37m[Tab/1-7]\x1b[0m Section  \x1b[1;37m[↑/↓]\x1b[0m Select  \x1b[1;32m[Enter]\x1b[0m Apply  \x1b[1;33m[e]\x1b[0m Export  \x1b[1;31m[q]\x1b[0m Quit";
    let active_name_str = active_theme_name.unwrap_or("default");
    let active_status = format!("Active: \x1b[1;32m{active_name_str}\x1b[0m");

    let spacing = cols.saturating_sub(visible_width(keys_hint) + visible_width(&active_status));
    screen.push_str(keys_hint);
    screen.push_str(&" ".repeat(spacing));
    screen.push_str(&active_status);
    screen.push_str("\x1b[K\r\n");

    if let Some(msg) = status_msg {
        screen.push_str(msg);
    } else if sel_entry.name.starts_with("kitty-") {
        screen.push_str("\x1b[1;36mTip: Kitty graphics image logo! High-res GPU rendering in Kitty, Ghostty & WezTerm [Enter to apply]\x1b[0m");
    } else if sel_entry.matches_distro(&info.distro_id) {
        screen.push_str("\x1b[1;32mTip: Recommended rice for your OS! Enter to apply as default, e to export custom JSONC\x1b[0m");
    } else {
        screen.push_str("\x1b[90mTip: Enter to apply, e to export to config.jsonc, Tab/1-7 to switch section, q to quit\x1b[0m");
    }
    screen.push_str("\x1b[K");

    let _ = write!(out, "{screen}");
    let _ = out.flush();
}

// ---------------- Numbered Non-Interactive Fallback ----------------

fn numbered_fallback(
    info: &SystemInfo,
    _cfg: &Config,
    themes: &[ThemeEntry],
    config_path: Option<&Path>,
) -> Result<(), String> {
    println!("\x1b[1;36mrustfetch themes (non-interactive mode):\x1b[0m\n");
    let items = build_tui_items(themes, &info.distro_id, TabFilter::All);
    for item in &items {
        match item {
            TuiItem::SectionHeader {
                title,
                count,
                category,
            } => {
                let cat_col = category.color_code();
                println!("\n{cat_col}── {title} ({count}) ──\x1b[0m");
            }
            TuiItem::Theme { theme_idx } => {
                let entry = &themes[*theme_idx];
                let tag = if entry.matches_distro(&info.distro_id) {
                    "rec"
                } else {
                    entry.source.tag()
                };
                let desc = entry.def.description.as_deref().unwrap_or("");
                println!(
                    "  \x1b[1;33m{:>2}.\x1b[0m {:<14} [{:<7}] {}",
                    *theme_idx + 1,
                    entry.name,
                    tag,
                    desc
                );
            }
        }
    }
    print!("\nEnter theme number to save as default (or 0 to quit): ");
    let _ = io::stdout().flush();
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .map_err(|e| e.to_string())?;
    let n: usize = line.trim().parse().unwrap_or(0);
    if n == 0 || n > themes.len() {
        println!("No changes saved.");
        return Ok(());
    }
    let chosen = &themes[n - 1].name;
    let saved = config::save_theme_name(config_path, chosen)?;
    println!(
        "\x1b[1;32m✓ Saved theme '{chosen}' to {}\x1b[0m",
        saved.display()
    );
    Ok(())
}

// ---------------- Main TUI Entry Point ----------------

pub fn run_setup(
    info: &SystemInfo,
    cfg: &Config,
    config_path: Option<&Path>,
    no_logo: bool,
    logo_override: Option<&str>,
) -> Result<(), String> {
    let themes = config::all_themes_for_distro(cfg, Some(&info.distro_id));
    if themes.is_empty() {
        return Err("no themes available".into());
    }

    if !stdin_is_tty() {
        return numbered_fallback(info, cfg, &themes, config_path);
    }

    #[cfg(not(unix))]
    {
        return numbered_fallback(info, cfg, &themes, config_path);
    }

    #[cfg(unix)]
    {
        let _guard = TerminalGuard::enter()?;
        let input = RawInput::new(_guard.fd);

        let active_name = config::active_theme_name(cfg);

        let mut active_tab_idx = 0usize;
        let mut active_tab = TAB_ORDER[active_tab_idx];
        let mut items = build_tui_items(&themes, &info.distro_id, active_tab);

        let mut selected_item_idx = active_name
            .as_deref()
            .and_then(|act| {
                items.iter().position(|it| match it {
                    TuiItem::Theme { theme_idx } => {
                        themes[*theme_idx].name.eq_ignore_ascii_case(act)
                    }
                    _ => false,
                })
            })
            .unwrap_or_else(|| {
                items
                    .iter()
                    .position(|it| matches!(it, TuiItem::Theme { .. }))
                    .unwrap_or(0)
            });

        let mut status_msg: Option<String> = None;

        loop {
            render_tui(
                info,
                cfg,
                &themes,
                &items,
                selected_item_idx,
                active_tab,
                active_name.as_deref(),
                status_msg.as_deref(),
                no_logo,
                logo_override,
            );

            match input.next_key() {
                Key::Up => {
                    let mut curr = selected_item_idx;
                    for _ in 0..items.len() {
                        curr = curr.checked_sub(1).unwrap_or(items.len().saturating_sub(1));
                        if matches!(items.get(curr), Some(TuiItem::Theme { .. })) {
                            selected_item_idx = curr;
                            break;
                        }
                    }
                    status_msg = None;
                }
                Key::Down => {
                    let mut curr = selected_item_idx;
                    for _ in 0..items.len() {
                        curr = (curr + 1) % items.len();
                        if matches!(items.get(curr), Some(TuiItem::Theme { .. })) {
                            selected_item_idx = curr;
                            break;
                        }
                    }
                    status_msg = None;
                }
                Key::PageUp => {
                    let mut curr = selected_item_idx;
                    for _ in 0..5 {
                        for _ in 0..items.len() {
                            curr = curr.checked_sub(1).unwrap_or(items.len().saturating_sub(1));
                            if matches!(items.get(curr), Some(TuiItem::Theme { .. })) {
                                break;
                            }
                        }
                    }
                    selected_item_idx = curr;
                    status_msg = None;
                }
                Key::PageDown => {
                    let mut curr = selected_item_idx;
                    for _ in 0..5 {
                        for _ in 0..items.len() {
                            curr = (curr + 1) % items.len();
                            if matches!(items.get(curr), Some(TuiItem::Theme { .. })) {
                                break;
                            }
                        }
                    }
                    selected_item_idx = curr;
                    status_msg = None;
                }
                Key::Home => {
                    if let Some(first_theme) = items
                        .iter()
                        .position(|it| matches!(it, TuiItem::Theme { .. }))
                    {
                        selected_item_idx = first_theme;
                    }
                    status_msg = None;
                }
                Key::End => {
                    if let Some(last_theme) = items
                        .iter()
                        .rposition(|it| matches!(it, TuiItem::Theme { .. }))
                    {
                        selected_item_idx = last_theme;
                    }
                    status_msg = None;
                }
                Key::ScrollUp => {
                    let mut curr = selected_item_idx;
                    for _ in 0..items.len() {
                        curr = curr.checked_sub(1).unwrap_or(items.len().saturating_sub(1));
                        if matches!(items.get(curr), Some(TuiItem::Theme { .. })) {
                            selected_item_idx = curr;
                            break;
                        }
                    }
                    status_msg = None;
                }
                Key::ScrollDown => {
                    let mut curr = selected_item_idx;
                    for _ in 0..items.len() {
                        curr = (curr + 1) % items.len();
                        if matches!(items.get(curr), Some(TuiItem::Theme { .. })) {
                            selected_item_idx = curr;
                            break;
                        }
                    }
                    status_msg = None;
                }
                Key::Tab => {
                    active_tab_idx = (active_tab_idx + 1) % TAB_ORDER.len();
                    active_tab = TAB_ORDER[active_tab_idx];
                    items = build_tui_items(&themes, &info.distro_id, active_tab);
                    selected_item_idx = items
                        .iter()
                        .position(|it| matches!(it, TuiItem::Theme { .. }))
                        .unwrap_or(0);
                    status_msg = None;
                }
                Key::BackTab => {
                    active_tab_idx = active_tab_idx
                        .checked_sub(1)
                        .unwrap_or(TAB_ORDER.len().saturating_sub(1));
                    active_tab = TAB_ORDER[active_tab_idx];
                    items = build_tui_items(&themes, &info.distro_id, active_tab);
                    selected_item_idx = items
                        .iter()
                        .position(|it| matches!(it, TuiItem::Theme { .. }))
                        .unwrap_or(0);
                    status_msg = None;
                }
                Key::Digit(d) => {
                    if d >= 1 && d <= TAB_ORDER.len() {
                        active_tab_idx = d - 1;
                        active_tab = TAB_ORDER[active_tab_idx];
                        items = build_tui_items(&themes, &info.distro_id, active_tab);
                        selected_item_idx = items
                            .iter()
                            .position(|it| matches!(it, TuiItem::Theme { .. }))
                            .unwrap_or(0);
                        status_msg = None;
                    }
                }
                Key::MouseClick { x, y } => {
                    let (cols, rows) = terminal_dimensions();
                    let mut header_rows = 0;
                    if let Some(art_lines) = banner::for_width(cols) {
                        header_rows += art_lines.len();
                    }
                    header_rows += 2;
                    let footer_rows = 3;
                    let main_rows = rows.saturating_sub(header_rows + footer_rows).max(6);
                    let is_split = cols >= 88;

                    // Footer buttons
                    if y >= rows.saturating_sub(1) {
                        if (20..=36).contains(&x) {
                            // [Enter] Apply
                            drop(_guard);
                            let selected_theme_idx = match items.get(selected_item_idx) {
                                Some(TuiItem::Theme { theme_idx }) => *theme_idx,
                                _ => 0,
                            };
                            let chosen = &themes[selected_theme_idx].name;
                            let target = config::save_theme_name(config_path, chosen)?;
                            println!(
                                "\x1b[1;32m✓ Theme '{chosen}' successfully applied and saved to {}\x1b[0m",
                                target.display()
                            );
                            return Ok(());
                        } else if (37..=50).contains(&x) {
                            // [e] Export
                            let selected_theme_idx = match items.get(selected_item_idx) {
                                Some(TuiItem::Theme { theme_idx }) => *theme_idx,
                                _ => 0,
                            };
                            let entry = &themes[selected_theme_idx];
                            let export_name = format!("{}-custom", entry.name);
                            match config::export_theme(config_path, &entry.def, &export_name) {
                                Ok(target) => {
                                    status_msg = Some(format!(
                                        "\x1b[1;32m✓ Exported theme '{}' to {} and set active!\x1b[0m",
                                        export_name,
                                        target.display()
                                    ));
                                }
                                Err(e) => {
                                    status_msg = Some(format!(
                                        "\x1b[1;31m✗ Failed to export theme: {e}\x1b[0m"
                                    ));
                                }
                            }
                        } else if (51..=64).contains(&x) {
                            // [q] Quit
                            drop(_guard);
                            println!("Setup closed without saving changes.");
                            return Ok(());
                        }
                    } else if is_split {
                        let left_width = 44.min(cols / 2 - 2).max(38);

                        // Click on pill bar row (header_rows + 1)
                        if y == header_rows + 1 && x <= left_width {
                            let pill_ranges = [
                                (1..=7, 0),   // 1:All
                                (8..=14, 1),  // 2:Rec
                                (15..=23, 2), // 3:Kitty
                                (24..=30, 3), // 4:Lay
                                (31..=38, 4), // 5:Dist
                                (39..=45, 5), // 6:Pal
                                (46..=52, 6), // 7:Art
                            ];
                            for (range, tab_i) in pill_ranges {
                                if range.contains(&x) && tab_i < TAB_ORDER.len() {
                                    active_tab_idx = tab_i;
                                    active_tab = TAB_ORDER[active_tab_idx];
                                    items = build_tui_items(&themes, &info.distro_id, active_tab);
                                    selected_item_idx = items
                                        .iter()
                                        .position(|it| matches!(it, TuiItem::Theme { .. }))
                                        .unwrap_or(0);
                                    status_msg = None;
                                    break;
                                }
                            }
                        } else {
                            // Click on theme list
                            let max_list_items = main_rows.saturating_sub(2);
                            let scroll_offset = if selected_item_idx < max_list_items / 2 {
                                0
                            } else if selected_item_idx + max_list_items / 2 >= items.len() {
                                items.len().saturating_sub(max_list_items)
                            } else {
                                selected_item_idx.saturating_sub(max_list_items / 2)
                            };
                            let list_start_y = header_rows + 2;
                            let list_end_y = list_start_y + max_list_items;
                            if x <= left_width && y >= list_start_y && y < list_end_y {
                                let row = y - list_start_y;
                                let target_idx = scroll_offset + row;
                                if target_idx < items.len() {
                                    match &items[target_idx] {
                                        TuiItem::Theme { .. } => {
                                            selected_item_idx = target_idx;
                                            status_msg = None;
                                        }
                                        TuiItem::SectionHeader { .. } => {
                                            if target_idx + 1 < items.len()
                                                && matches!(
                                                    items[target_idx + 1],
                                                    TuiItem::Theme { .. }
                                                )
                                            {
                                                selected_item_idx = target_idx + 1;
                                                status_msg = None;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Narrow mode
                        let max_list = (main_rows / 2).max(4);
                        let scroll_offset = if selected_item_idx < max_list / 2 {
                            0
                        } else if selected_item_idx + max_list / 2 >= items.len() {
                            items.len().saturating_sub(max_list)
                        } else {
                            selected_item_idx.saturating_sub(max_list / 2)
                        };
                        let list_start_y = header_rows + 1;
                        let list_end_y = list_start_y + max_list;
                        if y >= list_start_y && y < list_end_y {
                            let row = y - list_start_y;
                            let target_idx = scroll_offset + row;
                            if target_idx < items.len() {
                                match &items[target_idx] {
                                    TuiItem::Theme { .. } => {
                                        selected_item_idx = target_idx;
                                        status_msg = None;
                                    }
                                    TuiItem::SectionHeader { .. } => {
                                        if target_idx + 1 < items.len()
                                            && matches!(
                                                items[target_idx + 1],
                                                TuiItem::Theme { .. }
                                            )
                                        {
                                            selected_item_idx = target_idx + 1;
                                            status_msg = None;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Key::Export => {
                    let selected_theme_idx = match items.get(selected_item_idx) {
                        Some(TuiItem::Theme { theme_idx }) => *theme_idx,
                        _ => 0,
                    };
                    let entry = &themes[selected_theme_idx];
                    let export_name = format!("{}-custom", entry.name);
                    match config::export_theme(config_path, &entry.def, &export_name) {
                        Ok(target) => {
                            status_msg = Some(format!(
                                "\x1b[1;32m✓ Exported theme '{}' to {} and set active!\x1b[0m",
                                export_name,
                                target.display()
                            ));
                        }
                        Err(e) => {
                            status_msg =
                                Some(format!("\x1b[1;31m✗ Failed to export theme: {e}\x1b[0m"));
                        }
                    }
                }
                Key::Enter => {
                    drop(_guard);
                    let selected_theme_idx = match items.get(selected_item_idx) {
                        Some(TuiItem::Theme { theme_idx }) => *theme_idx,
                        _ => 0,
                    };
                    let chosen = &themes[selected_theme_idx].name;
                    let target = config::save_theme_name(config_path, chosen)?;
                    println!(
                        "\x1b[1;32m✓ Theme '{chosen}' successfully applied and saved to {}\x1b[0m",
                        target.display()
                    );
                    return Ok(());
                }
                Key::Quit => {
                    drop(_guard);
                    println!("Setup closed without saving changes.");
                    return Ok(());
                }
                Key::Other => {}
            }
        }
    }
}

/// Backwards compatibility alias for `--theme-picker` / `--themes`.
pub fn run_picker(
    info: &SystemInfo,
    cfg: &Config,
    config_path: Option<&Path>,
    no_logo: bool,
    logo_override: Option<&str>,
) -> Result<(), String> {
    run_setup(info, cfg, config_path, no_logo, logo_override)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ThemeDef, ThemeSource};

    #[test]
    fn test_classify_themes() {
        let kitty_entry = ThemeEntry {
            name: "kitty-modern".to_string(),
            def: ThemeDef::default(),
            source: ThemeSource::Builtin,
        };
        assert_eq!(classify_theme(&kitty_entry), ThemeCategory::Kitty);

        let layout_entry = ThemeEntry {
            name: "classic".to_string(),
            def: ThemeDef {
                modules: Some(vec![]),
                ..Default::default()
            },
            source: ThemeSource::Builtin,
        };
        assert_eq!(classify_theme(&layout_entry), ThemeCategory::Layouts);

        let distro_entry = ThemeEntry {
            name: "arch-rice".to_string(),
            def: ThemeDef::default(),
            source: ThemeSource::Builtin,
        };
        assert_eq!(classify_theme(&distro_entry), ThemeCategory::Distros);

        let custom_entry = ThemeEntry {
            name: "my-theme".to_string(),
            def: ThemeDef::default(),
            source: ThemeSource::File(std::path::PathBuf::from("/tmp/theme.json")),
        };
        assert_eq!(classify_theme(&custom_entry), ThemeCategory::Custom);
    }

    #[test]
    fn test_build_tui_items_filter() {
        let entries = vec![
            ThemeEntry {
                name: "kitty-modern".to_string(),
                def: ThemeDef::default(),
                source: ThemeSource::Builtin,
            },
            ThemeEntry {
                name: "ubuntu-warm".to_string(),
                def: ThemeDef {
                    distro: Some("ubuntu".to_string()),
                    ..Default::default()
                },
                source: ThemeSource::Builtin,
            },
            ThemeEntry {
                name: "dracula".to_string(),
                def: ThemeDef::default(),
                source: ThemeSource::Builtin,
            },
        ];

        let all_items = build_tui_items(&entries, "ubuntu", TabFilter::All);
        assert!(!all_items.is_empty());

        let kitty_items = build_tui_items(
            &entries,
            "ubuntu",
            TabFilter::Category(ThemeCategory::Kitty),
        );
        assert!(!kitty_items.is_empty());
        let has_kitty_theme = kitty_items
            .iter()
            .any(|it| matches!(it, TuiItem::Theme { theme_idx } if *theme_idx == 0));
        assert!(has_kitty_theme);
    }
}
