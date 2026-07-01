# Alacritty — macOS-focused fork

This is my personal fork of [Alacritty](https://github.com/alacritty/alacritty),
a fast, cross-platform OpenGL terminal emulator. It keeps Alacritty's core
behavior while adding a scrollbar and several macOS quality-of-life features.

## Added features

- A configurable scrollbar with thumb dragging and track clicking. The
  scrollbar is available on all supported platforms and disabled by default.
- Terminal dimensions (`columns×rows`) in macOS window titles.
- A custom macOS application icon.
- A centered first window, with subsequent macOS window cascading when
  no explicit window position is configured.
- A macOS **Settings…** menu item and <kbd>⌘</kbd>+<kbd>,</kbd> shortcut that
  open the active Alacritty configuration file in the default text editor.
- An `OpenConfig` binding action for assigning another macOS shortcut to the
  same behavior.

All other features come from upstream Alacritty. See the
[upstream feature overview](docs/features.md) for more information.

## Installation on macOS

This fork does not provide prebuilt binaries. Install the stable Rust toolchain
and the build requirements described in [INSTALL.md](INSTALL.md), then build the
application from source:

```sh
git clone --branch sfsam https://github.com/sfsam/alacritty.git
cd alacritty
make app
cp -r target/release/osx/Alacritty.app /Applications/
```

`make app` builds for the current architecture. To build a universal application
for both Apple silicon and Intel Macs, follow the
[universal binary instructions](INSTALL.md#universal-binary).

## Scrollbar configuration

Enable and customize the scrollbar in `alacritty.toml`:

```toml
[window.scrollbar]
show = true
track_color = "#222222"
thumb_color = "#444444"
```

The scrollbar reserves 12 logical pixels along the right edge. In a window, the
window grows to preserve the configured number of terminal columns. In
fullscreen mode, the scrollbar uses space from the terminal grid instead.

The defaults are `false`, `"#222222"`, and `"#444444"` respectively. Full fork
configuration documentation is available in
[`extra/man/alacritty.5.scd`](extra/man/alacritty.5.scd).

To bind the configuration-file action explicitly:

```toml
[[keyboard.bindings]]
key = "Comma"
mods = "Command"
action = "OpenConfig"
```

## License

This project is based on the work of the Alacritty maintainers and contributors.
It is not affiliated with or supported by the upstream project.

Alacritty is distributed under the terms of the
[Apache License 2.0](LICENSE-APACHE) or the [MIT License](LICENSE-MIT), at your
option.
