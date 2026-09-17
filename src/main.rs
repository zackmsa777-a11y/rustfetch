mod cli;
mod config;
mod info;
mod logos;
mod printer;
mod utils;

use cli::{parse_cli, print_help, print_modules};
use config::{generate_default_config, load_config};
use info::gather_info;
use logos::ALL_LOGOS;
use printer::{print_fetch, PrintOptions};
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

    let file_cfg = load_config(cli_opts.config_path.as_deref());

    let cfg_logo_name = file_cfg.get_logo_name();
    let cfg_logo_color = file_cfg.get_logo_color();
    let cfg_modules = file_cfg.get_normalized_modules();

    let effective_no_color = cli_opts.no_color || file_cfg.no_color.unwrap_or(false);
    let effective_no_logo = cli_opts.no_logo || file_cfg.no_logo.unwrap_or(false);
    let effective_logo = cli_opts
        .logo
        .as_deref()
        .or_else(|| cfg_logo_name.as_deref().filter(|s| *s != "auto"));
    let effective_logo_color = cli_opts
        .logo_color
        .as_deref()
        .or_else(|| cfg_logo_color.as_deref().filter(|s| *s != "auto"));
    let effective_structure = cli_opts.structure.as_ref().or(cfg_modules.as_ref());

    let effective_disks = cli_opts
        .disk_paths
        .as_ref()
        .or(file_cfg.disk_paths.as_ref());

    let system_info = gather_info(effective_disks.map(|v| v.as_slice()));

    let cfg_key_color = file_cfg.get_key_color();

    let print_opts = PrintOptions {
        no_color: effective_no_color,
        no_logo: effective_no_logo,
        logo_override: effective_logo,
        logo_color: effective_logo_color,
        key_color: cfg_key_color.as_deref(),
        structure: effective_structure.map(|v| v.as_slice()),
        json: cli_opts.json,
    };

    print_fetch(&system_info, &print_opts);
}

#[cfg(test)]
mod tests {
    use crate::cli::parse_cli;
    use crate::config::{strip_jsonc_comments, Config};
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
    fn strips_jsonc_syntax() {
        let jsonc = "{\n\"key\": \"value\"\n}\n";
        let cleaned = strip_jsonc_comments(jsonc);
        let parsed: serde_json::Value = serde_json::from_str(&cleaned).unwrap();
        assert_eq!(parsed["key"], "value");
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
}
