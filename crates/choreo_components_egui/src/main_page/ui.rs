use egui::Frame;
use egui::Ui;
use egui_material3::MaterialIconButton;
use egui_material3::MaterialSelect;

use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::InteractionMode;

const TOP_BAR_HEIGHT_PX: f32 = 84.0;
const DRAWER_WIDTH_LEFT_PX: f32 = 324.0;
const DRAWER_WIDTH_RIGHT_PX: f32 = 480.0;
const AUDIO_PANEL_HEIGHT_PX: f32 = 84.0;
const GRID_12_PX: f32 = 12.0;

pub fn draw(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();

    ui.spacing_mut().item_spacing = egui::vec2(GRID_12_PX, GRID_12_PX);
    draw_top_bar(ui, state, &mut actions);
    ui.separator();

    ui.vertical(|ui| {
        ui.horizontal_top(|ui| {
            if state.is_nav_open {
                draw_scenes_drawer(ui, state, &mut actions);
            }

            draw_floor_host(
                ui,
                state,
                if state.is_audio_player_open {
                    AUDIO_PANEL_HEIGHT_PX
                } else {
                    0.0
                },
            );

            if state.is_choreography_settings_open {
                draw_settings_drawer(ui);
            }
        });

        if state.is_audio_player_open {
            ui.separator();
            draw_audio_host(ui);
        }
    });

    actions
}

fn draw_top_bar(ui: &mut Ui, state: &ChoreoMainState, actions: &mut Vec<ChoreoMainAction>) {
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), TOP_BAR_HEIGHT_PX),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let nav_response = ui.add(
                MaterialIconButton::standard(nav_icon_name(state.is_nav_open))
                    .svg_data(nav_icon_svg(state.is_nav_open)),
            );
            if nav_response.clicked() {
                actions.push(top_bar_nav_action(state.is_nav_open));
            }
            let _ = nav_response.on_hover_text(if state.is_nav_open {
                "Close navigation"
            } else {
                "Open navigation"
            });

            ui.add_space(12.0);
            let previous_mode_index =
                effective_mode_index(state).clamp(0, mode_count() - 1) as usize;
            let mut selected_mode_index = Some(previous_mode_index);
            let mut mode_changed = false;
            ui.add_enabled_ui(state.is_mode_selection_enabled, |ui| {
                let mut mode_select = MaterialSelect::new(&mut selected_mode_index).label("Mode");
                for index in 0..mode_count() {
                    mode_select = mode_select.option(index as usize, mode_label(index));
                }
                mode_changed = ui.add(mode_select).changed();
            });
            if mode_changed
                && let Some(selected) = selected_mode_index
                && selected != previous_mode_index
            {
                actions.push(ChoreoMainAction::SelectMode {
                    index: selected as i32,
                });
            }

            ui.add_space(ui.available_width().max(0.0));

            let settings_response = ui.add(
                MaterialIconButton::standard(top_bar_settings_icon_name())
                    .svg_data(top_bar_settings_icon_svg()),
            );
            if settings_response.clicked() {
                actions.push(top_bar_settings_action(state.is_choreography_settings_open));
            }
            let _ = settings_response.on_hover_text(if state.is_choreography_settings_open {
                "Close choreography settings"
            } else {
                "Open choreography settings"
            });

            let home_response =
                ui.add(MaterialIconButton::standard(home_icon_name()).svg_data(home_icon_svg()));
            if home_response.clicked() {
                actions.push(ChoreoMainAction::ResetFloorViewport);
            }
            let _ = home_response.on_hover_text("Reset floor viewport");

            let open_image_response = ui.add(
                MaterialIconButton::standard(open_image_icon_name())
                    .svg_data(open_image_icon_svg()),
            );
            if open_image_response.clicked() {
                actions.push(ChoreoMainAction::RequestOpenImage {
                    file_path: String::new(),
                });
            }
            let _ = open_image_response.on_hover_text("Open image");

            let open_audio_response = ui.add(
                MaterialIconButton::standard(open_audio_icon_name())
                    .svg_data(open_audio_icon_svg()),
            );
            if open_audio_response.clicked() {
                actions.push(ChoreoMainAction::OpenAudioPanel);
            }
            let _ = open_audio_response.on_hover_text("Open audio");
        },
    );
}

