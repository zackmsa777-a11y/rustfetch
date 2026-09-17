use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct CliOptions {
    pub logo: Option<String>,
    pub logo_color: Option<String>,
    pub no_logo: bool,
    pub no_color: bool,
    pub structure: Option<Vec<String>>,
    pub disk_paths: Option<Vec<String>>,
    pub config_path: Option<PathBuf>,
    pub gen_config: bool,
    pub list_logos: bool,
    pub list_modules: bool,
    pub list_themes: bool,
    pub theme: Option<String>,
    pub set_theme: Option<String>,
    pub preview_themes: bool,
    pub theme_picker: bool,
    pub json: bool,
    pub help: bool,
    pub version: bool,
}

pub fn parse_cli(args: &[String]) -> Result<CliOptions, String> {
    let mut opts = CliOptions::default();
    let mut iter = args.iter().peekable();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--no-color" => opts.no_color = true,
            "--no-logo" => opts.no_logo = true,
            "--json" => opts.json = true,
            "--gen-config" => opts.gen_config = true,
            "--list-logos" => opts.list_logos = true,
            "--list-modules" => opts.list_modules = true,
            "--list-themes" => opts.list_themes = true,
            "--preview-themes" => opts.preview_themes = true,
            "--themes" | "--theme-picker" | "--tui" => opts.theme_picker = true,
            "--theme" => {
                if let Some(val) = iter.next() {
                    opts.theme = Some(val.clone());
                } else {
                    return Err("--theme requires a theme name".into());
                }
            }
            "--set-theme" => {
                if let Some(val) = iter.next() {
                    opts.set_theme = Some(val.clone());
                } else {
                    return Err("--set-theme requires a theme name".into());
                }
            }
            "-h" | "--help" => opts.help = true,
            "-v" | "--version" => opts.version = true,
            "--color" => {
                if let Some(val) = iter.next() {
                    match val.as_str() {
                        "never" => opts.no_color = true,
                        "always" => opts.no_color = false,
                        "auto" => {}
                        other => return Err(format!("invalid color mode: {other}")),
                    }
                } else {
                    return Err("--color requires a value (always|auto|never)".into());
                }
            }
            "--logo" => {
                if let Some(val) = iter.next() {
                    opts.logo = Some(val.clone());
                } else {
                    return Err("--logo requires a logo name".into());
                }
            }
            "--logo-color" => {
                if let Some(val) = iter.next() {
                    opts.logo_color = Some(val.clone());
                } else {
                    return Err("--logo-color requires a color name".into());
                }
            }
            "--structure" => {
                if let Some(val) = iter.next() {
                    let mods: Vec<String> = val
                        .split(&[':', ','][..])
                        .map(|s| s.trim().to_lowercase())
                        .filter(|s| !s.is_empty())
                        .collect();
                    opts.structure = Some(mods);
                } else {
                    return Err("--structure requires a list of modules".into());
                }
            }
            "--disk-paths" => {
                if let Some(val) = iter.next() {
                    let paths: Vec<String> = val
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    opts.disk_paths = Some(paths);
                } else {
                    return Err("--disk-paths requires comma-separated paths".into());
                }
            }
            "--config" => {
                if let Some(val) = iter.next() {
                    opts.config_path = Some(PathBuf::from(val));
                } else {
                    return Err("--config requires a file path".into());
                }
            }
            unknown => return Err(format!("unknown argument: {unknown}")),
        }
    }

    Ok(opts)
}

pub fn print_help() {
    println!("rustfetch {}", env!("CARGO_PKG_VERSION"));
    println!("A blazingly fast, highly extensible system information tool written in Rust.\n");
    println!("USAGE:");
    println!("    rustfetch [OPTIONS]\n");
    println!("OPTIONS:");
    println!("    --logo <NAME>          Specify a custom distro or OS logo");
    println!("    --logo-color <COLOR>   Override the logo primary ANSI color");
    println!("    --no-logo              Hide the ASCII distro logo");
    println!("    --no-color             Disable ANSI terminal colors");
    println!("    --color <MODE>         Color output mode (always, auto, never)");
    println!("    --structure <LIST>     Colon or comma-separated list of modules to display");
    println!(
        "    --disk-paths <PATHS>   Comma-separated list of mount paths to check (default: /)"
    );
    println!("    --config <PATH>        Path to custom JSON/JSONC configuration file");
    println!("    --gen-config           Print default JSON configuration to stdout");
    println!("    --list-logos           List all supported distro and OS logos");
    println!("    --list-modules         List all available information modules");
    println!("    --list-themes          List all available color themes");
    println!("    --theme <NAME>         Use a theme once without saving");
    println!("    --set-theme <NAME>     Save a theme as default in the config file");
    println!("    --preview-themes       Print a live preview of every theme");
    println!("    --themes               Interactive theme picker (TUI) with live preview");
    println!("    --json                 Output system information in structured JSON format");
    println!("    -v, --version          Print version information");
    println!("    -h, --help             Print help information\n");
    println!("EXAMPLES:");
    println!("    rustfetch --logo arch");
    println!("    rustfetch --theme gruvbox");
    println!("    rustfetch --themes");
    println!("    rustfetch --no-logo");
    println!("    rustfetch --structure title:os:kernel:cpu:gpu:memory:colors");
    println!("    rustfetch --json");
}

pub fn print_modules() {
    println!("Available rustfetch modules:");
    println!("  - title          (username@hostname)");
    println!("  - separator      (horizontal divider line)");
    println!("  - os             (operating system and architecture)");
    println!("  - host           (motherboard, chassis, model, virtualization)");
    println!("  - kernel         (kernel name and release version)");
    println!("  - uptime         (system uptime in days, hours, minutes)");
    println!(
        "  - packages       (package counts across dpkg, pacman, rpm, flatpak, snap, nix, apk, brew)"
    );
    println!("  - shell          (active shell path, name, and version)");
    println!("  - display        (resolution and connected monitors)");
    println!("  - de             (desktop environment name and version)");
    println!("  - wm             (window manager name)");
    println!("  - wm_theme       (window manager theme)");
    println!("  - theme          (GTK / desktop interface theme)");
    println!("  - icons          (icon theme)");
    println!("  - font           (system font)");
    println!("  - cursor         (cursor theme)");
    println!("  - terminal       (terminal emulator name and version)");
    println!("  - terminal_font  (terminal font name)");
    println!("  - cpu            (processor model, core count, frequency)");
    println!("  - gpu            (graphics card model)");
    println!("  - memory         (RAM used / total and percentage)");
    println!("  - swap           (swap used / total and percentage)");
    println!("  - disk           (disk usage, filesystem type, mount points)");
    println!("  - battery        (battery percentage and charge status)");
    println!("  - power_adapter  (AC adapter connected status)");
    println!("  - audio          (audio server and sound devices)");
    println!("  - local_ip       (primary network interface and IP address)");
    println!("  - locale         (system language and encoding)");
    println!("  - break          (blank newline separator)");
    println!("  - colors         (8 standard and 8 bright ANSI color palette blocks)");
}
