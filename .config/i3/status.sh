#!/bin/sh

cpu=$(nproc)
version=$(uname -r)
uptime=$(uptime -p)
battery=$(acpi -b)
date=$(date +'%-d/%-m/%Y %H:%M:%S')

echo [$cpu] [$version] [$uptime] [$battery] [$date]
