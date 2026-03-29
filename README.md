# just-write

A distraction-free plain text editor. Minimal UI, nothing else.

## Features

- Monospaced font, 18pt, black text on white background
- Text area is 80% of screen width (10% margin on each side)
- Toggle fullscreen with Shift+Cmd+F

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
