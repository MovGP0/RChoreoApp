use egui::Event;
use egui::Rect;
use egui::TouchPhase;
use egui::Ui;
use egui::epaint::PathShape;

use super::actions::FloorAction;
use super::state::FloorState;
use super::state::Point;
use super::state::TouchAction;
use super::state::TouchDeviceType;
use super::translations::floor_translations;

pub fn draw(ui: &mut Ui, state: &FloorState) -> Vec<FloorAction> {
    let mut actions: Vec<FloorAction> = Vec::new();
    let available = ui.available_size();
    actions.push(FloorAction::SetLayout {
        width_px: f64::from(available.x),
        height_px: f64::from(available.y),
    });
    actions.push(FloorAction::DrawFloor);

    let (rect, response) = ui.allocate_exact_size(available, egui::Sense::drag());
    let painter = ui.painter_at(rect);
    collect_interactions(ui, rect, response.hovered(), &mut actions);

    let to_screen_point = |point: Point| -> egui::Pos2 {
        egui::pos2(rect.min.x + point.x as f32, rect.min.y + point.y as f32)
    };
    let to_screen_rect = |x: f64, y: f64, width: f64, height: f64| -> Rect {
        Rect::from_min_size(
            egui::pos2(rect.min.x + x as f32, rect.min.y + y as f32),
            egui::vec2(width as f32, height as f32),
        )
    };

    if let Some(background) = state.background_rect {
        let fill_rect = to_screen_rect(
            background.x,
            background.y,
            background.width,
            background.height,
        );
        painter.rect_filled(fill_rect, 0.0, ui.visuals().extreme_bg_color);
    }

    for segment in &state.grid_lines {
        painter.line_segment(
            [to_screen_point(segment.from), to_screen_point(segment.to)],
            egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
        );
    }
    for segment in &state.center_mark_segments {
        painter.line_segment(
            [to_screen_point(segment.from), to_screen_point(segment.to)],
            egui::Stroke::new(2.0, ui.visuals().strong_text_color()),
        );
    }

    if let Some(bounds) = state.svg_overlay_bounds {
        painter.rect_stroke(
            to_screen_rect(bounds.x, bounds.y, bounds.width, bounds.height),
            0.0,
            egui::Stroke::new(1.0, ui.visuals().widgets.active.bg_fill),
            egui::StrokeKind::Middle,
        );
    }

    for segment in &state.path_segments {
        painter.line_segment(
            [to_screen_point(segment.from), to_screen_point(segment.to)],
            egui::Stroke::new(2.0, ui.visuals().selection.bg_fill),
        );
    }
    for segment in &state.dashed_path_segments {
        painter.add(PathShape::line(
            vec![to_screen_point(segment.from), to_screen_point(segment.to)],
            egui::Stroke::new(1.0, ui.visuals().selection.stroke.color),
        ));
    }

    for point in &state.position_circles {
        painter.circle_filled(
            to_screen_point(*point),
            6.0,
            ui.visuals().widgets.active.bg_fill,
        );
    }

    for label in &state.position_labels {
        painter.text(
            to_screen_point(Point::new(label.point.x + 12.0, label.point.y - 12.0)),
            egui::Align2::LEFT_TOP,
            &label.text,
            egui::TextStyle::Body.resolve(ui.style()),
            ui.visuals().strong_text_color(),
        );
    }
    for axis in &state.axis_labels {
        painter.text(
            to_screen_point(axis.position),
            egui::Align2::CENTER_CENTER,
            &axis.text,
            egui::TextStyle::Button.resolve(ui.style()),
            ui.visuals().widgets.noninteractive.fg_stroke.color,
        );
    }

    for segment in &state.selection_segments {
        painter.line_segment(
            [to_screen_point(segment.from), to_screen_point(segment.to)],
            egui::Stroke::new(1.0, ui.visuals().selection.bg_fill),
        );
    }

    if let Some(header_rect) = state.header_overlay_rect {
        let overlay = to_screen_rect(
            header_rect.x,
            header_rect.y,
            header_rect.width,
            header_rect.height,
        );
        painter.rect_filled(overlay, 0.0, ui.visuals().faint_bg_color);
    }
    if !state.legend_entries.is_empty() {
        let legend_width = (state.floor_width * 0.25) as f32;
        let legend_height = ((state.legend_entries.len() as f32) * 24.0 + 24.0).max(48.0);
        let legend_rect = Rect::from_min_size(
            to_screen_point(Point::new(
                state.floor_x + state.floor_width - f64::from(legend_width) - 12.0,
                state.floor_y + 12.0,
            )),
            egui::vec2(legend_width, legend_height),
        );
        painter.rect_filled(legend_rect, 6.0, ui.visuals().widgets.noninteractive.bg_fill);
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
                ui.visuals().strong_text_color(),
            );
        }
    }
    if let Some(remaining) = state.placement_remaining
        && remaining > 0
    {
        let strings = floor_translations("en");
        let start = to_screen_point(Point::new(state.floor_x + 12.0, state.floor_y + 12.0));
        painter.text(
            start,
            egui::Align2::LEFT_TOP,
            strings.placement_title,
            egui::TextStyle::Button.resolve(ui.style()),
            ui.visuals().strong_text_color(),
        );
        painter.text(
            egui::pos2(start.x, start.y + 24.0),
            egui::Align2::LEFT_TOP,
            strings.placement_hint,
            egui::TextStyle::Body.resolve(ui.style()),
            ui.visuals().text_color(),
        );
        painter.text(
            egui::pos2(start.x, start.y + 48.0),
            egui::Align2::LEFT_TOP,
            format!("{}{}", strings.placement_remaining_prefix, remaining),
            egui::TextStyle::Body.resolve(ui.style()),
            ui.visuals().selection.bg_fill,
        );
    }

    actions
}

