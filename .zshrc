unsetopt beep

bindkey -v

zstyle :compinstall filename '/home/brendan/.zshrc'

autoload -Uz compinit && compinit
autoload -Uz bashcompinit && bashcompinit
autoload -Uz colors && colors

PS1="%{$fg_bold[green]%}%n@%M%{$reset_color%} %{$fg_bold[blue]%}%~%{$reset_color%} %{$fg[green]%}%%%{$reset_color%} "

export PATH="$HOME/.local/bin/:$HOME/.cargo/bin/:$HOME/.cabal/bin/:$PATH"
