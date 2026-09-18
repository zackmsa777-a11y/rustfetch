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
    pub logo_type: Option<String>,
    pub logo_width: Option<usize>,
    pub logo_height: Option<usize>,
    pub padding_top: usize,
    pub padding_left: usize,
    pub padding_right: usize,
    pub key_color: Option<String>,
    pub title_color: Option<String>,
    pub value_color: Option<String>,
    pub separator: Option<String>,
    /// Explicit layout. `None` renders the default module order.
    pub modules: Option<Vec<ModuleSpec>>,
    /// When true, print info modules even if their value is empty.
    /// Default false: skip empty/None/whitespace-only info values.
    pub show_empty: bool,
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
        style.logo_type = logo.logo_type().map(|s| s.to_string());
        style.logo_width = logo.width();
        style.logo_height = logo.height();
        if logo.is_hidden() {
            style.no_logo = true;
        }
        let padding = logo.padding();
        style.padding_top = padding.top;
        style.padding_left = padding.left;
        style.padding_right = padding.right;
    }
    if let Some(padding) = def.padding {
        style.padding_top = padding.top;
        style.padding_left = padding.left;
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
    pub show_empty: bool,
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
        "command" => "Command",
        "custom" => "Custom",
        _ => "",
    }
}

/// True for structural layout modules that are not keyed info values.
#[allow(dead_code)]
pub fn is_structural_module(kind: &str) -> bool {
    matches!(kind, "title" | "separator" | "break" | "colors" | "custom")
}

