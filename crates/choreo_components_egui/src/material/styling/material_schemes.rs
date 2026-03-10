use egui::Color32;
use material_color_utilities::dynamiccolor::{
    DynamicScheme, DynamicSchemeBuilder, Platform, SpecVersion, Variant,
};
use material_color_utilities::hct::Hct;

const DEFAULT_SOURCE_ARGB: u32 = 0xFF1976D2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialScheme {
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
}

impl MaterialScheme {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            primary: Color32::TRANSPARENT,
            surface_tint: Color32::TRANSPARENT,
            on_primary: Color32::TRANSPARENT,
            primary_container: Color32::TRANSPARENT,
            on_primary_container: Color32::TRANSPARENT,
            secondary: Color32::TRANSPARENT,
            on_secondary: Color32::TRANSPARENT,
            secondary_container: Color32::TRANSPARENT,
            on_secondary_container: Color32::TRANSPARENT,
            tertiary: Color32::TRANSPARENT,
            on_tertiary: Color32::TRANSPARENT,
            tertiary_container: Color32::TRANSPARENT,
            on_tertiary_container: Color32::TRANSPARENT,
            error: Color32::TRANSPARENT,
            on_error: Color32::TRANSPARENT,
            error_container: Color32::TRANSPARENT,
            on_error_container: Color32::TRANSPARENT,
            background: Color32::TRANSPARENT,
            on_background: Color32::TRANSPARENT,
            surface: Color32::TRANSPARENT,
            on_surface: Color32::TRANSPARENT,
            surface_variant: Color32::TRANSPARENT,
            on_surface_variant: Color32::TRANSPARENT,
            outline: Color32::TRANSPARENT,
            outline_variant: Color32::TRANSPARENT,
            shadow: Color32::TRANSPARENT,
            scrim: Color32::TRANSPARENT,
            inverse_surface: Color32::TRANSPARENT,
            inverse_on_surface: Color32::TRANSPARENT,
            inverse_primary: Color32::TRANSPARENT,
            primary_fixed: Color32::TRANSPARENT,
            on_primary_fixed: Color32::TRANSPARENT,
            primary_fixed_dim: Color32::TRANSPARENT,
            on_primary_fixed_variant: Color32::TRANSPARENT,
            secondary_fixed: Color32::TRANSPARENT,
            on_secondary_fixed: Color32::TRANSPARENT,
            secondary_fixed_dim: Color32::TRANSPARENT,
            on_secondary_fixed_variant: Color32::TRANSPARENT,
            tertiary_fixed: Color32::TRANSPARENT,
            on_tertiary_fixed: Color32::TRANSPARENT,
            tertiary_fixed_dim: Color32::TRANSPARENT,
            on_tertiary_fixed_variant: Color32::TRANSPARENT,
            surface_dim: Color32::TRANSPARENT,
            surface_bright: Color32::TRANSPARENT,
            surface_container_lowest: Color32::TRANSPARENT,
            surface_container_low: Color32::TRANSPARENT,
            surface_container: Color32::TRANSPARENT,
            surface_container_high: Color32::TRANSPARENT,
            surface_container_highest: Color32::TRANSPARENT,
        }
    }
}

impl Default for MaterialScheme {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaterialSchemes {
    pub light: MaterialScheme,
    pub dark: MaterialScheme,
}

impl MaterialSchemes {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            light: MaterialScheme::empty(),
            dark: MaterialScheme::empty(),
        }
    }

    #[must_use]
    pub fn from_seed_colors(
        primary_hex: Option<&str>,
        secondary_hex: Option<&str>,
        tertiary_hex: Option<&str>,
    ) -> Self {
        let primary = primary_hex
            .and_then(hct_from_argb_hex)
            .unwrap_or_else(|| Hct::from_int(DEFAULT_SOURCE_ARGB));
        let secondary = secondary_hex.and_then(hct_from_argb_hex);
        let tertiary = tertiary_hex.and_then(hct_from_argb_hex);

        Self {
            light: build_scheme(&primary, secondary.as_ref(), tertiary.as_ref(), false),
            dark: build_scheme(&primary, secondary.as_ref(), tertiary.as_ref(), true),
        }
    }

    #[must_use]
    pub fn for_dark_mode(&self, is_dark: bool) -> MaterialScheme {
        if is_dark { self.dark } else { self.light }
    }
}

impl Default for MaterialSchemes {
    fn default() -> Self {
        Self::from_seed_colors(None, None, None)
    }
}

