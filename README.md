# ThinkPad Macropad

A small Linux keyboard macro tool.

The idea is to use the keyboard numpad as an extra layer of shortcuts.  
When macro mode is enabled, numpad keys can launch applications or run custom actions.

## Setup

Currently developed and tested on:

- Lenovo ThinkPad P16v Gen 1
- Ubuntu 26.04
- GNOME + Wayland

## Current features

- Listen to keyboard events using `evdev`
- Use NumLock as a macro layer toggle
- Launch applications from numpad shortcuts

## Example

With macro mode enabled:

| Key      | Action                 |
| -------- | ---------------------- |
| NumPad 0 | Open Ulauncher         |
| NumPad 1 | Open Firefox           |
| NumPad 2 | Open GNOME Text Editor |
| NumPad 3 | Open Home dir          |

## Build

Requires Rust.

```bash
cargo build --release
```

Run:

```bash
./target/release/thinkpad-macropad
```

## Notes

Currently the keyboard device is hardcoded (`/dev/input/event3`).

This will probably change later to automatically detect the correct keyboard device.

## Ideas

- [ ] Add instructions to add it as a service on the README
- [ ] Config file for custom shortcuts
- [ ] Run as a background service
- [ ] Support more actions
- [ ] Add layers/macros
