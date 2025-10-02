#!/usr/bin/env bash
# Helper script for using `rgd` with `yofi`.
# Usage: /path/to/rgd_yofi.sh

# Select a title with `yofi`
title=$(rgd list --fields title \
    | yofi dialog)
if [[ "$title" = "" ]]; then
    exit 1
fi

# Get the launch command for that title
cmd=$(rgd get --fields="launch-command" "$title")
if [[ "$cmd" = "" ]]; then
    exit 1
fi

# Execute the launch command
sh -c "$cmd"
