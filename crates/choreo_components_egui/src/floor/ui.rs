use egui::Ui;
use egui::epaint::PathShape;

use super::actions::FloorAction;
use super::state::FloorState;

pub fn draw(ui: &mut Ui, state: &FloorState) -> Vec<FloorAction> {
    let mut actions: Vec<FloorAction> = Vec::new();
    let available = ui.available_size();
    actions.push(FloorAction::SetLayout {
        width_px: f64::from(available.x),
        height_px: f64::from(available.y),
    });
    actions.push(FloorAction::DrawFloor);

    let (rect, _response) = ui.allocate_exact_size(available, egui::Sense::drag());
    let painter = ui.painter_at(rect);

    if let Some(background) = state.background_rect {
        let fill_rect = egui::Rect::from_min_size(
            egui::pos2(background.x as f32, background.y as f32),
            egui::vec2(background.width as f32, background.height as f32),
        );
        painter.rect_filled(fill_rect, 0.0, egui::Color32::from_gray(28));
    }

    for segment in &state.grid_lines {
        painter.line_segment(
            [
                egui::pos2(segment.from.x as f32, segment.from.y as f32),
                egui::pos2(segment.to.x as f32, segment.to.y as f32),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
        );
    }
    for segment in &state.center_mark_segments {
        painter.line_segment(
            [
                egui::pos2(segment.from.x as f32, segment.from.y as f32),
                egui::pos2(segment.to.x as f32, segment.to.y as f32),
            ],
            egui::Stroke::new(2.0, egui::Color32::from_rgb(208, 208, 208)),
        );
    }

    for segment in &state.path_segments {
        painter.line_segment(
            [
                egui::pos2(segment.from.x as f32, segment.from.y as f32),
                egui::pos2(segment.to.x as f32, segment.to.y as f32),
            ],
            egui::Stroke::new(2.0, egui::Color32::LIGHT_BLUE),
        );
    }
    for segment in &state.dashed_path_segments {
        painter.add(PathShape::line(
            vec![
                egui::pos2(segment.from.x as f32, segment.from.y as f32),
                egui::pos2(segment.to.x as f32, segment.to.y as f32),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(128, 196, 255)),
        ));
    }

    for point in &state.position_circles {
        painter.circle_filled(
            egui::pos2(point.x as f32, point.y as f32),
            6.0,
            egui::Color32::from_rgb(255, 188, 32),
        );
    }

    for label in &state.position_labels {
        painter.text(
            egui::pos2(label.point.x as f32 + 8.0, label.point.y as f32 - 8.0),
            egui::Align2::LEFT_TOP,
            &label.text,
            egui::TextStyle::Body.resolve(ui.style()),
            egui::Color32::WHITE,
        );
    }
    for axis in &state.axis_labels {
        painter.text(
            egui::pos2(axis.position.x as f32, axis.position.y as f32),
            egui::Align2::CENTER_CENTER,
            &axis.text,
            egui::TextStyle::Button.resolve(ui.style()),
            egui::Color32::from_gray(200),
        );
    }

    for segment in &state.selection_segments {
        painter.line_segment(
            [
                egui::pos2(segment.from.x as f32, segment.from.y as f32),
                egui::pos2(segment.to.x as f32, segment.to.y as f32),
            ],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 196, 255)),
        );
    }

    if let Some(header_rect) = state.header_overlay_rect {
        let overlay = egui::Rect::from_min_size(
            egui::pos2(header_rect.x as f32, header_rect.y as f32),
            egui::vec2(header_rect.width as f32, header_rect.height as f32),
        );
        painter.rect_filled(overlay, 0.0, egui::Color32::from_gray(18));
    }
    if !state.legend_entries.is_empty() {
        let legend_width = (state.floor_width * 0.25) as f32;
        let legend_height = ((state.legend_entries.len() as f32) * 24.0 + 24.0).max(48.0);
        let legend_rect = egui::Rect::from_min_size(
            egui::pos2(
                (state.floor_x + state.floor_width - f64::from(legend_width) - 12.0) as f32,
                (state.floor_y + 12.0) as f32,
            ),
            egui::vec2(legend_width, legend_height),
        );
        painter.rect_filled(legend_rect, 6.0, egui::Color32::from_gray(24));
        for (index, entry) in state.legend_entries.iter().enumerate() {
            let y = legend_rect.top() + 12.0 + index as f32 * 24.0;
            let color = egui::Color32::from_rgba_unmultiplied(
                entry.color[0],
                entry.color[1],
                entry.color[2],
                entry.color[3],
            );
            painter.circle_filled(egui::pos2(legend_rect.left() + 10.0, y + 6.0), 5.0, color);
            painter.text(
                egui::pos2(legend_rect.left() + 24.0, y),
                egui::Align2::LEFT_TOP,
                &entry.label,
                egui::TextStyle::Body.resolve(ui.style()),
                egui::Color32::WHITE,
            );
        }
    }
    if let Some(remaining) = state.placement_remaining
        && remaining > 0
    {
        painter.text(
            egui::pos2(state.floor_x as f32 + 12.0, state.floor_y as f32 + 12.0),
            egui::Align2::LEFT_TOP,
            format!("Remaining: {remaining}"),
            egui::TextStyle::Button.resolve(ui.style()),
            egui::Color32::from_rgb(255, 215, 64),
        );
    }

    actions
}
