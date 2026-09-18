#compdef rustfetch
# rustfetch zsh completion

_rustfetch() {
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
        '(-f --fast)'{-f,--fast}'[Minimal sfetch-like profile]'
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
}

_rustfetch "$@"
# flags: --logo --logo-color --logo-type --kitty --kitty-direct --kitty-icat --sixel --iterm --logo-width --logo-height --logo-padding --logo-padding-left --logo-padding-right --logo-padding-top --no-logo -f --fast --no-color --color --structure --disk-paths --config --gen-config --import-fastfetch --force --dry-run --list-logos --list-modules --list-themes --theme --set-theme --preview-themes --setup --themes --theme-picker --tui --json --show-empty --completions -h --help -v --version
