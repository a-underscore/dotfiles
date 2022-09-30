#!/bin/sh

cpus="Online CPUs: $(nproc)"
uptime_formatted=$(uptime -p)
date_formatted=$(date +'%-d/%-m/%Y %H:%M:%S')
linux_version=$(uname -r)

echo "[$cpus]" "[$uptime_formatted]"  "[$linux_version]"  "[$date_formatted]"
