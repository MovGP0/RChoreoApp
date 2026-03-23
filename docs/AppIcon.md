# App Icon Pipeline

## Source of Truth
- Canonical vector asset: `crates/choreo_components/assets/app_icon.svg`.
- `choreo_components::shell::app_icon_svg()` exposes this canonical SVG for all targets.
- Do not edit downstream copies first. Update the canonical SVG, then sync any packaging-specific copies from it.

## Desktop (`apps/desktop`)
- Startup uses `app_icon::load_window_icon()` to rasterize the canonical SVG into 256x256 RGBA pixels.
- Rasterization stack: `usvg` parse -> `resvg` render -> `tiny-skia` pixmap -> `egui::IconData`.
- `eframe::NativeOptions.viewport` sets the window icon via `ViewportBuilder::with_icon(...)`.
- Validation:
  - `apps/desktop/src/app_icon.rs` unit test checks that rasterization succeeds and produces a 256x256 RGBA buffer.

## WASM (`apps/wasm`)
- Browser favicon remains `apps/wasm/app_icon.svg` referenced by `index.html`.
- That file must be kept byte-equivalent to the canonical SVG (`crates/choreo_components/assets/app_icon.svg`) to avoid branding drift.
- Validation:
  - `crates/choreo_components/tests/shell/app_icon_spec.rs` asserts the canonical SVG contains the expected branding groups/colors and matches `apps/wasm/app_icon.svg` byte-for-byte.

## Android (`apps/android`)
- `cargo-apk` packaging metadata in `apps/android/Cargo.toml` includes `apps/android/res` and sets `application.icon = "@mipmap/ic_launcher"`.
- Generated launcher resources live in:
  - `apps/android/res/mipmap-mdpi/ic_launcher.png`
  - `apps/android/res/mipmap-hdpi/ic_launcher.png`
  - `apps/android/res/mipmap-xhdpi/ic_launcher.png`
  - `apps/android/res/mipmap-xxhdpi/ic_launcher.png`
  - `apps/android/res/mipmap-xxxhdpi/ic_launcher.png`
- Matching `ic_launcher_round.png` resources are generated alongside the standard launcher icons so Android packaging can reference either form.
- Regeneration:
  - Create a temporary script under `.temp/` that rasterizes `crates/choreo_components/assets/app_icon.svg` with the same `usvg`/`resvg`/`tiny-skia` stack used by desktop.
  - Write the Android density buckets at `48`, `72`, `96`, `144`, and `192` pixels to the `mipmap-*` directories above.
  - Delete `.temp/` after generation.
- Validation:
  - `crates/choreo_components/tests/shell/app_icon_spec.rs` asserts the Android launcher PNG resources exist with the expected density-specific dimensions.
