# rustfetch bash completion
_rustfetch() {
    local cur prev
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    case "${prev}" in
        --theme|--set-theme)
            COMPREPLY=( $(compgen -W "default groups hypr neon matrix paper catppuccin-mocha tokyo-night dracula nord gruvbox" -- "${cur}") )
            return 0
            ;;
        --color)
            COMPREPLY=( $(compgen -W "always auto never" -- "${cur}") )
            return 0
            ;;
        --logo-type)
            COMPREPLY=( $(compgen -W "kitty kitty-direct kitty-icat sixel iterm file builtin auto" -- "${cur}") )
            return 0
            ;;
        --completions)
            COMPREPLY=( $(compgen -W "bash zsh fish" -- "${cur}") )
            return 0
            ;;
        --logo|--logo-color)
            return 0
            ;;
        --structure)
            COMPREPLY=( $(compgen -W "title:os:kernel:cpu:gpu:memory:colors title,os,kernel" -- "${cur}") )
            return 0
            ;;
        --config|--kitty|--kitty-direct|--kitty-icat|--sixel|--iterm|--import-fastfetch)
            COMPREPLY=( $(compgen -f -- "${cur}") )
            return 0
            ;;
        --logo-width|--logo-height|--logo-padding|--logo-padding-left|--logo-padding-right|--logo-padding-top|--disk-paths)
            return 0
            ;;
    esac

    if [[ "${cur}" == -* ]]; then
        COMPREPLY=( $(compgen -W "--logo --logo-color --logo-type --kitty --kitty-direct --kitty-icat --sixel --iterm --logo-width --logo-height --logo-padding --logo-padding-left --logo-padding-right --logo-padding-top --no-logo --no-color --color --structure --disk-paths --config --gen-config --import-fastfetch --force --dry-run --list-logos --list-modules --list-themes --theme --set-theme --preview-themes --setup --themes --theme-picker --tui --json --show-empty --completions -h --help -v --version" -- "${cur}") )
        return 0
    fi
}
complete -F _rustfetch rustfetch
