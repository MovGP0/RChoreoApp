use choreo_master_mobile_json::Color;
use material_color_utilities::dynamiccolor::{
    DynamicScheme, DynamicSchemeBuilder, Platform, SpecVersion, Variant,
};
use material_color_utilities::hct::Hct;

use crate::preferences::Preferences;

use super::settings_view_model::SettingsViewModel;
use super::types::MaterialSchemeUpdater;

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialScheme {
    pub primary: Color,
    pub surface_tint: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,
    pub tertiary: Color,
    pub on_tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,
    pub error: Color,
    pub on_error: Color,
    pub error_container: Color,
    pub on_error_container: Color,
    pub background: Color,
    pub on_background: Color,
    pub surface: Color,
    pub on_surface: Color,
    pub surface_variant: Color,
    pub on_surface_variant: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub shadow: Color,
    pub scrim: Color,
    pub inverse_surface: Color,
    pub inverse_on_surface: Color,
    pub inverse_primary: Color,
    pub primary_fixed: Color,
    pub on_primary_fixed: Color,
    pub primary_fixed_dim: Color,
    pub on_primary_fixed_variant: Color,
    pub secondary_fixed: Color,
    pub on_secondary_fixed: Color,
    pub secondary_fixed_dim: Color,
    pub on_secondary_fixed_variant: Color,
    pub tertiary_fixed: Color,
    pub on_tertiary_fixed: Color,
    pub tertiary_fixed_dim: Color,
    pub on_tertiary_fixed_variant: Color,
    pub surface_dim: Color,
    pub surface_bright: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialSchemes {
    pub light: MaterialScheme,
    pub dark: MaterialScheme,
}

pub trait MaterialSchemeApplier {
    fn apply(&self, schemes: MaterialSchemes);
}

#[derive(Clone)]
pub struct MaterialSchemeHelper<A: MaterialSchemeApplier> {
    applier: A,
}

impl<A: MaterialSchemeApplier> MaterialSchemeHelper<A> {
    pub fn new(applier: A) -> Self {
        Self { applier }
    }

    pub fn build_schemes(settings: &SettingsViewModel) -> MaterialSchemes {
        let default_source = Hct::from_int(0xFF1976D2);

        let mut use_secondary = settings.use_secondary_color;
        let mut use_tertiary = settings.use_tertiary_color;
        if !settings.use_primary_color {
            use_secondary = false;
            use_tertiary = false;
        } else if !use_secondary {
            use_tertiary = false;
        }

        let primary = if settings.use_primary_color {
            hct_from_color(settings.primary_color.clone())
        } else {
            default_source.clone()
        };
        let secondary = use_secondary.then(|| hct_from_color(settings.secondary_color.clone()));
        let tertiary = use_tertiary.then(|| hct_from_color(settings.tertiary_color.clone()));

        let light = build_scheme(&primary, secondary.as_ref(), tertiary.as_ref(), false);
        let dark = build_scheme(&primary, secondary.as_ref(), tertiary.as_ref(), true);

        MaterialSchemes { light, dark }
    }
}

impl<A: MaterialSchemeApplier> MaterialSchemeUpdater for MaterialSchemeHelper<A> {
    fn update(&self, settings: &SettingsViewModel, _preferences: &dyn Preferences) {
        let schemes = Self::build_schemes(settings);
        self.applier.apply(schemes);
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

    let scheme = builder.build();
    map_scheme(&scheme)
}

fn map_scheme(scheme: &DynamicScheme) -> MaterialScheme {
    MaterialScheme {
        primary: color_from_argb(scheme.primary()),
        surface_tint: color_from_argb(scheme.surface_tint()),
        on_primary: color_from_argb(scheme.on_primary()),
        primary_container: color_from_argb(scheme.primary_container()),
        on_primary_container: color_from_argb(scheme.on_primary_container()),
        secondary: color_from_argb(scheme.secondary()),
        on_secondary: color_from_argb(scheme.on_secondary()),
        secondary_container: color_from_argb(scheme.secondary_container()),
        on_secondary_container: color_from_argb(scheme.on_secondary_container()),
        tertiary: color_from_argb(scheme.tertiary()),
        on_tertiary: color_from_argb(scheme.on_tertiary()),
        tertiary_container: color_from_argb(scheme.tertiary_container()),
        on_tertiary_container: color_from_argb(scheme.on_tertiary_container()),
        error: color_from_argb(scheme.error()),
        on_error: color_from_argb(scheme.on_error()),
        error_container: color_from_argb(scheme.error_container()),
        on_error_container: color_from_argb(scheme.on_error_container()),
        background: color_from_argb(scheme.background()),
        on_background: color_from_argb(scheme.on_background()),
        surface: color_from_argb(scheme.surface()),
        on_surface: color_from_argb(scheme.on_surface()),
        surface_variant: color_from_argb(scheme.surface_variant()),
        on_surface_variant: color_from_argb(scheme.on_surface_variant()),
        outline: color_from_argb(scheme.outline()),
        outline_variant: color_from_argb(scheme.outline_variant()),
        shadow: color_from_argb(scheme.shadow()),
        scrim: color_from_argb(scheme.scrim()),
        inverse_surface: color_from_argb(scheme.inverse_surface()),
        inverse_on_surface: color_from_argb(scheme.inverse_on_surface()),
        inverse_primary: color_from_argb(scheme.inverse_primary()),
        primary_fixed: color_from_argb(scheme.primary_fixed()),
        on_primary_fixed: color_from_argb(scheme.on_primary_fixed()),
        primary_fixed_dim: color_from_argb(scheme.primary_fixed_dim()),
        on_primary_fixed_variant: color_from_argb(scheme.on_primary_fixed_variant()),
        secondary_fixed: color_from_argb(scheme.secondary_fixed()),
        on_secondary_fixed: color_from_argb(scheme.on_secondary_fixed()),
        secondary_fixed_dim: color_from_argb(scheme.secondary_fixed_dim()),
        on_secondary_fixed_variant: color_from_argb(scheme.on_secondary_fixed_variant()),
        tertiary_fixed: color_from_argb(scheme.tertiary_fixed()),
        on_tertiary_fixed: color_from_argb(scheme.on_tertiary_fixed()),
        tertiary_fixed_dim: color_from_argb(scheme.tertiary_fixed_dim()),
        on_tertiary_fixed_variant: color_from_argb(scheme.on_tertiary_fixed_variant()),
        surface_dim: color_from_argb(scheme.surface_dim()),
        surface_bright: color_from_argb(scheme.surface_bright()),
        surface_container_lowest: color_from_argb(scheme.surface_container_lowest()),
        surface_container_low: color_from_argb(scheme.surface_container_low()),
        surface_container: color_from_argb(scheme.surface_container()),
        surface_container_high: color_from_argb(scheme.surface_container_high()),
        surface_container_highest: color_from_argb(scheme.surface_container_highest()),
    }
}

fn hct_from_color(color: Color) -> Hct {
    let argb = ((color.a as u32) << 24)
        | ((color.r as u32) << 16)
        | ((color.g as u32) << 8)
        | (color.b as u32);
    Hct::from_int(argb)
}

fn color_from_argb(argb: u32) -> Color {
    Color {
        a: ((argb >> 24) & 0xFF) as u8,
        r: ((argb >> 16) & 0xFF) as u8,
        g: ((argb >> 8) & 0xFF) as u8,
        b: (argb & 0xFF) as u8,
    }
}
