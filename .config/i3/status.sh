#!/bin/sh

cpu=$(nproc)
version=$(uname -r)
uptime=$(uptime -p)
date=$(date +'%-d/%-m/%Y %H:%M:%S')

echo [$cpu] [$version] [$uptime] [$date]