/// True for modules that resolve a system info value (may be empty).
pub fn is_info_module(kind: &str) -> bool {
    matches!(
        kind,
        "os" | "host"
            | "kernel"
            | "uptime"
            | "packages"
            | "shell"
            | "display"
            | "de"
            | "wm"
            | "wm_theme"
            | "theme"
            | "icons"
            | "font"
            | "cursor"
            | "terminal"
            | "terminal_font"
            | "cpu"
            | "gpu"
            | "memory"
            | "swap"
            | "disk"
            | "battery"
            | "power_adapter"
            | "audio"
            | "local_ip"
            | "public_ip"
            | "locale"
            | "command"
    )
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

fn resolve_dynamic_values(modules: &[ModuleSpec]) -> Vec<Option<String>> {
    let mut out: Vec<Option<String>> = modules
        .iter()
        .map(|spec| {
            if spec.kind == "custom" && spec.key.is_some() {
                spec.text
                    .clone()
                    .or_else(|| spec.format.clone())
                    .filter(|s| !s.is_empty())
            } else {
                None
            }
        })
        .collect();

    std::thread::scope(|s| {
        let mut handles = Vec::new();
        for (i, spec) in modules.iter().enumerate() {
            if spec.kind != "command" {
                continue;
            }
            let cmdline = spec.command.clone().or_else(|| spec.text.clone());
            let shell = spec.shell;
            let timeout_ms = spec.timeout_ms;
            let show_failure = spec.show_failure;
            handles.push((
                i,
                s.spawn(move || {
                    crate::info::command::run_module_command(
                        cmdline.as_deref(),
                        shell,
                        timeout_ms,
                        show_failure,
                    )
                }),
            ));
        }
        for (i, handle) in handles {
            out[i] = handle.join().ok().flatten();
        }
    });

    out
}

#[allow(clippy::too_many_arguments)]
fn push_keyed_line(
    lines: &mut Vec<String>,
    spec: &ModuleSpec,
    kind: &str,
    value: &str,
    key_col: &str,
    val_prefix: &str,
    value_col: &str,
    reset: &str,
) {
    let mut key = spec
        .key
        .clone()
        .unwrap_or_else(|| default_key(kind).to_string());
    if key.is_empty() && (kind == "command" || kind == "custom") {
        key = "Command".to_string();
    }
    if let Some(width) = spec.key_width {
        let pad = width.saturating_sub(key.chars().count());
        key.push_str(&" ".repeat(pad));
    }
    let val = match spec.format.as_deref() {
        Some(tmpl) if tmpl.contains("{}") || tmpl.contains("{1}") => apply_format(tmpl, value),
        Some(_) => value.to_string(),
        None => value.to_string(),
    };
    lines.push(format!("{key_col}{key}{val_prefix}{value_col}{val}{reset}"));
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
    let dynamic = resolve_dynamic_values(&modules);
    for (idx, spec) in modules.iter().enumerate() {
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
                if spec.key.is_some() {
                    match &dynamic[idx] {
                        Some(val) => push_keyed_line(
                            &mut lines,
                            spec,
                            kind,
                            val,
                            key_col,
                            &val_prefix,
                            &value_col,
                            reset,
                        ),
                        None if style.show_empty => push_keyed_line(
                            &mut lines,
                            spec,
                            kind,
                            "",
                            key_col,
                            &val_prefix,
                            &value_col,
                            reset,
                        ),
                        None => {}
                    }
                } else {
                    let text = spec
                        .format
                        .clone()
                        .or_else(|| spec.text.clone())
                        .unwrap_or_default();
                    if !text.is_empty() {
                        lines.push(text);
                    }
                }
            }
            "command" => match &dynamic[idx] {
                Some(val) => push_keyed_line(
                    &mut lines,
                    spec,
                    kind,
                    val,
                    key_col,
                    &val_prefix,
                    &value_col,
                    reset,
                ),
                None if style.show_empty => push_keyed_line(
                    &mut lines,
                    spec,
                    kind,
                    "",
                    key_col,
                    &val_prefix,
                    &value_col,
                    reset,
                ),
                None => {}
            },
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
                } else if style.show_empty {
                    let mut key = spec
                        .key
                        .clone()
                        .unwrap_or_else(|| default_key("disk").to_string());
                    if let Some(width) = spec.key_width {
                        let pad = width.saturating_sub(key.chars().count());
                        key.push_str(&" ".repeat(pad));
                    }
                    lines.push(format!("{key_col}{key}{val_prefix}{value_col}{reset}"));
                }
            }
            _ => {
                let Some(val) = module_value(info, kind) else {
                    if is_info_module(kind) {
                        if style.show_empty {
                            let mut key = spec
                                .key
                                .clone()
                                .unwrap_or_else(|| default_key(kind).to_string());
                            if let Some(width) = spec.key_width {
                                let pad = width.saturating_sub(key.chars().count());
                                key.push_str(&" ".repeat(pad));
                            }
                            lines.push(format!("{key_col}{key}{val_prefix}{value_col}{reset}"));
                        }
                        continue;
                    }
                    // Unknown kind: treat as custom text / section label.
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
    let show_logo = !style.no_logo;
    let module_lines = format_styled_module_lines(info, style);

    if !show_logo {
        return module_lines;
    }

    let logo_str = style.logo.as_deref().unwrap_or("");
    let is_img = crate::kitty::is_image_path(logo_str);
    let explicit_image = matches!(
        style.logo_type.as_deref(),
        Some("kitty")
            | Some("kitty-direct")
            | Some("kitty-icat")
            | Some("sixel")
            | Some("iterm")
            | Some("iterm2")
    );
    let auto_or_unset = matches!(style.logo_type.as_deref(), None | Some("auto"));
    let protocol = if explicit_image {
        crate::kitty::resolve_logo_protocol(style.logo_type.as_deref())
    } else if is_img && auto_or_unset {
        crate::kitty::detect_image_protocol()
    } else {
        None
    };

    let maybe_img = if !style.no_color && protocol.is_some() && (explicit_image || is_img) {
        crate::kitty::resolve_image_path(logo_str)
    } else {
        None
    };

    if let (Some(img_path), Some(proto)) = (maybe_img, protocol) {
        let img_opts = crate::kitty::KittyImageOptions {
            req_w: style.logo_width,
            req_h: style.logo_height,
            padding_top: style.padding_top,
            padding_left: style.padding_left,
            padding_right: style.padding_right,
            direct: proto == "kitty-direct",
            icat: proto == "kitty-icat",
        };
        if let Ok(lines) =
            crate::kitty::render_image_logo_lines(&img_path, proto, &img_opts, &module_lines)
        {
            return lines;
        }
    }

    let art = resolve_logo(style.logo.as_deref(), &info.distro_id);
    if art.is_empty() {
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
    let left_pad = " ".repeat(style.padding_left);
    let total = top_pad.len().max(module_lines.len());
    let mut out = Vec::with_capacity(total);
    for i in 0..total {
        let logo_part = match top_pad.get(i) {
            Some(line) => {
                let pad = max_logo_width.saturating_sub(visible_width(line));
                format!("{left_pad}{line}{}{gap}", " ".repeat(pad))
            }
            None => format!("{left_pad}{}", " ".repeat(max_logo_width + gap.len())),
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
        show_empty: opts.show_empty,
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

#[cfg(test)]
mod show_empty_tests {
    use super::*;
    use crate::info::types::SystemInfo;

    fn sample_info() -> SystemInfo {
        SystemInfo {
            user: "user".into(),
            hostname: "host".into(),
            os: Some("TestOS 1.0".into()),
            shell: None,
            de: Some("   ".into()),
            gpu: None,
            ..Default::default()
        }
    }

    fn style_with(modules: &[&str], show_empty: bool) -> RenderStyle {
        RenderStyle {
            no_color: true,
            no_logo: true,
            show_empty,
            modules: Some(
                modules
                    .iter()
                    .map(|k| ModuleSpec {
                        kind: (*k).to_string(),
                        ..Default::default()
                    })
                    .collect(),
            ),
            ..Default::default()
        }
    }

    #[test]
    fn skips_empty_info_modules_by_default() {
        let info = sample_info();
        let style = style_with(
            &[
                "title",
                "separator",
                "os",
                "shell",
                "de",
                "gpu",
                "break",
                "colors",
            ],
            false,
        );
        let lines = format_styled_module_lines(&info, &style);
        // title + separator + os + break (colors skipped under no_color)
        assert!(
            lines.iter().any(|l| l.contains("OS")),
            "expected OS line: {lines:?}"
        );
        assert!(
            !lines.iter().any(|l| l == "shell" || l.starts_with("Shell")),
            "shell must be hidden: {lines:?}"
        );
        assert!(
            !lines.iter().any(|l| l == "de" || l.starts_with("DE")),
            "whitespace-only DE must be hidden: {lines:?}"
        );
        assert!(
            !lines.iter().any(|l| l == "gpu" || l.starts_with("GPU")),
            "gpu must be hidden: {lines:?}"
        );
        assert_eq!(lines[0], "user@host");
        assert_eq!(lines[1], "-".repeat("user@host".len()));
        assert!(
            lines.iter().any(|l| l.is_empty()),
            "break must remain: {lines:?}"
        );
    }

    #[test]
    fn show_empty_prints_blank_info_keys() {
        let info = sample_info();
        let style = style_with(&["os", "shell", "de", "gpu"], true);
        let lines = format_styled_module_lines(&info, &style);
        assert_eq!(lines.len(), 4, "{lines:?}");
        assert!(lines[0].starts_with("OS:"));
        assert!(lines[0].contains("TestOS"));
        assert_eq!(lines[1], "Shell: ");
        assert_eq!(lines[2], "DE: ");
        assert_eq!(lines[3], "GPU: ");
    }

    #[test]
    fn structural_modules_not_dropped() {
        let info = sample_info();
        let style = RenderStyle {
            no_color: false,
            no_logo: true,
            show_empty: false,
            modules: Some(
                ["title", "separator", "shell", "break", "colors"]
                    .iter()
                    .map(|k| ModuleSpec {
                        kind: (*k).to_string(),
                        ..Default::default()
                    })
                    .collect(),
            ),
            ..Default::default()
        };
        let lines = format_styled_module_lines(&info, &style);
        assert!(
            lines
                .iter()
                .any(|l| l.contains("user") && l.contains("host"))
        );
        assert!(lines.iter().any(|l| l.chars().all(|c| c == '-')));
        assert!(lines.iter().any(|l| l.is_empty()), "break kept: {lines:?}");
        assert!(
            lines.iter().any(|l| l.contains("[40m")),
            "colors kept: {lines:?}"
        );
        assert!(
            !lines.iter().any(|l| l == "shell" || l.contains("Shell")),
            "empty shell hidden: {lines:?}"
        );
    }

    #[test]
    fn command_module_renders_echo() {
        let info = sample_info();
        let style = RenderStyle {
            no_color: true,
            no_logo: true,
            show_empty: false,
            modules: Some(vec![
                ModuleSpec {
                    kind: "os".into(),
                    ..Default::default()
                },
                ModuleSpec {
                    kind: "command".into(),
                    key: Some("Echo".into()),
                    command: Some("echo hello".into()),
                    ..Default::default()
                },
                ModuleSpec {
                    kind: "custom".into(),
                    key: Some("Git".into()),
                    text: Some("zackmsa777-a11y".into()),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };
        let lines = format_styled_module_lines(&info, &style);
        assert!(lines.iter().any(|l| l == "Echo: hello"), "{lines:?}");
        assert!(
            lines.iter().any(|l| l == "Git: zackmsa777-a11y"),
            "{lines:?}"
        );
    }

    #[test]
    fn command_failure_hides_when_empty() {
        let info = sample_info();
        let style = RenderStyle {
            no_color: true,
            no_logo: true,
            show_empty: false,
            modules: Some(vec![ModuleSpec {
                kind: "command".into(),
                key: Some("Fail".into()),
                command: Some("false".into()),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let lines = format_styled_module_lines(&info, &style);
        assert!(lines.is_empty(), "{lines:?}");
    }

    #[test]
    fn command_timeout_hides_when_empty() {
        let info = sample_info();
        let style = RenderStyle {
            no_color: true,
            no_logo: true,
            show_empty: false,
            modules: Some(vec![ModuleSpec {
                kind: "command".into(),
                key: Some("Slow".into()),
                command: Some("sleep 5".into()),
                timeout_ms: 100,
                ..Default::default()
            }]),
            ..Default::default()
        };
        let lines = format_styled_module_lines(&info, &style);
        assert!(lines.is_empty(), "{lines:?}");
    }

    #[test]
    fn legacy_custom_format_still_structural() {
        let info = sample_info();
        let style = RenderStyle {
            no_color: true,
            no_logo: true,
            show_empty: false,
            modules: Some(vec![ModuleSpec {
                kind: "custom".into(),
                format: Some("~~~ section ~~~".into()),
                ..Default::default()
            }]),
            ..Default::default()
        };
        let lines = format_styled_module_lines(&info, &style);
        assert_eq!(lines, vec!["~~~ section ~~~".to_string()]);
    }
}
