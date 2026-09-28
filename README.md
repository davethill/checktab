# checktab

A small floating multi-tab checklist app for [niri](https://niri.wf).

- Multiple checklists as tabs (`+` to create, double-click a tab to rename,
  `+` turns into `Sure?` to delete the active list)
- Add items with the input box (Enter), toggle by clicking, remove with `×`
- "Clear completed" in the footer
- State persists to `~/.local/share/checktab/checklists.json` (plain JSON)

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
