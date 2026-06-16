use egui::Event;
use egui::Rect;
use egui::TouchPhase;
use egui::Ui;

use crate::material::styling::material_palette::MaterialPalette;
use crate::material::styling::material_palette::material_palette_for_visuals;

use super::actions::FloorAction;
use super::axis_label_item;
use super::canvas_item;
use super::dancer_item;
use super::header_item;
use super::legend_item;
use super::path_item;
use super::placement_hint_item;
use super::selection_item;
use super::state::CanvasViewHandle;
use super::state::FloorLayer;
use super::state::FloorState;
use super::state::Point;
use super::state::PointerButton;
use super::state::PointerEventArgs;
use super::state::TouchAction;
use super::state::TouchDeviceType;
use super::state::TouchEventArgs;
pub use super::tokens::FloorCanvasColorRoles;
pub use super::tokens::floor_canvas_color_roles;

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

    let palette = material_palette_for_visuals(ui.visuals());
    let color_roles = floor_canvas_color_roles(palette);
    let style = ui.style();

    for layer in &state.layer_order {
        draw_layer(*layer, &painter, rect, state, style, palette, color_roles);
    }

    legend_item::draw_legend(&painter, rect, state, style, palette);
    placement_hint_item::draw_placement_hint(&painter, rect, state, style, palette);

    actions
}

fn draw_layer(
    layer: FloorLayer,
    painter: &egui::Painter,
    rect: Rect,
    state: &FloorState,
    style: &egui::Style,
    palette: MaterialPalette,
    color_roles: FloorCanvasColorRoles,
) {
    match layer {
        FloorLayer::Background => {
            canvas_item::draw_background(painter, rect, state, color_roles);
        }
        FloorLayer::GridLines => {
            canvas_item::draw_grid(painter, rect, state, palette, color_roles);
        }
        FloorLayer::FloorSvg => {
            canvas_item::draw_svg_overlay_bounds(painter, rect, state, palette);
        }
        FloorLayer::PathSegments => {
            path_item::draw_paths(painter, rect, state, palette, color_roles);
        }
        FloorLayer::PositionCircles => {
            dancer_item::draw_position_circles(painter, rect, state, palette);
        }
        FloorLayer::PositionNumbers => {
            dancer_item::draw_position_numbers(painter, rect, state, style, palette);
        }
        FloorLayer::SelectionSegments => {
            selection_item::draw_selection(painter, rect, state, palette);
        }
        FloorLayer::HeaderOverlay => {
            axis_label_item::draw_axis_labels(painter, rect, state, style, palette);
            header_item::draw_header(painter, rect, state, style, palette);
        }
    }
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
