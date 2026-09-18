use crate::art;
use crate::config::{ModuleSpec, ThemeDef, parse_color};
use crate::info::types::SystemInfo;
use crate::logos::logo_art;
use crate::utils::visible_width;

/// Everything needed to render one fetch.
#[derive(Debug, Clone, Default)]
pub struct RenderStyle {
    pub no_color: bool,
    pub no_logo: bool,
    /// Logo name, ASCII-art path, `art:<name>`, `auto`, or `none`.
    pub logo: Option<String>,
    pub logo_color: Option<String>,
    pub padding_top: usize,
    pub padding_right: usize,
    pub key_color: Option<String>,
    pub title_color: Option<String>,
    pub value_color: Option<String>,
    pub separator: Option<String>,
    /// Explicit layout. `None` renders the default module order.
    pub modules: Option<Vec<ModuleSpec>>,
}

/// Translate a theme definition into a render style; CLI flags are layered on
/// top of this by the caller.
pub fn style_from_theme(def: &ThemeDef) -> RenderStyle {
    let mut style = RenderStyle {
        key_color: def.keys.clone(),
        title_color: def.title.clone(),
        value_color: def.value.clone(),
        separator: def.separator.clone(),
        logo_color: def.logo_color.clone(),
        ..Default::default()
    };

    if let Some(logo) = def.logo.as_ref() {
        style.logo = logo.source().map(|s| s.to_string());
        if logo.is_hidden() {
            style.no_logo = true;
        }
        let padding = logo.padding();
        style.padding_top = padding.top;
        style.padding_right = padding.right;
    }
    if let Some(padding) = def.padding {
        style.padding_top = padding.top;
        style.padding_right = padding.right;
    }

    if let Some(modules) = def.modules.as_ref() {
        style.modules = Some(crate::config::modules_to_specs(modules));
    }

    style
}

/// Backward-compatible print options.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
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

/// Modules rendered when neither the theme nor the config supplies a layout.
pub fn default_modules() -> Vec<ModuleSpec> {
    const ORDER: &[&str] = &[
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
    ORDER
        .iter()
        .map(|k| ModuleSpec {
            kind: (*k).to_string(),
            ..Default::default()
        })
        .collect()
}

/// Default label for a module kind.
pub fn default_key(kind: &str) -> &'static str {
    match kind {
        "os" => "OS",
        "host" => "Host",
        "kernel" => "Kernel",
        "uptime" => "Uptime",
        "packages" => "Packages",
        "shell" => "Shell",
        "display" => "Display",
        "de" => "DE",
        "wm" => "WM",
        "wm_theme" => "WM Theme",
        "theme" => "Theme",
        "icons" => "Icons",
        "font" => "Font",
        "cursor" => "Cursor",
        "terminal" => "Terminal",
        "terminal_font" => "Terminal Font",
        "cpu" => "CPU",
        "gpu" => "GPU",
        "memory" => "Memory",
        "swap" => "Swap",
        "disk" => "Disk",
        "battery" => "Battery",
        "power_adapter" => "Power Adapter",
        "audio" => "Audio",
        "local_ip" => "Local IP",
        "locale" => "Locale",
        _ => "",
    }
}

