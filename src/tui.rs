use crate::config::{self, parse_color};
use crate::info::types::SystemInfo;
use crate::printer::{PrintOptions, print_fetch};
use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

/// Short module list used for theme previews.
const PREVIEW_STRUCTURE: &[&str] = &[
    "title",
    "separator",
    "os",
    "kernel",
    "cpu",
    "memory",
    "colors",
];

/// Resolve display-ready parts for a theme name against a config.
fn parts_for(cfg: &config::Config, name: &str) -> config::ThemeColors {
    config::lookup_theme(cfg, name).unwrap_or_default()
}

/// One-line color swatch describing a theme.
pub fn describe(name: &str, colors: &config::ThemeColors) -> String {
    let mut bits = Vec::new();
    if let Some(t) = colors.title.as_deref() {
        bits.push(format!("title={t}"));
    }
    if let Some(k) = colors.keys.as_deref() {
        bits.push(format!("keys={k}"));
    }
    if let Some(v) = colors.value.as_deref() {
        bits.push(format!("value={v}"));
    }
    if let Some(s) = colors.separator.as_deref() {
        bits.push(format!("sep={s:?}"));
    }
    if let Some(l) = colors.logo_color.as_deref() {
        bits.push(format!("logo={l}"));
    }
    if bits.is_empty() {
        format!("{name} (distro default colors)")
    } else {
        format!("{name} ({})", bits.join(", "))
    }
}

fn is_custom(cfg: &config::Config, name: &str) -> bool {
    cfg.themes.as_ref().is_some_and(|m| {
        m.keys().any(|k| k.eq_ignore_ascii_case(name))
            && !config::BUILTIN_THEME_NAMES
                .iter()
                .any(|b| b.eq_ignore_ascii_case(name))
    })
}

/// Print a live preview of every available theme using real system info.
pub fn preview_all(
    info: &SystemInfo,
    cfg: &config::Config,
    no_color: bool,
    no_logo: bool,
    logo_override: Option<&str>,
) {
    let names = config::all_theme_names(cfg);
    let structure: Vec<String> = PREVIEW_STRUCTURE.iter().map(|s| s.to_string()).collect();
    for name in &names {
        let colors = parts_for(cfg, name);
        let tag = if is_custom(cfg, name) {
            "custom"
        } else {
            "built-in"
        };
        println!("=== {name} [{tag}] ===");
        println!("    {}", describe(name, &colors));
        let opts = PrintOptions {
            no_color,
            no_logo,
            logo_override,
            logo_color: colors.logo_color.as_deref(),
            key_color: colors.keys.as_deref(),
            title_color: colors.title.as_deref(),
            value_color: colors.value.as_deref(),
            separator: colors.separator.as_deref(),
            structure: Some(&structure),
            json: false,
        };
        print_fetch(info, &opts);
        println!();
    }
    println!("Use `rustfetch --theme <NAME>` to try one, `--themes` for the picker,");
    println!("or `--set-theme <NAME>` to save it as default.");
}

// ---------- interactive picker (no new dependencies) ----------

fn stdin_is_tty() -> bool {
    unsafe { libc::isatty(io::stdin().as_raw_fd()) != 0 }
}
#[cfg(unix)]
fn bytes_available(fd: i32) -> usize {
    let mut n: libc::c_int = 0;
    unsafe {
        if libc::ioctl(fd, libc::FIONREAD, &mut n) != 0 {
            return 0;
        }
    }
    n.max(0) as usize
}

#[cfg(unix)]
struct RawGuard {
    fd: i32,
    orig: libc::termios,
}

#[cfg(unix)]
impl RawGuard {
    fn enter() -> Result<Self, String> {
        use std::os::unix::io::AsRawFd;
        let fd = io::stdin().as_raw_fd();
        unsafe {
            let mut t: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(fd, &mut t) != 0 {
                return Err("tcgetattr failed".into());
            }
            let orig = t;
            t.c_lflag &= !(libc::ICANON | libc::ECHO);
            t.c_cc[libc::VMIN] = 1;
            t.c_cc[libc::VTIME] = 0;
            if libc::tcsetattr(fd, libc::TCSANOW, &t) != 0 {
                return Err("tcsetattr failed".into());
            }
            Ok(Self { fd, orig })
        }
    }
}

#[cfg(unix)]
impl Drop for RawGuard {
    fn drop(&mut self) {
        unsafe {
            libc::tcsetattr(self.fd, libc::TCSANOW, &self.orig);
        }
    }
}

enum Key {
    Up,
    Down,
    Enter,
    Quit,
    Save,
    Other,
}

