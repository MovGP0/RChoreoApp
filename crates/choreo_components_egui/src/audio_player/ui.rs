use egui::Ui;

use super::actions::AudioPlayerAction;
use super::state::AudioPlayerState;
use super::state::play_pause_glyph;
use super::state::PlayPauseGlyph;

pub fn draw(ui: &mut Ui, state: &AudioPlayerState) -> Vec<AudioPlayerAction> {
    let mut actions: Vec<AudioPlayerAction> = Vec::new();
    ui.heading("Audio Player");
    ui.label(&state.duration_label);
    ui.label(format!("Speed {}", state.speed_label));

    if ui.button("Init").clicked() {
        actions.push(AudioPlayerAction::Initialize);
    }
    let play_pause_label = match play_pause_glyph(state.is_playing) {
        PlayPauseGlyph::Play => "Play",
        PlayPauseGlyph::Pause => "Pause",
    };
    if ui.button(play_pause_label).clicked() {
        actions.push(AudioPlayerAction::TogglePlayPause);
    }
    if ui.button("Stop").clicked() {
        actions.push(AudioPlayerAction::Stop);
    }
    if ui
        .add_enabled(state.can_link_scene_to_position, egui::Button::new("Link Scene"))
        .clicked()
    {
        actions.push(AudioPlayerAction::LinkSceneToPosition);
    }
    actions
}
