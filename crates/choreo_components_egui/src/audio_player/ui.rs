use egui::Ui;
use egui::Image;
use egui_material3::MaterialIconButton;
use egui_material3::MaterialSlider;

use crate::material::components;
use crate::slider_with_ticks::ui::SliderWithTicksInteraction;
use crate::slider_with_ticks::ui::SliderWithTicksUiState;
use crate::ui_icons;
use crate::ui_icons::UiIconKey;
use crate::ui_style::typography;
use crate::ui_style::typography::TypographyRole;

use super::actions::AudioPlayerAction;
use super::state::AudioPlayerState;
use super::state::PlayPauseGlyph;
use super::state::play_pause_glyph;

const GRID_12_PX: f32 = 12.0;
const SPEED_LABEL_WIDTH_PX: f32 = 48.0;
const DURATION_LABEL_WIDTH_PX: f32 = 72.0;

pub fn draw(ui: &mut Ui, state: &AudioPlayerState) -> Vec<AudioPlayerAction> {
    let mut actions: Vec<AudioPlayerAction> = Vec::new();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = GRID_12_PX;

        let mut normalized_speed =
            normalize_speed_to_slider_value(state.speed, state.minimum_speed, state.maximum_speed)
                as f32;
        let speed_response = ui.add(
            MaterialSlider::new(&mut normalized_speed, 0.0..=100.0)
                .enabled(state.can_set_speed)
                .show_value(false)
                .width(240.0),
        );
        if speed_response.changed() {
            actions.push(AudioPlayerAction::SpeedChanged {
                speed: denormalize_speed_from_slider_value(
                    f64::from(normalized_speed),
                    state.minimum_speed,
                    state.maximum_speed,
                ),
            });
        }
        ui.add_sized(
            [SPEED_LABEL_WIDTH_PX, 0.0],
            egui::Label::new(typography::rich_text_for_role(
                &state.speed_label,
                speed_label_role(),
            ))
            .truncate(),
        );

        let position = state.pending_seek_position.unwrap_or(state.position);
        let interactions = crate::slider_with_ticks::ui::draw(
            ui,
            SliderWithTicksUiState {
                enabled: state.can_seek,
                minimum: 0.0,
                maximum: state.duration.max(0.0),
                value: position,
                tick_values: &state.tick_values,
                tick_color: Some(ui.visuals().selection.bg_fill),
                width: 240.0,
            },
        );

        for interaction in interactions {
            match interaction {
                SliderWithTicksInteraction::DragStarted => {
                    actions.push(AudioPlayerAction::PositionDragStarted);
                }
                SliderWithTicksInteraction::ValueChanged { value, is_dragging } => {
                    if is_dragging {
                        actions.push(AudioPlayerAction::PositionPreviewChanged { position: value });
                    } else {
                        actions.push(AudioPlayerAction::SeekToPosition { position: value });
                    }
                }
                SliderWithTicksInteraction::DragCompleted { value } => {
                    actions.push(AudioPlayerAction::PositionDragCompleted { position: value });
                }
            }
        }

        ui.add_sized(
            [DURATION_LABEL_WIDTH_PX, 0.0],
            egui::Label::new(typography::rich_text_for_role(
                &state.duration_label,
                duration_label_role(),
            ))
            .truncate(),
        );

        if ui
            .add(
                MaterialIconButton::standard(link_icon_name())
                    .svg_data(link_icon_svg())
                    .enabled(state.can_link_scene_to_position),
            )
            .clicked()
        {
            actions.push(AudioPlayerAction::LinkSceneToPosition);
        }

        let play_pause_response =
            components::icon_button(ui, play_pause_image(state.is_playing), false);
        if play_pause_response.clicked() {
            actions.push(AudioPlayerAction::TogglePlayPause);
        }
    });
    actions
}

#[must_use]
pub fn play_pause_icon_label(is_playing: bool) -> &'static str {
    play_pause_icon_name(is_playing)
}

#[must_use]
pub fn play_pause_icon_name(is_playing: bool) -> &'static str {
    play_pause_icon_spec(is_playing).token
}

#[must_use]
pub fn link_icon_name() -> &'static str {
    link_icon_spec().token
}

#[must_use]
pub const fn speed_label_role() -> TypographyRole {
    TypographyRole::BodyMedium
}

#[must_use]
pub const fn duration_label_role() -> TypographyRole {
    TypographyRole::BodyMedium
}

#[must_use]
pub fn play_pause_icon_svg(is_playing: bool) -> &'static str {
    play_pause_icon_spec(is_playing).svg
}

#[must_use]
pub fn link_icon_svg() -> &'static str {
    link_icon_spec().svg
}

fn play_pause_icon_spec(is_playing: bool) -> ui_icons::UiIconSpec {
    match play_pause_glyph(is_playing) {
        PlayPauseGlyph::Play => ui_icons::icon(UiIconKey::AudioPlay),
        PlayPauseGlyph::Pause => ui_icons::icon(UiIconKey::AudioPause),
    }
}

fn play_pause_image(is_playing: bool) -> Image<'static> {
    match play_pause_glyph(is_playing) {
        PlayPauseGlyph::Play => {
            Image::new(egui::include_image!("../../assets/icons/Play.svg"))
        }
        PlayPauseGlyph::Pause => {
            Image::new(egui::include_image!("../../assets/icons/Pause.svg"))
        }
    }
}

fn link_icon_spec() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::AudioLink)
}

#[must_use]
pub fn normalize_speed_to_slider_value(speed: f64, minimum_speed: f64, maximum_speed: f64) -> f64 {
    if maximum_speed <= minimum_speed {
        return 0.0;
    }
    ((speed - minimum_speed) / (maximum_speed - minimum_speed) * 100.0).clamp(0.0, 100.0)
}

#[must_use]
pub fn denormalize_speed_from_slider_value(
    value: f64,
    minimum_speed: f64,
    maximum_speed: f64,
) -> f64 {
    let normalized = value.clamp(0.0, 100.0) / 100.0;
    minimum_speed + (maximum_speed - minimum_speed) * normalized
}