#[cfg(unix)]
fn read_key() -> Key {
    use std::os::unix::io::AsRawFd;
    let fd = io::stdin().as_raw_fd();
    let mut buf = [0u8; 1];
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    if lock.read_exact(&mut buf).is_err() {
        return Key::Other;
    }
    match buf[0] {
        b'\r' | b'\n' => Key::Enter,
        b'q' | b'Q' => Key::Quit,
        b's' | b'S' => Key::Save,
        b'k' | b'K' => Key::Up,
        b'j' | b'J' => Key::Down,
        0x1b => {
            if bytes_available(fd) == 0 {
                return Key::Quit;
            }
            let mut seq = [0u8; 2];
            if lock.read_exact(&mut seq).is_err() {
                return Key::Quit;
            }
            if seq[0] == b'[' {
                match seq[1] {
                    b'A' => Key::Up,
                    b'B' => Key::Down,
                    _ => Key::Other,
                }
            } else {
                Key::Other
            }
        }
        _ => Key::Other,
    }
}

fn render_picker(
    info: &SystemInfo,
    cfg: &config::Config,
    names: &[String],
    selected: usize,
    no_logo: bool,
    logo_override: Option<&str>,
) {
    let mut out = io::stdout();
    let _ = write!(out, "\x1b[2J\x1b[H");
    let _ = writeln!(
        out,
        "rustfetch themes  (up/down or j/k, Enter = save & quit, s = save, q/Esc = quit)"
    );
    let _ = writeln!(out);
    for (i, name) in names.iter().enumerate() {
        let colors = parts_for(cfg, name);
        // Small colored dot preview using the keys color.
        let dot = match colors.keys.as_deref().and_then(parse_color) {
            Some(c) => format!("{c}\u{25cf}\x1b[0m"),
            None => "\u{25cb}".to_string(),
        };
        let tag = if is_custom(cfg, name) { "*" } else { " " };
        if i == selected {
            let _ = writeln!(out, "> [{tag}] {dot} {name}");
        } else {
            let _ = writeln!(out, "  [{tag}] {dot} {name}");
        }
    }
    let _ = writeln!(
        out,
        "\n* = your custom theme from config `themes`. Legend above."
    );
    let name = &names[selected];
    let colors = parts_for(cfg, name);
    let _ = writeln!(out, "--- preview: {} ---", describe(name, &colors));
    let _ = out.flush();

    // Full live preview with the highlighted theme.
    let opts = PrintOptions {
        no_color: false,
        no_logo,
        logo_override,
        logo_color: colors.logo_color.as_deref(),
        key_color: colors.keys.as_deref(),
        title_color: colors.title.as_deref(),
        value_color: colors.value.as_deref(),
        separator: colors.separator.as_deref(),
        structure: None,
        json: false,
    };
    print_fetch(info, &opts);
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "[{}] {}",
        names[selected],
        describe(&names[selected], &colors)
    );
    let _ = out.flush();
}

/// Numbered fallback when stdin is not a TTY.
fn numbered_fallback(
    cfg: &config::Config,
    names: &[String],
    config_path: Option<&std::path::Path>,
) -> Result<(), String> {
    println!("rustfetch themes (non-interactive terminal):");
    for (i, name) in names.iter().enumerate() {
        let colors = parts_for(cfg, name);
        println!("  {}. {}", i + 1, describe(name, &colors));
    }
    print!("Enter number to save (or 0 to quit): ");
    let _ = io::stdout().flush();
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .map_err(|e| e.to_string())?;
    let n: usize = line.trim().parse().unwrap_or(0);
    if n == 0 || n > names.len() {
        println!("No changes saved.");
        return Ok(());
    }
    let saved = config::save_theme_name(config_path, &names[n - 1])?;
    println!("Saved theme '{}' to {}", names[n - 1], saved.display());
    Ok(())
}

pub fn run_picker(
    info: &SystemInfo,
    cfg: &config::Config,
    config_path: Option<&std::path::Path>,
    no_logo: bool,
    logo_override: Option<&str>,
) -> Result<(), String> {
    let names = config::all_theme_names(cfg);
    if names.is_empty() {
        return Err("no themes available".into());
    }

    if !stdin_is_tty() {
        return numbered_fallback(cfg, &names, config_path);
    }

    #[cfg(not(unix))]
    {
        return numbered_fallback(cfg, &names, config_path);
    }

    #[cfg(unix)]
    {
        let _raw = RawGuard::enter()?;
        // Start on the currently configured theme if it exists.
        let mut selected = cfg
            .theme
            .as_ref()
            .and_then(|t| t.name())
            .and_then(|n| names.iter().position(|x| x.eq_ignore_ascii_case(n)))
            .unwrap_or(0);

        loop {
            render_picker(info, cfg, &names, selected, no_logo, logo_override);
            match read_key() {
                Key::Up => {
                    selected = selected.checked_sub(1).unwrap_or(names.len() - 1);
                }
                Key::Down => {
                    selected = (selected + 1) % names.len();
                }
                Key::Enter | Key::Save => {
                    drop(_raw);
                    let saved = config::save_theme_name(config_path, &names[selected])?;
                    println!("\nSaved theme '{}' to {}", names[selected], saved.display());
                    return Ok(());
                }
                Key::Quit => {
                    drop(_raw);
                    println!("\nNo changes saved.");
                    return Ok(());
                }
                Key::Other => {}
            }
        }
    }
}
