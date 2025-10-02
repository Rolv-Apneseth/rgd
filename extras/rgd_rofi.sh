#!/bin/env bash
# Custom mode for `rofi`, including box art images.
# Usage: rofi -modi games:/path/to/rgd_rofi.sh -show games -show-icons -kb-custom-1 'Control+Shift+space'
#
# Tip: use a theme, such as the one from `rofi-games`, to display the box art images more prominently.

rofi_list()
{
    # 1. List entries
    # 2. Skip entries which don't have box art
    # 3. Format results for use by Rofi
    rgd list --fields="title,path-box-art,launch-command,path-game-dir" \
        | grep -P -v '\t\t' \
        | awk -F '\t' '{print $1 "\000icon\037" $2 "\037info\037" $1 "\t" $3 "\t" $4 "\n"}'
}

case $ROFI_RETV in
    # Display entries on startup
    0)
        echo -en "\000use-hot-keys\037true\n" # Enable custom keys
        rofi_list
        ;;
    # Handle regular select (kb-accept-entry, default: <Enter>)
    1)
        if [ ! "$ROFI_INFO" = "" ]; then
            # Run launch command for the selected game
            coproc sh -c "$(echo "$ROFI_INFO" | cut -d$'\t' -f2)" > /dev/null 2>&1
        fi
        exit
        ;;
    # Handle custom keybind 1 (kb-custom-1) - open game's root directory
    # For more about custom rofi keybinds, see <https://davatorium.github.io/rofi/current/rofi-keys.5>
    10)
        if [ ! "$ROFI_INFO" = "" ]; then
            dir=$(echo "$ROFI_INFO" | cut -d$'\t' -f3)
            # Open the root directory for the selected game if it exists
            if [ ! "$dir" = "" ]; then
                coproc xdg-open "$dir" > /dev/null 2>&1
            # Otherwise, reset the list
            else
                rofi_list
            fi
        fi
        exit
        ;;
    # Other keybinds just reset the list
    *) rofi_list ;;
esac
