if [[ ! $DISPLAY ]] && [[ $(tty) == "/dev/tty1" ]]; then
	startx
fi