fn build_scheme(
    primary: &Hct,
    secondary: Option<&Hct>,
    tertiary: Option<&Hct>,
    is_dark: bool,
) -> MaterialScheme {
    let mut builder = DynamicSchemeBuilder::default()
        .source_color_hct(primary.clone())
        .variant(Variant::Content)
        .is_dark(is_dark)
        .platform(Platform::Phone)
        .contrast_level(0.5)
        .spec_version(SpecVersion::Spec2025);

    if let Some(secondary) = secondary {
        builder = builder.secondary_palette_key_color(secondary.clone());
    }

    if let Some(tertiary) = tertiary {
        builder = builder.tertiary_palette_key_color(tertiary.clone());
    }

    map_scheme(&builder.build())
}

fn map_scheme(scheme: &DynamicScheme) -> MaterialScheme {
    MaterialScheme {
        primary: color32_from_argb(scheme.primary()),
        surface_tint: color32_from_argb(scheme.surface_tint()),
        on_primary: color32_from_argb(scheme.on_primary()),
        primary_container: color32_from_argb(scheme.primary_container()),
        on_primary_container: color32_from_argb(scheme.on_primary_container()),
        secondary: color32_from_argb(scheme.secondary()),
        on_secondary: color32_from_argb(scheme.on_secondary()),
        secondary_container: color32_from_argb(scheme.secondary_container()),
        on_secondary_container: color32_from_argb(scheme.on_secondary_container()),
        tertiary: color32_from_argb(scheme.tertiary()),
        on_tertiary: color32_from_argb(scheme.on_tertiary()),
        tertiary_container: color32_from_argb(scheme.tertiary_container()),
        on_tertiary_container: color32_from_argb(scheme.on_tertiary_container()),
        error: color32_from_argb(scheme.error()),
        on_error: color32_from_argb(scheme.on_error()),
        error_container: color32_from_argb(scheme.error_container()),
        on_error_container: color32_from_argb(scheme.on_error_container()),
        background: color32_from_argb(scheme.background()),
        on_background: color32_from_argb(scheme.on_background()),
        surface: color32_from_argb(scheme.surface()),
        on_surface: color32_from_argb(scheme.on_surface()),
        surface_variant: color32_from_argb(scheme.surface_variant()),
        on_surface_variant: color32_from_argb(scheme.on_surface_variant()),
        outline: color32_from_argb(scheme.outline()),
        outline_variant: color32_from_argb(scheme.outline_variant()),
        shadow: color32_from_argb(scheme.shadow()),
        scrim: color32_from_argb(scheme.scrim()),
        inverse_surface: color32_from_argb(scheme.inverse_surface()),
        inverse_on_surface: color32_from_argb(scheme.inverse_on_surface()),
        inverse_primary: color32_from_argb(scheme.inverse_primary()),
        primary_fixed: color32_from_argb(scheme.primary_fixed()),
        on_primary_fixed: color32_from_argb(scheme.on_primary_fixed()),
        primary_fixed_dim: color32_from_argb(scheme.primary_fixed_dim()),
        on_primary_fixed_variant: color32_from_argb(scheme.on_primary_fixed_variant()),
        secondary_fixed: color32_from_argb(scheme.secondary_fixed()),
        on_secondary_fixed: color32_from_argb(scheme.on_secondary_fixed()),
        secondary_fixed_dim: color32_from_argb(scheme.secondary_fixed_dim()),
        on_secondary_fixed_variant: color32_from_argb(scheme.on_secondary_fixed_variant()),
        tertiary_fixed: color32_from_argb(scheme.tertiary_fixed()),
        on_tertiary_fixed: color32_from_argb(scheme.on_tertiary_fixed()),
        tertiary_fixed_dim: color32_from_argb(scheme.tertiary_fixed_dim()),
        on_tertiary_fixed_variant: color32_from_argb(scheme.on_tertiary_fixed_variant()),
        surface_dim: color32_from_argb(scheme.surface_dim()),
        surface_bright: color32_from_argb(scheme.surface_bright()),
        surface_container_lowest: color32_from_argb(scheme.surface_container_lowest()),
        surface_container_low: color32_from_argb(scheme.surface_container_low()),
        surface_container: color32_from_argb(scheme.surface_container()),
        surface_container_high: color32_from_argb(scheme.surface_container_high()),
        surface_container_highest: color32_from_argb(scheme.surface_container_highest()),
    }
}

fn hct_from_argb_hex(value: &str) -> Option<Hct> {
    parse_argb_hex(value).map(Hct::from_int)
}

fn parse_argb_hex(value: &str) -> Option<u32> {
    let trimmed = value.trim();
    if trimmed.len() != 9 || !trimmed.starts_with('#') {
        return None;
    }

    u32::from_str_radix(&trimmed[1..], 16).ok()
}

