# RChoreoApp (Slint Hello World)

This workspace contains a minimal Slint hello world app for desktop, Android, and Web (WASM), plus shared Rust logic.

## Crates

- `crates/shared`: shared Rust logic
- `apps/desktop`: Windows/macOS/Linux (and iOS via cross-compile)
- `apps/android`: Android `cdylib` entry point
- `apps/wasm`: WebAssembly build (canvas-based)

## Desktop (Windows/macOS/Linux)

```sh
cargo run -p rchoreo_desktop
```

## Web (WASM)

```sh
cargo install wasm-pack
wasm-pack build --release --target web apps/wasm
```

Serve the `apps/wasm/index.html` over a local web server (the Slint runtime uses ES modules and won’t load from `file://`).

```sh
python -m http.server
```

## Android

Install the Android SDK/NDK and set `ANDROID_HOME`, `ANDROID_NDK_ROOT`, and optionally `JAVA_HOME`. Then build the `rchoreo_android` crate as a `cdylib` using your preferred Android tooling (for example `cargo-apk` or `cargo-ndk`) and target an Android ABI such as `aarch64-linux-android`.

Example with `cargo-apk`:

```sh
cargo install cargo-apk
cargo apk run --target aarch64-linux-android --lib -p rchoreo_android
```

## iOS

Slint supports iOS via the Winit backend and Skia renderer, and requires macOS + Xcode/Xcodegen. Add the iOS Rust targets, then cross-compile the `rchoreo_desktop` crate and wire it into an Xcode project as described in the Slint iOS guide:

```sh
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
cargo build --target aarch64-apple-ios -p rchoreo_desktop
```
