use choreo_master_mobile_json::Color;
use material_color_utilities::dynamiccolor::{
    DynamicScheme, DynamicSchemeBuilder, Platform, SpecVersion, Variant,
};
use material_color_utilities::hct::Hct;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

pub struct BooleanNegationConverter;

impl BooleanNegationConverter {
    pub fn convert(value: bool) -> bool {
        !value
    }
}

#[derive(Debug)]
pub struct SettingsViewModel {
    pub theme_mode: ThemeMode,
    pub use_system_theme: bool,
    pub use_primary_color: bool,
    pub use_secondary_color: bool,
    pub use_tertiary_color: bool,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub tertiary_color: Color,
    disposables: CompositeDisposable,
}

impl SettingsViewModel {
    pub fn new(mut behaviors: Vec<Box<dyn Behavior<SettingsViewModel>>>) -> Self {
        let mut view_model = Self {
            theme_mode: ThemeMode::Light,
            use_system_theme: false,
            use_primary_color: false,
            use_secondary_color: false,
            use_tertiary_color: false,
            primary_color: default_primary_color(),
            secondary_color: default_secondary_color(),
            tertiary_color: default_tertiary_color(),
            disposables: CompositeDisposable::new(),
        };

        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors.drain(..) {
            behavior.activate(&mut view_model, &mut disposables);
        }

        view_model.disposables = disposables;
        view_model
    }

    pub fn dispose(&mut self) {
        self.disposables.dispose_all();
    }
}

impl Default for SettingsViewModel {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

pub fn default_primary_color() -> Color {
    Color {
        a: 255,
        r: 0x19,
        g: 0x76,
        b: 0xD2,
    }
}

pub fn default_secondary_color() -> Color {
    Color {
        a: 255,
        r: 0x67,
        g: 0x5A,
        b: 0x84,
    }
}

pub fn default_tertiary_color() -> Color {
    Color {
        a: 255,
        r: 0x82,
        g: 0x5A,
        b: 0x2C,
    }
}

pub trait MaterialSchemeUpdater {
    fn update(&self, settings: &SettingsViewModel, preferences: &dyn Preferences);
}

pub struct LoadSettingsPreferencesBehavior<P: Preferences> {
    preferences: P,
}

impl<P: Preferences> LoadSettingsPreferencesBehavior<P> {
    pub fn new(preferences: P) -> Self {
        Self { preferences }
    }

    pub fn load(&self, view_model: &mut SettingsViewModel) {
        let stored_theme = self
            .preferences
            .get_string(choreo_models::SettingsPreferenceKeys::THEME, "Light");
        view_model.theme_mode = if stored_theme == "Dark" {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };

        view_model.use_system_theme = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_SYSTEM_THEME, true);
        view_model.use_primary_color = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_PRIMARY_COLOR, false);
        view_model.use_secondary_color = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_SECONDARY_COLOR, false)
            && view_model.use_primary_color;
        view_model.use_tertiary_color = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::USE_TERTIARY_COLOR, false)
            && view_model.use_secondary_color;

        view_model.primary_color = self.get_color_from_preferences(
            choreo_models::SettingsPreferenceKeys::PRIMARY_COLOR,
            default_primary_color(),
        );
        view_model.secondary_color = self.get_color_from_preferences(
            choreo_models::SettingsPreferenceKeys::SECONDARY_COLOR,
            default_secondary_color(),
        );
        view_model.tertiary_color = self.get_color_from_preferences(
            choreo_models::SettingsPreferenceKeys::TERTIARY_COLOR,
            default_tertiary_color(),
        );
    }

    fn get_color_from_preferences(&self, key: &str, fallback: Color) -> Color {
        let stored = self.preferences.get_string(key, "");
        if !stored.trim().is_empty()
            && let Some(parsed) = Color::from_hex(&stored)
        {
            return parsed;
        }

        fallback
    }
}

impl<P: Preferences> Behavior<SettingsViewModel> for LoadSettingsPreferencesBehavior<P> {
    fn activate(&self, view_model: &mut SettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "LoadSettingsPreferencesBehavior",
            "SettingsViewModel",
        );
        self.load(view_model);
    }
}

pub struct ColorPreferencesBehavior<P: Preferences, U: MaterialSchemeUpdater> {
    preferences: P,
    updater: U,
}

impl<P: Preferences, U: MaterialSchemeUpdater> ColorPreferencesBehavior<P, U> {
    pub fn new(preferences: P, updater: U) -> Self {
        Self { preferences, updater }
    }

    pub fn update_use_primary_color(&self, view_model: &mut SettingsViewModel, enabled: bool) {
        view_model.use_primary_color = enabled;
        self.preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_PRIMARY_COLOR,
            enabled,
        );

