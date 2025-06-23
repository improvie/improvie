# improvie

<div align="center">

**メディアファイルを整理しプレイリストを簡単に作成できる _"improve like the movie"_ をコンセプトにしたTauriアプリ**

[![License](https://img.shields.io/github/license/improvie/improvie?style=flat-square)](LICENSE)
[![Release](https://img.shields.io/github/v/release/improvie/improvie?style=flat-square)](https://github.com/improvie/improvie/releases/latest)
[![Stars](https://img.shields.io/github/stars/improvie/improvie?style=flat-square)](https://github.com/improvie/improvie/stargazers)

[English](./README.md) | 日本語

</div>

`improvie` は [Tauri](https://tauri.app/) で構築された軽量なソフトウェアで、
ローカルのビデオやオーディオファイルを簡単に管理できます。
メディアの閲覧、カスタムプレイリストの作成、
スムーズな再生をクリーンで直感的なインターフェースで提供します

## ✨ 機能

- 📂 **メディア管理**
  ローカルのビデオやオーディオファイルを簡単に管理。

- 🎵 **プレイリスト作成**
  お気に入りの曲やビデオをまとめてプレイリストを作成。

- 🎧 **軽量メディアプレーヤー**
  シンプルで直感的なインターフェースを持つメディアプレーヤー。

- ⚡ **Tauriで構築**
  Tauriを使用しており、軽量で高速なアプリケーション。

- 🌐 **多言語対応**
  日本語と英語をサポートし、ユーザーは好みの言語で使用可能。

- 🎨 **テーマカスタマイズ**
  アプリの見た目をカスタマイズ可能。

## 📥 インストール

最新リリースは[GitHubリリースページ](https://github.com/improvie/improvie/releases/latest)からダウンロードできます。

## 📸 スクリーンショット

<!-- Add screenshots here when available -->
<!-- ![screenshot](screenshots/main.png) -->

## 🤝 コントリビューション

コントリビューションを歓迎します！詳しくは[CONTRIBUTING-JA.md](CONTRIBUTING-JA.md)をご覧ください。

## 📜 ライセンス

このプロジェクトは[Apache 2.0](LICENSE)の下でライセンスされています。

### LINE Seed JP

このソフトウェアには、[LINE Seed JP](https://seed.line.me/index_jp.html)フォントが含まれており、[OFL-1.1](./src-tauri/licenses/OFL.txt)の下でライセンスされています。

### FFmpeg

このソフトウェアは、[FFmpeg](http://ffmpeg.org) のライブラリを静的にリンクしており、[LGPL-2.1](./src-tauri/licenses/LGPLv2.1.txt) の下でライセンスされています。

- **Version**: 7.1.1
- **License**: LGPLv2.1
- **Link Type**: Static
- **Build System**: [vcpkg](https://github.com/microsoft/vcpkg)
- **vcpkg Commit**: `3e5b8de5f6ebe844bee9d9eba0aed35c652e3c9c`
- **Build Info**: See [vcpkg.json](./vcpkg.json)

このソフトウェアはFFmpegを静的リンクしています。
LGPL v2.1ライセンスに基づき、ユーザーがFFmpegを変更または再リンクできるよう、
本アプリケーションのソースコードおよびビルド手順を提供しています。

## 🛠 開発

### 必要なツール

- [bun](https://bun.sh/): JavaScript all in one toolkit
- [rust](https://www.rust-lang.org/ja/tools/install): Rust programming language
- [tauri-cli](https://v2.tauri.app/ja/reference/cli/): Tauri CLI
- [FFmpeg](https://ffmpeg.org/): Video and audio processing library

### アプリの起動

`FFmpeg` をインストールしたならば、以下のコマンドでアプリが起動します。

```bash
cargo tauri dev
```

### アプリのビルド

`vcpkg` を使用して`FFmpeg`をビルドする必要があります。

```bash
# この`repository`のルートディレクトリで実行

# windows以外の場合
vcpkg install
# windowsの場合は、以下を実行してください。
vcpkg install --triplet x64-windows-static-md
```

そしたら `vcpkg_installed` というディレクトリができます。
そこでOSごとに別々にディレクトリーができます。(以下はあくまで一例です)

- windows: `vcpkg_installed/x64-windows-static-md`
- macOS: `vcpkg_installed/arm64-osx`
- linux: `vcpkg_installed/x64-linux`

その後、以下のコマンドでアプリをビルドします。

```bash
# arm64-osxのところは各自のOSに合わせてください
FFMPEG_DIR=$PWD/vcpkg_installed/arm64-osx cargo tauri build
```
