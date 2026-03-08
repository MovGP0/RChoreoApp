use egui::Color32;
use egui::Visuals;

use crate::settings::state::ThemeMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MaterialScheme {
    primary: Color32,
    surface_tint: Color32,
    on_primary: Color32,
    primary_container: Color32,
    on_primary_container: Color32,
    secondary: Color32,
    on_secondary: Color32,
    secondary_container: Color32,
    on_secondary_container: Color32,
    tertiary: Color32,
    on_tertiary: Color32,
    tertiary_container: Color32,
    on_tertiary_container: Color32,
    error: Color32,
    on_error: Color32,
    error_container: Color32,
    on_error_container: Color32,
    background: Color32,
    on_background: Color32,
    surface: Color32,
    on_surface: Color32,
    surface_variant: Color32,
    on_surface_variant: Color32,
    outline: Color32,
    outline_variant: Color32,
    shadow: Color32,
    scrim: Color32,
    inverse_surface: Color32,
    inverse_on_surface: Color32,
    inverse_primary: Color32,
    primary_fixed: Color32,
    on_primary_fixed: Color32,
    primary_fixed_dim: Color32,
    on_primary_fixed_variant: Color32,
    secondary_fixed: Color32,
    on_secondary_fixed: Color32,
    secondary_fixed_dim: Color32,
    on_secondary_fixed_variant: Color32,
    tertiary_fixed: Color32,
    on_tertiary_fixed: Color32,
    tertiary_fixed_dim: Color32,
    on_tertiary_fixed_variant: Color32,
    surface_dim: Color32,
    surface_bright: Color32,
    surface_container_lowest: Color32,
    surface_container_low: Color32,
    surface_container: Color32,
    surface_container_high: Color32,
    surface_container_highest: Color32,
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
        from_scheme(LIGHT_SCHEME)
    }

    #[must_use]
    pub fn dark() -> Self {
        from_scheme(DARK_SCHEME)
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
    MaterialPalette::from_visuals(visuals)
}

const LIGHT_SCHEME: MaterialScheme = MaterialScheme {
    primary: rgb(68, 94, 145),
    surface_tint: rgb(68, 94, 145),
    on_primary: rgb(255, 255, 255),
    primary_container: rgb(216, 226, 255),
    on_primary_container: rgb(0, 26, 66),
    secondary: rgb(87, 94, 113),
    on_secondary: rgb(255, 255, 255),
    secondary_container: rgb(219, 226, 249),
    on_secondary_container: rgb(20, 27, 44),
    tertiary: rgb(113, 85, 115),
    on_tertiary: rgb(255, 255, 255),
    tertiary_container: rgb(252, 215, 251),
    on_tertiary_container: rgb(41, 19, 45),
    error: rgb(186, 26, 26),
    on_error: rgb(255, 255, 255),
    error_container: rgb(255, 218, 214),
    on_error_container: rgb(65, 0, 2),
    background: rgb(249, 249, 255),
    on_background: rgb(26, 27, 32),
    surface: rgb(249, 249, 255),
    on_surface: rgb(26, 27, 32),
    surface_variant: rgb(225, 226, 236),
    on_surface_variant: rgb(68, 71, 79),
    outline: rgb(117, 119, 127),
    outline_variant: rgb(196, 198, 208),
    shadow: rgb(0, 0, 0),
    scrim: rgb(0, 0, 0),
    inverse_surface: rgb(47, 48, 54),
    inverse_on_surface: rgb(240, 240, 247),
    inverse_primary: rgb(173, 198, 255),
    primary_fixed: rgb(216, 226, 255),
    on_primary_fixed: rgb(0, 26, 66),
    primary_fixed_dim: rgb(173, 198, 255),
    on_primary_fixed_variant: rgb(43, 70, 120),
    secondary_fixed: rgb(219, 226, 249),
    on_secondary_fixed: rgb(20, 27, 44),
    secondary_fixed_dim: rgb(191, 198, 220),
    on_secondary_fixed_variant: rgb(63, 71, 89),
    tertiary_fixed: rgb(252, 215, 251),
    on_tertiary_fixed: rgb(41, 19, 45),
    tertiary_fixed_dim: rgb(222, 188, 223),
    on_tertiary_fixed_variant: rgb(88, 62, 91),
    surface_dim: rgb(217, 217, 224),
    surface_bright: rgb(246, 246, 255),
    surface_container_lowest: rgb(255, 255, 255),
    surface_container_low: rgb(243, 243, 250),
    surface_container: rgb(238, 237, 244),
    surface_container_high: rgb(232, 231, 239),
    surface_container_highest: rgb(226, 226, 233),
};

