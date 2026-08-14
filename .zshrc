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

zstyle :compinstall filename "$HOME/.zshrc"

autoload -Uz compinit && compinit
autoload -Uz bashcompinit && bashcompinit
autoload -Uz colors && colors

export PATH="$HOME/.dotnet/:$HOME/.cargo/bin/:$HOME/.local/bin/:$PATH"
export EDITOR="nvim"
export VISUAL=$EDITOR
export HISTFILE="$HOME/.zhistory"
export SAVEHIST=1000
export HISTSIZE="$SAVEHIST"
export PROMPT="%{$fg_bold[green]%}%n@%M%{$reset_color%} %{$fg_bold[blue]%}%~%{$reset_color%} %{$fg_bold[green]%}%%%{$reset_color%} "

bindkey -e
[ -s "/home/a_/.jabba/jabba.sh" ] && source "/home/a_/.jabba/jabba.sh"

# Add RVM to PATH for scripting. Make sure this is the last PATH variable change.
export XDG_DATA_DIRS="XDG_DATA_DIRS:$HOME/.local/share/flatpak/exports/share:/var/lib/flatpak/exports/share";
export PATH="$PATH:$HOME/.rvm/bin:$HOME/.local/share/gem/ruby/3.3.0/bin"
export PATH="$PATH:$HOME/.dotnet/tools"
export PATH="$PATH:/home/a_/.dotnet/sdk/10.0.101/"
export DOTNET_ROOT="$HOME/.dotnet"

. "$HOME/.local/bin/env"
