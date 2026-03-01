use egui::Ui;

use super::actions::AudioPlayerAction;
use super::state::AudioPlayerState;

pub fn draw(ui: &mut Ui, state: &AudioPlayerState) -> Vec<AudioPlayerAction> {
    let mut actions: Vec<AudioPlayerAction> = Vec::new();
    ui.heading("audio_player (egui scaffold)");
    ui.label(format!("flags: {}", state.flags.len()));
    if ui.button("Initialize").clicked() {
        actions.push(AudioPlayerAction::Initialize);
    }
    actions
}
