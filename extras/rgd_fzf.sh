#!/usr/bin/env bash
# Helper script for using `rgd` with `fzf`, providing image previews.
# Usage: /path/to/rgd_fzf.sh

case "$1" in
    preview)
        # Preview, with box art image
        image="$(echo "$2" | cut -d$'\t' -f3)"
        # Image previews in fzf are tough, but experiment with some of the options below (and any
        # other methods of displaying images in the terminal that you're know of) and see
        # what works best
        if [ ! "$image" = "" ]; then
            magick - -colors 16 sixel:- < "$image" # Works best for me on wezterm, but produces a small image
            # Slow, large image, works well on wezterm but does not work on kitty
            # chafa -f sixel -s "${FZF_PREVIEW_COLUMNS}x${FZF_PREVIEW_LINES}" "$image"
            # Fast, large image, but blurry
            # viu -b "$image"
            # If you use kitty, this is the best option
            # kitty icat --clear --transfer-mode=memory --stdin=no --place="${FZF_PREVIEW_COLUMNS}x$FZF_PREVIEW_LINES"@0x0 "$image"
        # If no image was found, just display the game's properties in JSON format.
        else
            # If you don't need image previews, replace the entire if-else block with just this line
            rgd get --json "$(echo "$2" | cut -d$'\t' -f1)"
        fi
        ;;
    *)
        # 1. List entries
        # 2. Select an entry with `fzf`
        # 3. If an entry was selected, launch that entry
        rgd list --fields="title,launch-command,path-box-art" \
            | fzf -d $'\t' --with-nth 1 --preview "$(realpath "$0") preview {}" \
            | {
                read -r output && sh -c "$(echo "$output" | cut -d$'\t' -f2)"
            }
        ;;
esac
