#!/bin/fish

set battery (acpi -b)
set uptime_formatted (uptime | cut -d ',' -f1  | cut -d ' ' -f4,5)
set date_formatted (date +'%-d/%-m/%Y %H:%M:%S')
set linux_version (uname -r | cut -d '-' -f1)

echo "[$battery]" "[$uptime_formatted]"  "[$linux_version]"  "[$date_formatted]"
