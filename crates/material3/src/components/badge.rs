use egui::Align2;
use egui::CornerRadius;
use egui::FontId;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::pos2;
use egui::vec2;

use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::TypographyRole;
use crate::styling::material_typography::font_id_for_role;

pub fn badge(ui: &mut Ui, text: &str) -> Response {
    let palette = material_palette_for_visuals(ui.visuals());
    let badge_rect = badge_rect(text, font_id_for_role(TypographyRole::LabelSmall), ui);
    let (rect, response) = ui.allocate_exact_size(badge_rect.size(), Sense::hover());

    let draw_rect = Rect::from_min_size(rect.min, badge_rect.size());
    let background_rect = badge_background_rect(draw_rect, text);
    let fill = palette.error;
    let text_color = palette.on_error;
    let rounding = CornerRadius::same((background_rect.height() / 2.0).round() as u8);

    ui.painter().rect_filled(background_rect, rounding, fill);
    if !text.is_empty() {
        ui.painter().text(
            draw_rect.center(),
            Align2::CENTER_CENTER,
            text,
            font_id_for_role(TypographyRole::LabelSmall),
            text_color,
        );
    }

    response
}

#[must_use]
pub fn badge_size(text: &str, font_id: FontId, ui: &Ui) -> egui::Vec2 {
    let metrics = material_style_metrics();
    if text.is_empty() {
        return vec2(metrics.sizes.size_16, metrics.sizes.size_16);
    }

    let galley =
        ui.fonts(|fonts| fonts.layout_no_wrap(text.to_owned(), font_id, ui.visuals().text_color()));
    let width = (galley.size().x + metrics.paddings.padding_4 * 2.0).max(metrics.sizes.size_16);
    let height = galley.size().y.max(metrics.sizes.size_16);
    vec2(width, height)
}

#[must_use]
pub fn badge_rect(text: &str, font_id: FontId, ui: &Ui) -> Rect {
    Rect::from_min_size(pos2(0.0, 0.0), badge_size(text, font_id, ui))
}

#[must_use]
fn badge_background_rect(draw_rect: Rect, text: &str) -> Rect {
    if text.is_empty() {
        let dot_size = vec2(6.0, 6.0);
        return Rect::from_center_size(draw_rect.center(), dot_size);
    }

    draw_rect
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::badge_size;
    use crate::styling::material_typography::TypographyRole;
    use crate::styling::material_typography::font_id_for_role;

    #[test]
    fn empty_badge_keeps_minimum_outer_size() {
        let context = Context::default();
        let mut size = egui::Vec2::ZERO;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                size = badge_size("", font_id_for_role(TypographyRole::LabelSmall), ui);
            });
        });
        assert_eq!(size.x, 16.0);
        assert_eq!(size.y, 16.0);
    }

    #[test]
    fn text_badge_respects_minimum_size() {
        let context = Context::default();
        let mut size = egui::Vec2::ZERO;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                size = badge_size("1", font_id_for_role(TypographyRole::LabelSmall), ui);
            });
        });
        assert!(size.x >= 16.0);
        assert!(size.y >= 16.0);
    }
}
