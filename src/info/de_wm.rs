use crate::utils::{clean, run_cmd, value};
use std::env;
use std::fs;
use std::path::Path;

pub struct DesktopInfo {
    pub de: Option<String>,
    pub wm: Option<String>,
    pub wm_theme: Option<String>,
    pub theme: Option<String>,
    pub icons: Option<String>,
    pub font: Option<String>,
    pub cursor: Option<String>,
}

pub fn detect_de_wm() -> DesktopInfo {
    let xdg_desktop = env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .or_else(|_| env::var("GDMSESSION"))
        .ok()
        .map(|s| clean(&s));

    let de = if let Some(ref desk) = xdg_desktop {
        let desk_upper = desk.to_uppercase();
        if desk_upper.contains("GNOME") {
            let ver = run_cmd("gnome-shell", &["--version"])
                .and_then(|v| v.split_whitespace().nth(2).map(|s| s.to_string()));
            if let Some(v) = ver {
                Some(format!("GNOME {v}"))
            } else {
                Some("GNOME".into())
            }
        } else if desk_upper.contains("KDE") || desk_upper.contains("PLASMA") {
            let ver = run_cmd("plasmashell", &["--version"])
                .and_then(|v| v.split_whitespace().nth(1).map(|s| s.to_string()));
            if let Some(v) = ver {
                Some(format!("KDE Plasma {v}"))
            } else {
                Some("KDE Plasma".into())
            }
        } else if desk_upper.contains("XFCE") {
            let ver = run_cmd("xfce4-session", &["--version"]).and_then(|v| {
                v.lines()
                    .next()
                    .and_then(|l| l.split_whitespace().nth(1).map(|s| s.to_string()))
            });
            if let Some(v) = ver {
                Some(format!("XFCE {v}"))
            } else {
                Some("XFCE".into())
            }
        } else if desk_upper.contains("CINNAMON") {
            Some("Cinnamon".into())
        } else if desk_upper.contains("MATE") {
            Some("MATE".into())
        } else if desk_upper.contains("LXQT") {
            Some("LXQt".into())
        } else if desk_upper.contains("DEEPIN") {
            Some("Deepin".into())
        } else if desk_upper.contains("PANTHEON") {
            Some("Pantheon".into())
        } else {
            Some(desk.clone())
        }
    } else {
        None
    };

    let wm = detect_wm();
    let (theme, icons, font, cursor) = detect_gtk_settings();

    DesktopInfo {
        de,
        wm,
        wm_theme: None,
        theme,
        icons,
        font,
        cursor,
    }
}

pub fn detect_wm() -> Option<String> {
    let wm_candidates = [
        ("Hyprland", "hyprland"),
        ("Sway", "sway"),
        ("Wayfire", "wayfire"),
        ("River", "river"),
        ("Niri", "niri"),
        ("KWin", "kwin"),
        ("Mutter", "mutter"),
        ("i3", "i3"),
        ("bspwm", "bspwm"),
        ("dwm", "dwm"),
        ("awesome", "awesome"),
        ("xmonad", "xmonad"),
        ("Xfwm4", "xfwm4"),
        ("Openbox", "openbox"),
        ("Fluxbox", "fluxbox"),
        ("herbstluftwm", "herbstluftwm"),
        ("qtile", "qtile"),
    ];

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let comm_path = path.join("comm");
            if let Ok(comm) = fs::read_to_string(comm_path) {
                let name = comm.trim().to_lowercase();
                for (wm_name, candidate) in &wm_candidates {
                    if name == *candidate || name.contains(candidate) {
                        return Some(wm_name.to_string());
                    }
                }
            }
        }
    }

    if let Ok(desk) = env::var("XDG_CURRENT_DESKTOP") {
        let d = desk.to_lowercase();
        if d.contains("gnome") {
            return Some("Mutter".into());
        } else if d.contains("kde") || d.contains("plasma") {
            return Some("KWin".into());
        } else if d.contains("xfce") {
            return Some("Xfwm4".into());
        }
    }

    None
}

fn detect_gtk_settings() -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let home = match env::var("HOME") {
        Ok(h) => h,
        Err(_) => return (None, None, None, None),
    };

    let files = [
        format!("{home}/.config/gtk-3.0/settings.ini"),
        format!("{home}/.config/gtk-4.0/settings.ini"),
        format!("{home}/.gtkrc-2.0"),
    ];

    for file_path in &files {
        if let Ok(content) = fs::read_to_string(Path::new(file_path)) {
            let theme = value(&content, "gtk-theme-name", '=').map(clean);
            let icons = value(&content, "gtk-icon-theme-name", '=').map(clean);
            let font = value(&content, "gtk-font-name", '=').map(clean);
            let cursor = value(&content, "gtk-cursor-theme-name", '=').map(clean);

            if theme.is_some() || icons.is_some() || font.is_some() || cursor.is_some() {
                return (theme, icons, font, cursor);
            }
        }
    }

    (None, None, None, None)
}
