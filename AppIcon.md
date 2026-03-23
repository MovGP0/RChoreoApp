# App Icon Pipeline

## Source of Truth
- Canonical vector asset: `crates/choreo_components_egui/assets/app_icon.svg`.
- `choreo_components_egui::shell::app_icon_svg()` exposes this canonical SVG for all egui targets.
- Do not edit downstream copies first. Update the canonical SVG, then sync any packaging-specific copies from it.

## Desktop egui (`apps/desktop_egui`)
- Startup uses `app_icon::load_window_icon()` to rasterize the canonical SVG into 256x256 RGBA pixels.
- Rasterization stack: `usvg` parse -> `resvg` render -> `tiny-skia` pixmap -> `egui::IconData`.
- `eframe::NativeOptions.viewport` sets the window icon via `ViewportBuilder::with_icon(...)`.
- Validation:
  - `apps/desktop_egui/src/app_icon.rs` unit test checks that rasterization succeeds and produces a 256x256 RGBA buffer.

## WASM egui (`apps/wasm_egui`)
- Browser favicon remains `apps/wasm_egui/app_icon.svg` referenced by `index.html`.
- That file must be kept byte-equivalent to the canonical SVG (`crates/choreo_components_egui/assets/app_icon.svg`) to avoid branding drift.
- Validation:
  - `crates/choreo_components_egui/tests/shell/app_icon_spec.rs` asserts the canonical SVG contains the expected branding groups/colors and matches `apps/wasm_egui/app_icon.svg` byte-for-byte.

## Android egui (`apps/android_egui`)
- `cargo-apk` packaging metadata in `apps/android_egui/Cargo.toml` includes `apps/android_egui/res` and sets `application.icon = "@mipmap/ic_launcher"`.
- Generated launcher resources live in:
  - `apps/android_egui/res/mipmap-mdpi/ic_launcher.png`
  - `apps/android_egui/res/mipmap-hdpi/ic_launcher.png`
  - `apps/android_egui/res/mipmap-xhdpi/ic_launcher.png`
  - `apps/android_egui/res/mipmap-xxhdpi/ic_launcher.png`
  - `apps/android_egui/res/mipmap-xxxhdpi/ic_launcher.png`
- Matching `ic_launcher_round.png` resources are generated alongside the standard launcher icons so Android packaging can reference either form.
- Regeneration:
  - Create a temporary script under `.temp/` that rasterizes `crates/choreo_components_egui/assets/app_icon.svg` with the same `usvg`/`resvg`/`tiny-skia` stack used by desktop egui.
  - Write the Android density buckets at `48`, `72`, `96`, `144`, and `192` pixels to the `mipmap-*` directories above.
  - Delete `.temp/` after generation.
- Validation:
  - `crates/choreo_components_egui/tests/shell/app_icon_spec.rs` asserts the Android launcher PNG resources exist with the expected density-specific dimensions.