fn color32_from_argb(argb: u32) -> Color32 {
    let alpha = ((argb >> 24) & 0xFF) as u8;
    let red = ((argb >> 16) & 0xFF) as u8;
    let green = ((argb >> 8) & 0xFF) as u8;
    let blue = (argb & 0xFF) as u8;
    Color32::from_rgba_unmultiplied(red, green, blue, alpha)
}

#[cfg(test)]
mod tests {
    use super::MaterialScheme;
    use super::MaterialSchemes;
    use egui::Color32;

    #[test]
    fn material_scheme_ports_the_full_slint_role_surface() {
        let scheme = MaterialScheme {
            primary: Color32::from_rgb(1, 2, 3),
            surface_tint: Color32::from_rgb(4, 5, 6),
            on_primary: Color32::from_rgb(7, 8, 9),
            primary_container: Color32::from_rgb(10, 11, 12),
            on_primary_container: Color32::from_rgb(13, 14, 15),
            secondary: Color32::from_rgb(16, 17, 18),
            on_secondary: Color32::from_rgb(19, 20, 21),
            secondary_container: Color32::from_rgb(22, 23, 24),
            on_secondary_container: Color32::from_rgb(25, 26, 27),
            tertiary: Color32::from_rgb(28, 29, 30),
            on_tertiary: Color32::from_rgb(31, 32, 33),
            tertiary_container: Color32::from_rgb(34, 35, 36),
            on_tertiary_container: Color32::from_rgb(37, 38, 39),
            error: Color32::from_rgb(40, 41, 42),
            on_error: Color32::from_rgb(43, 44, 45),
            error_container: Color32::from_rgb(46, 47, 48),
            on_error_container: Color32::from_rgb(49, 50, 51),
            background: Color32::from_rgb(52, 53, 54),
            on_background: Color32::from_rgb(55, 56, 57),
            surface: Color32::from_rgb(58, 59, 60),
            on_surface: Color32::from_rgb(61, 62, 63),
            surface_variant: Color32::from_rgb(64, 65, 66),
            on_surface_variant: Color32::from_rgb(67, 68, 69),
            outline: Color32::from_rgb(70, 71, 72),
            outline_variant: Color32::from_rgb(73, 74, 75),
            shadow: Color32::from_rgb(76, 77, 78),
            scrim: Color32::from_rgb(79, 80, 81),
            inverse_surface: Color32::from_rgb(82, 83, 84),
            inverse_on_surface: Color32::from_rgb(85, 86, 87),
            inverse_primary: Color32::from_rgb(88, 89, 90),
            primary_fixed: Color32::from_rgb(91, 92, 93),
            on_primary_fixed: Color32::from_rgb(94, 95, 96),
            primary_fixed_dim: Color32::from_rgb(97, 98, 99),
            on_primary_fixed_variant: Color32::from_rgb(100, 101, 102),
            secondary_fixed: Color32::from_rgb(103, 104, 105),
            on_secondary_fixed: Color32::from_rgb(106, 107, 108),
            secondary_fixed_dim: Color32::from_rgb(109, 110, 111),
            on_secondary_fixed_variant: Color32::from_rgb(112, 113, 114),
            tertiary_fixed: Color32::from_rgb(115, 116, 117),
            on_tertiary_fixed: Color32::from_rgb(118, 119, 120),
            tertiary_fixed_dim: Color32::from_rgb(121, 122, 123),
            on_tertiary_fixed_variant: Color32::from_rgb(124, 125, 126),
            surface_dim: Color32::from_rgb(127, 128, 129),
            surface_bright: Color32::from_rgb(130, 131, 132),
            surface_container_lowest: Color32::from_rgb(133, 134, 135),
            surface_container_low: Color32::from_rgb(136, 137, 138),
            surface_container: Color32::from_rgb(139, 140, 141),
            surface_container_high: Color32::from_rgb(142, 143, 144),
            surface_container_highest: Color32::from_rgb(145, 146, 147),
        };

        assert_eq!(scheme.primary, Color32::from_rgb(1, 2, 3));
        assert_eq!(scheme.surface_tint, Color32::from_rgb(4, 5, 6));
        assert_eq!(scheme.on_primary, Color32::from_rgb(7, 8, 9));
        assert_eq!(scheme.primary_container, Color32::from_rgb(10, 11, 12));
        assert_eq!(scheme.on_primary_container, Color32::from_rgb(13, 14, 15));
        assert_eq!(scheme.secondary, Color32::from_rgb(16, 17, 18));
        assert_eq!(scheme.on_secondary, Color32::from_rgb(19, 20, 21));
        assert_eq!(scheme.secondary_container, Color32::from_rgb(22, 23, 24));
        assert_eq!(scheme.on_secondary_container, Color32::from_rgb(25, 26, 27));
        assert_eq!(scheme.tertiary, Color32::from_rgb(28, 29, 30));
        assert_eq!(scheme.on_tertiary, Color32::from_rgb(31, 32, 33));
        assert_eq!(scheme.tertiary_container, Color32::from_rgb(34, 35, 36));
        assert_eq!(scheme.on_tertiary_container, Color32::from_rgb(37, 38, 39));
        assert_eq!(scheme.error, Color32::from_rgb(40, 41, 42));
        assert_eq!(scheme.on_error, Color32::from_rgb(43, 44, 45));
        assert_eq!(scheme.error_container, Color32::from_rgb(46, 47, 48));
        assert_eq!(scheme.on_error_container, Color32::from_rgb(49, 50, 51));
        assert_eq!(scheme.background, Color32::from_rgb(52, 53, 54));
        assert_eq!(scheme.on_background, Color32::from_rgb(55, 56, 57));
        assert_eq!(scheme.surface, Color32::from_rgb(58, 59, 60));
        assert_eq!(scheme.on_surface, Color32::from_rgb(61, 62, 63));
        assert_eq!(scheme.surface_variant, Color32::from_rgb(64, 65, 66));
        assert_eq!(scheme.on_surface_variant, Color32::from_rgb(67, 68, 69));
        assert_eq!(scheme.outline, Color32::from_rgb(70, 71, 72));
        assert_eq!(scheme.outline_variant, Color32::from_rgb(73, 74, 75));
        assert_eq!(scheme.shadow, Color32::from_rgb(76, 77, 78));
        assert_eq!(scheme.scrim, Color32::from_rgb(79, 80, 81));
        assert_eq!(scheme.inverse_surface, Color32::from_rgb(82, 83, 84));
        assert_eq!(scheme.inverse_on_surface, Color32::from_rgb(85, 86, 87));
        assert_eq!(scheme.inverse_primary, Color32::from_rgb(88, 89, 90));
        assert_eq!(scheme.primary_fixed, Color32::from_rgb(91, 92, 93));
        assert_eq!(scheme.on_primary_fixed, Color32::from_rgb(94, 95, 96));
        assert_eq!(scheme.primary_fixed_dim, Color32::from_rgb(97, 98, 99));
        assert_eq!(
            scheme.on_primary_fixed_variant,
            Color32::from_rgb(100, 101, 102)
        );
        assert_eq!(scheme.secondary_fixed, Color32::from_rgb(103, 104, 105));
        assert_eq!(scheme.on_secondary_fixed, Color32::from_rgb(106, 107, 108));
        assert_eq!(scheme.secondary_fixed_dim, Color32::from_rgb(109, 110, 111));
        assert_eq!(
            scheme.on_secondary_fixed_variant,
            Color32::from_rgb(112, 113, 114)
        );
        assert_eq!(scheme.tertiary_fixed, Color32::from_rgb(115, 116, 117));
        assert_eq!(scheme.on_tertiary_fixed, Color32::from_rgb(118, 119, 120));
        assert_eq!(scheme.tertiary_fixed_dim, Color32::from_rgb(121, 122, 123));
        assert_eq!(
            scheme.on_tertiary_fixed_variant,
            Color32::from_rgb(124, 125, 126)
        );
        assert_eq!(scheme.surface_dim, Color32::from_rgb(127, 128, 129));
        assert_eq!(scheme.surface_bright, Color32::from_rgb(130, 131, 132));
        assert_eq!(
            scheme.surface_container_lowest,
            Color32::from_rgb(133, 134, 135)
        );
        assert_eq!(
            scheme.surface_container_low,
            Color32::from_rgb(136, 137, 138)
        );
        assert_eq!(scheme.surface_container, Color32::from_rgb(139, 140, 141));
        assert_eq!(
            scheme.surface_container_high,
            Color32::from_rgb(142, 143, 144)
        );
        assert_eq!(
            scheme.surface_container_highest,
            Color32::from_rgb(145, 146, 147)
        );
    }

    #[test]
    fn default_schemes_produce_distinct_light_and_dark_role_sets() {
        let schemes = MaterialSchemes::default();

        assert_ne!(schemes.light, MaterialScheme::empty());
        assert_ne!(schemes.dark, MaterialScheme::empty());
        assert_ne!(schemes.light.primary, schemes.dark.primary);
        assert_ne!(schemes.light.background, schemes.dark.background);
    }

    #[test]
    fn custom_seed_colors_recalculate_light_and_dark_roles() {
        let schemes = MaterialSchemes::from_seed_colors(
            Some("#FF336699"),
            Some("#FF884422"),
            Some("#FF227744"),
        );

        assert_ne!(
            schemes.light.primary,
            MaterialSchemes::default().light.primary
        );
        assert_ne!(
            schemes.dark.tertiary_container,
            MaterialSchemes::default().dark.tertiary_container
        );
    }
}
