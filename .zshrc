setopt aliases
unsetopt beep

bindkey -v

zstyle :compinstall filename '$HOME/.zshrc'

autoload -Uz compinit && compinit
autoload -Uz bashcompinit && bashcompinit
autoload -Uz colors && colors

PROMPT="%{$fg_bold[green]%}%n@%M%{$reset_color%} %{$fg_bold[blue]%}%~%{$reset_color%} %{$fg_bold[green]%}%%%{$reset_color%} "

export PATH="$HOME/.cargo/bin/:$HOME/.local/bin/:$PATH"
