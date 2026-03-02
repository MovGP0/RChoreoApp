use choreo_master_mobile_json::Color;
use egui::Align2;
use egui::Button;
use egui::Color32;
use egui::CornerRadius;
use egui::FontId;
use egui::Frame;
use egui::Margin;
use egui::RichText;
use egui::ScrollArea;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::pos2;
use egui::vec2;

use super::actions::DancersAction;
use super::state::DancerState;
use super::state::DancersState;

pub fn draw(ui: &mut Ui, state: &DancersState) -> Vec<DancersAction> {
    let mut actions: Vec<DancersAction> = Vec::new();
    const GRID: f32 = 12.0;

    ui.heading("Dancers");
    ui.horizontal(|ui| {
        if ui
            .add_sized(
                [3.0 * GRID, 3.0 * GRID],
                Button::new(RichText::new("+").size(18.0)),
            )
            .on_hover_text("+")
            .clicked()
        {
            actions.push(DancersAction::AddDancer);
        }
        if ui
            .add_enabled(
                state.can_delete_dancer,
                Button::new(RichText::new("-").size(18.0)),
            )
            .on_hover_text("-")
            .clicked()
        {
            actions.push(DancersAction::DeleteSelectedDancer);
        }
    });

    Frame::new()
        .fill(ui.visuals().faint_bg_color)
        .stroke(Stroke::new(
            1.0,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(12))
        .inner_margin(Margin::same(8))
        .show(ui, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (index, dancer) in state.dancers.iter().enumerate() {
                        let is_selected = selected_dancer_index(state) == Some(index);
                        if draw_dancer_list_item_view(ui, dancer, is_selected).clicked() {
                            actions.push(DancersAction::SelectDancer { index });
                        }
                    }
                });
        });

    actions
}

#[must_use]
pub fn selected_dancer_index(state: &DancersState) -> Option<usize> {
    let selected_dancer_id = state
        .selected_dancer
        .as_ref()
        .map(|dancer| dancer.dancer_id)?;
    state
        .dancers
        .iter()
        .position(|dancer| dancer.dancer_id == selected_dancer_id)
}

#[must_use]
pub fn selected_role_index(state: &DancersState) -> Option<usize> {
    let selected_role_name = state
        .selected_role
        .as_ref()
        .map(|role| role.name.as_str())?;
    state
        .roles
        .iter()
        .position(|role| role.name == selected_role_name)
}

#[must_use]
pub fn selected_icon_index(state: &DancersState) -> Option<usize> {
    let selected_key = state
        .selected_icon_option
        .as_ref()
        .map(|option| option.key.as_str())?;
    state
        .icon_options
        .iter()
        .position(|option| option.key == selected_key)
}

#[must_use]
pub fn dancer_supporting_text(dancer: &DancerState) -> String {
    format!(
        "{} ({})  [{}]",
        dancer.role.name, dancer.role.z_index, dancer.shortcut
    )
}

#[must_use]
pub fn dancer_role_details_text(dancer: &DancerState) -> String {
    dancer_supporting_text(dancer)
}

fn draw_dancer_list_item_view(
    ui: &mut Ui,
    dancer: &DancerState,
    is_selected: bool,
) -> egui::Response {
    // Slint row height is 56px; keep this for visual parity with the source list item view.
    let row_size = vec2(ui.available_width(), 56.0);
    let (row_rect, response) = ui.allocate_exact_size(row_size, Sense::click());
    if !ui.is_rect_visible(row_rect) {
        return response;
    }

    let visuals = ui.style().visuals.clone();
    let background = if is_selected {
        visuals.selection.bg_fill
    } else {
        visuals.extreme_bg_color
    };
    ui.painter()
        .rect_filled(row_rect.shrink2(vec2(0.0, 3.0)), 8.0, background);

    let content_rect = row_rect.shrink2(vec2(0.0, 3.0));
    let swatch_rect = egui::Rect::from_min_size(
        pos2(content_rect.left() + 10.0, content_rect.center().y - 14.0),
        vec2(28.0, 28.0),
    );
    ui.painter()
        .rect_filled(swatch_rect, 6.0, color_to_egui(&dancer.color));

    let title_color = if is_selected {
        visuals.selection.stroke.color
    } else {
        visuals.text_color()
    };
    let subtitle_color = visuals.weak_text_color();
    let text_left = content_rect.left() + 46.0;

    ui.painter().text(
        pos2(text_left, content_rect.top() + 8.0),
        Align2::LEFT_TOP,
        dancer.name.as_str(),
        FontId::proportional(14.0),
        title_color,
    );
    ui.painter().text(
        pos2(text_left, content_rect.top() + 28.0),
        Align2::LEFT_TOP,
        dancer_supporting_text(dancer),
        FontId::proportional(12.0),
        subtitle_color,
    );

    response
}

fn color_to_egui(color: &Color) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r, color.g, color.b, color.a)
}
