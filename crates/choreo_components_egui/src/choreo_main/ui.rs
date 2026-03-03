use egui::Ui;
use egui_material3::MaterialButton;

use crate::dialog_host::ui::DialogHostProps;
use crate::dialog_host::ui::draw_dialog_host;
use crate::dancers;

use super::actions::ChoreoMainAction;
use super::state::ChoreoMainState;
use super::state::InteractionMode;
use super::state::MainContent;

pub fn draw(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();

    let close_requested = draw_dialog_host(
        ui,
        &DialogHostProps {
            id_source: "choreo_main_dialog_host",
            is_open: state.is_dialog_open,
            close_on_click_away: true,
            overlay_color: ui.visuals().window_fill().linear_multiply(0.7),
            dialog_background: ui.visuals().widgets.noninteractive.bg_fill,
            dialog_text_color: ui.visuals().text_color(),
            dialog_padding: 24,
            dialog_margin: 24.0,
            dialog_corner_radius: 12,
            dialog_content: state.dialog_content.as_deref().unwrap_or_default(),
        },
        |ui| {
            draw_top_bar(ui, state, &mut actions);

            ui.separator();
            ui.horizontal(|ui| {
                if state.is_nav_open {
                    ui.vertical(|ui| {
                        ui.set_min_width(324.0);
                        ui.heading("Scenes");
                        ui.label("Main page drawer host (left)");
                        if ui.add(MaterialButton::new("Close Navigation")).clicked() {
                            actions.push(ChoreoMainAction::CloseNav);
                        }
                    });
                    ui.separator();
                }

                ui.vertical(|ui| {
                    ui.heading("Floor");
                    match state.content {
                        MainContent::Main => {
                            ui.label("Main page content");
                            ui.label(format!(
                                "selected scene: {}",
                                state.floor_scene_name.as_deref().unwrap_or("none")
                            ));
                            ui.label(format!("audio position: {:.2}s", state.audio_position_seconds));
                            ui.label(format!("draw requests: {}", state.draw_floor_request_count));
                            ui.label("content: main");
                        }
                        MainContent::Settings => {
                            ui.heading("Settings");
                            ui.label("content: settings");
                        }
                        MainContent::Dancers => {
                            ui.label("content: dancers");
                            ui.separator();
                            let dancer_actions = dancers::ui::draw(ui, &state.dancers_state);
                            for action in dancer_actions {
                                actions.push(ChoreoMainAction::DancersAction(action));
                            }
                        }
                    }
                });

                if state.is_choreography_settings_open {
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.set_min_width(480.0);
                        ui.heading("Choreography Settings");
                        ui.label("Main page drawer host (right)");
                        if ui.add(MaterialButton::new("Close Settings")).clicked() {
                            actions.push(ChoreoMainAction::CloseSettings);
                        }
                    });
                }
            });

            if state.is_audio_player_open {
                ui.separator();
                ui.group(|ui| {
                    ui.set_min_height(84.0);
                    ui.heading("Audio Player");
                    if ui.add(MaterialButton::new("Close Audio")).clicked() {
                        actions.push(ChoreoMainAction::CloseAudioPanel);
                    }
                });
            }

            ui.separator();
            if ui.add(MaterialButton::new("Initialize")).clicked() {
                actions.push(ChoreoMainAction::Initialize);
            }
        },
    );

    if close_requested {
        actions.push(ChoreoMainAction::HideDialog);
    }

    actions
}

fn draw_top_bar(ui: &mut Ui, state: &ChoreoMainState, actions: &mut Vec<ChoreoMainAction>) {
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 84.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let nav_label = if state.is_nav_open {
                "Close Nav"
            } else {
                "Open Nav"
            };
            if ui.add(MaterialButton::new(nav_label)).clicked() {
                actions.push(ChoreoMainAction::ToggleNav);
            }

            ui.add_space(12.0);
            let previous_mode_index = effective_mode_index(state);
            let mut selected_mode_index = previous_mode_index;
            ui.add_enabled_ui(state.is_mode_selection_enabled, |ui| {
                egui::ComboBox::from_label("Mode")
                    .selected_text(mode_label(selected_mode_index))
                    .show_ui(ui, |ui| {
                        for index in 0..mode_count() {
                            ui.selectable_value(
                                &mut selected_mode_index,
                                index,
                                mode_label(index),
                            );
                        }
                    });
            });
            if selected_mode_index != previous_mode_index {
                actions.push(ChoreoMainAction::SelectMode {
                    index: selected_mode_index,
                });
            }

            ui.add_space(12.0);
            let settings_label = if state.is_choreography_settings_open {
                "Settings Open"
            } else {
                "Open Settings"
            };
            if ui.add(MaterialButton::new(settings_label)).clicked() {
                actions.push(ChoreoMainAction::OpenSettings);
            }
            if ui.add(MaterialButton::new("Home")).clicked() {
                actions.push(ChoreoMainAction::ResetFloorViewport);
            }
            if ui.add(MaterialButton::new("Open Image")).clicked() {
                actions.push(ChoreoMainAction::RequestOpenImage {
                    file_path: String::new(),
                });
            }
            if ui.add(MaterialButton::new("Open Audio")).clicked() {
                actions.push(ChoreoMainAction::OpenAudioPanel);
            }
        },
    );
}

#[must_use]
pub fn mode_label(mode_index: i32) -> &'static str {
    match mode_index {
        0 => "View",
        1 => "Move",
        2 => "Rotate Center",
        3 => "Rotate Dancer",
        4 => "Scale",
        5 => "Line of Sight",
        _ => "Mode",
    }
}

#[must_use]
pub fn mode_count() -> i32 {
    6
}

fn effective_mode_index(state: &ChoreoMainState) -> i32 {
    if state.selected_mode_index >= 0 {
        return state.selected_mode_index;
    }
    match state.interaction_mode {
        InteractionMode::View => 0,
        InteractionMode::Move => 1,
        InteractionMode::RotateAroundCenter => 2,
        InteractionMode::RotateAroundDancer => 3,
        InteractionMode::Scale => 4,
        InteractionMode::LineOfSight => 5,
    }
}
