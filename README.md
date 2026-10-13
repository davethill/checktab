# checktab

A small floating multi-tab checklist app for [niri](https://niri.wf).

- Multiple checklists as tabs with horizontal scrolling
- Top toolbar:
  - Pin toggle (pinned lists display a pin icon and cannot be accidentally deleted)
  - Rename active list
  - Delete active list (requires confirmation `Sure?`; disabled on permanent tabs)
  - Create new list
- Add items with the input box (Enter), toggle checkboxes, remove items with `×`
- Footer actions:
  - "Clear checks": unchecks all items while preserving all item fields and list contents
  - "Clear completed": removes checked/completed items
  - Theme selector: switch between **Gruvbox**, **Nord**, **Catppuccin**, and **Dracula** (both Light and Dark versions)
- Built-in embedded icon font ensures icons render crisp and reliably across all Linux environments
- State and theme persist to `~/.local/share/checktab/checklists.json` (plain JSON)

## Build

```sh
nix develop   # shell.nix: rustc, cargo, clippy, jq, niri
cargo build --release
# -> target/release/checktab
```

## niri setup (already applied in ~/dotfiles)

`~/.config/niri/config.kdl`:

```kdl
window-rule {
    match app-id="checktab"
    open-floating true
    default-column-width { proportion 0.30; }
    default-window-height { fixed 600; }
}

binds {
    Mod+Shift+3 { spawn-sh "~/.config/niri/scripts/checktab.sh"; }
}
```

`~/.config/niri/scripts/checktab.sh` toggles: focus if unfocused, close if
focused, spawn if not running (same pattern as `scratchpad.sh`).

Reload niri config with `niri msg action reload` (or restart niri).

## TODO / M4 polish

- [ ] Nix derivation (`package.rustApplication`) so `checktab` is on `$PATH`
- [ ] Item edit-in-place (double-click)
- [ ] Reorder items / lists
- [ ] Match dark theme to niri accent exactly
- [ ] Optional: layer-shell (wlr-layer-shell) instead of floating window rule
