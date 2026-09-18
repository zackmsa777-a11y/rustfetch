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
        "\x1b[1;36m=== rustfetch theme gallery ({} available{}) ===\x1b[0m\n",
        themes.len(),
        detected_hdr
    );

    for entry in &themes {
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

    println!(
        "\x1b[1;32mUse `rustfetch --setup` to choose interactively with live full-screen preview,\x1b[0m"
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
    let mut in_esc = false;
    let mut vis = 0;
    for c in s.chars() {
        if in_esc {
            out.push(c);
            if c.is_ascii_alphabetic() {
                in_esc = false;
            }
        } else if c == '\x1b' {
            in_esc = true;
            out.push(c);
        } else {
            if vis >= max_width {
                break;
            }
            out.push(c);
            vis += 1;
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

#[allow(clippy::too_many_arguments)]
fn render_tui(
    info: &SystemInfo,
    _cfg: &Config,
    themes: &[ThemeEntry],
    selected: usize,
    active_theme_name: Option<&str>,
    status_msg: Option<&str>,
    no_logo: bool,
    logo_override: Option<&str>,
) {
    let (cols, rows) = terminal_dimensions();
    let mut out = io::stdout();
    let mut screen = String::with_capacity(16384);

    // Jump to top-left and prepare frame
    screen.push_str("\x1b[H");

    // 1. ASCII ART BANNER AT TOP
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

    // Subtitle / Tagline
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

    // Horizontal divider
    let divider = "─".repeat(cols);
    screen.push_str("\x1b[90m");
    screen.push_str(&divider);
    screen.push_str("\x1b[0m\x1b[K\r\n");
    header_rows += 1;

    // Calculate layout areas
    let footer_rows = 3;
    let main_rows = rows.saturating_sub(header_rows + footer_rows).max(6);

    let sel_entry = &themes[selected];
    let is_split = cols >= 88;

    if is_split {
        // Split Screen: Left Pane = Theme List (~36 cols), Right Pane = Live Preview
        let left_width = 38.min(cols / 3 + 6);
        let right_width = cols.saturating_sub(left_width + 3);

        // Pre-render preview for right pane
        let mut style = style_from_theme(&sel_entry.def);
        if no_logo {
            style.no_logo = true;
        }
        if let Some(logo) = logo_override {
            style.logo = Some(logo.to_string());
        }
        let preview_lines = render_lines(info, &style);

        // Scroll window for left pane
        let max_list_items = main_rows.saturating_sub(2);
        let scroll_offset = if selected < max_list_items / 2 {
            0
        } else if selected + max_list_items / 2 >= themes.len() {
            themes.len().saturating_sub(max_list_items)
        } else {
            selected.saturating_sub(max_list_items / 2)
        };

        // Header for columns
        let list_header = format!(
            "┌─ Themes ({}/{}) ─{}",
            selected + 1,
            themes.len(),
            "─".repeat(
                left_width.saturating_sub(18 + format!("{}/{}", selected + 1, themes.len()).len())
            )
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

        for row in 0..main_rows.saturating_sub(1) {
            // Left pane content
            let theme_idx = scroll_offset + row;
            let left_cell = if theme_idx < themes.len() {
                let t = &themes[theme_idx];
                let is_sel = theme_idx == selected;
                let is_active = active_theme_name.is_some_and(|a| a.eq_ignore_ascii_case(&t.name));

                let dot_col = t
                    .def
                    .keys
                    .as_deref()
                    .or(t.def.title.as_deref())
                    .and_then(parse_color)
                    .unwrap_or_else(|| "\x1b[37m".to_string());

                let prefix = if is_sel { "\x1b[1;32m▸ " } else { "  " };
                let dot = format!("{dot_col}●\x1b[0m");

                let tag_str = if t.matches_distro(&info.distro_id) {
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
                    format!("\x1b[1;37;44m {:<12}\x1b[0m", t.name)
                } else {
                    format!("{:<13}", t.name)
                };

                format!("{prefix}{active_marker}{dot} {name_display} {tag_str}{layout_flag}")
            } else {
                String::new()
            };

            // Right pane content
            let right_cell = if row == 0 {
                // Description line on top of preview
                if let Some(ref desc) = sel_entry.def.description {
                    format!("\x1b[3m\x1b[90m// {desc}\x1b[0m")
                } else {
                    format!("\x1b[90m// {}\x1b[0m", sel_entry.def.summary())
                }
            } else {
                let p_idx = row.saturating_sub(1);
                preview_lines.get(p_idx).cloned().unwrap_or_default()
            };

            let left_formatted = pad_right_visible(&left_cell, left_width);
            let right_formatted = pad_right_visible(&right_cell, right_width);

            screen.push_str(&left_formatted);
            screen.push_str("\x1b[90m │ \x1b[0m");
            screen.push_str(&right_formatted);
            screen.push_str("\x1b[K\r\n");
        }
    } else {
        // Narrow Terminal: Single column selector with info below
        let max_list = (main_rows / 2).max(4);
        let scroll_offset = if selected < max_list / 2 {
            0
        } else if selected + max_list / 2 >= themes.len() {
            themes.len().saturating_sub(max_list)
        } else {
            selected.saturating_sub(max_list / 2)
        };

        screen.push_str(&format!(
            "\x1b[1;36m── Themes ({}/{}) ────────────────────────\x1b[0m\x1b[K\r\n",
            selected + 1,
            themes.len()
        ));

        for row in 0..max_list {
            let idx = scroll_offset + row;
            if idx < themes.len() {
                let t = &themes[idx];
                let is_sel = idx == selected;
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
                let tag_str = if t.matches_distro(&info.distro_id) {
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

        // Preview snippet below
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

    // 3. FOOTER
    screen.push_str("\x1b[90m");
    screen.push_str(&"─".repeat(cols));
    screen.push_str("\x1b[0m\x1b[K\r\n");

    // Shortcut bar
    let keys_hint = "\x1b[1;37m[↑/↓]\x1b[0m Select  \x1b[1;32m[Enter]\x1b[0m Apply  \x1b[1;33m[e]\x1b[0m Export  \x1b[1;31m[q]\x1b[0m Quit";
    let active_name_str = active_theme_name.unwrap_or("default");
    let active_status = format!("Active: \x1b[1;32m{active_name_str}\x1b[0m");

    let spacing = cols.saturating_sub(visible_width(keys_hint) + visible_width(&active_status));
    screen.push_str(keys_hint);
    screen.push_str(&" ".repeat(spacing));
    screen.push_str(&active_status);
    screen.push_str("\x1b[K\r\n");

    // Status or hint message line
    if let Some(msg) = status_msg {
        screen.push_str(msg);
    } else {
        screen.push_str("\x1b[90mTip: Enter to apply, e to export to ~/.config/rustfetch/config.jsonc, q to quit\x1b[0m");
    }
    screen.push_str("\x1b[K");

    let _ = write!(out, "{screen}");
    let _ = out.flush();
}

// ---------------- Numbered Non-Interactive Fallback ----------------

fn numbered_fallback(
    _cfg: &Config,
    themes: &[ThemeEntry],
    config_path: Option<&Path>,
) -> Result<(), String> {
    println!("\x1b[1;36mrustfetch themes (non-interactive mode):\x1b[0m\n");
    for (i, entry) in themes.iter().enumerate() {
        let tag = entry.source.tag();
        let desc = entry.def.description.as_deref().unwrap_or("");
        println!(
            "  \x1b[1;33m{:>2}.\x1b[0m {:<14} [{:<7}] {}",
            i + 1,
            entry.name,
            tag,
            desc
        );
    }
    print!("\nEnter number to save as default (or 0 to quit): ");
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
        return numbered_fallback(cfg, &themes, config_path);
    }

    #[cfg(not(unix))]
    {
        return numbered_fallback(cfg, &themes, config_path);
    }

    #[cfg(unix)]
    {
        let _guard = TerminalGuard::enter()?;
        let input = RawInput::new(_guard.fd);

        let active_name = config::active_theme_name(cfg);
        let mut selected = active_name
            .as_deref()
            .and_then(|act| themes.iter().position(|t| t.name.eq_ignore_ascii_case(act)))
            .unwrap_or(0);

        let mut status_msg: Option<String> = None;

        loop {
            render_tui(
                info,
                cfg,
                &themes,
                selected,
                active_name.as_deref(),
                status_msg.as_deref(),
                no_logo,
                logo_override,
            );

            match input.next_key() {
                Key::Up => {
                    selected = selected.checked_sub(1).unwrap_or(themes.len() - 1);
                    status_msg = None;
                }
                Key::Down => {
                    selected = (selected + 1) % themes.len();
                    status_msg = None;
                }
                Key::PageUp => {
                    selected = selected.saturating_sub(5);
                    status_msg = None;
                }
                Key::PageDown => {
                    selected = (selected + 5).min(themes.len() - 1);
                    status_msg = None;
                }
                Key::Home => {
                    selected = 0;
                    status_msg = None;
                }
                Key::End => {
                    selected = themes.len() - 1;
                    status_msg = None;
                }
                Key::ScrollUp => {
                    selected = selected.saturating_sub(1);
                    status_msg = None;
                }
                Key::ScrollDown => {
                    selected = (selected + 1).min(themes.len() - 1);
                    status_msg = None;
                }
                Key::MouseClick { x, y } => {
                    let (cols, rows) = terminal_dimensions();
                    let mut header_rows = 0;
                    if let Some(art_lines) = banner::for_width(cols) {
                        header_rows += art_lines.len();
                    }
                    header_rows += 2; // subtitle + divider
                    let footer_rows = 3;
                    let main_rows = rows.saturating_sub(header_rows + footer_rows).max(6);
                    let is_split = cols >= 88;

                    // Click on footer buttons (row rows - 1 is shortcut bar, row rows is tip)
                    if y >= rows.saturating_sub(1) {
                        if (15..=29).contains(&x) {
                            // [Enter] Apply
                            drop(_guard);
                            let chosen = &themes[selected].name;
                            let target = config::save_theme_name(config_path, chosen)?;
                            println!(
                                "\x1b[1;32m✓ Theme '{chosen}' successfully applied and saved to {}\x1b[0m",
                                target.display()
                            );
                            return Ok(());
                        } else if (30..=41).contains(&x) {
                            // [e] Export
                            let entry = &themes[selected];
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
                        } else if (42..=55).contains(&x) {
                            // [q] Quit
                            drop(_guard);
                            println!("Setup closed without saving changes.");
                            return Ok(());
                        }
                    } else if is_split {
                        let left_width = 38.min(cols / 3 + 6);
                        let max_list_items = main_rows.saturating_sub(2);
                        let scroll_offset = if selected < max_list_items / 2 {
                            0
                        } else if selected + max_list_items / 2 >= themes.len() {
                            themes.len().saturating_sub(max_list_items)
                        } else {
                            selected.saturating_sub(max_list_items / 2)
                        };
                        let list_start_y = header_rows + 2;
                        let list_end_y = list_start_y + main_rows.saturating_sub(1);
                        if x <= left_width && y >= list_start_y && y < list_end_y {
                            let row = y - list_start_y;
                            let target_idx = scroll_offset + row;
                            if target_idx < themes.len() {
                                selected = target_idx;
                                status_msg = None;
                            }
                        }
                    } else {
                        // Narrow mode
                        let max_list = (main_rows / 2).max(4);
                        let scroll_offset = if selected < max_list / 2 {
                            0
                        } else if selected + max_list / 2 >= themes.len() {
                            themes.len().saturating_sub(max_list)
                        } else {
                            selected.saturating_sub(max_list / 2)
                        };
                        let list_start_y = header_rows + 2;
                        let list_end_y = list_start_y + max_list;
                        if y >= list_start_y && y < list_end_y {
                            let row = y - list_start_y;
                            let target_idx = scroll_offset + row;
                            if target_idx < themes.len() {
                                selected = target_idx;
                                status_msg = None;
                            }
                        }
                    }
                }
                Key::Export => {
                    let entry = &themes[selected];
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
                    let chosen = &themes[selected].name;
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
