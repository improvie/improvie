# improvie

<div align="center">

**A Tauri application with the concept of _"improve like the movie"_ that allows you to organize media file and easily create playlists**

[![License](https://img.shields.io/github/license/improvie/improvie?style=flat-square)](LICENSE)
[![Release](https://img.shields.io/github/v/release/improvie/improvie?style=flat-square)](https://github.com/improvie/improvie/releases/latest)
[![Stars](https://img.shields.io/github/stars/improvie/improvie?style=flat-square)](https://github.com/improvie/improvie/stargazers)

English | [日本語](./README-JA.md)

</div>

`improvie` is a lightweight software built with [Tauri](https://tauri.app/),
allowing you to easily manage local video and audio files.
It provides a clean and intuitive interface for browsing media,
creating custom playlists, and smooth playback.

## ✨ Features

- 📂 **Media Management**
  Easily manage local video and audio files.

- 🎵 **Playlist Creation**
  Create playlists by compiling your favorite songs and videos.

- 🎧 **Lightweight Media Player**
  A media player with a simple and intuitive interface.

- ⚡ **Built with Tauri**
  Uses Tauri for a lightweight and fast application.

- 🌐 **Multilingual Support**
  Supports Japanese and English, allowing users to use it in their preferred language.

- 🎨 **Theme Customization**
  Customize the appearance of the application.

## 📥 Installation

The latest release can be downloaded from the [GitHub Releases page](https://github.com/improvie/improvie/releases/latest).

## 📸 Screenshots

<!-- Add screenshots here when available -->
<!-- ![screenshot](screenshots/main.png) -->

## 🤝 Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## 📜 License

This project is licensed under the [Apache 2.0](LICENSE).

### LINE Seed JP

This software includes the [LINE Seed JP](https://seed.line.me/index_jp.html) font, which is licensed under the [OFL-1.1](./src-tauri/licenses/OFL.txt).

### FFmpeg

This software statically links the [FFmpeg](http://ffmpeg.org) library, licensed under [LGPL-2.1](./src-tauri/licenses/LGPLv2.1.txt).

- **Version**: 7.1.1
- **License**: LGPLv2.1
- **Link Type**: Static
- **Build System**: [vcpkg](https://github.com/microsoft/vcpkg)
- **vcpkg Commit**: `3e5b8de5f6ebe844bee9d9eba0aed35c652e3c9c`
- **Build Info**: See [vcpkg.json](./vcpkg.json)

This software statically links FFmpeg.
In compliance with the LGPL v2.1 license,
the source code of this application and
its build instructions are provided to allow users to modify or re-link FFmpeg.

## 🛠 Development

### Required Tools

- [bun](https://bun.sh/): JavaScript all-in-one toolkit
- [rust](https://www.rust-lang.org/tools/install): Rust programming language
- [tauri-cli](https://v2.tauri.app/reference/cli/): Tauri CLI
- [FFmpeg](https://ffmpeg.org/): Video and audio processing library

### Running the Application

After installing `FFmpeg`, you can start the application with the following command:

```bash
cargo tauri dev
```

### Building the Application

You need to build `FFmpeg` using `vcpkg`.

```bash
# Run this in the root directory of this repository

# For non-Windows systems
vcpkg install
# For Windows systems, run the following instead:
vcpkg install --triplet x64-windows-static-md
```

This will create a directory called `vcpkg_installed`, which will contain subdirectories specific to each OS (the following are examples):

- Windows: `vcpkg_installed/x64-windows-static-md`
- macOS: `vcpkg_installed/arm64-osx`
- Linux: `vcpkg_installed/x64-linux`

Afterward, build the application with the following command:

```bash
# Replace 'arm64-osx' with your OS-specific directory
# `--features static` is static linking of FFmpeg (default is dynamic linking)
FFMPEG_DIR=$PWD/vcpkg_installed/arm64-osx cargo tauri build --features static
```
