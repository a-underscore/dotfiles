#!/bin/sh

batt=$(acpi -b)
date=$(date +'%-d/%-m/%Y %H:%M:%S')

echo "[ $date ]"
