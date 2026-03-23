use egui::Event;
use egui::Rect;
use egui::TouchPhase;
use egui::Ui;
use egui::epaint::PathShape;

use super::actions::FloorAction;
use super::state::CanvasViewHandle;
use super::state::FloorState;
use super::state::Point;
use super::state::PointerButton;
use super::state::PointerEventArgs;
use super::state::TouchAction;
use super::state::TouchDeviceType;
use super::state::TouchEventArgs;
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
    let floor_stroke = egui::Stroke::new(2.0, ui.visuals().selection.bg_fill);
    let floor_fill = color32_from_rgba(state.floor_color);
    let radius = floor_position_radius(state).max(6.0);

    if let Some(background) = state.background_rect {
        let fill_rect = to_screen_rect(
            background.x,
            background.y,
            background.width,
            background.height,
        );
        painter.rect_filled(fill_rect, 0.0, ui.visuals().extreme_bg_color);
    }
    let floor_rect = to_screen_rect(
        state.floor_x,
        state.floor_y,
        state.floor_width,
        state.floor_height,
    );
    painter.rect_filled(floor_rect, 0.0, floor_fill);

    for segment in &state.grid_lines {
        painter.line_segment(
            [to_screen_point(segment.from), to_screen_point(segment.to)],
            egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
        );
    }
    for segment in &state.center_mark_segments {
        painter.line_segment(
            [to_screen_point(segment.from), to_screen_point(segment.to)],
            floor_stroke,
        );
    }
    painter.circle_filled(
        to_screen_point(Point::new(state.center_x, state.center_y)),
        4.0,
        ui.visuals().selection.bg_fill,
    );
    painter.rect_stroke(floor_rect, 0.0, floor_stroke, egui::StrokeKind::Middle);

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
            floor_stroke,
        );
    }
    for segment in &state.dashed_path_segments {
        painter.add(PathShape::line(
            vec![to_screen_point(segment.from), to_screen_point(segment.to)],
            egui::Stroke::new(1.0, ui.visuals().selection.stroke.color),
        ));
    }

    if state.rendered_positions.is_empty() {
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
    } else {
        for position in &state.rendered_positions {
            let center = to_screen_point(position.point);
            if position.is_selected {
                painter.circle_stroke(
                    center,
                    radius + 4.0,
                    egui::Stroke::new(3.0, ui.visuals().selection.bg_fill),
                );
            }
            painter.circle_filled(center, radius, color32_from_rgba(position.fill_color));
            painter.circle_stroke(
                center,
                radius,
                egui::Stroke::new(2.0, color32_from_rgba(position.border_color)),
            );
            if !position.shortcut.trim().is_empty() {
                painter.text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    &position.shortcut,
                    egui::FontId::proportional((radius * 1.15).max(12.0)),
                    color32_from_rgba(position.text_color),
                );
            }
        }
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
        if !state.choreography_name.trim().is_empty() {
            painter.text(
                egui::pos2(overlay.center().x, overlay.top() + 12.0),
                egui::Align2::CENTER_TOP,
                &state.choreography_name,
                egui::TextStyle::Heading.resolve(ui.style()),
                ui.visuals().strong_text_color(),
            );
        }
        if !state.scene_name.trim().is_empty() {
            painter.text(
                egui::pos2(overlay.center().x, overlay.top() + 36.0),
                egui::Align2::CENTER_TOP,
                &state.scene_name,
                egui::TextStyle::Body.resolve(ui.style()),
                ui.visuals().widgets.noninteractive.fg_stroke.color,
            );
        }
    }
    if let Some(legend_panel_rect) = state.legend_panel_rect {
        let legend_rect = to_screen_rect(
            legend_panel_rect.x,
            legend_panel_rect.y,
            legend_panel_rect.width,
            legend_panel_rect.height,
        );
        painter.rect_filled(
            legend_rect,
            6.0,
            ui.visuals().widgets.noninteractive.bg_fill,
        );
        painter.rect_stroke(
            legend_rect,
            6.0,
            egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
            egui::StrokeKind::Middle,
        );
        let padding = 12.0;
        let row_height = 24.0;
        let square_x = legend_rect.left() + padding;
        let shortcut_x = square_x + 24.0;
        let name_x = shortcut_x + 48.0;
        let position_x = legend_rect.right() - padding;
        let start_y = legend_rect.top() + padding;
        for (index, entry) in state.legend_entries.iter().enumerate() {
            let y = start_y + index as f32 * row_height;
            let color = color32_from_rgba(entry.color);
            painter.circle_filled(egui::pos2(square_x, y + 6.0), 6.0, color);
            if !entry.shortcut.trim().is_empty() {
                painter.text(
                    egui::pos2(shortcut_x, y),
                    egui::Align2::LEFT_TOP,
                    &entry.shortcut,
                    egui::TextStyle::Body.resolve(ui.style()),
                    ui.visuals().strong_text_color(),
                );
            }
            painter.text(
                egui::pos2(name_x, y),
                egui::Align2::LEFT_TOP,
                &entry.name,
                egui::TextStyle::Body.resolve(ui.style()),
                ui.visuals().strong_text_color(),
            );
            if !entry.position_text.trim().is_empty() {
                painter.text(
                    egui::pos2(position_x, y),
                    egui::Align2::RIGHT_TOP,
                    &entry.position_text,
                    egui::TextStyle::Small.resolve(ui.style()),
                    ui.visuals().widgets.noninteractive.fg_stroke.color,
                );
            }
        }
    }
    if let Some(remaining) = state.placement_remaining
        && remaining > 0
    {
        let strings = floor_translations("en");
        let start = egui::pos2(rect.left() + 12.0, rect.top() + 12.0);
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

fn color32_from_rgba(color: [u8; 4]) -> egui::Color32 {
    egui::Color32::from_rgba_unmultiplied(color[0], color[1], color[2], color[3])
}

fn floor_position_radius(state: &FloorState) -> f32 {
    let width_meters = f64::from((state.floor_left + state.floor_right).max(1));
    let height_meters = f64::from((state.floor_front + state.floor_back).max(1));
    let scale_x = state.floor_width / width_meters;
    let scale_y = state.floor_height / height_meters;
    let scale = scale_x.min(scale_y);
    ((state.dancer_size.max(1.0) * scale) / 2.0) as f32
}

fn collect_interactions(ui: &Ui, rect: Rect, is_hovered: bool, actions: &mut Vec<FloorAction>) {
    let canvas_view = CanvasViewHandle::default();
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
                    let event_args = PointerEventArgs {
                        position: point,
                        button: map_pointer_button(*button),
                        is_in_contact: *pressed,
                    };
                    if *pressed {
                        actions.push(FloorAction::PointerPressedWithContext {
                            canvas_view,
                            event_args,
                        });
                    } else {
                        actions.push(FloorAction::PointerReleasedWithContext {
                            canvas_view,
                            event_args,
                        });
                    }
                }
                Event::PointerMoved(pos) if rect.contains(*pos) => {
                    let point = to_canvas_point(rect, *pos);
                    last_canvas_point = Some(point);
                    actions.push(FloorAction::PointerMovedWithContext {
                        canvas_view,
                        event_args: PointerEventArgs {
                            position: point,
                            button: PointerButton::Primary,
                            is_in_contact: input.pointer.primary_down(),
                        },
                    });
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
                        actions.push(FloorAction::PointerWheelChangedWithContext {
                            canvas_view,
                            delta_x: f64::from(delta.x),
                            delta_y: f64::from(delta.y),
                            control_modifier: modifiers.ctrl,
                            position: cursor,
                        });
                    }
                }
                Event::Touch { id, phase, pos, .. } if rect.contains(*pos) => {
                    let point = to_canvas_point(rect, *pos);
                    last_canvas_point = Some(point);
                    actions.push(FloorAction::TouchWithContext {
                        canvas_view,
                        event_args: TouchEventArgs {
                            id: id.0 as i64,
                            action: map_touch_phase(*phase),
                            device_type: TouchDeviceType::Touch,
                            location: point,
                            in_contact: !matches!(phase, TouchPhase::End | TouchPhase::Cancel),
                        },
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

fn map_pointer_button(button: egui::PointerButton) -> PointerButton {
    match button {
        egui::PointerButton::Primary => PointerButton::Primary,
        egui::PointerButton::Secondary
        | egui::PointerButton::Middle
        | egui::PointerButton::Extra1
        | egui::PointerButton::Extra2 => PointerButton::Secondary,
    }
}
