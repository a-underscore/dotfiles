setopt appendhistory
setopt aliases
setopt completealiases
setopt complete_in_word
setopt extended_glob
setopt extended_history
setopt hist_ignore_dups
setopt hist_expire_dups_first
setopt hist_ignore_space

unsetopt beep

HISTFILE=~/.zhistory
SAVEHIST=1000
HISTSIZE=$SAVEHIST

bindkey -v

zstyle :compinstall filename '$HOME/.zshrc'

autoload -Uz compinit && compinit
autoload -Uz bashcompinit && bashcompinit
autoload -Uz colors && colors

PROMPT="%{$fg_bold[green]%}%n@%M%{$reset_color%} %{$fg_bold[blue]%}%~%{$reset_color%} %{$fg_bold[green]%}%%%{$reset_color%} "

export PATH="$HOME/.cargo/bin/:$HOME/.local/bin/:$PATH"
