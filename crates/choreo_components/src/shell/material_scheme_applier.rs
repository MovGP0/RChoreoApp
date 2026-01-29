use slint::{Color as SlintColor, ComponentHandle, Weak};

use crate::settings::MaterialSchemeApplier;
use crate::settings::MaterialScheme as SettingsMaterialScheme;
use crate::settings::MaterialSchemes as SettingsMaterialSchemes;
use crate::MaterialPalette;
use crate::ShellHost;
use crate::{MaterialScheme as SlintMaterialScheme, MaterialSchemes as SlintMaterialSchemes};

#[derive(Clone)]
pub struct ShellMaterialSchemeApplier
{
    view: Weak<ShellHost>,
}

impl ShellMaterialSchemeApplier
{
    pub fn new(view: &ShellHost) -> Self
    {
        Self {
            view: view.as_weak(),
        }
    }
}

impl MaterialSchemeApplier for ShellMaterialSchemeApplier
{
    fn apply(&self, schemes: SettingsMaterialSchemes)
    {
        let Some(view) = self.view.upgrade() else
        {
            return;
        };

        let palette = view.global::<MaterialPalette<'_>>();
        palette.set_schemes(map_schemes(&schemes));
    }
}

fn map_schemes(schemes: &SettingsMaterialSchemes) -> SlintMaterialSchemes
{
    SlintMaterialSchemes {
        light: map_scheme(&schemes.light),
        dark: map_scheme(&schemes.dark),
    }
}

fn map_scheme(scheme: &SettingsMaterialScheme) -> SlintMaterialScheme
{
    SlintMaterialScheme {
        primary: map_color(&scheme.primary),
        surfaceTint: map_color(&scheme.surface_tint),
        onPrimary: map_color(&scheme.on_primary),
        primaryContainer: map_color(&scheme.primary_container),
        onPrimaryContainer: map_color(&scheme.on_primary_container),
        secondary: map_color(&scheme.secondary),
        onSecondary: map_color(&scheme.on_secondary),
        secondaryContainer: map_color(&scheme.secondary_container),
        onSecondaryContainer: map_color(&scheme.on_secondary_container),
        tertiary: map_color(&scheme.tertiary),
        onTertiary: map_color(&scheme.on_tertiary),
        tertiaryContainer: map_color(&scheme.tertiary_container),
        onTertiaryContainer: map_color(&scheme.on_tertiary_container),
        error: map_color(&scheme.error),
        onError: map_color(&scheme.on_error),
        errorContainer: map_color(&scheme.error_container),
        onErrorContainer: map_color(&scheme.on_error_container),
        background: map_color(&scheme.background),
        onBackground: map_color(&scheme.on_background),
        surface: map_color(&scheme.surface),
        onSurface: map_color(&scheme.on_surface),
        surfaceVariant: map_color(&scheme.surface_variant),
        onSurfaceVariant: map_color(&scheme.on_surface_variant),
        outline: map_color(&scheme.outline),
        outlineVariant: map_color(&scheme.outline_variant),
        shadow: map_color(&scheme.shadow),
        scrim: map_color(&scheme.scrim),
        inverseSurface: map_color(&scheme.inverse_surface),
        inverseOnSurface: map_color(&scheme.inverse_on_surface),
        inversePrimary: map_color(&scheme.inverse_primary),
        primaryFixed: map_color(&scheme.primary_fixed),
        onPrimaryFixed: map_color(&scheme.on_primary_fixed),
        primaryFixedDim: map_color(&scheme.primary_fixed_dim),
        onPrimaryFixedVariant: map_color(&scheme.on_primary_fixed_variant),
        secondaryFixed: map_color(&scheme.secondary_fixed),
        onSecondaryFixed: map_color(&scheme.on_secondary_fixed),
        secondaryFixedDim: map_color(&scheme.secondary_fixed_dim),
        onSecondaryFixedVariant: map_color(&scheme.on_secondary_fixed_variant),
        tertiaryFixed: map_color(&scheme.tertiary_fixed),
        onTertiaryFixed: map_color(&scheme.on_tertiary_fixed),
        tertiaryFixedDim: map_color(&scheme.tertiary_fixed_dim),
        onTertiaryFixedVariant: map_color(&scheme.on_tertiary_fixed_variant),
        surfaceDim: map_color(&scheme.surface_dim),
        surfaceBright: map_color(&scheme.surface_bright),
        surfaceContainerLowest: map_color(&scheme.surface_container_lowest),
        surfaceContainerLow: map_color(&scheme.surface_container_low),
        surfaceContainer: map_color(&scheme.surface_container),
        surfaceContainerHigh: map_color(&scheme.surface_container_high),
        surfaceContainerHighest: map_color(&scheme.surface_container_highest),
    }
}

fn map_color(color: &choreo_master_mobile_json::Color) -> SlintColor
{
    SlintColor::from_argb_u8(color.a, color.r, color.g, color.b)
}
