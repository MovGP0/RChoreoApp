use egui::Ui;
use egui_material3::MaterialIconButton;

use crate::dancers;
use crate::dialog_host::ui::DialogHostProps;
use crate::dialog_host::ui::dialog_metrics_tokens;
use crate::dialog_host::ui::draw_dialog_host;
use crate::main_page;
use crate::ui_style::material_style_metrics::material_style_metrics;
use crate::ui_icons;
use crate::ui_icons::UiIconKey;

use super::actions::ChoreoMainAction;
use super::state::ChoreoMainState;
use super::state::MainContent;

#[must_use]
pub const fn content_spacing_token() -> f32 {
    material_style_metrics().spacings.spacing_12
}

pub fn draw(ui: &mut Ui, state: &ChoreoMainState) -> Vec<ChoreoMainAction> {
    let mut actions: Vec<ChoreoMainAction> = Vec::new();
    let dialog_metrics = dialog_metrics_tokens();

    let close_requested = draw_dialog_host(
        ui,
        &DialogHostProps {
            id_source: "choreo_main_dialog_host",
            is_open: state.is_dialog_open,
            close_on_click_away: true,
            overlay_color: ui.visuals().window_fill().linear_multiply(0.7),
            dialog_background: ui.visuals().widgets.noninteractive.bg_fill,
            dialog_text_color: ui.visuals().text_color(),
            dialog_padding: dialog_metrics.dialog_padding,
            dialog_margin: dialog_metrics.dialog_margin,
            dialog_corner_radius: dialog_metrics.dialog_corner_radius,
            dialog_content: state.dialog_content.as_deref().unwrap_or_default(),
        },
        |ui| {
            let spacing = content_spacing_token();
            ui.spacing_mut().item_spacing = egui::vec2(spacing, spacing);

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
    let back_icon = ui_icons::icon(UiIconKey::SettingsNavigateBack);
    if ui
        .add(MaterialIconButton::standard(back_icon.token).svg_data(back_icon.svg))
        .on_hover_text("Back")
        .clicked()
    {
        actions.push(ChoreoMainAction::NavigateToMain);
    }
}

fn draw_full_dancers_page(
    ui: &mut Ui,
    state: &ChoreoMainState,
    actions: &mut Vec<ChoreoMainAction>,
) {
    let dancer_actions = dancers::ui::draw(ui, &state.dancers_state);
    for action in dancer_actions {
        actions.push(ChoreoMainAction::DancersAction(action));
    }
}

pub use crate::main_page::ui::home_icon_name;
pub use crate::main_page::ui::map_audio_host_action;
pub use crate::main_page::ui::map_floor_host_action;
pub use crate::main_page::ui::mode_count;
pub use crate::main_page::ui::mode_label;
pub use crate::main_page::ui::nav_icon_name;
pub use crate::main_page::ui::open_audio_icon_name;
pub use crate::main_page::ui::open_image_icon_name;
pub use crate::main_page::ui::top_bar_nav_action;
pub use crate::main_page::ui::top_bar_settings_action;
pub use crate::main_page::ui::top_bar_settings_icon_name;
