#!/usr/bin/env bash

_shellmate_shortcut() {
    local input="$READLINE_LINE"
    if [[ -z "$input" ]]; then
        return
    fi

    local prompt="$input"
    if [[ "$input" == "@ai "* ]]; then
        prompt="${input#@ai }"
    elif [[ "$input" == "@ai" ]]; then
        READLINE_LINE=""
        READLINE_POINT=0
        return
    fi

    READLINE_LINE="@ai $prompt"
    READLINE_POINT=${#READLINE_LINE}
}

_shellmate_handle_prefix() {
    local cmd="$1"
    shift
    local full_cmd="$cmd $*"

    if [[ "$cmd" != "@ai" ]]; then
        return 1
    fi

    local prompt="$*"

    if [[ -z "$prompt" ]]; then
        return 0
    fi

    local result exit_code
    result=$(shellmate generate "$prompt" --shell bash)
    exit_code=$?

    if [[ $exit_code -eq 0 && -n "$result" ]]; then
        echo -ne "\e[s"
        echo -n "$result "
        read -rsn1 key
        if [[ -z "$key" || "$key" == $'\n' ]]; then
            echo -e "\e[32m✓\e[0m"
            history -s "$result"
            eval "$result"
        elif [[ "$key" == $'\e' ]]; then
            echo -ne "\e[u\e[0J"
            printf '\e[9;90m%s\e[0m \e[31m✗\e[0m\n' "$result"
        fi
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

_shellmate_cd_complete() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    if [[ "$cur" != \#* ]]; then
        COMPREPLY=()
        return
    fi

    local keyword="${cur#\#}"
    local dirs_file
    dirs_file="$(_shellmate_dirs_file)"
    [[ ! -f "$dirs_file" ]] && { COMPREPLY=(); return; }

    local matches=()
    while IFS= read -r dir; do
        [[ -z "$dir" || ! -d "$dir" ]] && continue
        if [[ -z "$keyword" ]] || [[ "${dir,,}" == *"${keyword,,}"* ]]; then
            matches+=("$dir")
        fi
    done < "$dirs_file"

    if [[ ${#matches[@]} -eq 0 ]]; then
        COMPREPLY=()
    elif [[ ${#matches[@]} -eq 1 ]]; then
        COMPREPLY=("${matches[0]}")
    else
        for i in "${!matches[@]}"; do
            printf '  %d) %s\n' "$((i + 1))" "${matches[$i]}" >&2
        done
        COMPREPLY=()
    fi
}

if [[ -z "$_SHELLMATE_BASH_LOADED" ]]; then
    _SHELLMATE_BASH_LOADED=1

    if [[ -n "$(type -t command_not_found_handle 2>/dev/null)" ]]; then
        _shellmate_orig_cnfh="$(declare -f command_not_found_handle)"
        eval "_shellmate_prev_cnfh() { ${_shellmate_orig_cnfh#command_not_found_handle} }"
    fi

    command_not_found_handle() {
        if _shellmate_handle_prefix "$@"; then
            return $?
        fi
        if type _shellmate_prev_cnfh &>/dev/null; then
            _shellmate_prev_cnfh "$@"
            return $?
        fi
        echo "bash: $1: 未找到命令" >&2
        return 127
    }

    if command -v shellmate &>/dev/null; then
        bind -x '"\C-xc": _shellmate_shortcut'
        bind '"__SHELLMATE_BIND_KEY_BASH__": "\C-xc\C-j"'
    fi

    _shellmate_prev_pwd="$PWD"
    _shellmate_prompt_cd_record() {
        if [[ "$PWD" != "$_shellmate_prev_pwd" ]]; then
            _shellmate_cd_record
            _shellmate_prev_pwd="$PWD"
        fi
    }
    _shellmate_orig_prompt_cmd="${PROMPT_COMMAND:-}"
    PROMPT_COMMAND='_shellmate_prompt_cd_record; '"${PROMPT_COMMAND:+$_shellmate_orig_prompt_cmd; }"

    complete -o nospace -F _shellmate_cd_complete cd
fi
