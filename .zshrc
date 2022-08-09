unsetopt beep

bindkey -v

zstyle :compinstall filename '/home/brendan/.zshrc'

autoload -Uz compinit && compinit
autoload -Uz bashcompinit && bashcompinit
autoload -Uz colors && colors

PS1="%{$fg_bold[green]%}%n@%M%{$reset_color%} %{$fg_bold[blue]%}%~%{$reset_color%} %{$fg[green]%}$>%{$reset_color%} "

alias ls='ls --color=auto'
alias lock="loginctl suspend && i3lock"
alias c="clear"
alias q="exit"
alias ff="firefox"
alias ffp="firefox --private-window"
alias spotify="flatpak run com.spotify.Client"
