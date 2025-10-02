#!/usr/bin/env bash
# Helper script for using `rgd` with `bemenu`, providing image previews.
# Usage: /path/to/rgd_bemenu.sh

# Select a title with `bemenu`
title=$(rgd list --fields="title" \
    | bemenu --list 10 --ignorecase)
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
