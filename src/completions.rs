pub fn generate(shell: &str) -> Result<String, String> {
    match shell.to_lowercase().as_str() {
        "bash" => Ok(bash()),
        "zsh" => Ok(zsh()),
        "fish" => Ok(fish()),
        other => Err(format!(
            "unsupported shell '{other}' (expected bash, zsh, or fish)"
        )),
    }
}

fn flag_list() -> &'static str {
    "--logo --logo-color --logo-type --kitty --kitty-direct --kitty-icat --sixel --iterm \
--logo-width --logo-height --logo-padding --logo-padding-left --logo-padding-right \
--logo-padding-top --no-logo -f --fast --no-color --color --structure --disk-paths --config \
--gen-config --import-fastfetch --force --dry-run --list-logos --list-modules \
--list-themes --theme --set-theme --preview-themes --setup --themes --theme-picker \
--tui --json --show-empty --completions -h --help -v --version"
}

fn bash() -> String {
    let flags = flag_list();
    format!(
        r#"# rustfetch bash completion
_rustfetch() {{
    local cur prev
    COMPREPLY=()
    cur="${{COMP_WORDS[COMP_CWORD]}}"
    prev="${{COMP_WORDS[COMP_CWORD-1]}}"

    case "${{prev}}" in
        --theme|--set-theme)
            COMPREPLY=( $(compgen -W "default groups hypr neon matrix paper catppuccin-mocha tokyo-night dracula nord gruvbox" -- "${{cur}}") )
            return 0
            ;;
        --color)
            COMPREPLY=( $(compgen -W "always auto never" -- "${{cur}}") )
            return 0
            ;;
        --logo-type)
            COMPREPLY=( $(compgen -W "kitty kitty-direct kitty-icat sixel iterm file builtin auto" -- "${{cur}}") )
            return 0
            ;;
        --completions)
            COMPREPLY=( $(compgen -W "bash zsh fish" -- "${{cur}}") )
            return 0
            ;;
        --logo|--logo-color)
            return 0
            ;;
        --structure)
            COMPREPLY=( $(compgen -W "title:os:kernel:cpu:gpu:memory:colors title,os,kernel" -- "${{cur}}") )
            return 0
            ;;
        --config|--kitty|--kitty-direct|--kitty-icat|--sixel|--iterm|--import-fastfetch)
            COMPREPLY=( $(compgen -f -- "${{cur}}") )
            return 0
            ;;
        --logo-width|--logo-height|--logo-padding|--logo-padding-left|--logo-padding-right|--logo-padding-top|--disk-paths)
            return 0
            ;;
    esac

    if [[ "${{cur}}" == -* ]]; then
        COMPREPLY=( $(compgen -W "{flags}" -- "${{cur}}") )
        return 0
    fi
}}
complete -F _rustfetch rustfetch
"#
    )
}

fn zsh() -> String {
    let flags = flag_list();
    format!(
        r#"#compdef rustfetch
# rustfetch zsh completion

_rustfetch() {{
    local -a opts
    opts=(
        '--logo[Specify a custom distro or OS logo]:logo:'
        '--logo-color[Override the logo primary ANSI color]:color:'
        '--logo-type[Logo type]:type:(kitty kitty-direct kitty-icat sixel iterm file builtin auto)'
        '--kitty[Display an image logo using Kitty graphics]:path:_files'
        '--kitty-direct[Display an image using direct Kitty transfer]:path:_files'
        '--kitty-icat[Display an image via kitten icat]:path:_files'
        '--sixel[Display an image logo using Sixel]:path:_files'
        '--iterm[Display an image logo using iTerm2 inline images]:path:_files'
        '--logo-width[Width in terminal cells for image logos]:num:'
        '--logo-height[Height in terminal cells for image logos]:num:'
        '--logo-padding[Horizontal padding between logo and modules]:num:'
        '--logo-padding-left[Left padding spaces]:num:'
        '--logo-padding-right[Right padding spaces]:num:'
        '--logo-padding-top[Top padding lines]:num:'
        '--no-logo[Hide the ASCII distro logo]'
        '(-f --fast)'{{-f,--fast}}'[Minimal sfetch-like profile]'
        '--no-color[Disable ANSI terminal colors]'
        '--color[Color output mode]:mode:(always auto never)'
        '--structure[Colon or comma-separated modules]:modules:'
        '--disk-paths[Comma-separated mount paths]:paths:'
        '--config[Path to JSON/JSONC config]:path:_files'
        '--gen-config[Print default JSON configuration]'
        '--import-fastfetch[Import a fastfetch config]:path:_files'
        '--force[Overwrite existing config when importing]'
        '--dry-run[Print mapped config without writing]'
        '--list-logos[List supported distro and OS logos]'
        '--list-modules[List available information modules]'
        '--list-themes[List available color themes and layouts]'
        '--theme[Use a theme once without saving]:theme:'
        '--set-theme[Save a theme as default]:theme:'
        '--preview-themes[Print a live preview of every theme]'
        '--setup[Interactive full-screen theme and layout setup TUI]'
        '--themes[Alias for --setup]'
        '--theme-picker[Alias for --setup]'
        '--tui[Alias for --setup]'
        '--json[Output system information as JSON]'
        '--show-empty[Show info modules even when empty]'
        '--completions[Print shell completion script]:shell:(bash zsh fish)'
        '-h[Print help information]'
        '--help[Print help information]'
        '-v[Print version information]'
        '--version[Print version information]'
    )
    _arguments -s -S $opts
}}

_rustfetch "$@"
# flags: {flags}
"#
    )
}

