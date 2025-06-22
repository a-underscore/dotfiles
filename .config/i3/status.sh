#!/bin/sh

date=$(date +'%-d/%-m/%Y %H:%M:%S')
batt=$(acpi -b)

echo "[ $batt ] [ $date ]"
