#!/bin/sh

battery=$(acpi -b)
uptime=$(uptime -p)
date=$(date +'%-d/%-m/%Y %H:%M:%S')
version=$(uname -r)

echo "[$battery]" "[$uptime]"  "[$version]"  "[$date]"
