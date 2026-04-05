#!/usr/bin/env bash

_shellmate_shortcut() {
    local input="$READLINE_LINE"
    if [[ -z "$input" ]]; then
        return
    fi

    local prompt="$input"
    local prefixes=("@ai" "#ai" "/ai")
    for prefix in "${prefixes[@]}"; do
        if [[ "$input" == "${prefix} "* ]]; then
            prompt="${input#${prefix} }"
            break
        fi
        if [[ "$input" == "${prefix}" ]]; then
            READLINE_LINE=""
            READLINE_POINT=0
            return
        fi
    done

    READLINE_LINE="@ai $prompt"
    READLINE_POINT=${#READLINE_LINE}
}

_shellmate_handle_prefix() {
    local cmd="$1"
    shift
    local full_cmd="$cmd $*"

    local prefixes=("@ai" "#ai" "/ai")
    local matched=0
    local prompt=""

    for prefix in "${prefixes[@]}"; do
        if [[ "$cmd" == "$prefix" ]]; then
            prompt="$*"
            matched=1
            break
        fi
    done

    if [[ $matched -eq 0 ]]; then
        return 1
    fi

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
        bind '"\C-g": "\C-xc\C-j"'
    fi
fi
