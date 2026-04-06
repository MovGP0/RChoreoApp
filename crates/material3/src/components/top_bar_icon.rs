use egui::Button;
use egui::Color32;
use egui::Image;
use egui::Response;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::components::centered_icon_rect;
use crate::components::paint_icon;
use crate::styling::material_palette::MaterialPalette;
use crate::styling::material_palette::material_palette_for_visuals;

const ICON_BUTTON_SIZE_PX: f32 = 48.0;
const ICON_GLYPH_SIZE_PX: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopBarIcon {
    Settings,
    Home,
    Image,
    Audio,
}

#[must_use]
pub const fn icon_uri(icon: TopBarIcon) -> &'static str {
    match icon {
        TopBarIcon::Settings => "bytes://top_bar/settings.svg",
        TopBarIcon::Home => "bytes://top_bar/home.svg",
        TopBarIcon::Image => "bytes://top_bar/image.svg",
        TopBarIcon::Audio => "bytes://top_bar/audio.svg",
    }
}

pub fn icon_image(icon: TopBarIcon) -> Image<'static> {
    match icon {
        TopBarIcon::Settings => Image::new(egui::include_image!("../../assets/icons/Pen.svg")),
        TopBarIcon::Home => Image::new(egui::include_image!("../../assets/icons/Home.svg")),
        TopBarIcon::Image => Image::new(egui::include_image!("../../assets/icons/Svg.svg")),
        TopBarIcon::Audio => Image::new(egui::include_image!("../../assets/icons/PlayCircle.svg")),
    }
}

#[must_use]
pub fn top_bar_icon_button(ui: &mut Ui, image: Image<'static>, checked: bool) -> Response {
    top_bar_icon_button_enabled(ui, image, checked, true)
}

#[must_use]
pub fn top_bar_icon_button_enabled(
    ui: &mut Ui,
    image: Image<'static>,
    checked: bool,
    enabled: bool,
) -> Response {
    let tint = icon_tint(material_palette_for_visuals(ui.visuals()), checked, enabled);
    let response = ui.add_enabled(
        enabled,
        Button::new("")
            .selected(checked)
            .frame(true)
            .frame_when_inactive(false)
            .stroke(Stroke::NONE)
            .corner_radius(ICON_BUTTON_SIZE_PX / 2.0)
            .min_size(vec2(ICON_BUTTON_SIZE_PX, ICON_BUTTON_SIZE_PX)),
    );
    paint_icon(
        ui,
        &image,
        centered_icon_rect(response.rect, vec2(ICON_GLYPH_SIZE_PX, ICON_GLYPH_SIZE_PX)),
        tint,
    );
    response
}

#[must_use]
fn icon_tint(palette: MaterialPalette, checked: bool, enabled: bool) -> Color32 {
    if !enabled {
        return palette.on_surface.gamma_multiply(palette.disable_opacity);
    }

    if checked {
        palette.on_secondary_container
    } else {
        palette.on_surface
    }
}

#[cfg(test)]
mod tests {
    use egui::Color32;

    use super::MaterialPalette;
    use super::icon_tint;

    fn palette_fixture() -> MaterialPalette {
        let mut palette = MaterialPalette::light();
        palette.on_surface = Color32::from_rgb(10, 20, 30);
        palette.on_secondary_container = Color32::from_rgb(40, 50, 60);
        palette.disable_opacity = 0.38;
        palette
    }

    #[test]
    fn enabled_top_bar_icon_tints_use_material_roles() {
        let palette = palette_fixture();

        assert_eq!(icon_tint(palette, false, true), palette.on_surface);
        assert_eq!(
            icon_tint(palette, true, true),
            palette.on_secondary_container
        );
    }

    #[test]
    fn disabled_top_bar_icon_tint_uses_material_disabled_opacity() {
        let palette = palette_fixture();

        assert_eq!(
            icon_tint(palette, false, false),
            palette.on_surface.gamma_multiply(palette.disable_opacity)
        );
        assert_eq!(
            icon_tint(palette, true, false),
            palette.on_surface.gamma_multiply(palette.disable_opacity)
        );
    }
}
