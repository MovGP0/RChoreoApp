use egui::Ui;
use egui_material3::MaterialButton;

use super::actions::AudioPlayerAction;
use super::state::AudioPlayerState;
use super::state::PlayPauseGlyph;
use super::state::play_pause_glyph;

pub fn draw(ui: &mut Ui, state: &AudioPlayerState) -> Vec<AudioPlayerAction> {
    let mut actions: Vec<AudioPlayerAction> = Vec::new();
    ui.horizontal(|ui| {
        let mut normalized_speed = normalize_speed_to_slider_value(
            state.speed,
            state.minimum_speed,
            state.maximum_speed,
        );
        let speed_response = ui.add_enabled(
            state.can_set_speed,
            egui::Slider::new(&mut normalized_speed, 0.0..=100.0)
                .show_value(false)
                .text("Speed"),
        );
        if speed_response.changed() {
            actions.push(AudioPlayerAction::SpeedChanged {
                speed: denormalize_speed_from_slider_value(
                    normalized_speed,
                    state.minimum_speed,
                    state.maximum_speed,
                ),
            });
        }
        ui.label(&state.speed_label);

        let maximum = state.duration.max(0.0);
        let mut position = state.pending_seek_position.unwrap_or(state.position);
        let timeline_response = ui.add_enabled(
            state.can_seek,
            egui::Slider::new(&mut position, 0.0..=maximum)
                .show_value(false)
                .text("Position"),
        );
        draw_timeline_ticks(ui, timeline_response.rect, maximum, &state.tick_values);
        if timeline_response.drag_started() {
            actions.push(AudioPlayerAction::PositionDragStarted);
        }
        if timeline_response.changed() {
            if timeline_response.dragged() {
                actions.push(AudioPlayerAction::PositionPreviewChanged { position });
            } else {
                actions.push(AudioPlayerAction::SeekToPosition { position });
            }
        }
        if timeline_response.drag_stopped() {
            actions.push(AudioPlayerAction::PositionDragCompleted { position });
        }

        ui.label(&state.duration_label);

        if ui
            .add_enabled(
                state.can_link_scene_to_position,
                MaterialButton::new("[link]"),
            )
            .clicked()
        {
            actions.push(AudioPlayerAction::LinkSceneToPosition);
        }

        if ui
            .add(MaterialButton::new(play_pause_icon_label(state.is_playing)))
            .clicked()
        {
            actions.push(AudioPlayerAction::TogglePlayPause);
        }
    });
    actions
}

#[must_use]
pub fn play_pause_icon_label(is_playing: bool) -> &'static str {
    match play_pause_glyph(is_playing) {
        PlayPauseGlyph::Play => "[>]",
        PlayPauseGlyph::Pause => "[||]",
    }
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

fn draw_timeline_ticks(ui: &Ui, rect: egui::Rect, duration: f64, ticks: &[f64]) {
    if duration <= 0.0 || ticks.is_empty() {
        return;
    }
    let painter = ui.painter();
    let tick_top = rect.bottom() + 2.0;
    let tick_bottom = tick_top + 8.0;
    for tick in ticks {
        if *tick < 0.0 || *tick > duration {
            continue;
        }
        let t = (*tick / duration) as f32;
        let x = egui::lerp(rect.x_range(), t);
        painter.line_segment(
            [egui::pos2(x, tick_top), egui::pos2(x, tick_bottom)],
            egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.fg_stroke.color),
        );
    }
}
