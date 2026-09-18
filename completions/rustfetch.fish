# rustfetch fish completion
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
# flags: --logo --logo-color --logo-type --kitty --kitty-direct --kitty-icat --sixel --iterm --logo-width --logo-height --logo-padding --logo-padding-left --logo-padding-right --logo-padding-top --no-logo --no-color --color --structure --disk-paths --config --gen-config --import-fastfetch --force --dry-run --list-logos --list-modules --list-themes --theme --set-theme --preview-themes --setup --themes --theme-picker --tui --json --show-empty --completions -h --help -v --version
