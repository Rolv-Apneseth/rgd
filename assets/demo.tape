Output demo.gif

Require cargo

Set Shell "bash"
Set FontSize 28
Set Width 1920
Set Height 1080
Set FontFamily "Inconsolata Nerd Font Mono"
Set PlaybackSpeed 0.5

Type "rgd list --fields title" Sleep 250ms  Enter
Sleep 1s
Type "clear" Sleep 50ms Enter

Type "rgd list --json | jq" Sleep 250ms Enter
Sleep 1s
Type "clear" Sleep 50ms Enter

Type "rgd list --fields title | sort | fzf --preview 'rgd get {} --json'"
Sleep 250ms Enter
Sleep 2s
Up 24 Sleep 150ms
Down 7 Sleep 150ms
Sleep 2s