const DARK_SCHEME: MaterialScheme = MaterialScheme {
    primary: rgb(173, 198, 255),
    surface_tint: rgb(173, 198, 255),
    on_primary: rgb(17, 47, 96),
    primary_container: rgb(43, 70, 120),
    on_primary_container: rgb(216, 226, 255),
    secondary: rgb(191, 198, 220),
    on_secondary: rgb(41, 48, 65),
    secondary_container: rgb(63, 71, 89),
    on_secondary_container: rgb(219, 226, 249),
    tertiary: rgb(222, 188, 223),
    on_tertiary: rgb(64, 40, 67),
    tertiary_container: rgb(88, 62, 91),
    on_tertiary_container: rgb(252, 215, 251),
    error: rgb(255, 180, 171),
    on_error: rgb(105, 0, 5),
    error_container: rgb(147, 0, 10),
    on_error_container: rgb(255, 218, 214),
    background: rgb(17, 19, 24),
    on_background: rgb(226, 226, 233),
    surface: rgb(17, 19, 24),
    on_surface: rgb(226, 226, 233),
    surface_variant: rgb(68, 71, 79),
    on_surface_variant: rgb(196, 198, 208),
    outline: rgb(142, 144, 153),
    outline_variant: rgb(68, 71, 79),
    shadow: rgb(0, 0, 0),
    scrim: rgb(0, 0, 0),
    inverse_surface: rgb(226, 226, 233),
    inverse_on_surface: rgb(47, 48, 54),
    inverse_primary: rgb(68, 94, 145),
    primary_fixed: rgb(216, 226, 255),
    on_primary_fixed: rgb(0, 26, 66),
    primary_fixed_dim: rgb(173, 198, 255),
    on_primary_fixed_variant: rgb(43, 70, 120),
    secondary_fixed: rgb(219, 226, 249),
    on_secondary_fixed: rgb(20, 27, 44),
    secondary_fixed_dim: rgb(191, 198, 220),
    on_secondary_fixed_variant: rgb(63, 71, 89),
    tertiary_fixed: rgb(252, 215, 251),
    on_tertiary_fixed: rgb(41, 19, 45),
    tertiary_fixed_dim: rgb(222, 188, 223),
    on_tertiary_fixed_variant: rgb(88, 62, 91),
    surface_dim: rgb(17, 19, 24),
    surface_bright: rgb(55, 57, 62),
    surface_container_lowest: rgb(12, 14, 19),
    surface_container_low: rgb(26, 27, 32),
    surface_container: rgb(30, 31, 37),
    surface_container_high: rgb(40, 42, 47),
    surface_container_highest: rgb(51, 53, 58),
};

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

const fn rgb(red: u8, green: u8, blue: u8) -> Color32 {
    Color32::from_rgb(red, green, blue)
}

fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Color32 {
    Color32::from_rgba_unmultiplied(red, green, blue, alpha)
}

#[cfg(test)]
mod tests {
    use super::MaterialPalette;
    use super::material_palette;
    use crate::settings::state::ThemeMode;

    #[test]
    fn light_palette_matches_slint_seed_values() {
        let palette = MaterialPalette::light();

        assert_eq!(palette.primary, egui::Color32::from_rgb(68, 94, 145));
        assert_eq!(
            palette.surface_container_highest,
            egui::Color32::from_rgb(226, 226, 233)
        );
        assert_eq!(palette.background_modal.a(), 128);
    }

    #[test]
    fn dark_palette_matches_slint_seed_values() {
        let palette = MaterialPalette::dark();

        assert_eq!(palette.primary, egui::Color32::from_rgb(173, 198, 255));
        assert_eq!(palette.on_primary, egui::Color32::from_rgb(17, 47, 96));
        assert_eq!(palette.surface_dim, egui::Color32::from_rgb(17, 19, 24));
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
}
