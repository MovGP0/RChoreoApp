use choreo_master_mobile_json::Color;
use egui::Align2;
use egui::Color32;
use egui::Response;
use egui::Sense;
use egui::Ui;

use crate::dancers::dancer_list_item_view::colors_for_selection;
use crate::dancers::dancer_list_item_view::layout_for_row_rect;
use crate::dancers::dancer_list_item_view::row_size;
use crate::dancers::dancer_list_item_view::subtitle_role;
use crate::dancers::dancer_list_item_view::title_role;
use crate::dancers::dancer_list_item_view::tokens::item_corner_radius_token;
use crate::dancers::dancer_list_item_view::tokens::swatch_corner_radius_token;
use crate::dancers::state::DancerState;
use crate::material::styling::material_typography as typography;

#[must_use]
pub fn draw(ui: &mut Ui, dancer: &DancerState, is_selected: bool) -> Response {
    let (row_rect, response) =
        ui.allocate_exact_size(row_size(ui.available_width()), Sense::click());
    if !ui.is_rect_visible(row_rect) {
        return response;
    }

    let layout = layout_for_row_rect(row_rect);
    let colors = colors_for_selection(&ui.style().visuals, is_selected);

    ui.painter().rect(
        layout.content_rect,
        item_corner_radius_token(),
        colors.background,
        egui::Stroke::new(1.0, colors.border),
        egui::StrokeKind::Middle,
    );
    ui.painter().rect_filled(
        layout.swatch_rect,
        swatch_corner_radius_token(),
        color_to_egui(&dancer.color),
    );
    ui.painter().text(
        layout.title_position,
        Align2::LEFT_TOP,
        dancer.name.as_str(),
        typography::font_id_for_role(title_role()),
        colors.title,
    );
    ui.painter().text(
        layout.subtitle_position,
        Align2::LEFT_TOP,
        supporting_text(dancer),
        typography::font_id_for_role(subtitle_role()),
        colors.subtitle,
    );

    response
}

#[must_use]
pub fn supporting_text(dancer: &DancerState) -> String {
    format!(
        "{} ({})  [{}]",
        dancer.role.name, dancer.role.z_index, dancer.shortcut
    )
}

#[must_use]
pub fn role_details_text(dancer: &DancerState) -> String {
    supporting_text(dancer)
}

fn color_to_egui(color: &Color) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r, color.g, color.b, color.a)
}
