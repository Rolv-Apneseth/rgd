#!/usr/bin/env bash
# Helper script for using `rgd` with `dmenu`, providing image previews.
# Usage: /path/to/rgd_dmenu.sh

# Select a title with `dmenu`
title=$(rgd list --fields="title" \
    | dmenu -i)
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
