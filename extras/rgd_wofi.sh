#!/usr/bin/env bash
# Helper script for using `rgd` with `wofi`, providing image previews.
# Usage: /path/to/rgd_wofi.sh

# 1. List games
# 2. Remove entries without box art
# 3. Format entries for display with `wofi`
# 4. Use `wofi` to display entries, showing the image and title only
# 5. If an entry was selected, run the command to launch the game
rgd list --fields="title,path-box-art,launch-command" \
    | grep -P -v '\t\t' \
    | awk -F '\t' '{print "img:" $2 ":text:\t  " $1 "\t" $3}' \
    | wofi -I --dmenu --prompt "Games" --pre-display-cmd="echo \"%s\" | awk -F '\t' 'NF{NF-=1};1'" -Dimage_size=150 -Dynamic_lines=true --insensitive \
    | {
        read -r output && sh -c "$(echo "$output" | cut -d$'\t' -f3)"
    }
