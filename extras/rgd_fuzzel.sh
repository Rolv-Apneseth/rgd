#!/usr/bin/env bash
# Helper script for using `rgd` with `fuzzel`.
# Usage: /path/to/rgd_fuzzel.sh
#
# NOTE: only PNG and SVG files are supported by `fuzzel` (see <https://codeberg.org/dnkl/fuzzel/issues/601>).
# This means many games (e.g. everything from Steam) would not show any icon, as they are JPG files.
# For this reason, the below script does not support icons - but if that ever changes this can be modified
# to use the below commented code.
#
# choice=$(rgd list --fields="title,path-icon,launch-command,path-game-dir" \
#     | grep -P -v '\t\t' \
#     | awk -F '\t' '{print $0 "\000icon\037" $2}' \
#     | fuzzel --dmenu --placeholder "Games" --counter --with-nth 1 --line-height=50px)
# exit_code=$?
#
# if [ "$choice" = "" ]; then
#     exit 1
# fi

# List games and select an entry with `fuzzel`
choice=$(rgd list --fields="title,launch-command,path-game-dir" \
    | fuzzel --dmenu --placeholder "Games" --counter --with-nth 1)
exit_code=$?

# Exit early if no entry was selected
if [ "$choice" = "" ]; then
    exit 1
fi

# Custom keybinds used below can be configured in your `fuzzel.ini`.
# Open the game's root directory `custom-0` (default: ALT+0)
if [ "$exit_code" -eq 19 ]; then
    dir=$(echo "$choice" | cut -d$'\t' -f3)
    [[ -d "$dir" ]] && xdg-open "$dir" # Open only if the directory exists
# Default case, run the command to launch the game
else
    cmd=$(echo "$choice" | cut -d$'\t' -f2)
    [[ ! "$cmd" = "" ]] && sh -c "$cmd"
fi
