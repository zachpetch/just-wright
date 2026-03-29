# just-write

A distraction-free plain text editor. Full screen, minimal UI, nothing else.

## Features

- Always full screen with no window decorations
- Monospaced font, 18pt, light background (#eeeeee) with dark text (#333333)
- Text area is 80% of screen width (10% margin on each side)

## Keyboard shortcuts

| Shortcut | Action |
|---|---|
| Cmd+N | New file |
| Cmd+O | Open file |
| Cmd+S | Save (overwrites if previously saved, otherwise prompts) |
| Shift+Cmd+S | Save as |
| Shift+Cmd+F | Toggle fullscreen |
| Cmd+Z | Undo |
| Shift+Cmd+Z | Redo |
| Cmd+Q | Quit |

## Building

Requires [Rust](https://rustup.rs/) (edition 2024).

```
cargo build --release
```

The binary will be at `target/release/just-write`.

## Running

```
cargo run --release
```