/// Value of a module, or `None` when the module has nothing to show.
pub fn module_value(info: &SystemInfo, kind: &str) -> Option<String> {
    let v = match kind {
        "os" => info.os.clone(),
        "host" => info.host.clone(),
        "kernel" => info.kernel.clone(),
        "uptime" => info.uptime.clone(),
        "packages" => info.packages.clone(),
        "shell" => info.shell.clone(),
        "display" => info.display.clone(),
        "de" => info.de.clone(),
        "wm" => info.wm.clone(),
        "wm_theme" => info.wm_theme.clone(),
        "theme" => info.theme.clone(),
        "icons" => info.icons.clone(),
        "font" => info.font.clone(),
        "cursor" => info.cursor.clone(),
        "terminal" => info.terminal.clone(),
        "terminal_font" => info.terminal_font.clone(),
        "cpu" => info.cpu.clone(),
        "gpu" => info.gpu.clone(),
        "memory" => info.memory.clone(),
        "swap" => info.swap.clone(),
        "disk" => info.disk.as_ref().map(|d| d.join(", ")),
        "battery" => info.battery.clone(),
        "power_adapter" => info.power_adapter.clone(),
        "audio" => info.audio.clone(),
        "local_ip" => info.local_ip.clone(),
        "public_ip" => info.public_ip.clone(),
        "locale" => info.locale.clone(),
        _ => None,
    };
    v.filter(|s| !s.trim().is_empty())
}

/// Modules whose value carries an `interface: value` annotation.
fn is_annotated(kind: &str) -> bool {
    matches!(kind, "display" | "local_ip")
}

/// `{}` / `{1}` in a theme's `format` string is replaced by the module value.
fn apply_format(template: &str, value: &str) -> String {
    if template.contains("{}") {
        template.replace("{}", value)
    } else if template.contains("{1}") {
        template.replace("{1}", value)
    } else if template.is_empty() {
        value.to_string()
    } else {
        template.to_string()
    }
}

/// Resolved logo art: lines (uncoloured) plus the default colour escape.
pub struct LogoArt {
    pub lines: Vec<String>,
    pub default_color: &'static str,
}

impl LogoArt {
    pub fn is_empty(&self) -> bool {
        self.lines.iter().all(|l| l.trim().is_empty())
    }
}

/// True when a logo `source` value points at a file on disk.
fn looks_like_path(source: &str) -> bool {
    source.contains('/') || source.starts_with('~') || source.ends_with(".txt")
}

/// Resolve `auto`, a distro logo name, `art:<name>`, a file path, or `none`.
pub fn resolve_logo(source: Option<&str>, distro: &str) -> LogoArt {
    let raw = match source {
        None | Some("auto") | Some("") => distro,
        Some(other) => other,
    };

    if raw == "none" {
        return LogoArt {
            lines: Vec::new(),
            default_color: "",
        };
    }

    let art_name = raw.strip_prefix("art:").unwrap_or(raw);
    if let Some(art_lines) = art::art(art_name) {
        let lines = art_lines.iter().map(|l| (*l).to_string()).collect();
        return LogoArt {
            lines,
            default_color: "\x1b[1;35m",
        };
    }

    if looks_like_path(raw) {
        let path = crate::config::expand_tilde(raw);
        if let Ok(content) = std::fs::read_to_string(&path) {
            let lines: Vec<String> = content.lines().map(|l| l.trim_end().to_string()).collect();
            return LogoArt {
                lines,
                default_color: "\x1b[1;37m",
            };
        }
    }

    let (lines, color) = logo_art(raw);
    LogoArt {
        lines: lines.iter().map(|l| (*l).to_string()).collect(),
        default_color: color,
    }
}

/// Colour the logo lines, honouring `no_color` and a theme colour override.
fn colorize_logo(art: &LogoArt, no_color: bool, override_color: Option<&str>) -> Vec<String> {
    if no_color {
        return art.lines.clone();
    }
    let color = override_color
        .filter(|c| *c != "auto")
        .and_then(parse_color)
        .unwrap_or_else(|| art.default_color.to_string());
    if color.is_empty() {
        return art.lines.clone();
    }
    art.lines
        .iter()
        .map(|l| {
            if l.trim().is_empty() {
                l.clone()
            } else {
                format!("{color}{l}\x1b[0m")
            }
        })
        .collect()
}

