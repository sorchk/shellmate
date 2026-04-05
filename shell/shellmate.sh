#!/bin/sh

_shellmate_handle_prefix() {
    cmd="$1"
    shift
    prompt="$*"

    case "$cmd" in
        @ai|#ai|/ai) ;;
        *) return 1 ;;
    esac

    if [ -z "$prompt" ]; then
        return 0
    fi

    result=$(shellmate generate "$prompt" --shell sh)
    exit_code=$?

    if [ $exit_code -eq 0 ] && [ -n "$result" ]; then
        printf '%s ' "$result"
        key=$(dd bs=1 count=1 2>/dev/null)
        if [ -z "$key" ] || [ "$key" = "$(printf '\n')" ]; then
            printf '\033[32m✓\033[0m\n'
            eval "$result"
        elif [ "$key" = "$(printf '\033')" ]; then
            printf '\033[31m✗\033[0m\n'
        fi
    fi
    return 0
}

if [ -z "$_SHELLMATE_SH_LOADED" ]; then
    _SHELLMATE_SH_LOADED=1

    if [ -n "${BASH_VERSION:-}" ]; then
        _shellmate_orig_cnfh=""
        if type command_not_found_handle >/dev/null 2>&1; then
            _shellmate_orig_cnfh="$(type command_not_found_handle 2>/dev/null)"
        fi

        command_not_found_handle() {
            if _shellmate_handle_prefix "$@"; then
                return $?
            fi
            if [ -n "$_shellmate_orig_cnfh" ]; then
                eval "$_shellmate_orig_cnfh"
                return $?
            fi
            printf 'sh: %s: 未找到命令\n' "$1" >&2
            return 127
        }
    fi
fi
