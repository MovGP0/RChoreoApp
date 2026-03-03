use egui::Ui;

use crate::dancers;
use crate::dialog_host::ui::DialogHostProps;
use crate::dialog_host::ui::draw_dialog_host;
use crate::main_page;

use super::actions::ChoreoMainAction;
use super::state::ChoreoMainState;
use super::state::MainContent;

const GRID_12_PX: f32 = 12.0;

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
            ui.spacing_mut().item_spacing = egui::vec2(GRID_12_PX, GRID_12_PX);

            match state.content {
                MainContent::Settings => draw_full_settings_page(ui, &mut actions),
                MainContent::Dancers => draw_full_dancers_page(ui, state, &mut actions),
                MainContent::Main => {
                    actions.extend(main_page::ui::draw(ui, state));
                }
            }
        },
    );

    if close_requested {
        actions.push(ChoreoMainAction::HideDialog);
    }

    actions
}

fn draw_full_settings_page(ui: &mut Ui, actions: &mut Vec<ChoreoMainAction>) {
    ui.heading("Settings");
    if ui.button("Back").clicked() {
        actions.push(ChoreoMainAction::NavigateToMain);
    }
}

fn draw_full_dancers_page(
    ui: &mut Ui,
    state: &ChoreoMainState,
    actions: &mut Vec<ChoreoMainAction>,
) {
    ui.horizontal(|ui| {
        ui.heading("Dancers");
        if ui.button("Back").clicked() {
            actions.push(ChoreoMainAction::NavigateToMain);
        }
    });
    ui.separator();
    let dancer_actions = dancers::ui::draw(ui, &state.dancers_state);
    for action in dancer_actions {
        actions.push(ChoreoMainAction::DancersAction(action));
    }
}

pub use crate::main_page::ui::home_icon_name;
pub use crate::main_page::ui::mode_count;
pub use crate::main_page::ui::mode_label;
pub use crate::main_page::ui::nav_icon_name;
pub use crate::main_page::ui::open_audio_icon_name;
pub use crate::main_page::ui::open_image_icon_name;
pub use crate::main_page::ui::top_bar_nav_action;
pub use crate::main_page::ui::top_bar_settings_action;
pub use crate::main_page::ui::top_bar_settings_icon_name;
