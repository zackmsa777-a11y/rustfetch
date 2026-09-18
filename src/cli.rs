use std::path::PathBuf;

pub const FAST_MODULES: &[&str] = &["os", "host", "kernel", "wm", "terminal"];

#[derive(Debug, Clone, Default)]
pub struct CliOptions {
    pub logo: Option<String>,
    pub logo_color: Option<String>,
    pub logo_type: Option<String>,
    pub logo_width: Option<usize>,
    pub logo_height: Option<usize>,
    pub logo_padding_top: Option<usize>,
    pub logo_padding_left: Option<usize>,
    pub logo_padding_right: Option<usize>,
    pub no_logo: bool,
    pub no_color: bool,
    pub fast: bool,
    pub structure: Option<Vec<String>>,
    pub disk_paths: Option<Vec<String>>,
    pub config_path: Option<PathBuf>,
    pub gen_config: bool,
    pub import_fastfetch: bool,
    pub import_fastfetch_path: Option<PathBuf>,
    pub force: bool,
    pub dry_run: bool,
    pub list_logos: bool,
    pub list_modules: bool,
    pub list_themes: bool,
    pub theme: Option<String>,
    pub set_theme: Option<String>,
    pub preview_themes: bool,
    pub theme_picker: bool,
    pub json: bool,
    pub show_empty: bool,
    pub completions: Option<String>,
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
            "-f" | "--fast" => opts.fast = true,
            "--json" => opts.json = true,
            "--show-empty" => opts.show_empty = true,
            "--completions" => {
                if let Some(val) = iter.next() {
                    opts.completions = Some(val.clone());
                } else {
                    return Err("--completions requires a shell name (bash|zsh|fish)".into());
                }
            }
            "--gen-config" => opts.gen_config = true,
            "--import-fastfetch" => {
                opts.import_fastfetch = true;
                if let Some(peek) = iter.peek()
                    && !peek.starts_with('-')
                {
                    opts.import_fastfetch_path = Some(PathBuf::from(iter.next().unwrap()));
                }
            }
            "--force" => opts.force = true,
            "--dry-run" => opts.dry_run = true,
            "--list-logos" => opts.list_logos = true,
            "--list-modules" => opts.list_modules = true,
            "--list-themes" => opts.list_themes = true,
            "--preview-themes" => opts.preview_themes = true,
            "--setup" | "--themes" | "--theme-picker" | "--tui" => opts.theme_picker = true,
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
            "--kitty" => {
                if let Some(val) = iter.next() {
                    opts.logo = Some(val.clone());
                    opts.logo_type = Some("kitty".into());
                } else {
                    return Err("--kitty requires an image file path".into());
                }
            }
            "--kitty-direct" => {
                if let Some(val) = iter.next() {
                    opts.logo = Some(val.clone());
                    opts.logo_type = Some("kitty-direct".into());
                } else {
                    return Err("--kitty-direct requires an image file path".into());
                }
            }
            "--kitty-icat" => {
                if let Some(val) = iter.next() {
                    opts.logo = Some(val.clone());
                    opts.logo_type = Some("kitty-icat".into());
                } else {
                    return Err("--kitty-icat requires an image file path".into());
                }
            }
            "--sixel" => {
                if let Some(val) = iter.next() {
                    opts.logo = Some(val.clone());
                    opts.logo_type = Some("sixel".into());
                } else {
                    return Err("--sixel requires an image file path".into());
                }
            }
            "--iterm" => {
                if let Some(val) = iter.next() {
                    opts.logo = Some(val.clone());
                    opts.logo_type = Some("iterm".into());
                } else {
                    return Err("--iterm requires an image file path".into());
                }
            }
            "--logo-type" => {
                if let Some(val) = iter.next() {
                    opts.logo_type = Some(val.to_lowercase());
                } else {
                    return Err("--logo-type requires a type name (kitty|kitty-direct|kitty-icat|sixel|iterm|file|builtin|auto)".into());
                }
            }
            "--logo-width" => {
                if let Some(val) = iter.next() {
                    match val.parse::<usize>() {
                        Ok(w) => opts.logo_width = Some(w),
                        Err(_) => return Err("--logo-width requires a positive integer".into()),
                    }
                } else {
                    return Err("--logo-width requires a number".into());
                }
            }
            "--logo-height" => {
                if let Some(val) = iter.next() {
                    match val.parse::<usize>() {
                        Ok(h) => opts.logo_height = Some(h),
                        Err(_) => return Err("--logo-height requires a positive integer".into()),
                    }
                } else {
                    return Err("--logo-height requires a number".into());
                }
            }
            "--logo-padding" => {
                if let Some(val) = iter.next() {
                    match val.parse::<usize>() {
                        Ok(p) => {
                            opts.logo_padding_left = Some(p);
                            opts.logo_padding_right = Some(p);
                        }
                        Err(_) => return Err("--logo-padding requires a positive integer".into()),
                    }
                } else {
                    return Err("--logo-padding requires a number".into());
                }
            }
            "--logo-padding-left" => {
                if let Some(val) = iter.next() {
                    match val.parse::<usize>() {
                        Ok(p) => opts.logo_padding_left = Some(p),
                        Err(_) => {
                            return Err("--logo-padding-left requires a positive integer".into());
                        }
                    }
                } else {
                    return Err("--logo-padding-left requires a number".into());
                }
            }
            "--logo-padding-right" => {
                if let Some(val) = iter.next() {
                    match val.parse::<usize>() {
                        Ok(p) => opts.logo_padding_right = Some(p),
                        Err(_) => {
                            return Err("--logo-padding-right requires a positive integer".into());
                        }
                    }
                } else {
                    return Err("--logo-padding-right requires a number".into());
                }
            }
            "--logo-padding-top" => {
                if let Some(val) = iter.next() {
                    match val.parse::<usize>() {
                        Ok(p) => opts.logo_padding_top = Some(p),
                        Err(_) => {
                            return Err("--logo-padding-top requires a positive integer".into());
                        }
                    }
                } else {
                    return Err("--logo-padding-top requires a number".into());
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
    println!(
        "    --logo-type <TYPE>     Logo type: kitty, kitty-direct, kitty-icat, sixel, iterm, file, builtin, auto"
    );
    println!("    --kitty <PATH>         Display an image logo using the Kitty graphics protocol");
    println!("    --kitty-direct <PATH>  Display an image using direct Kitty file transfer");
    println!("    --kitty-icat <PATH>    Display an image via kitten icat tool");
    println!("    --sixel <PATH>         Display an image logo using the Sixel protocol");
    println!("    --iterm <PATH>         Display an image logo using iTerm2 inline images");
    println!("    --logo-width <NUM>     Width in terminal cells for image logos");
    println!("    --logo-height <NUM>    Height in terminal cells for image logos");
    println!("    --no-logo              Hide the ASCII distro logo");
    println!(
        "    -f, --fast             Minimal sfetch-like profile (os/host/kernel/wm/terminal, builtin logo)"
    );
    println!("    --no-color             Disable ANSI terminal colors");
    println!("    --color <MODE>         Color output mode (always, auto, never)");
    println!("    --structure <LIST>     Colon or comma-separated list of modules to display");
    println!(
        "    --disk-paths <PATHS>   Comma-separated list of mount paths to check (default: /)"
    );
    println!("    --config <PATH>        Path to custom JSON/JSONC configuration file");
    println!("    --gen-config           Print default JSON configuration to stdout");
    println!("    --import-fastfetch [PATH]  Import a fastfetch JSON/JSONC config into rustfetch");
    println!("                           Default search: ~/.config/fastfetch/config.jsonc|.json");
    println!("                           Writes to ~/.config/rustfetch/config.jsonc (or --config)");
    println!("    --force                Overwrite an existing rustfetch config when importing");
    println!("    --dry-run              Print the mapped config to stdout without writing");
    println!("    --list-logos           List all supported distro and OS logos");
    println!("    --list-modules         List all available information modules");
    println!("    --list-themes          List all available color themes and layouts");
    println!("    --theme <NAME>         Use a theme once without saving");
    println!("    --set-theme <NAME>     Save a theme as default in the config file");
    println!("    --preview-themes       Print a live preview of every theme");
    println!("    --setup, --themes      Interactive full-screen theme & layout setup TUI");
    println!("    --json                 Output system information in structured JSON format");
    println!("    --show-empty           Show info modules even when their value is empty");
    println!("    --completions <SHELL>  Print shell completion script (bash, zsh, fish)");
    println!("    -v, --version          Print version information");
    println!("    -h, --help             Print help information\n");
    println!("EXAMPLES:");
    println!("    rustfetch --setup");
    println!("    rustfetch --theme groups");
    println!("    rustfetch --theme nyarch");
    println!("    rustfetch --logo arch");
    println!("    rustfetch --no-logo");
    println!("    rustfetch --fast");
    println!("    rustfetch --structure title:os:kernel:cpu:gpu:memory:colors");
    println!("    rustfetch --import-fastfetch");
    println!("    rustfetch --import-fastfetch ~/.config/fastfetch/config.jsonc --dry-run");
    println!("    rustfetch --json");
    println!("    rustfetch --show-empty");
    println!("    # custom modules: see --list-modules (command / custom)");
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
    println!("  - command        (run a shell/command and show stdout; key + command/text)");
    println!("  - custom         (static line: key+text, or format-only decorative text)");
    println!("  - break          (blank newline separator)");
    println!("  - colors         (8 standard and 8 bright ANSI color palette blocks)");
    println!();
    println!("Command / custom examples (config.jsonc):");
    println!(
        "  {{ \"type\": \"command\", \"key\": \"Weather\", \"command\": \"curl -s wttr.in/?format=3\" }}"
    );
    println!(
        "  {{ \"type\": \"command\", \"key\": \"Editor\", \"text\": \"$EDITOR --version\", \"shell\": true }}"
    );
    println!("  {{ \"type\": \"custom\", \"key\": \"Git\", \"text\": \"zackmsa777-a11y\" }}");
    println!(
        "  Defaults: timeout 1500ms; failures/timeouts hide (respect showEmpty); shell=false."
    );
}
