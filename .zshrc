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

export PATH="$HOME/.cargo/bin/:$HOME/.local/bin/:$PATH"
export EDITOR="nvim"
export VISUAL=$EDITOR
export HISTFILE="$HOME/.zhistory"
export SAVEHIST=1000
export HISTSIZE="$SAVEHIST"
export PROMPT="%{$fg_bold[green]%}%n@%M%{$reset_color%} %{$fg_bold[blue]%}%~%{$reset_color%} %{$fg_bold[green]%}%%%{$reset_color%} "

bindkey -e
[ -s "/home/a_/.jabba/jabba.sh" ] && source "/home/a_/.jabba/jabba.sh"

# Add RVM to PATH for scripting. Make sure this is the last PATH variable change.
# NOTE: build this value from scratch instead of appending to the inherited one.
# The original line was `export XDG_DATA_DIRS="XDG_DATA_DIRS:..."` - because the
# word XDG_DATA_DIRS was never expanded, /usr/local/share and /usr/share were
# dropped from the search path. xdg-mime/xdg-open then found no .desktop files,
# so every handler lookup returned nothing and xdg-open silently did nothing.
# /usr/local/share:/usr/share is Gentoo's default (see /etc/profile.env).
# Starting from scratch also repairs shells that already inherited the bad value.
export XDG_DATA_DIRS="/usr/local/share:/usr/share:$HOME/.local/share/flatpak/exports/share:/var/lib/flatpak/exports/share"
export PATH="$PATH:$HOME/.rvm/bin:$HOME/.local/share/gem/ruby/3.3.0/bin"

# NOTE: a file named "env" inside ~/.local/bin shadows /usr/bin/env for every
# program, because that directory precedes /usr/bin in PATH. A 0-byte leftover
# of that name silently broke anything invoking `env` -- most visibly xdg-open,
# which launches desktop-file handlers via `env "$command"` (the browser never
# started, and the failure was masked because exit_success ran anyway).
# The empty file has been removed. If you install `uv` later, do NOT leave its
# env helper named "env" in this directory; source it under another name.
