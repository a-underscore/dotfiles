#!/bin/sh

cpus=$(lscpu | sed "6q;d" | sed "s/:[ ^]*0/: 0/g")
uptime_formatted=$(uptime | cut -d ',' -f1  | cut -d ' ' -f4,5)
date_formatted=$(date +'%-d/%-m/%Y %H:%M:%S')
linux_version=$(uname -r | cut -d '-' -f1)

echo "[$cpus]" "[$uptime_formatted]"  "[$linux_version]"  "[$date_formatted]"
