use egui::Color32;
use egui::CornerRadius;
use egui::Id;
use egui::PopupCloseBehavior;
use egui::Rect;
use egui::RichText;
use egui::ScrollArea;
use egui::Sense;
use egui::Shape;
use egui::Stroke;
use egui::TextStyle;
use egui::TextWrapMode;
use egui::Ui;
use egui::WidgetText;
use egui::pos2;
use egui::vec2;

const FIELD_CORNER_RADIUS: u8 = 4;
const FIELD_TEXT_PADDING_PX: f32 = 16.0;
const FIELD_ARROW_INSET_PX: f32 = 16.0;
const FIELD_ARROW_SIZE_PX: f32 = 12.0;
const MENU_MAX_VISIBLE_ITEMS: usize = 6;

#[must_use]
pub fn mode_dropdown(
    ui: &mut Ui,
    id: Id,
    selected_index: Option<usize>,
    labels: &[&str],
    enabled: bool,
    width: f32,
    item_height: f32,
) -> Option<usize> {
    let popup_id = id.with("popup");
    let selected_text = selected_index
        .and_then(|index| labels.get(index))
        .copied()
        .unwrap_or_default();
    let (rect, response) = ui.allocate_exact_size(vec2(width, item_height), Sense::click());
    if enabled && response.clicked() {
        egui::Popup::toggle_id(ui.ctx(), popup_id);
    }

    let is_open = egui::Popup::is_id_open(ui.ctx(), popup_id);
    paint_dropdown_field(ui, rect, selected_text, enabled, is_open, is_open);

    let mut selected = None;
    let _ = egui::Popup::from_response(&response)
        .id(popup_id)
        .open_memory(None)
        .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
        .align(egui::RectAlign::TOP_START)
        .width(width)
        .show(|ui| {
            ui.set_min_width(width);

            let menu_fill = ui.visuals().widgets.noninteractive.bg_fill;
            let menu_stroke = ui.visuals().widgets.noninteractive.bg_stroke;
            egui::Frame::new()
                .fill(menu_fill)
                .stroke(menu_stroke)
                .corner_radius(CornerRadius::same(FIELD_CORNER_RADIUS))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

                    ScrollArea::vertical()
                        .max_height(item_height * MENU_MAX_VISIBLE_ITEMS as f32)
                        .show(ui, |ui| {
                            ui.set_min_width(width);
                            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                            for (index, label) in labels.iter().enumerate() {
                                let is_selected = selected_index == Some(index);
                                let response = ui.add_sized(
                                    vec2(width, item_height),
                                    egui::Button::selectable(
                                        is_selected,
                                        RichText::new(*label).text_style(TextStyle::Body),
                                    )
                                    .corner_radius(0.0),
                                );
                                if response.clicked() {
                                    selected = Some(index);
                                    egui::Popup::close_id(ui.ctx(), popup_id);
                                }
                            }
                        });
                });
        });

    selected
}

fn paint_dropdown_field(
    ui: &Ui,
    rect: Rect,
    text: &str,
    enabled: bool,
    active: bool,
    arrow_up: bool,
) {
    let visuals = &ui.visuals().widgets;
    let fill = if enabled {
        visuals.inactive.weak_bg_fill
    } else {
        visuals.inactive.weak_bg_fill.gamma_multiply(0.38)
    };
    let text_color = if enabled {
        ui.visuals().text_color()
    } else {
        ui.visuals().text_color().gamma_multiply(0.38)
    };
    let indicator_color = if active {
        visuals.active.fg_stroke.color
    } else {
        visuals.inactive.fg_stroke.color
    };
    let indicator_height = if active { 3.0 } else { 1.0 };

    ui.painter()
        .rect_filled(rect, CornerRadius::same(FIELD_CORNER_RADIUS), fill);

    let arrow_center = pos2(rect.right() - FIELD_ARROW_INSET_PX, rect.center().y);
    paint_dropdown_arrow(ui, arrow_center, indicator_color, arrow_up);

    let text_max_width =
        (arrow_center.x - FIELD_ARROW_SIZE_PX - (rect.left() + FIELD_TEXT_PADDING_PX)).max(0.0);
    let galley = WidgetText::from(text.to_owned()).into_galley(
        ui,
        Some(TextWrapMode::Truncate),
        text_max_width,
        TextStyle::Body,
    );
    let text_pos = pos2(
        rect.left() + FIELD_TEXT_PADDING_PX,
        rect.center().y - galley.size().y * 0.5,
    );
    ui.painter().galley(text_pos, galley, text_color);

    let indicator_rect = Rect::from_min_max(
        pos2(rect.left(), rect.bottom() - indicator_height),
        rect.right_bottom(),
    );
    ui.painter()
        .rect_filled(indicator_rect, 0.0, indicator_color);
}

fn paint_dropdown_arrow(ui: &Ui, center: egui::Pos2, color: Color32, arrow_up: bool) {
    let half_width = FIELD_ARROW_SIZE_PX * 0.5;
    let half_height = FIELD_ARROW_SIZE_PX * 0.35;
    let points = if arrow_up {
        vec![
            pos2(center.x - half_width, center.y + half_height),
            pos2(center.x, center.y - half_height),
            pos2(center.x + half_width, center.y + half_height),
        ]
    } else {
        vec![
            pos2(center.x - half_width, center.y - half_height),
            pos2(center.x + half_width, center.y - half_height),
            pos2(center.x, center.y + half_height),
        ]
    };
    ui.painter()
        .add(Shape::convex_polygon(points, color, Stroke::NONE));
}
