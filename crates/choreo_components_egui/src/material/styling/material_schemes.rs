use egui::Color32;

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
}

impl Default for MaterialSchemes {
    fn default() -> Self {
        Self::empty()
    }
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
    fn material_schemes_default_to_empty_light_and_dark_schemes() {
        let schemes = MaterialSchemes::default();

        assert_eq!(schemes.light, MaterialScheme::empty());
        assert_eq!(schemes.dark, MaterialScheme::empty());
    }
}
