[[ -s "$HOME/.rvm/scripts/rvm" ]] && source "$HOME/.rvm/scripts/rvm" # Load RVM into a shell session *as a function*

# gnome-keyring: expose the Secret Service control socket and the keyring SSH
# agent to the graphical session. Set before `startx` so i3 (and everything it
# launches) inherits these; without them SSH_AUTH_SOCK is unset and ssh-add
# cannot reach the keyring's agent.
if [[ -n $XDG_RUNTIME_DIR ]]; then
  export GNOME_KEYRING_CONTROL="$XDG_RUNTIME_DIR/keyring"
  export SSH_AUTH_SOCK="$XDG_RUNTIME_DIR/keyring/ssh"
fi

if [[ -z $DISPLAY && $XDG_VTNR -eq 1 ]]; then
  exec startx
fi
