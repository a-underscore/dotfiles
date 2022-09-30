#!/bin/sh

cpu="$(nproc)"
uptime=$(uptime -p)
date=$(date +'%-d/%-m/%Y %H:%M:%S')
version=$(uname -r)

echo "[$cpu]" "[$version]" "[$uptime]" "[$date]"
