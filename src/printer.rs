use crate::config::parse_color;
use crate::info::types::SystemInfo;
use crate::logos::get_logo;
use crate::utils::visible_width;

pub struct PrintOptions<'a> {
    pub no_color: bool,
    pub no_logo: bool,
    pub logo_override: Option<&'a str>,
    pub logo_color: Option<&'a str>,
    pub key_color: Option<&'a str>,
    pub title_color: Option<&'a str>,
    pub value_color: Option<&'a str>,
    pub separator: Option<&'a str>,
    pub structure: Option<&'a [String]>,
    pub json: bool,
}

pub fn format_module_lines(
    info: &SystemInfo,
    no_color: bool,
    key_color: &str,
    title_color: Option<&str>,
    value_color: Option<&str>,
    separator: Option<&str>,
    structure: Option<&[String]>,
) -> Vec<String> {
    let reset = if no_color { "" } else { "\x1b[0m" };
    let key_col = if no_color { "" } else { key_color };
    let title_col = if no_color {
        ""
    } else {
        title_color.unwrap_or(key_color)
    };
    let value_col: &str = if no_color {
        ""
    } else {
        &value_color.and_then(parse_color).unwrap_or_default()
    };
    let sep = separator.unwrap_or(": ");
    let val_prefix = if no_color {
        sep.to_string()
    } else {
        format!("{reset}{sep}")
    };

    let default_order = [
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
        "locale",
        "break",
        "colors",
    ];

    let modules: Vec<&str> = if let Some(custom) = structure {
        custom.iter().map(|s| s.as_str()).collect()
    } else {
        default_order.to_vec()
    };

    let mut lines = Vec::new();
    let title_plain = format!("{}@{}", info.user, info.hostname);

    for m in modules {
        match m {
            "title" => {
                if no_color {
                    lines.push(title_plain.clone());
                } else {
                    lines.push(format!(
                        "{title_col}{}{reset}@{title_col}{}{reset}",
                        info.user, info.hostname
                    ));
                }
            }
            "separator" => {
                lines.push("-".repeat(title_plain.len()));
            }
            "os" => {
                if let Some(ref val) = info.os {
                    lines.push(format!("{key_col}OS:{val_prefix}{value_col}{val}{reset}"));
                }
            }
            "host" => {
                if let Some(ref val) = info.host {
                    lines.push(format!("{key_col}Host:{val_prefix}{value_col}{val}{reset}"));
                }
            }
            "kernel" => {
                if let Some(ref val) = info.kernel {
                    lines.push(format!(
                        "{key_col}Kernel:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "uptime" => {
                if let Some(ref val) = info.uptime {
                    lines.push(format!(
                        "{key_col}Uptime:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "packages" => {
                if let Some(ref val) = info.packages {
                    lines.push(format!(
                        "{key_col}Packages:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "shell" => {
                if let Some(ref val) = info.shell {
                    lines.push(format!(
                        "{key_col}Shell:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "display" => {
                if let Some(ref val) = info.display {
                    if let Some((conn, rest)) = val.split_once(':') {
                        lines.push(format!(
                            "{key_col}Display ({conn}):{val_prefix}{value_col}{rest}{reset}"
                        ));
                    } else {
                        lines.push(format!(
                            "{key_col}Display:{val_prefix}{value_col}{val}{reset}"
                        ));
                    }
                }
            }
            "de" => {
                if let Some(ref val) = info.de {
                    lines.push(format!("{key_col}DE:{val_prefix}{value_col}{val}{reset}"));
                }
            }
            "wm" => {
                if let Some(ref val) = info.wm {
                    lines.push(format!("{key_col}WM:{val_prefix}{value_col}{val}{reset}"));
                }
            }
            "wm_theme" => {
                if let Some(ref val) = info.wm_theme {
                    lines.push(format!(
                        "{key_col}WM Theme:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "theme" => {
                if let Some(ref val) = info.theme {
                    lines.push(format!(
                        "{key_col}Theme:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "icons" => {
                if let Some(ref val) = info.icons {
                    lines.push(format!(
                        "{key_col}Icons:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "font" => {
                if let Some(ref val) = info.font {
                    lines.push(format!("{key_col}Font:{val_prefix}{value_col}{val}{reset}"));
                }
            }
            "cursor" => {
                if let Some(ref val) = info.cursor {
                    lines.push(format!(
                        "{key_col}Cursor:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "terminal" => {
                if let Some(ref val) = info.terminal {
                    lines.push(format!(
                        "{key_col}Terminal:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "terminal_font" => {
                if let Some(ref val) = info.terminal_font {
                    lines.push(format!(
                        "{key_col}Terminal Font:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "cpu" => {
                if let Some(ref val) = info.cpu {
                    lines.push(format!("{key_col}CPU:{val_prefix}{value_col}{val}{reset}"));
                }
            }
            "gpu" => {
                if let Some(ref val) = info.gpu {
                    lines.push(format!("{key_col}GPU:{val_prefix}{value_col}{val}{reset}"));
                }
            }
            "memory" => {
                if let Some(ref val) = info.memory {
                    lines.push(format!(
                        "{key_col}Memory:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "swap" => {
                if let Some(ref val) = info.swap {
                    lines.push(format!("{key_col}Swap:{val_prefix}{value_col}{val}{reset}"));
                }
            }
            "disk" => {
                if let Some(ref disks) = info.disk {
                    for d in disks {
                        if let Some((mount, rest)) = d.split_once(':') {
                            lines.push(format!(
                                "{key_col}Disk ({mount}):{val_prefix}{value_col}{rest}{reset}"
                            ));
                        } else {
                            lines.push(format!("{key_col}Disk:{val_prefix}{value_col}{d}{reset}"));
                        }
                    }
                }
            }
            "battery" => {
                if let Some(ref val) = info.battery {
                    lines.push(format!(
                        "{key_col}Battery:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "power_adapter" => {
                if let Some(ref val) = info.power_adapter {
                    lines.push(format!(
                        "{key_col}Power Adapter:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "audio" => {
                if let Some(ref val) = info.audio {
                    lines.push(format!(
                        "{key_col}Audio:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "local_ip" => {
                if let Some(ref val) = info.local_ip {
                    if let Some((iface, rest)) = val.split_once(':') {
                        lines.push(format!(
                            "{key_col}Local IP ({iface}):{val_prefix}{value_col}{rest}{reset}"
                        ));
                    } else {
                        lines.push(format!(
                            "{key_col}Local IP:{val_prefix}{value_col}{val}{reset}"
                        ));
                    }
                }
            }
            "locale" => {
                if let Some(ref val) = info.locale {
                    lines.push(format!(
                        "{key_col}Locale:{val_prefix}{value_col}{val}{reset}"
                    ));
                }
            }
            "break" => {
                lines.push(String::new());
            }
            "colors" if !no_color => {
                lines.push(format!(
                    "\x1b[40m   \x1b[41m   \x1b[42m   \x1b[43m   \x1b[44m   \x1b[45m   \x1b[46m   \x1b[47m   {reset}"
                ));
                lines.push(format!(
                    "\x1b[100m   \x1b[101m   \x1b[102m   \x1b[103m   \x1b[104m   \x1b[105m   \x1b[106m   \x1b[107m   {reset}"
                ));
            }
            _ => {}
        }
    }

    lines
}

pub fn print_fetch(info: &SystemInfo, opts: &PrintOptions) {
    if opts.json {
        if let Ok(serialized) = serde_json::to_string_pretty(info) {
            println!("{serialized}");
        }
        return;
    }

    let logo_key = opts.logo_override.unwrap_or(info.distro_id.as_str());

    let (logo_lines, default_key_color) = get_logo(logo_key, opts.no_color, opts.logo_color);
    let resolved_key_color: &str = if opts.no_color {
        ""
    } else if let Some(k) = opts.key_color {
        &parse_color(k).unwrap_or_else(|| default_key_color.to_string())
    } else {
        default_key_color
    };
    let resolved_title_color: String = opts.title_color.and_then(parse_color).unwrap_or_default();
    let info_lines = format_module_lines(
        info,
        opts.no_color,
        resolved_key_color,
        if resolved_title_color.is_empty() {
            None
        } else {
            Some(&resolved_title_color)
        },
        opts.value_color,
        opts.separator,
        opts.structure,
    );

    if opts.no_logo {
        for line in info_lines {
            println!("{line}");
        }
        return;
    }

    let max_logo_width = logo_lines
        .iter()
        .map(|l| visible_width(l))
        .max()
        .unwrap_or(0);

    let total_lines = logo_lines.len().max(info_lines.len());
    for i in 0..total_lines {
        let logo_part = if i < logo_lines.len() {
            let l = &logo_lines[i];
            let pad = max_logo_width.saturating_sub(visible_width(l));
            format!("{l}{}", " ".repeat(pad))
        } else {
            " ".repeat(max_logo_width)
        };

        if i < info_lines.len() {
            println!("{logo_part}   {}", info_lines[i]);
        } else {
            println!("{logo_part}");
        }
    }
}
