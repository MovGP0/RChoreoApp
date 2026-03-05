# App Icon Pipeline

## Source of Truth
- Canonical vector asset: `crates/choreo_components/ui/app_icon.svg`.
- `choreo_components_egui::shell::app_icon_svg()` exposes this canonical SVG for all egui targets.

## Desktop egui (`apps/desktop_egui`)
- Startup uses `app_icon::load_window_icon()` to rasterize the canonical SVG into 256x256 RGBA pixels.
- Rasterization stack: `usvg` parse -> `resvg` render -> `tiny-skia` pixmap -> `egui::IconData`.
- `eframe::NativeOptions.viewport` sets the window icon via `ViewportBuilder::with_icon(...)`.

## WASM egui (`apps/wasm_egui`)
- Browser favicon remains `apps/wasm_egui/app_icon.svg` referenced by `index.html`.
- That file must be kept byte-equivalent to the canonical SVG (`crates/choreo_components/ui/app_icon.svg`) to avoid branding drift.

## Android egui (`apps/android_egui`)
- This repository currently has no Android resource packaging tree (`AndroidManifest.xml`, `res/mipmap-*`, Gradle module).
- Launcher icon wiring must be tracked and implemented when packaging resources are added.
