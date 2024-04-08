dbus-update-activation-environment --all &
gnome-keyring-daemon --foreground &
gnome-keyring-daemon --start --components=secrets &

exec dbus-run-session i3
