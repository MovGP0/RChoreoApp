use std::cell::RefCell;
use std::collections::HashMap;

use egui::Color32;
use egui::Context;
use egui::Visuals;

use crate::material::styling::material_schemes::MaterialScheme;
use crate::material::styling::material_schemes::MaterialSchemes;
use crate::settings::state::SettingsState;
use crate::settings::state::ThemeMode;

thread_local! {
    static CURRENT_MATERIAL_PALETTE: RefCell<Option<MaterialPalette>> = const { RefCell::new(None) };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialPalette {
    pub primary: Color32,
    pub surface_tint: Color32,
    pub on_primary: Color32,
    pub primary_container: Color32,
    pub on_primary_container: Color32,
    pub secondary: Color32,
    pub on_secondary: Color32,
    pub secondary_container: Color32,
    pub on_secondary_container: Color32,
    pub tertiary: Color32,
    pub on_tertiary: Color32,
    pub tertiary_container: Color32,
    pub on_tertiary_container: Color32,
    pub error: Color32,
    pub on_error: Color32,
    pub error_container: Color32,
    pub on_error_container: Color32,
    pub background: Color32,
    pub on_background: Color32,
    pub surface: Color32,
    pub on_surface: Color32,
    pub surface_variant: Color32,
    pub on_surface_variant: Color32,
    pub outline: Color32,
    pub outline_variant: Color32,
    pub shadow: Color32,
    pub scrim: Color32,
    pub inverse_surface: Color32,
    pub inverse_on_surface: Color32,
    pub inverse_primary: Color32,
    pub primary_fixed: Color32,
    pub on_primary_fixed: Color32,
    pub primary_fixed_dim: Color32,
    pub on_primary_fixed_variant: Color32,
    pub secondary_fixed: Color32,
    pub on_secondary_fixed: Color32,
    pub secondary_fixed_dim: Color32,
    pub on_secondary_fixed_variant: Color32,
    pub tertiary_fixed: Color32,
    pub on_tertiary_fixed: Color32,
    pub tertiary_fixed_dim: Color32,
    pub on_tertiary_fixed_variant: Color32,
    pub surface_dim: Color32,
    pub surface_bright: Color32,
    pub surface_container_lowest: Color32,
    pub surface_container_low: Color32,
    pub surface_container: Color32,
    pub surface_container_high: Color32,
    pub surface_container_highest: Color32,
    pub shadow_15: Color32,
    pub shadow_30: Color32,
    pub background_modal: Color32,
    pub state_layer_opacity_hover: f32,
    pub state_layer_opacity_focus: f32,
    pub state_layer_opacity_press: f32,
    pub state_layer_opacity_disabled: f32,
    pub state_layer_opacity_drag: f32,
    pub disable_opacity: f32,
}

impl MaterialPalette {
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
}

impl Default for MaterialPalette {
    fn default() -> Self {
        Self::light()
    }
}

#[must_use]
pub fn material_palette(is_dark: bool) -> MaterialPalette {
    MaterialPalette::from_dark_mode(is_dark)
}

#[must_use]
pub fn material_palette_for_theme_mode(theme_mode: ThemeMode) -> MaterialPalette {
    MaterialPalette::from_theme_mode(theme_mode)
}

#[must_use]
pub fn material_palette_for_visuals(visuals: &Visuals) -> MaterialPalette {
    CURRENT_MATERIAL_PALETTE.with(|current| {
        current
            .borrow()
            .unwrap_or_else(|| MaterialPalette::from_visuals(visuals))
    })
}

#[must_use]
pub fn material_palette_for_settings_state(state: &SettingsState) -> MaterialPalette {
    MaterialPalette::from_schemes(
        &state.material_scheme,
        matches!(state.theme_mode, ThemeMode::Dark),
    )
}

pub fn apply_material_visuals(context: &Context, state: &SettingsState) {
    let palette = material_palette_for_settings_state(state);
    let is_dark = matches!(state.theme_mode, ThemeMode::Dark);
    sync_egui_material3_theme(palette, is_dark);
    let mut visuals = if is_dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };

    visuals.override_text_color = Some(palette.on_background);
    visuals.hyperlink_color = palette.primary;
    visuals.faint_bg_color = palette.surface_container_low;
    visuals.extreme_bg_color = palette.surface_container_highest;
    visuals.code_bg_color = palette.surface_container_high;
    visuals.warn_fg_color = palette.error;
    visuals.error_fg_color = palette.error;
    visuals.window_fill = palette.surface_container;
    visuals.panel_fill = palette.background;
    visuals.window_stroke.color = palette.outline_variant;
    visuals.selection.bg_fill = palette.secondary_container;
    visuals.selection.stroke.color = palette.on_secondary_container;

    visuals.widgets.noninteractive.bg_fill = palette.surface_container_low;
    visuals.widgets.noninteractive.weak_bg_fill = palette.surface_container_low;
    visuals.widgets.noninteractive.bg_stroke.color = palette.outline_variant;
    visuals.widgets.noninteractive.fg_stroke.color = palette.on_surface;

    visuals.widgets.inactive.bg_fill = palette.surface_container;
    visuals.widgets.inactive.weak_bg_fill = palette.surface_container;
    visuals.widgets.inactive.bg_stroke.color = palette.outline_variant;
    visuals.widgets.inactive.fg_stroke.color = palette.on_surface;

    visuals.widgets.hovered.bg_fill = palette.surface_container_high;
    visuals.widgets.hovered.weak_bg_fill = palette.surface_container_high;
    visuals.widgets.hovered.bg_stroke.color = palette.outline;
    visuals.widgets.hovered.fg_stroke.color = palette.on_surface;

    visuals.widgets.active.bg_fill = palette.primary_container;
    visuals.widgets.active.weak_bg_fill = palette.primary_container;
    visuals.widgets.active.bg_stroke.color = palette.primary;
    visuals.widgets.active.fg_stroke.color = palette.on_primary_container;

    visuals.widgets.open.bg_fill = palette.secondary_container;
    visuals.widgets.open.weak_bg_fill = palette.secondary_container;
    visuals.widgets.open.bg_stroke.color = palette.secondary;
    visuals.widgets.open.fg_stroke.color = palette.on_secondary_container;

    context.set_visuals(visuals);
}

