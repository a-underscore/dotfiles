#!/bin/sh

BASE_DIR=$(dirname "$0")

. $BASE_DIR/config.sh

echo $SCREEN_LAYOUT

$BASE_DIR/$SCREEN_LAYOUT.sh
