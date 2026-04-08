#!/usr/bin/env zsh

_shellmate_shortcut() {
    local input="$BUFFER"
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
            BUFFER=""
            CURSOR=0
            zle reset-prompt
            return
        fi
    done

    BUFFER="@ai $prompt"
    CURSOR=${#BUFFER}
    zle .accept-line
}

_shellmate_preexec() {
    local cmd="$3"
    local prefixes=("@ai" "#ai" "/ai")
    for prefix in "${prefixes[@]}"; do
        if [[ "$cmd" == "${prefix} "* || "$cmd" == "${prefix}" ]]; then
            local prompt="${cmd#${prefix} }"
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
    done
    return 0
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
fi