fn sync_egui_material3_theme(palette: MaterialPalette, is_dark: bool) {
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
    next_theme.selected_colors = material_palette_selected_colors(palette);
    egui_material3::update_global_theme(next_theme);
}

fn material_palette_selected_colors(palette: MaterialPalette) -> HashMap<String, Color32> {
    [
        ("primary", palette.primary),
        ("surfaceTint", palette.surface_tint),
        ("onPrimary", palette.on_primary),
        ("primaryContainer", palette.primary_container),
        ("onPrimaryContainer", palette.on_primary_container),
        ("secondary", palette.secondary),
        ("onSecondary", palette.on_secondary),
        ("secondaryContainer", palette.secondary_container),
        ("onSecondaryContainer", palette.on_secondary_container),
        ("tertiary", palette.tertiary),
        ("onTertiary", palette.on_tertiary),
        ("tertiaryContainer", palette.tertiary_container),
        ("onTertiaryContainer", palette.on_tertiary_container),
        ("error", palette.error),
        ("onError", palette.on_error),
        ("errorContainer", palette.error_container),
        ("onErrorContainer", palette.on_error_container),
        ("background", palette.background),
        ("onBackground", palette.on_background),
        ("surface", palette.surface),
        ("onSurface", palette.on_surface),
        ("surfaceVariant", palette.surface_variant),
        ("onSurfaceVariant", palette.on_surface_variant),
        ("outline", palette.outline),
        ("outlineVariant", palette.outline_variant),
        ("shadow", palette.shadow),
        ("scrim", palette.scrim),
        ("inverseSurface", palette.inverse_surface),
        ("inverseOnSurface", palette.inverse_on_surface),
        ("inversePrimary", palette.inverse_primary),
        ("primaryFixed", palette.primary_fixed),
        ("onPrimaryFixed", palette.on_primary_fixed),
        ("primaryFixedDim", palette.primary_fixed_dim),
        ("onPrimaryFixedVariant", palette.on_primary_fixed_variant),
        ("secondaryFixed", palette.secondary_fixed),
        ("onSecondaryFixed", palette.on_secondary_fixed),
        ("secondaryFixedDim", palette.secondary_fixed_dim),
        ("onSecondaryFixedVariant", palette.on_secondary_fixed_variant),
        ("tertiaryFixed", palette.tertiary_fixed),
        ("onTertiaryFixed", palette.on_tertiary_fixed),
        ("tertiaryFixedDim", palette.tertiary_fixed_dim),
        ("onTertiaryFixedVariant", palette.on_tertiary_fixed_variant),
        ("surfaceDim", palette.surface_dim),
        ("surfaceBright", palette.surface_bright),
        ("surfaceContainerLowest", palette.surface_container_lowest),
        ("surfaceContainerLow", palette.surface_container_low),
        ("surfaceContainer", palette.surface_container),
        ("surfaceContainerHigh", palette.surface_container_high),
        ("surfaceContainerHighest", palette.surface_container_highest),
    ]
    .into_iter()
    .map(|(name, color)| (name.to_string(), color))
    .collect()
}

pub fn with_current_material_palette<R>(palette: MaterialPalette, draw: impl FnOnce() -> R) -> R {
    CURRENT_MATERIAL_PALETTE.with(|current| {
        let previous = current.replace(Some(palette));
        let result = draw();
        current.replace(previous);
        result
    })
}

