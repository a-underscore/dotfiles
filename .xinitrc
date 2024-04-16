dbus-update-activation-environment --all &
gnome-keyring-daemon --start --components=secrets &

xmodmap .Xmodmap

exec dbus-run-session i3
