#!/bin/sh

battery=$(acpi -b)
uptime_formatted=$(uptime | cut -d ',' -f1  | cut -d ' ' -f4,5)
date_formatted=$(date +'%-d/%-m/%Y %H:%M:%S')
linux_version=$(uname -r | cut -d '-' -f1)

echo "[$battery]" "[$uptime_formatted]"  "[$linux_version]"  "[$date_formatted]"