fn collect_interactions(ui: &Ui, rect: Rect, is_hovered: bool, actions: &mut Vec<FloorAction>) {
    ui.input(|input| {
        let mut last_canvas_point: Option<Point> = None;
        for event in &input.events {
            match event {
                Event::PointerButton {
                    pos,
                    button,
                    pressed,
                    ..
                } if rect.contains(*pos) => {
                    let point = to_canvas_point(rect, *pos);
                    last_canvas_point = Some(point);
                    if *pressed {
                        actions.push(FloorAction::PointerPressed { point });
                    } else {
                        actions.push(FloorAction::PointerReleased { point });
                    }
                    if *button == egui::PointerButton::Secondary {
                        actions.push(FloorAction::ClearSelection);
                    }
                }
                Event::PointerMoved(pos) if rect.contains(*pos) => {
                    let point = to_canvas_point(rect, *pos);
                    last_canvas_point = Some(point);
                    actions.push(FloorAction::PointerMoved { point });
                }
                Event::MouseWheel {
                    delta, modifiers, ..
                } => {
                    let cursor = input
                        .pointer
                        .hover_pos()
                        .filter(|hover_pos| rect.contains(*hover_pos))
                        .map(|hover_pos| to_canvas_point(rect, hover_pos))
                        .or(last_canvas_point);
                    if is_hovered || cursor.is_some() {
                        actions.push(FloorAction::PointerWheelChanged {
                            delta_x: f64::from(delta.x),
                            delta_y: f64::from(delta.y),
                            ctrl: modifiers.ctrl,
                            cursor,
                        });
                    }
                }
                Event::Touch {
                    id, phase, pos, ..
                } if rect.contains(*pos) => {
                    let point = to_canvas_point(rect, *pos);
                    last_canvas_point = Some(point);
                    actions.push(FloorAction::Touch {
                        id: id.0 as i64,
                        action: map_touch_phase(*phase),
                        point,
                        is_in_contact: !matches!(phase, TouchPhase::End | TouchPhase::Cancel),
                        device: TouchDeviceType::Touch,
                    });
                }
                _ => {}
            }
        }
    });
}

fn to_canvas_point(rect: Rect, position: egui::Pos2) -> Point {
    Point::new(
        f64::from(position.x - rect.min.x),
        f64::from(position.y - rect.min.y),
    )
}

fn map_touch_phase(phase: TouchPhase) -> TouchAction {
    match phase {
        TouchPhase::Start => TouchAction::Pressed,
        TouchPhase::Move => TouchAction::Moved,
        TouchPhase::End => TouchAction::Released,
        TouchPhase::Cancel => TouchAction::Cancelled,
    }
}