fn draw_scenes_drawer(ui: &mut Ui, state: &ChoreoMainState, actions: &mut Vec<ChoreoMainAction>) {
    ui.allocate_ui_with_layout(
        egui::vec2(DRAWER_WIDTH_LEFT_PX, ui.available_height()),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(DRAWER_WIDTH_LEFT_PX);
                for (index, scene) in state.scenes.iter().enumerate() {
                    let is_selected = state.selected_scene_index == Some(index);
                    let label = if let Some(timestamp) = scene.timestamp_seconds {
                        format!("{} ({timestamp:.1}s)", scene.name)
                    } else {
                        scene.name.clone()
                    };
                    if ui.selectable_label(is_selected, label).clicked() {
                        actions.push(ChoreoMainAction::SelectScene { index });
                    }
                }
            });
        },
    );
}

fn draw_floor_host(ui: &mut Ui, state: &ChoreoMainState, audio_height_px: f32) {
    let reserved_height = audio_height_px.max(0.0);
    ui.allocate_ui_with_layout(
        egui::vec2(
            ui.available_width()
                - if state.is_choreography_settings_open {
                    DRAWER_WIDTH_RIGHT_PX
                } else {
                    0.0
                },
            (ui.available_height() - reserved_height).max(360.0),
        ),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_size(egui::vec2(
                    ui.available_width(),
                    ui.available_height().max(360.0),
                ));
            });
        },
    );
}

fn draw_settings_drawer(ui: &mut Ui) {
    ui.allocate_ui_with_layout(
        egui::vec2(DRAWER_WIDTH_RIGHT_PX, ui.available_height()),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            Frame::group(ui.style()).show(ui, |ui| {
                ui.set_min_width(DRAWER_WIDTH_RIGHT_PX);
                ui.set_min_height(ui.available_height());
            });
        },
    );
}

fn draw_audio_host(ui: &mut Ui) {
    Frame::group(ui.style()).show(ui, |ui| {
        ui.set_min_height(AUDIO_PANEL_HEIGHT_PX);
    });
}

#[must_use]
pub fn top_bar_nav_action(is_nav_open: bool) -> ChoreoMainAction {
    if is_nav_open {
        ChoreoMainAction::CloseNav
    } else {
        ChoreoMainAction::ToggleNav
    }
}

#[must_use]
pub fn top_bar_settings_action(is_settings_open: bool) -> ChoreoMainAction {
    if is_settings_open {
        ChoreoMainAction::CloseSettings
    } else {
        ChoreoMainAction::OpenSettings
    }
}

#[must_use]
pub fn nav_icon_name(is_nav_open: bool) -> &'static str {
    if is_nav_open {
        "close"
    } else {
        "menu"
    }
}

#[must_use]
pub fn top_bar_settings_icon_name() -> &'static str {
    "edit"
}

#[must_use]
pub fn home_icon_name() -> &'static str {
    "home"
}

#[must_use]
pub fn open_image_icon_name() -> &'static str {
    "image"
}

#[must_use]
pub fn open_audio_icon_name() -> &'static str {
    "play_circle"
}

#[must_use]
pub fn nav_icon_svg(is_nav_open: bool) -> &'static str {
    if is_nav_open {
        include_str!("../../../choreo_components/ui/icons/Close.svg")
    } else {
        include_str!("../../../choreo_components/ui/icons/Menu.svg")
    }
}

#[must_use]
pub fn top_bar_settings_icon_svg() -> &'static str {
    include_str!("../../../choreo_components/ui/icons/Pen.svg")
}

#[must_use]
pub fn home_icon_svg() -> &'static str {
    include_str!("../../../choreo_components/ui/icons/Home.svg")
}

#[must_use]
pub fn open_image_icon_svg() -> &'static str {
    include_str!("../../../choreo_components/ui/icons/Svg.svg")
}

#[must_use]
pub fn open_audio_icon_svg() -> &'static str {
    include_str!("../../../choreo_components/ui/icons/PlayCircle.svg")
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
