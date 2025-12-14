#!/bin/sh

date=$(date +'%-d/%-m/%Y %H:%M:%S')
batt=$(acpi -b 2>/dev/null)

if [ -z $batt ]; then
	echo "[ $date ]"
else
	echo "[ $batt ] [ $date ]"
fi
