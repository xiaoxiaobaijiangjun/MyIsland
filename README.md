<div align="center">
  <img src="resources/icon.png" width="100" alt="MyIsland">
  <h1>MyIsland</h1>
  <p>A <b>Dynamic Island</b> for Windows — built with Rust + Skia.</p>
  <p>
    <img src="https://img.shields.io/badge/Rust-1.96+-orange?logo=rust" alt="Rust">
    <img src="https://img.shields.io/badge/license-GPL--3.0-blue" alt="License">
  </p>
</div>

## Features

- **🎵 Music + Synced Lyrics** — Windows SMTC integration, real-time lyrics with smooth transitions, album art, and audio visualizer
- **💧 Water Reminder** — Configurable interval (default 30min), active hours, full-island popup notification
- **⏱️ 3 Visual Styles** — Default / Mica (Win11) / Dynamic Color
- **✨ Smooth Animations** — Spring physics for all island transitions
- **🖱️ Scroll to Switch** — Mouse wheel cycles between Music and Lyrics pages
- **⚙️ Customizable** — Global scale, dock position, font, language, and more

## Download

Get the latest EXE from [Releases](https://github.com/xiaoxiaoguai/MyIsland/releases).

## Build from Source

```bash
# Requirements: Rust (MSVC toolchain) + Visual Studio 2022 Build Tools (C++ workload)

rustup default stable-msvc
cargo build --release
# Output: target/release/MyIsland.exe
```

### Plugin API

MyIsland supports external plugins via the `myisland-plugin-api` crate. Place compiled `.dll` files in `%APPDATA%/MyIsland/plugins/{plugin-name}/`.

## Usage

- **Double-click** the island to expand/collapse
- **Scroll wheel** while expanded to switch between Music and Lyrics pages
- **Right-click tray icon** for Settings, Show/Hide, and Exit
- **Settings** → **General** → **Water Reminder** to enable drinking reminders

## License

GPL-3.0