/// Render the module column (no logo) exactly as it will appear.
pub fn format_styled_module_lines(info: &SystemInfo, style: &RenderStyle) -> Vec<String> {
    let no_color = style.no_color;
    let reset = if no_color { "" } else { "\x1b[0m" };
    let default_key_color = style
        .key_color
        .as_deref()
        .filter(|c| *c != "auto")
        .and_then(parse_color)
        .unwrap_or_default();
    let title_col = style
        .title_color
        .as_deref()
        .filter(|c| *c != "auto")
        .and_then(parse_color)
        .unwrap_or_else(|| default_key_color.clone());
    let value_col = style
        .value_color
        .as_deref()
        .filter(|c| *c != "auto")
        .and_then(parse_color)
        .unwrap_or_default();
    let sep = style.separator.as_deref().unwrap_or(": ");
    let val_prefix = if no_color {
        sep.to_string()
    } else {
        format!("{reset}{sep}")
    };

    let modules = style.modules.clone().unwrap_or_else(default_modules);
    let title_plain = format!("{}@{}", info.user, info.hostname);

    let mut lines = Vec::new();
    for spec in &modules {
        let kind = spec.kind.as_str();

        if kind == "break" {
            lines.push(String::new());
            continue;
        }

        let key_color = spec
            .key_color
            .as_deref()
            .filter(|c| *c != "auto")
            .and_then(parse_color)
            .unwrap_or_else(|| default_key_color.clone());
        let key_col = if no_color { "" } else { key_color.as_str() };

        match kind {
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
            "separator" => lines.push("-".repeat(title_plain.len())),
            "colors" => {
                if !no_color {
                    lines.push(format!(
                        "\x1b[40m   \x1b[41m   \x1b[42m   \x1b[43m   \x1b[44m   \x1b[45m   \x1b[46m   \x1b[47m   {reset}"
                    ));
                    lines.push(format!(
                        "\x1b[100m   \x1b[101m   \x1b[102m   \x1b[103m   \x1b[104m   \x1b[105m   \x1b[106m   \x1b[107m   {reset}"
                    ));
                }
            }
            "custom" => {
                let text = spec.format.clone().unwrap_or_default();
                if !text.is_empty() {
                    lines.push(text);
                }
            }
            "disk" => {
                if let Some(ref disks) = info.disk {
                    for d in disks {
                        let (mut key, val) = if let Some(ref k) = spec.key {
                            (k.clone(), d.clone())
                        } else if let Some((mount, rest)) = d.split_once(':') {
                            (format!("Disk ({mount})"), rest.trim_start().to_string())
                        } else {
                            ("Disk".to_string(), d.clone())
                        };

                        if let Some(width) = spec.key_width {
                            let pad = width.saturating_sub(key.chars().count());
                            key.push_str(&" ".repeat(pad));
                        }

                        let val = match spec.format.as_deref() {
                            Some(tmpl) => apply_format(tmpl, &val),
                            None => val,
                        };
                        lines.push(format!("{key_col}{key}{val_prefix}{value_col}{val}{reset}"));
                    }
                }
            }
            _ => {
                let Some(val) = module_value(info, kind) else {
                    // If it's a bare string like "[ system ]" that isn't a known module,
                    // treat it as custom text.
                    if let Some(ref fmt) = spec.format {
                        lines.push(fmt.clone());
                    } else if spec.key.is_none() && !kind.is_empty() {
                        lines.push(kind.to_string());
                    }
                    continue;
                };

                let (mut key, val) = if is_annotated(kind) {
                    match val.split_once(':') {
                        Some((conn, rest)) => (
                            spec.key
                                .clone()
                                .unwrap_or_else(|| format!("{} ({})", default_key(kind), conn)),
                            rest.trim_start().to_string(),
                        ),
                        None => (
                            spec.key
                                .clone()
                                .unwrap_or_else(|| default_key(kind).to_string()),
                            val,
                        ),
                    }
                } else {
                    (
                        spec.key
                            .clone()
                            .unwrap_or_else(|| default_key(kind).to_string()),
                        val,
                    )
                };

                if let Some(width) = spec.key_width {
                    let pad = width.saturating_sub(key.chars().count());
                    key.push_str(&" ".repeat(pad));
                }

                let val = match spec.format.as_deref() {
                    Some(tmpl) => apply_format(tmpl, &val),
                    None => val,
                };
                lines.push(format!("{key_col}{key}{val_prefix}{value_col}{val}{reset}"));
            }
        }
    }

    lines
}

