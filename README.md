# ChoreoApp (Rust+Slint Version)

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

Serve the `apps/wasm/index.html` over a local web server
(the Slint runtime uses ES modules and won’t load from `file://`).

```sh
python -m http.server
```

## Android

### Android prerequisites

1. Install Rust with rustup and add the Android targets you plan to build.
2. Install the Android SDK and NDK (Android Studio is fine). Ensure these SDK components are installed:
   - Android SDK Platform (an API level, e.g., Android 34)
   - Android SDK Build-Tools
   - Platform Tools
   - NDK (Side by side)
3. Set:
   - `ANDROID_HOME` to the SDK path (or `ANDROID_SDK_ROOT`, though it is deprecated).
   - `ANDROID_NDK_HOME` if you want to pin the NDK path (otherwise `cargo-ndk` auto-detects the latest NDK from the SDK).

Example (PowerShell):
```powershell
$env:ANDROID_HOME = "$env:LOCALAPPDATA\Android\Sdk"
$env:ANDROID_SDK_ROOT = $env:ANDROID_HOME
$env:ANDROID_NDK_HOME = "$env:LOCALAPPDATA\Android\Sdk\ndk\<version>"
```

Then build the `rchoreo_android` crate as a `cdylib`
using your preferred Android tooling (for example `cargo-apk` or `cargo-ndk`)
and target an Android ABI such as `aarch64-linux-android`.

### Android APK builds (cargo-apk)

Use `cargo-apk` when you want a straightforward APK build/run workflow. After
installing it, target an Android ABI such as `aarch64-linux-android`:

```sh
cargo install cargo-apk
cargo apk build --target aarch64-linux-android --lib -p rchoreo_android
```

To control where build artifacts land, pass `--target-dir` (or set `CARGO_TARGET_DIR`):

```sh
cargo apk build --target aarch64-linux-android --lib -p rchoreo_android --target-dir target/android
```

APK output location (default):
`target/aarch64-linux-android/debug/apk/rchoreo_android.apk`

APK output location (with `--target-dir target/android`):
`target/android/aarch64-linux-android/debug/apk/rchoreo_android.apk`

### Android NDK builds (cargo-ndk)

Install the Rust targets you need, then install `cargo-ndk`, and build for the
desired Android ABIs. If you installed the NDK via Android Studio, `cargo-ndk`
auto-detects the latest NDK; you can override with `ANDROID_NDK_HOME`.

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
cargo install cargo-ndk
cargo ndk -t armeabi-v7a -t arm64-v8a -o ./jniLibs build --release
```

To control cargo build artifacts, set `CARGO_TARGET_DIR`, and use `-o` to place
JNI libs under a known folder:

```sh
CARGO_TARGET_DIR=target/android cargo ndk -t armeabi-v7a -t arm64-v8a -o target/android/jniLibs build --release
```

**Reference:** [cargo-ndk](https://github.com/bbqsrc/cargo-ndk)

### Load the APK in the Android emulator

1. Start an emulator from Android Studio (Device Manager), or via the `emulator` CLI.
2. Install the APK with `adb`:

Example (default target dir):
```sh
adb install -r target/aarch64-linux-android/debug/apk/rchoreo_android.apk
```

Example (custom `--target-dir target/android`):
```sh
adb install -r target/android/aarch64-linux-android/debug/apk/rchoreo_android.apk
```

3. Launch it from the emulator’s app list.

## iOS

Slint supports iOS via the Winit backend and Skia renderer, and requires macOS + Xcode/Xcodegen.
Add the iOS Rust targets, then cross-compile the `rchoreo_desktop` crate
and wire it into an Xcode project as described in the Slint iOS guide:

```sh
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
cargo build --target aarch64-apple-ios -p rchoreo_desktop
```
