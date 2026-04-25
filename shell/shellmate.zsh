#!/usr/bin/env zsh

_shellmate_shortcut() {
    local input="$BUFFER"
    if [[ -z "$input" ]]; then
        return
    fi

    local prompt="$input"
    if [[ "$input" == "@ai "* ]]; then
        prompt="${input#@ai }"
    elif [[ "$input" == "@ai" ]]; then
        BUFFER=""
        CURSOR=0
        zle reset-prompt
        return
    fi

    BUFFER="@ai $prompt"
    CURSOR=${#BUFFER}
    zle .accept-line
}

_shellmate_preexec() {
    local cmd="$3"
    if [[ "$cmd" == "@ai "* || "$cmd" == "@ai" ]]; then
        local prompt="${cmd#@ai }"
        prompt="${prompt% }"

        local result exit_code
        result=$(shellmate generate "$prompt" --shell zsh)
        exit_code=$?

        if [[ $exit_code -eq 0 && -n "$result" ]]; then
            echo -ne "\e[s"
            echo -n "$result "
            read -rsn1 key
            if [[ -z "$key" || "$key" == $'\n' ]]; then
                echo -e "\e[32m✓\e[0m"
                print -S "$result"
                eval "$result"
            elif [[ "$key" == $'\e' ]]; then
                echo -ne "\e[u\e[0J"
                printf '\e[9;90m%s\e[0m \e[31m✗\e[0m\n' "$result"
            fi
        fi
        return 1
    fi
    return 0
}

_shellmate_dirs_file() {
    echo "${HOME}/.shellmate/dirs"
}

_shellmate_cd_record() {
    local dirs_file
    dirs_file="$(_shellmate_dirs_file)"
    local cwd
    cwd="$(pwd)"
    [[ ! -d "$cwd" ]] && return 0

    local tmp
    tmp="$(mktemp)"
    {
        echo "$cwd"
        if [[ -f "$dirs_file" ]]; then
            grep -v -x -F "$cwd" "$dirs_file" 2>/dev/null
        fi
    } | head -20 > "$tmp"
    mkdir -p "${dirs_file%/*}"
    mv "$tmp" "$dirs_file"
}

_shellmate_chpwd() {
    _shellmate_cd_record
}

_shellmate_cd_matches() {
    local keyword="$1"
    local dirs_file
    dirs_file="$(_shellmate_dirs_file)"
    [[ ! -f "$dirs_file" ]] && return

    local matches=()
    while IFS= read -r dir; do
        [[ -z "$dir" || ! -d "$dir" ]] && continue
        if [[ -z "$keyword" ]] || [[ "${(L)dir}" == *"${(L)keyword}"* ]]; then
            matches+=("$dir")
        fi
    done < "$dirs_file"
    printf '%s\n' "${matches[@]}"
}

_shellmate_cd_complete() {
    local words=("${=PREFIX}")
    local cur="${words[-1]}"

    if [[ "$cur" != \#* ]]; then
        _default
        return
    fi

    local keyword="${cur#\#}"
    local matches
    matches=($(_shellmate_cd_matches "$keyword"))

    if [[ ${#matches[@]} -eq 0 ]]; then
        _message "no matching directories"
    elif [[ ${#matches[@]} -eq 1 ]]; then
        compadd -U -Q -- "${matches[1]}"
    else
        compadd -U -Q -a matches
    fi
}

if [[ -z "$_SHELLMATE_ZSH_LOADED" ]]; then
    _SHELLMATE_ZSH_LOADED=1

    zle -N _shellmate_shortcut
    bindkey '__SHELLMATE_BIND_KEY_ZSH__' _shellmate_shortcut

    autoload -Uz add-zsh-hook
    _shellmate_preexec_hook() {
        _shellmate_preexec "$@"
    }
    add-zsh-hook preexec _shellmate_preexec_hook
    add-zsh-hook chpwd _shellmate_chpwd

    compdef _shellmate_cd_complete cd
fi
