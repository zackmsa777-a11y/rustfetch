mod art;
mod banner;
mod cli;
mod config;
mod info;
mod kitty;
mod logos;
mod presets;
mod printer;
mod tui;
mod utils;

use cli::{parse_cli, print_help, print_modules};
use config::{generate_default_config, load_config};
use info::gather_info;
use logos::ALL_LOGOS;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let cli_opts = match parse_cli(&args) {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(1);
        }
    };

    if cli_opts.help {
        print_help();
        return;
    }

    if cli_opts.version {
        println!("rustfetch {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if cli_opts.gen_config {
        println!("{}", generate_default_config());
        return;
    }

    if cli_opts.list_logos {
        println!("Supported distro and OS logos:");
        for logo in ALL_LOGOS {
            println!("  - {logo}");
        }
        return;
    }

    if cli_opts.list_modules {
        print_modules();
        return;
    }

    let mut file_cfg = load_config(cli_opts.config_path.as_deref());

    if cli_opts.list_themes {
        let (_, distro_id, distro_name) = crate::info::os::detect_os();
        let themes = config::all_themes_for_distro(&file_cfg, Some(&distro_id));
        if !distro_name.is_empty() {
            println!("Available rustfetch themes (detected: {distro_name}):");
        } else {
            println!("Available rustfetch themes:");
        }
        for entry in themes {
            let rec = if entry.matches_distro(&distro_id) {
                " \x1b[1;32m[recommended]\x1b[0m"
            } else {
                ""
            };
            println!("  - {}{rec}", tui::describe(&entry.name, &entry.def));
        }
        println!(
            "\nCustom themes: add .jsonc files to ~/.config/rustfetch/themes/ or define in config."
        );
        println!("Run `rustfetch --setup` to launch the interactive full-screen setup gallery.");
        return;
    }

    let cfg_logo_name = file_cfg.get_logo_name();
    let cfg_modules = file_cfg.get_normalized_modules();

    let effective_no_color = cli_opts.no_color || file_cfg.no_color.unwrap_or(false);
    let effective_no_logo = cli_opts.no_logo || file_cfg.no_logo.unwrap_or(false);
    let effective_logo = cli_opts
        .logo
        .as_deref()
        .or_else(|| cfg_logo_name.as_deref().filter(|s| *s != "auto"));
    let effective_structure = cli_opts.structure.as_ref().or(cfg_modules.as_ref());

    let effective_disks = cli_opts
        .disk_paths
        .as_ref()
        .or(file_cfg.disk_paths.as_ref());

    let system_info = gather_info(effective_disks.map(|v| v.as_slice()));

    if cli_opts.preview_themes {
        tui::preview_all(
            &system_info,
            &file_cfg,
            effective_no_color,
            effective_no_logo,
            effective_logo,
        );
        return;
    }

    if cli_opts.theme_picker {
        if let Err(e) = tui::run_picker(
            &system_info,
            &file_cfg,
            cli_opts.config_path.as_deref(),
            effective_no_logo,
            effective_logo,
        ) {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
        return;
    }

    // `--set-theme` persists first, then renders with the new theme below.
    if let Some(name) = cli_opts.set_theme.clone() {
        if config::lookup_theme(&file_cfg, &name).is_none() {
            eprintln!("error: unknown theme '{name}' (see --list-themes)");
            std::process::exit(1);
        }
        match config::save_theme_name(cli_opts.config_path.as_deref(), &name) {
            Ok(saved) => {
                eprintln!("Saved theme '{name}' to {}", saved.display());
                file_cfg = load_config(cli_opts.config_path.as_deref());
            }
            Err(e) => {
                eprintln!("error saving theme: {e}");
                std::process::exit(1);
            }
        }
    }

    if let Some(name) = cli_opts.theme.as_deref()
        && config::lookup_theme(&file_cfg, name).is_none()
    {
        eprintln!("error: unknown theme '{name}' (see --list-themes)");
        std::process::exit(1);
    }

    // Theme resolution order: --theme flag > config theme (+ custom themes
    // override built-in presets of the same name) > legacy display colors.
    let resolved_entry = config::resolve_active_theme(&file_cfg, cli_opts.theme.as_deref());
    let mut style = printer::style_from_theme(&resolved_entry.def);

    if effective_no_color {
        style.no_color = true;
    }
    if effective_no_logo {
        style.no_logo = true;
    }
    if let Some(logo) = effective_logo {
        style.logo = Some(logo.to_string());
    }
    let cfg_key_color = file_cfg.get_key_color();
    if let Some(ref kcol) = cfg_key_color
        && style.key_color.is_none()
    {
        style.key_color = Some(kcol.clone());
    }
    let cfg_logo_color = file_cfg.get_logo_color();
    let effective_logo_color = cli_opts
        .logo_color
        .as_deref()
        .or(style.logo_color.as_deref())
        .or_else(|| cfg_logo_color.as_deref().filter(|s| *s != "auto"));
    if let Some(lcol) = effective_logo_color {
        style.logo_color = Some(lcol.to_string());
    }

    let cfg_logo_type = file_cfg.get_logo_type();
    let effective_logo_type = cli_opts
        .logo_type
        .clone()
        .or(style.logo_type.clone())
        .or(cfg_logo_type);
    if let Some(t) = effective_logo_type {
        style.logo_type = Some(t);
    }

    let cfg_logo_width = file_cfg.get_logo_width();
    let effective_logo_width = cli_opts.logo_width.or(style.logo_width).or(cfg_logo_width);
    if let Some(w) = effective_logo_width {
        style.logo_width = Some(w);
    }

    let cfg_logo_height = file_cfg.get_logo_height();
    let effective_logo_height = cli_opts
        .logo_height
        .or(style.logo_height)
        .or(cfg_logo_height);
    if let Some(h) = effective_logo_height {
        style.logo_height = Some(h);
    }

    if let Some(p) = cli_opts.logo_padding_top {
        style.padding_top = p;
    }
    if let Some(p) = cli_opts.logo_padding_left {
        style.padding_left = p;
    }
    if let Some(p) = cli_opts.logo_padding_right {
        style.padding_right = p;
    } else if let Some(cfg_pad) = file_cfg.get_logo_padding() {
        if cli_opts.logo_padding_top.is_none() && cfg_pad.top > 0 {
            style.padding_top = cfg_pad.top;
        }
        if cli_opts.logo_padding_left.is_none() && cfg_pad.left > 0 {
            style.padding_left = cfg_pad.left;
        }
        if cli_opts.logo_padding_right.is_none() && cfg_pad.right > 0 {
            style.padding_right = cfg_pad.right;
        }
    }
    if let Some(structure) = effective_structure {
        style.modules = Some(
            structure
                .iter()
                .map(|name| config::ModuleSpec {
                    kind: config::normalize_module_name(name),
                    ..Default::default()
                })
                .collect(),
        );
    }
    if style.separator.is_none()
        && let Some(d) = file_cfg.display.as_ref()
        && let Some(ref s) = d.separator
    {
        style.separator = Some(s.clone());
    }

    if cli_opts.json {
        if let Ok(serialized) = serde_json::to_string_pretty(&system_info) {
            println!("{serialized}");
        }
    } else {
        printer::print_fetch_styled(&system_info, &style);
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::parse_cli;
    use crate::config::{Config, parse_color, resolve_theme, strip_jsonc_comments};
    use crate::info::cpu::format_cpu;
    use crate::info::memory::format_memory;
    use crate::info::swap::format_swap;
    use crate::info::uptime::{format_uptime, parse_uptime_str};
    use crate::logos::get_logo;
    use crate::utils::{clean, strip_ansi, value, visible_width};

    #[test]
    fn parses_cli_flags_correctly() {
        let args = vec![
            "--no-color".to_string(),
            "--no-logo".to_string(),
            "--logo".to_string(),
            "arch".to_string(),
            "--structure".to_string(),
            "title,os,kernel".to_string(),
        ];
        let opts = parse_cli(&args).unwrap();
        assert!(opts.no_color);
        assert!(opts.no_logo);
        assert_eq!(opts.logo.as_deref(), Some("arch"));
        assert_eq!(
            opts.structure.unwrap(),
            vec!["title".to_string(), "os".to_string(), "kernel".to_string()]
        );
    }

    #[test]
    fn parses_kitty_image_flags() {
        let args = vec![
            "--kitty".to_string(),
            "/path/to/logo.png".to_string(),
            "--logo-width".to_string(),
            "35".to_string(),
            "--logo-height".to_string(),
            "18".to_string(),
            "--logo-padding-left".to_string(),
            "2".to_string(),
        ];
        let opts = parse_cli(&args).unwrap();
        assert_eq!(opts.logo.as_deref(), Some("/path/to/logo.png"));
        assert_eq!(opts.logo_type.as_deref(), Some("kitty"));
        assert_eq!(opts.logo_width, Some(35));
        assert_eq!(opts.logo_height, Some(18));
        assert_eq!(opts.logo_padding_left, Some(2));
    }

    #[test]
    fn rejects_unknown_cli_arguments() {
        let args = vec!["--unknown-flag".to_string()];
        assert!(parse_cli(&args).is_err());
    }

    #[test]
    fn sanitizes_and_strips_ansi() {
        let raw = "\x1b[1;31mRust\x1b[0m\tFast";
        assert_eq!(strip_ansi(raw), "Rust\tFast");
        assert_eq!(visible_width(raw), 9);
        assert_eq!(clean("Clean\u{1b}[32mData\n"), "Clean[32mData");
    }

    #[test]
    fn parses_delimiter_separated_values() {
        let content = "NAME=RustOS\nVERSION=\"1.0.0\"\nEMPTY=\n";
        assert_eq!(value(content, "NAME", '='), Some("RustOS"));
        assert_eq!(value(content, "VERSION", '='), Some("1.0.0"));
        assert_eq!(value(content, "EMPTY", '='), Some(""));
        assert_eq!(value(content, "MISSING", '='), None);
    }

    #[test]
    fn formats_uptime_properly() {
        assert_eq!(format_uptime(90061), "1 day, 1 hour, 1 min");
        assert_eq!(format_uptime(3660), "1 hour, 1 min");
        assert_eq!(format_uptime(45), "45 secs");
        assert_eq!(
            parse_uptime_str("12345.67 89012.34"),
            Some("3 hours, 25 mins".into())
        );
        assert_eq!(parse_uptime_str("not-a-number"), None);
    }

    #[test]
    fn calculates_memory_usage() {
        let mem = "MemTotal: 16777216 kB\nMemAvailable: 8388608 kB\n";
        assert_eq!(
            format_memory(mem),
            Some("8.00 GiB / 16.00 GiB (50%)".into())
        );
        assert_eq!(format_memory("MemTotal: 0 kB\nMemAvailable: 0 kB"), None);
        assert_eq!(format_memory("invalid"), None);
    }

    #[test]
    fn calculates_swap_usage() {
        let disabled = "SwapTotal: 0 kB\nSwapFree: 0 kB\n";
        assert_eq!(format_swap(disabled), Some("Disabled".into()));
        let enabled = "SwapTotal: 4194304 kB\nSwapFree: 2097152 kB\n";
        assert_eq!(
            format_swap(enabled),
            Some("2.00 GiB / 4.00 GiB (50%)".into())
        );
    }

    #[test]
    fn formats_cpu_string_accurately() {
        let cpuinfo = "processor\t: 0\nmodel name\t: AMD Ryzen 9 5900X 12-Core Processor\ncpu MHz\t: 3700.0\nprocessor\t: 1\n";
        assert_eq!(
            format_cpu(cpuinfo),
            Some("AMD Ryzen 9 5900X 12-Core Processor (2) @ 3.70 GHz".into())
        );
    }

    #[test]
    fn loads_and_resolves_logos() {
        let (arch_lines, col) = get_logo("arch", false, None);
        assert!(!arch_lines.is_empty());
        assert_eq!(col, "\x1b[1;36m");

        let (ubuntu_lines, _) = get_logo("ubuntu", true, None);
        assert!(!ubuntu_lines.is_empty());
        assert!(!ubuntu_lines[0].contains("\x1b["));

        let (rust_lines, _) = get_logo("rust", false, Some("green"));
        assert!(!rust_lines.is_empty());
        assert!(rust_lines[0].contains("\x1b[1;32m"));
    }

    #[test]
    fn resolves_theme_presets_and_overrides() {
        let neon = resolve_theme("neon").unwrap();
        assert_eq!(neon.keys.as_deref(), Some("cyan"));
        assert_eq!(neon.title.as_deref(), Some("magenta"));
        assert_eq!(neon.value.as_deref(), Some("white"));

        let default = resolve_theme("default").unwrap();
        assert!(default.keys.is_none() && default.title.is_none());

        assert!(resolve_theme("nonexistent").is_none());
        assert_eq!(
            resolve_theme("Dracula").unwrap().keys.as_deref(),
            Some("141")
        );
    }

    #[test]
    fn parses_color_formats() {
        assert_eq!(parse_color("cyan"), Some("\x1b[1;36m".to_string()));
        assert_eq!(parse_color("208"), Some("\x1b[38;5;208m".to_string()));
        assert_eq!(
            parse_color("#7aa2f7"),
            Some("\x1b[38;2;122;162;247m".to_string())
        );
        assert_eq!(
            parse_color("7aa2f7"),
            Some("\x1b[38;2;122;162;247m".to_string())
        );
        assert_eq!(parse_color("auto"), None);
        assert_eq!(parse_color("999"), None);
    }

    #[test]
    fn parses_theme_config_with_overrides() {
        let json_str = r##"{
            "theme": {
                "name": "nord",
                "title": "#bf616a",
                "separator": " -> "
            }
        }"##;
        let cfg: Config = serde_json::from_str(json_str).unwrap();
        let theme = cfg.theme.unwrap();
        assert_eq!(theme.name(), Some("nord"));
        let over = theme.overrides();
        assert_eq!(over.title.as_deref(), Some("#bf616a"));
        assert_eq!(over.separator.as_deref(), Some(" -> "));
        assert!(over.keys.is_none());

        let short: Config = serde_json::from_str(r#"{"theme": "gruvbox"}"#).unwrap();
        assert_eq!(short.theme.as_ref().unwrap().name(), Some("gruvbox"));

        let custom: Config = serde_json::from_str(
            r#"{"themes": {"my-theme": {"keys": "cyan", "logo_color": "red"}}}"#,
        )
        .unwrap();
        let found = crate::config::lookup_theme(&custom, "my-theme").unwrap();
        assert_eq!(found.keys.as_deref(), Some("cyan"));
        assert_eq!(found.logo_color.as_deref(), Some("red"));
        assert!(crate::config::all_theme_names(&custom).contains(&"my-theme".to_string()));
    }

    #[test]
    fn strips_jsonc_syntax() {
        let jsonc = "{\n\"key\": \"value\"\n}\n";
        let cleaned = strip_jsonc_comments(jsonc);
        let parsed: serde_json::Value = serde_json::from_str(&cleaned).unwrap();
        assert_eq!(parsed["key"], "value");
    }

    #[test]
    fn theme_preview_lines_use_theme_colors() {
        use crate::config::parse_color;
        use crate::info::types::SystemInfo;
        use crate::printer::format_module_lines;
        let info = SystemInfo {
            user: "user".into(),
            hostname: "host".into(),
            os: Some("TestOS 1.0 x86_64".into()),
            ..Default::default()
        };
        let cfg: Config =
            serde_json::from_str(r#"{"themes": {"testy": {"keys": "cyan", "separator": " => "}}}"#)
                .unwrap();
        let colors = crate::config::lookup_theme(&cfg, "testy").unwrap();
        let key_ansi = colors.keys.as_deref().and_then(parse_color).unwrap();
        let structure = vec!["os".to_string()];
        let lines = format_module_lines(
            &info,
            false,
            &key_ansi,
            None,
            colors.value.as_deref(),
            colors.separator.as_deref(),
            Some(&structure),
        );
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("OS"));
        assert!(lines[0].contains("=>"));
        assert!(lines[0].contains("\x1b[1;36m"));
    }

    #[test]
    fn parses_fastfetch_config_structures() {
        let json_str = r#"{
            "logo": {
                "source": "oracle",
                "color": { "1": "red" }
            },
            "display": {
                "color": { "keys": "green" }
            },
            "modules": [
                "title",
                { "type": "os", "key": "Operating System" },
                { "type": "wmtheme", "key": "Theme" },
                "colors"
            ]
        }"#;
        let cfg: Config = serde_json::from_str(json_str).unwrap();
        assert_eq!(cfg.get_logo_name().as_deref(), Some("oracle"));
        assert_eq!(cfg.get_logo_color().as_deref(), Some("red"));
        assert_eq!(cfg.get_key_color().as_deref(), Some("green"));
        let mods = cfg.get_normalized_modules().unwrap();
        assert_eq!(mods, vec!["title", "os", "wm_theme", "colors"]);
    }

    #[test]
    fn resolves_extended_distro_logos() {
        let (bedrock, col) = get_logo("bedrock", false, None);
        assert!(!bedrock.is_empty());
        assert_eq!(col, "\x1b[1;37m");

        let (oracle, _) = get_logo("oracle", false, None);
        assert!(!oracle.is_empty());

        let (rocky, _) = get_logo("rocky", false, None);
        assert!(!rocky.is_empty());

        let (almalinux, _) = get_logo("almalinux", false, None);
        assert!(!almalinux.is_empty());

        let (centos, _) = get_logo("centos", false, None);
        assert!(!centos.is_empty());

        let (rpi, _) = get_logo("raspberry", false, None);
        assert!(!rpi.is_empty());

        let (freebsd, _) = get_logo("freebsd", false, None);
        assert!(!freebsd.is_empty());

        let (macos, _) = get_logo("macos", false, None);
        assert!(!macos.is_empty());

        let (windows, _) = get_logo("windows", false, None);
        assert!(!windows.is_empty());
    }

    #[test]
    fn resolves_custom_art_pieces() {
        use crate::art;
        assert!(art::art("ferris").is_some());
        assert!(art::art("crab").is_some());
        assert!(art::art("tux").is_some());
        assert!(art::art("penguin").is_some());
        assert!(art::art("ubuntu-mini").is_some());
        assert!(art::art("coffee").is_some());
        assert!(art::art("heart").is_some());
        assert!(art::art("ghost").is_some());
        assert!(art::art("arch-mini").is_some());
    }

    #[test]
    fn distro_theme_prioritization() {
        use crate::config;
        let cfg = Config::default();

        // On Ubuntu: Ubuntu themes must appear at the top
        let ubuntu_themes = config::all_themes_for_distro(&cfg, Some("ubuntu"));
        assert!(ubuntu_themes.len() > 5);
        assert!(ubuntu_themes[0].matches_distro("ubuntu"));
        assert!(ubuntu_themes[1].matches_distro("ubuntu"));
        assert!(ubuntu_themes[2].matches_distro("ubuntu"));
        assert!(ubuntu_themes[0].name.starts_with("ubuntu-"));

        // On Arch: Arch themes must appear at the top
        let arch_themes = config::all_themes_for_distro(&cfg, Some("arch"));
        assert!(arch_themes[0].matches_distro("arch"));
        assert_eq!(arch_themes[0].name, "arch-clean");

        // On Debian: Debian themes must appear at the top
        let debian_themes = config::all_themes_for_distro(&cfg, Some("debian"));
        assert!(debian_themes[0].matches_distro("debian"));
        assert_eq!(debian_themes[0].name, "debian-swirl");

        // On Bedrock: Bedrock themes must appear at the top
        let bedrock_themes = config::all_themes_for_distro(&cfg, Some("bedrock"));
        assert!(bedrock_themes[0].matches_distro("bedrock"));
        assert_eq!(bedrock_themes[0].name, "bedrock-strata");

        // On Gentoo: Gentoo themes must appear at the top
        let gentoo_themes = config::all_themes_for_distro(&cfg, Some("gentoo"));
        assert!(gentoo_themes[0].matches_distro("gentoo"));
        assert_eq!(gentoo_themes[0].name, "gentoo-purple");

        // On CachyOS: CachyOS themes must appear at the top
        let cachyos_themes = config::all_themes_for_distro(&cfg, Some("cachyos"));
        assert!(cachyos_themes[0].matches_distro("cachyos"));
        assert_eq!(cachyos_themes[0].name, "cachyos-speed");

        // On Linux Mint: Mint themes must appear at the top
        let mint_themes = config::all_themes_for_distro(&cfg, Some("mint"));
        assert!(mint_themes[0].matches_distro("mint"));
        assert_eq!(mint_themes[0].name, "mint-fresh");

        // On openSUSE: openSUSE themes must appear at the top
        let opensuse_themes = config::all_themes_for_distro(&cfg, Some("opensuse"));
        assert!(opensuse_themes[0].matches_distro("opensuse"));
        assert_eq!(opensuse_themes[0].name, "opensuse-geek");

        // On Pop!_OS: Pop themes must appear at the top
        let pop_themes = config::all_themes_for_distro(&cfg, Some("pop"));
        assert!(pop_themes[0].matches_distro("pop"));
        assert_eq!(pop_themes[0].name, "pop-cosmic");

        // On Void: Void themes must appear at the top
        let void_themes = config::all_themes_for_distro(&cfg, Some("void"));
        assert!(void_themes[0].matches_distro("void"));
        assert_eq!(void_themes[0].name, "void-xbps");

        // On Alpine: Alpine themes must appear at the top
        let alpine_themes = config::all_themes_for_distro(&cfg, Some("alpine"));
        assert!(alpine_themes[0].matches_distro("alpine"));
        assert_eq!(alpine_themes[0].name, "alpine-peak");

        // On Manjaro: Manjaro themes must appear at the top
        let manjaro_themes = config::all_themes_for_distro(&cfg, Some("manjaro"));
        assert!(manjaro_themes[0].matches_distro("manjaro"));
        assert_eq!(manjaro_themes[0].name, "manjaro-teal");

        // On Kali: Kali themes must appear at the top
        let kali_themes = config::all_themes_for_distro(&cfg, Some("kali"));
        assert!(kali_themes[0].matches_distro("kali"));
        assert_eq!(kali_themes[0].name, "kali-dragon");

        // On EndeavourOS: EndeavourOS themes must appear at the top
        let endeavour_themes = config::all_themes_for_distro(&cfg, Some("endeavouros"));
        assert!(endeavour_themes[0].matches_distro("endeavouros"));
        assert_eq!(endeavour_themes[0].name, "endeavour-space");

        // On Red Hat: Red Hat themes must appear at the top
        let redhat_themes = config::all_themes_for_distro(&cfg, Some("redhat"));
        assert!(redhat_themes[0].matches_distro("redhat"));
        assert_eq!(redhat_themes[0].name, "redhat-shadow");
    }
}
