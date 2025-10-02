#!/usr/bin/env bash
# Helper script for using `rgd` with `tofi`.
# Usage: /path/to/rgd_tofi.sh

# Select a title with `tofi`
title=$(rgd list --fields="title" \
    | tofi)
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
