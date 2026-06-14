use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;

use egui::Color32;
use egui::Visuals;

use crate::ThemeMode;
use crate::styling::material_schemes::MaterialScheme;
use crate::styling::material_schemes::MaterialSchemes;

thread_local! {
    static CURRENT_MATERIAL_THEME: RefCell<Option<MaterialTheme>> = const { RefCell::new(None) };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialTheme {
    pub scheme: MaterialScheme,
    pub elevation: MaterialElevationTokens,
    pub overlay: MaterialOverlayTokens,
    pub state_layer_opacities: MaterialStateLayerOpacityTokens,
}

// Compatibility alias for existing callers. A Material 3 palette is a tonal
// palette; this type represents resolved scheme roles plus reusable UI tokens.
pub type MaterialPalette = MaterialTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialElevationTokens {
    pub shadow_15: Color32,
    pub shadow_30: Color32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialOverlayTokens {
    pub background_modal: Color32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialStateLayerOpacityTokens {
    pub hover: f32,
    pub focus: f32,
    pub press: f32,
    pub disabled: f32,
    pub drag: f32,
    pub content_disabled: f32,
}

impl Default for MaterialElevationTokens {
    fn default() -> Self {
        Self {
            shadow_15: rgba(0, 0, 0, 38),
            shadow_30: rgba(0, 0, 0, 77),
        }
    }
}

impl Default for MaterialOverlayTokens {
    fn default() -> Self {
        Self {
            background_modal: rgba(0, 0, 0, 128),
        }
    }
}

impl Default for MaterialStateLayerOpacityTokens {
    fn default() -> Self {
        Self {
            hover: 0.08,
            focus: 0.10,
            press: 0.10,
            disabled: 0.12,
            drag: 0.16,
            content_disabled: 0.38,
        }
    }
}

impl Deref for MaterialTheme {
    type Target = MaterialScheme;

    fn deref(&self) -> &Self::Target {
        &self.scheme
    }
}

impl DerefMut for MaterialTheme {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scheme
    }
}

impl MaterialTheme {
    #[must_use]
    pub fn light() -> Self {
        Self::from_scheme(MaterialSchemes::default().light)
    }

    #[must_use]
    pub fn dark() -> Self {
        Self::from_scheme(MaterialSchemes::default().dark)
    }

    #[must_use]
    pub fn from_dark_mode(is_dark: bool) -> Self {
        if is_dark { Self::dark() } else { Self::light() }
    }

    #[must_use]
    pub fn from_theme_mode(theme_mode: ThemeMode) -> Self {
        Self::from_dark_mode(matches!(theme_mode, ThemeMode::Dark))
    }

    #[must_use]
    pub fn from_visuals(visuals: &Visuals) -> Self {
        Self::from_dark_mode(visuals.dark_mode)
    }

    #[must_use]
    pub fn from_scheme(scheme: MaterialScheme) -> Self {
        from_scheme(scheme)
    }

    #[must_use]
    pub fn from_schemes(schemes: &MaterialSchemes, is_dark: bool) -> Self {
        Self::from_scheme(schemes.for_dark_mode(is_dark))
    }

    #[must_use]
    pub fn is_dark(self) -> bool {
        let red = f32::from(self.background.r()) / 255.0;
        let green = f32::from(self.background.g()) / 255.0;
        let blue = f32::from(self.background.b()) / 255.0;
        let luminance = (0.2126 * red) + (0.7152 * green) + (0.0722 * blue);
        luminance < 0.5
    }
}

impl Default for MaterialTheme {
    fn default() -> Self {
        Self::light()
    }
}

#[must_use]
pub fn material_palette(is_dark: bool) -> MaterialTheme {
    MaterialTheme::from_dark_mode(is_dark)
}

#[must_use]
pub fn material_palette_for_theme_mode(theme_mode: ThemeMode) -> MaterialTheme {
    MaterialTheme::from_theme_mode(theme_mode)
}

#[must_use]
pub fn material_palette_for_visuals(visuals: &Visuals) -> MaterialTheme {
    CURRENT_MATERIAL_THEME.with(|current| {
        current
            .borrow()
            .unwrap_or_else(|| MaterialTheme::from_visuals(visuals))
    })
}

#[must_use]
pub fn material_is_dark_for_visuals(visuals: &Visuals) -> bool {
    CURRENT_MATERIAL_THEME.with(|current| {
        current
            .borrow()
            .map_or(visuals.dark_mode, MaterialTheme::is_dark)
    })
}

#[must_use]
pub fn material_palette_for_theme(
    schemes: &MaterialSchemes,
    theme_mode: ThemeMode,
) -> MaterialTheme {
    MaterialTheme::from_schemes(schemes, matches!(theme_mode, ThemeMode::Dark))
}

pub fn sync_material_theme(schemes: &MaterialSchemes, theme_mode: ThemeMode) {
    let palette = material_palette_for_theme(schemes, theme_mode);
    let is_dark = matches!(theme_mode, ThemeMode::Dark);
    sync_egui_material3_theme(palette, is_dark);
}

fn sync_egui_material3_theme(palette: MaterialTheme, is_dark: bool) {
    let global_theme = egui_material3::get_global_theme();
    let Ok(theme) = global_theme.lock() else {
        return;
    };
    let mut next_theme = theme.clone();
    drop(theme);

    next_theme.theme_mode = if is_dark {
        egui_material3::ThemeMode::Dark
    } else {
        egui_material3::ThemeMode::Light
    };
    next_theme.selected_colors = material_theme_selected_colors(palette);
    egui_material3::update_global_theme(next_theme);
}

fn material_theme_selected_colors(theme: MaterialTheme) -> HashMap<String, Color32> {
    [
        ("primary", theme.primary),
        ("surfaceTint", theme.surface_tint),
        ("onPrimary", theme.on_primary),
        ("primaryContainer", theme.primary_container),
        ("onPrimaryContainer", theme.on_primary_container),
        ("secondary", theme.secondary),
        ("onSecondary", theme.on_secondary),
        ("secondaryContainer", theme.secondary_container),
        ("onSecondaryContainer", theme.on_secondary_container),
        ("tertiary", theme.tertiary),
        ("onTertiary", theme.on_tertiary),
        ("tertiaryContainer", theme.tertiary_container),
        ("onTertiaryContainer", theme.on_tertiary_container),
        ("error", theme.error),
        ("onError", theme.on_error),
        ("errorContainer", theme.error_container),
        ("onErrorContainer", theme.on_error_container),
        ("background", theme.background),
        ("onBackground", theme.on_background),
        ("surface", theme.surface),
        ("onSurface", theme.on_surface),
        ("surfaceVariant", theme.surface_variant),
        ("onSurfaceVariant", theme.on_surface_variant),
        ("outline", theme.outline),
        ("outlineVariant", theme.outline_variant),
        ("shadow", theme.shadow),
        ("scrim", theme.scrim),
        ("inverseSurface", theme.inverse_surface),
        ("inverseOnSurface", theme.inverse_on_surface),
        ("inversePrimary", theme.inverse_primary),
        ("primaryFixed", theme.primary_fixed),
        ("onPrimaryFixed", theme.on_primary_fixed),
        ("primaryFixedDim", theme.primary_fixed_dim),
        ("onPrimaryFixedVariant", theme.on_primary_fixed_variant),
        ("secondaryFixed", theme.secondary_fixed),
        ("onSecondaryFixed", theme.on_secondary_fixed),
        ("secondaryFixedDim", theme.secondary_fixed_dim),
        ("onSecondaryFixedVariant", theme.on_secondary_fixed_variant),
        ("tertiaryFixed", theme.tertiary_fixed),
        ("onTertiaryFixed", theme.on_tertiary_fixed),
        ("tertiaryFixedDim", theme.tertiary_fixed_dim),
        ("onTertiaryFixedVariant", theme.on_tertiary_fixed_variant),
        ("surfaceDim", theme.surface_dim),
        ("surfaceBright", theme.surface_bright),
        ("surfaceContainerLowest", theme.surface_container_lowest),
        ("surfaceContainerLow", theme.surface_container_low),
        ("surfaceContainer", theme.surface_container),
        ("surfaceContainerHigh", theme.surface_container_high),
        ("surfaceContainerHighest", theme.surface_container_highest),
    ]
    .into_iter()
    .map(|(name, color)| (name.to_string(), color))
    .collect()
}

pub fn with_current_material_palette<R>(palette: MaterialTheme, draw: impl FnOnce() -> R) -> R {
    CURRENT_MATERIAL_THEME.with(|current| {
        let previous = current.replace(Some(palette));
        let result = draw();
        current.replace(previous);
        result
    })
}

fn from_scheme(scheme: MaterialScheme) -> MaterialTheme {
    MaterialTheme {
        scheme,
        elevation: MaterialElevationTokens::default(),
        overlay: MaterialOverlayTokens::default(),
        state_layer_opacities: MaterialStateLayerOpacityTokens::default(),
    }
}

fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(red, green, blue, alpha)
}

#[cfg(test)]
mod tests {
    use crate::ThemeMode;

    use super::MaterialPalette;
    use super::material_is_dark_for_visuals;
    use super::material_palette;
    use super::material_palette_for_theme;
    use super::material_palette_for_visuals;
    use super::sync_material_theme;
    use super::with_current_material_palette;
    use crate::styling::material_schemes::MaterialSchemes;

    #[test]
    fn light_palette_matches_dynamic_default_light_scheme() {
        let palette = MaterialPalette::light();
        let default_schemes = MaterialSchemes::default();

        assert_eq!(palette.primary, default_schemes.light.primary);
        assert_eq!(
            palette.surface_container_highest,
            default_schemes.light.surface_container_highest
        );
        assert_eq!(palette.overlay.background_modal.a(), 128);
    }

    #[test]
    fn dark_palette_matches_dynamic_default_dark_scheme() {
        let palette = MaterialPalette::dark();
        let default_schemes = MaterialSchemes::default();

        assert_eq!(palette.primary, default_schemes.dark.primary);
        assert_eq!(palette.on_primary, default_schemes.dark.on_primary);
        assert_eq!(palette.surface_dim, default_schemes.dark.surface_dim);
    }

    #[test]
    fn theme_mode_selection_matches_requested_scheme() {
        assert_eq!(
            MaterialPalette::from_theme_mode(ThemeMode::Light).primary,
            MaterialPalette::light().primary
        );
        assert_eq!(
            MaterialPalette::from_theme_mode(ThemeMode::Dark).primary,
            MaterialPalette::dark().primary
        );
        assert_eq!(
            material_palette(true).primary,
            MaterialPalette::dark().primary
        );
    }

    #[test]
    fn state_layer_and_shadow_opacities_match_slint_constants() {
        let palette = MaterialPalette::default();

        assert_eq!(palette.state_layer_opacities.hover, 0.08);
        assert_eq!(palette.state_layer_opacities.focus, 0.10);
        assert_eq!(palette.state_layer_opacities.press, 0.10);
        assert_eq!(palette.state_layer_opacities.disabled, 0.12);
        assert_eq!(palette.state_layer_opacities.drag, 0.16);
        assert_eq!(palette.state_layer_opacities.content_disabled, 0.38);
        assert_eq!(palette.elevation.shadow_15.a(), 38);
        assert_eq!(palette.elevation.shadow_30.a(), 77);
    }

    #[test]
    fn frame_local_palette_overrides_visual_dark_mode_fallback() {
        let mut custom_palette = MaterialPalette::light();
        custom_palette.primary = egui::Color32::from_rgb(12, 34, 56);

        with_current_material_palette(custom_palette, || {
            let resolved = material_palette_for_visuals(&egui::Visuals::dark());
            assert_eq!(resolved.primary, custom_palette.primary);
        });
    }

    #[test]
    fn current_palette_darkness_overrides_visual_dark_mode_fallback() {
        let mut custom_palette = MaterialPalette::light();
        custom_palette.background = egui::Color32::from_rgb(10, 10, 10);

        with_current_material_palette(custom_palette, || {
            assert!(material_is_dark_for_visuals(&egui::Visuals::light()));
        });
    }

    #[test]
    fn sync_material_theme_updates_egui_material3_global_theme_colors() {
        let schemes = MaterialSchemes::from_seed_colors(Some("#FF336699"), None, None);
        let theme_mode = ThemeMode::Dark;
        let expected = material_palette_for_theme(&schemes, theme_mode);

        sync_material_theme(&schemes, theme_mode);

        assert_eq!(
            egui_material3::get_global_color("primary"),
            expected.primary
        );
        assert_eq!(
            egui_material3::get_global_color("onSurface"),
            expected.on_surface
        );
    }
}
