[![Crates.io](https://img.shields.io/crates/v/bevy_wicon)](https://crates.io/crates/bevy_wicon)

# Bevy Wicon

The window icon on Windows (title bar, taskbar and Alt+Tab) for [Bevy](https://bevyengine.org), from one PNG image built
into your program. (On Linux with X11, the title bar too; other platforms ignore it.)

## How to use

Add the `WindowIconPlugin` with a square PNG image, preferably 256 x 256:
```rust
App::new().add_plugins(WindowIconPlugin::new(include_bytes!("../assets/icon.png")));
```

The taskbar (and Alt+Tab) show the image as it is (at most 256 x 256). The title bar shows it resized to 32 x 32 (16 x 16
at 100% display scale).

## Version Compatibility

| bevy | bevy_wicon |
|------|------------|
| 0.18.x | 0.18.x |
| 0.20.x | 0.20.x |
