use egui::Ui;

use crate::choreo_main;
use crate::floor;

pub type MaterialPalette = crate::shell::MaterialScheme;
pub type ShellHost = crate::shell_host::ShellHostViewModel;
pub type ScenesInfo = crate::scenes::state::ScenesState;
pub type SceneListItem = crate::scenes::state::SceneItemState;
pub type ChoreoInfo = crate::choreo_info::state::ChoreoInfoState;
pub type ChoreographySettings = crate::choreography_settings::state::ChoreographySettingsState;
pub type AudioPlayerInfo = crate::audio_player::state::AudioPlayerState;
pub type SettingsInfo = crate::settings::state::SettingsState;
pub type SceneInfo = crate::scenes::state::SceneItemState;
pub type FloorPosition = crate::floor::state::FloorPosition;
pub type LineSegment = crate::floor::state::LineSegment;
pub type AxisLabel = crate::floor::state::AxisLabel;
pub type LegendEntry = crate::floor::state::LegendEntry;
pub type FloorInfo = crate::floor::state::FloorState;
pub type FloorMetricsInfo = crate::floor::state::FloorLayoutMetrics;
pub type FloorLegendEntries = Vec<LegendEntry>;
pub type MainRootState = crate::choreo_main::state::ChoreoMainState;
pub type MainRootAction = crate::choreo_main::actions::ChoreoMainAction;

pub struct Translations;

impl Translations {
    #[must_use]
    pub fn text(locale: &str, key: &str) -> Option<&'static str> {
        let normalized_key = slint_key_to_i18n_key(key);
        choreo_i18n::translation_with_fallback(locale, normalized_key.as_str())
    }

    #[must_use]
    pub fn app_title(locale: &str) -> &'static str {
        Self::text(locale, "app_title").unwrap_or(crate::shell::app_title())
    }
}

fn slint_key_to_i18n_key(key: &str) -> String {
    let mut normalized = String::with_capacity(key.len());
    for segment in key.split('_').filter(|segment| !segment.is_empty()) {
        let mut chars = segment.chars();
        if let Some(first) = chars.next() {
            normalized.push(first.to_ascii_uppercase());
            normalized.extend(chars);
        }
    }

    normalized
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloorCurve {
    pub start: floor::state::Point,
    pub end: floor::state::Point,
    pub control1: Option<floor::state::Point>,
    pub control2: Option<floor::state::Point>,
}

impl Default for FloorCurve {
    fn default() -> Self {
        Self {
            start: floor::state::Point::new(0.0, 0.0),
            end: floor::state::Point::new(0.0, 0.0),
            control1: None,
            control2: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ZoomPanInfo {
    pub zoom_factor: f64,
    pub pan_x: f64,
    pub pan_y: f64,
}

impl ZoomPanInfo {
    #[must_use]
    pub fn user_scale(zoom_factor: f64) -> f64 {
        if zoom_factor > 0.0 { zoom_factor } else { 1.0 }
    }

    #[must_use]
    pub fn base_scale(scale_x: f64, scale_y: f64) -> f64 {
        scale_x.min(scale_y)
    }

    #[must_use]
    pub fn scale(base_scale: f64, user_scale: f64) -> f64 {
        base_scale * user_scale
    }

    #[must_use]
    pub fn text_scale(scale: f64) -> f64 {
        if scale > 0.0 { scale / 48.0 } else { 1.0 }
    }

    #[must_use]
    pub fn base_text_scale(base_scale: f64) -> f64 {
        if base_scale > 0.0 {
            base_scale / 48.0
        } else {
            1.0
        }
    }

    #[must_use]
    pub fn apply_zoom_and_pan(base_value: f64, user_scale: f64, user_translate_px: f64) -> f64 {
        (base_value * user_scale) + user_translate_px
    }

    #[must_use]
    pub fn corrected_text_size(target_text_size: f64, zoom_factor: f64) -> f64 {
        let target_size_px = target_text_size * zoom_factor;
        if target_size_px <= 0.0 {
            return 0.0;
        }

        let correction_factor = 0.947_233_3 + (-0.000_691_666_7 * (target_size_px - 64.0));
        let clamped_factor = correction_factor.max(0.70);
        target_size_px / clamped_factor
    }
}

pub fn draw(ui: &mut Ui, state: &MainRootState) -> Vec<MainRootAction> {
    choreo_main::ui::draw(ui, state)
}