fn fish() -> String {
    let flags = flag_list();
    format!(
        r#"# rustfetch fish completion
complete -c rustfetch -f

complete -c rustfetch -l logo -d 'Specify a custom distro or OS logo' -r
complete -c rustfetch -l logo-color -d 'Override the logo primary ANSI color' -r
complete -c rustfetch -l logo-type -d 'Logo type' -r -a 'kitty kitty-direct kitty-icat sixel iterm file builtin auto'
complete -c rustfetch -l kitty -d 'Display an image logo using Kitty graphics' -r -F
complete -c rustfetch -l kitty-direct -d 'Display an image using direct Kitty transfer' -r -F
complete -c rustfetch -l kitty-icat -d 'Display an image via kitten icat' -r -F
complete -c rustfetch -l sixel -d 'Display an image logo using Sixel' -r -F
complete -c rustfetch -l iterm -d 'Display an image logo using iTerm2 inline images' -r -F
complete -c rustfetch -l logo-width -d 'Width in terminal cells for image logos' -r
complete -c rustfetch -l logo-height -d 'Height in terminal cells for image logos' -r
complete -c rustfetch -l logo-padding -d 'Horizontal padding between logo and modules' -r
complete -c rustfetch -l logo-padding-left -d 'Left padding spaces' -r
complete -c rustfetch -l logo-padding-right -d 'Right padding spaces' -r
complete -c rustfetch -l logo-padding-top -d 'Top padding lines' -r
complete -c rustfetch -l no-logo -d 'Hide the ASCII distro logo'
complete -c rustfetch -s f -l fast -d 'Minimal sfetch-like profile'
complete -c rustfetch -l no-color -d 'Disable ANSI terminal colors'
complete -c rustfetch -l color -d 'Color output mode' -r -a 'always auto never'
complete -c rustfetch -l structure -d 'Colon or comma-separated modules' -r
complete -c rustfetch -l disk-paths -d 'Comma-separated mount paths' -r
complete -c rustfetch -l config -d 'Path to JSON/JSONC config' -r -F
complete -c rustfetch -l gen-config -d 'Print default JSON configuration'
complete -c rustfetch -l import-fastfetch -d 'Import a fastfetch config' -r -F
complete -c rustfetch -l force -d 'Overwrite existing config when importing'
complete -c rustfetch -l dry-run -d 'Print mapped config without writing'
complete -c rustfetch -l list-logos -d 'List supported distro and OS logos'
complete -c rustfetch -l list-modules -d 'List available information modules'
complete -c rustfetch -l list-themes -d 'List available color themes and layouts'
complete -c rustfetch -l theme -d 'Use a theme once without saving' -r
complete -c rustfetch -l set-theme -d 'Save a theme as default' -r
complete -c rustfetch -l preview-themes -d 'Print a live preview of every theme'
complete -c rustfetch -l setup -d 'Interactive full-screen theme and layout setup TUI'
complete -c rustfetch -l themes -d 'Alias for --setup'
complete -c rustfetch -l theme-picker -d 'Alias for --setup'
complete -c rustfetch -l tui -d 'Alias for --setup'
complete -c rustfetch -l json -d 'Output system information as JSON'
complete -c rustfetch -l show-empty -d 'Show info modules even when empty'
complete -c rustfetch -l completions -d 'Print shell completion script' -r -a 'bash zsh fish'
complete -c rustfetch -s h -l help -d 'Print help information'
complete -c rustfetch -s v -l version -d 'Print version information'
# flags: {flags}
"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bash_completion_mentions_key_flags() {
        let script = generate("bash").unwrap();
        assert!(!script.is_empty());
        assert!(script.contains("--import-fastfetch"));
        assert!(script.contains("--show-empty"));
        assert!(script.contains("--setup"));
        assert!(script.contains("--json"));
        assert!(script.contains("--no-logo"));
        assert!(script.contains("--theme"));
        assert!(script.contains("--structure"));
        assert!(script.contains("--list-themes"));
        assert!(script.contains("--list-modules"));
        assert!(script.contains("--list-logos"));
        assert!(script.contains("--completions"));
        assert!(script.contains("--sixel"));
        assert!(script.contains("--iterm"));
        assert!(script.contains("sixel"));
        assert!(script.contains("iterm"));
    }

    #[test]
    fn zsh_and_fish_completions_are_nonempty() {
        let z = generate("zsh").unwrap();
        let f = generate("fish").unwrap();
        assert!(z.contains("--import-fastfetch"));
        assert!(f.contains("--show-empty"));
        assert!(generate("powershell").is_err());
    }
}
