setopt aliases
setopt append_history
setopt complete_aliases
setopt complete_in_word
setopt extended_glob
setopt extended_history
setopt hist_expire_dups_first
setopt hist_ignore_dups
setopt hist_ignore_space
setopt rm_star_silent

unsetopt beep

export PATH="$HOME/.cargo/bin/:$HOME/.local/bin/:$PATH"
export EDITOR=nvim

HISTFILE=~/.zhistory
SAVEHIST=1000
HISTSIZE=$SAVEHIST

zstyle :compinstall filename '$HOME/.zshrc'

autoload -Uz compinit && compinit
autoload -Uz bashcompinit && bashcompinit
autoload -Uz colors && colors

PROMPT="%{$fg_bold[green]%}%n@%M%{$reset_color%} %{$fg_bold[blue]%}%~%{$reset_color%} %{$fg_bold[green]%}%%%{$reset_color%} "