fn from_scheme(scheme: MaterialScheme) -> MaterialPalette {
    MaterialPalette {
        primary: scheme.primary,
        surface_tint: scheme.surface_tint,
        on_primary: scheme.on_primary,
        primary_container: scheme.primary_container,
        on_primary_container: scheme.on_primary_container,
        secondary: scheme.secondary,
        on_secondary: scheme.on_secondary,
        secondary_container: scheme.secondary_container,
        on_secondary_container: scheme.on_secondary_container,
        tertiary: scheme.tertiary,
        on_tertiary: scheme.on_tertiary,
        tertiary_container: scheme.tertiary_container,
        on_tertiary_container: scheme.on_tertiary_container,
        error: scheme.error,
        on_error: scheme.on_error,
        error_container: scheme.error_container,
        on_error_container: scheme.on_error_container,
        background: scheme.background,
        on_background: scheme.on_background,
        surface: scheme.surface,
        on_surface: scheme.on_surface,
        surface_variant: scheme.surface_variant,
        on_surface_variant: scheme.on_surface_variant,
        outline: scheme.outline,
        outline_variant: scheme.outline_variant,
        shadow: scheme.shadow,
        scrim: scheme.scrim,
        inverse_surface: scheme.inverse_surface,
        inverse_on_surface: scheme.inverse_on_surface,
        inverse_primary: scheme.inverse_primary,
        primary_fixed: scheme.primary_fixed,
        on_primary_fixed: scheme.on_primary_fixed,
        primary_fixed_dim: scheme.primary_fixed_dim,
        on_primary_fixed_variant: scheme.on_primary_fixed_variant,
        secondary_fixed: scheme.secondary_fixed,
        on_secondary_fixed: scheme.on_secondary_fixed,
        secondary_fixed_dim: scheme.secondary_fixed_dim,
        on_secondary_fixed_variant: scheme.on_secondary_fixed_variant,
        tertiary_fixed: scheme.tertiary_fixed,
        on_tertiary_fixed: scheme.on_tertiary_fixed,
        tertiary_fixed_dim: scheme.tertiary_fixed_dim,
        on_tertiary_fixed_variant: scheme.on_tertiary_fixed_variant,
        surface_dim: scheme.surface_dim,
        surface_bright: scheme.surface_bright,
        surface_container_lowest: scheme.surface_container_lowest,
        surface_container_low: scheme.surface_container_low,
        surface_container: scheme.surface_container,
        surface_container_high: scheme.surface_container_high,
        surface_container_highest: scheme.surface_container_highest,
        shadow_15: rgba(0, 0, 0, 38),
        shadow_30: rgba(0, 0, 0, 77),
        background_modal: rgba(0, 0, 0, 128),
        state_layer_opacity_hover: 0.08,
        state_layer_opacity_focus: 0.10,
        state_layer_opacity_press: 0.10,
        state_layer_opacity_disabled: 0.12,
        state_layer_opacity_drag: 0.16,
        disable_opacity: 0.38,
    }
}

fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(red, green, blue, alpha)
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use crate::settings::actions::SettingsAction;
    use crate::settings::reducer::reduce;
    use crate::settings::state::SettingsState;

    use super::apply_material_visuals;
    use super::MaterialPalette;
    use super::material_palette;
    use super::material_palette_for_settings_state;
    use super::material_palette_for_visuals;
    use super::with_current_material_palette;
    use crate::material::styling::material_schemes::MaterialSchemes;
    use crate::settings::state::ThemeMode;

    #[test]
    fn light_palette_matches_dynamic_default_light_scheme() {
        let palette = MaterialPalette::light();
        let default_schemes = MaterialSchemes::default();

        assert_eq!(palette.primary, default_schemes.light.primary);
        assert_eq!(
            palette.surface_container_highest,
            default_schemes.light.surface_container_highest
        );
        assert_eq!(palette.background_modal.a(), 128);
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

        assert_eq!(palette.state_layer_opacity_hover, 0.08);
        assert_eq!(palette.state_layer_opacity_focus, 0.10);
        assert_eq!(palette.state_layer_opacity_press, 0.10);
        assert_eq!(palette.state_layer_opacity_disabled, 0.12);
        assert_eq!(palette.state_layer_opacity_drag, 0.16);
        assert_eq!(palette.disable_opacity, 0.38);
        assert_eq!(palette.shadow_15.a(), 38);
        assert_eq!(palette.shadow_30.a(), 77);
    }

    #[test]
    fn frame_local_palette_overrides_visual_dark_mode_fallback() {
        let custom_palette = MaterialPalette {
            primary: egui::Color32::from_rgb(12, 34, 56),
            ..MaterialPalette::light()
        };

        with_current_material_palette(custom_palette, || {
            let resolved = material_palette_for_visuals(&egui::Visuals::dark());
            assert_eq!(resolved.primary, custom_palette.primary);
        });
    }

    #[test]
    fn apply_material_visuals_updates_egui_material3_global_theme_colors() {
        let context = Context::default();
        let mut state = SettingsState::default();

        reduce(
            &mut state,
            SettingsAction::UpdateUseSystemTheme { enabled: false },
        );
        reduce(
            &mut state,
            SettingsAction::UpdateUsePrimaryColor { enabled: true },
        );
        reduce(
            &mut state,
            SettingsAction::UpdatePrimaryColorHex {
                value: "#FF336699".to_string(),
            },
        );
        reduce(
            &mut state,
            SettingsAction::UpdateIsDarkMode { enabled: true },
        );

        let expected = material_palette_for_settings_state(&state);

        apply_material_visuals(&context, &state);

        assert_eq!(egui_material3::get_global_color("primary"), expected.primary);
        assert_eq!(egui_material3::get_global_color("onSurface"), expected.on_surface);
    }
}