/// Legacy/convenience signature used by unit tests and CLI runners.
#[allow(dead_code)]
pub fn format_module_lines(
    info: &SystemInfo,
    no_color: bool,
    key_color: &str,
    title_color: Option<&str>,
    value_color: Option<&str>,
    separator: Option<&str>,
    structure: Option<&[String]>,
) -> Vec<String> {
    let style = RenderStyle {
        no_color,
        key_color: if key_color.is_empty() {
            None
        } else {
            Some(key_color.to_string())
        },
        title_color: title_color.map(|s| s.to_string()),
        value_color: value_color.map(|s| s.to_string()),
        separator: separator.map(|s| s.to_string()),
        modules: structure.map(|s| {
            s.iter()
                .map(|name| ModuleSpec {
                    kind: crate::config::normalize_module_name(name),
                    ..Default::default()
                })
                .collect()
        }),
        ..Default::default()
    };
    format_styled_module_lines(info, &style)
}

/// Render logo + module column as it will be printed.
pub fn render_lines(info: &SystemInfo, style: &RenderStyle) -> Vec<String> {
    let art = resolve_logo(style.logo.as_deref(), &info.distro_id);
    let show_logo = !style.no_logo && !art.is_empty();
    let module_lines = format_styled_module_lines(info, style);

    if !show_logo {
        return module_lines;
    }

    let logo_lines = colorize_logo(&art, style.no_color, style.logo_color.as_deref());
    let max_logo_width = logo_lines
        .iter()
        .map(|l| visible_width(l))
        .max()
        .unwrap_or(0);

    let mut top_pad = vec![String::new(); style.padding_top];
    top_pad.extend(logo_lines);

    let gap = " ".repeat(style.padding_right.max(3));
    let total = top_pad.len().max(module_lines.len());
    let mut out = Vec::with_capacity(total);
    for i in 0..total {
        let logo_part = match top_pad.get(i) {
            Some(line) => {
                let pad = max_logo_width.saturating_sub(visible_width(line));
                format!("{line}{}{gap}", " ".repeat(pad))
            }
            None => " ".repeat(max_logo_width + gap.len()),
        };
        match module_lines.get(i) {
            Some(line) => out.push(format!("{logo_part}{line}")),
            None => out.push(logo_part.trim_end().to_string()),
        }
    }
    out
}

pub fn print_fetch_styled(info: &SystemInfo, style: &RenderStyle) {
    for line in render_lines(info, style) {
        println!("{line}");
    }
}

#[allow(dead_code)]
pub fn print_fetch(info: &SystemInfo, opts: &PrintOptions) {
    if opts.json {
        if let Ok(serialized) = serde_json::to_string_pretty(info) {
            println!("{serialized}");
        }
        return;
    }

    let style = RenderStyle {
        no_color: opts.no_color,
        no_logo: opts.no_logo,
        logo: opts.logo_override.map(|s| s.to_string()),
        logo_color: opts.logo_color.map(|s| s.to_string()),
        key_color: opts.key_color.map(|s| s.to_string()),
        title_color: opts.title_color.map(|s| s.to_string()),
        value_color: opts.value_color.map(|s| s.to_string()),
        separator: opts.separator.map(|s| s.to_string()),
        modules: opts.structure.map(|s| {
            s.iter()
                .map(|name| ModuleSpec {
                    kind: crate::config::normalize_module_name(name),
                    ..Default::default()
                })
                .collect()
        }),
        ..Default::default()
    };

    print_fetch_styled(info, &style);
}