        if !enabled {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::PRIMARY_COLOR);
            view_model.use_secondary_color = false;
            view_model.use_tertiary_color = false;
        }

        self.updater.update(view_model, &self.preferences);
    }

    pub fn update_use_secondary_color(&self, view_model: &mut SettingsViewModel, enabled: bool) {
        if enabled && !view_model.use_primary_color {
            view_model.use_secondary_color = false;
            return;
        }

        view_model.use_secondary_color = enabled;
        self.preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_SECONDARY_COLOR,
            enabled,
        );

        if !enabled {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::SECONDARY_COLOR);
            view_model.use_tertiary_color = false;
        }

        self.updater.update(view_model, &self.preferences);
    }

    pub fn update_use_tertiary_color(&self, view_model: &mut SettingsViewModel, enabled: bool) {
        if enabled && !view_model.use_secondary_color {
            view_model.use_tertiary_color = false;
            return;
        }

        view_model.use_tertiary_color = enabled;
        self.preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_TERTIARY_COLOR,
            enabled,
        );

        if !enabled {
            self.preferences
                .remove(choreo_models::SettingsPreferenceKeys::TERTIARY_COLOR);
        }

        self.updater.update(view_model, &self.preferences);
    }

    pub fn update_primary_color(&self, view_model: &mut SettingsViewModel, color: Color) {
        let hex = color.to_hex();
        view_model.primary_color = color;
        if view_model.use_primary_color {
            self.preferences.set_string(
                choreo_models::SettingsPreferenceKeys::PRIMARY_COLOR,
                hex,
            );
            self.updater.update(view_model, &self.preferences);
        }
    }

    pub fn update_secondary_color(&self, view_model: &mut SettingsViewModel, color: Color) {
        let hex = color.to_hex();
        view_model.secondary_color = color;
        if view_model.use_secondary_color {
            self.preferences.set_string(
                choreo_models::SettingsPreferenceKeys::SECONDARY_COLOR,
                hex,
            );
            self.updater.update(view_model, &self.preferences);
        }
    }

    pub fn update_tertiary_color(&self, view_model: &mut SettingsViewModel, color: Color) {
        let hex = color.to_hex();
        view_model.tertiary_color = color;
        if view_model.use_tertiary_color {
            self.preferences.set_string(
                choreo_models::SettingsPreferenceKeys::TERTIARY_COLOR,
                hex,
            );
            self.updater.update(view_model, &self.preferences);
        }
    }
}

impl<P: Preferences, U: MaterialSchemeUpdater> Behavior<SettingsViewModel>
    for ColorPreferencesBehavior<P, U>
{
    fn activate(&self, _view_model: &mut SettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ColorPreferencesBehavior", "SettingsViewModel");
    }
}

pub struct SwitchDarkLightModeBehavior<P: Preferences, U: MaterialSchemeUpdater> {
    preferences: P,
    updater: U,
}

impl<P: Preferences, U: MaterialSchemeUpdater> SwitchDarkLightModeBehavior<P, U> {
    pub fn new(preferences: P, updater: U) -> Self {
        Self { preferences, updater }
    }

    pub fn update_is_dark_mode(&self, view_model: &mut SettingsViewModel, is_dark: bool) {
        if view_model.use_system_theme {
            return;
        }

        view_model.theme_mode = if is_dark { ThemeMode::Dark } else { ThemeMode::Light };
        let theme = if is_dark { "Dark" } else { "Light" };
        self.preferences
            .set_string(choreo_models::SettingsPreferenceKeys::THEME, theme.to_string());
        self.updater.update(view_model, &self.preferences);
    }

    pub fn update_use_system_theme(&self, view_model: &mut SettingsViewModel, use_system: bool) {
        view_model.use_system_theme = use_system;
        self.preferences.set_bool(
            choreo_models::SettingsPreferenceKeys::USE_SYSTEM_THEME,
            use_system,
        );

        if use_system {
            self.updater.update(view_model, &self.preferences);
            return;
        }

        let is_dark = matches!(view_model.theme_mode, ThemeMode::Dark);
        let theme = if is_dark { "Dark" } else { "Light" };
        self.preferences
            .set_string(choreo_models::SettingsPreferenceKeys::THEME, theme.to_string());
        self.updater.update(view_model, &self.preferences);
    }
}

impl<P: Preferences, U: MaterialSchemeUpdater> Behavior<SettingsViewModel>
    for SwitchDarkLightModeBehavior<P, U>
{
    fn activate(&self, _view_model: &mut SettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "SwitchDarkLightModeBehavior",
            "SettingsViewModel",
        );
    }
}

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

pub struct SettingsDependencies<P: Preferences, U: MaterialSchemeUpdater> {
    pub preferences: P,
    pub scheme_updater: U,
}

pub fn build_settings_view_model<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static>(
    deps: SettingsDependencies<P, U>,
) -> SettingsViewModel {
    SettingsViewModel::new(build_settings_behaviors(deps))
}

pub fn build_settings_behaviors<P: Preferences + Clone + 'static, U: MaterialSchemeUpdater + Clone + 'static>(
    deps: SettingsDependencies<P, U>,
) -> Vec<Box<dyn Behavior<SettingsViewModel>>> {
    let preferences = deps.preferences;
    let updater = deps.scheme_updater;

    vec![
        Box::new(LoadSettingsPreferencesBehavior::new(preferences.clone())),
        Box::new(SwitchDarkLightModeBehavior::new(
            preferences.clone(),
            updater.clone(),
        )),
        Box::new(ColorPreferencesBehavior::new(preferences, updater)),
    ]
}
