use egui::Button;
use egui::Image;
use egui::Response;
use egui::Ui;
use egui::vec2;

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
        TopBarIcon::Settings => Image::new(egui::include_image!("../../../assets/icons/Pen.svg")),
        TopBarIcon::Home => Image::new(egui::include_image!("../../../assets/icons/Home.svg")),
        TopBarIcon::Image => Image::new(egui::include_image!("../../../assets/icons/Svg.svg")),
        TopBarIcon::Audio => {
            Image::new(egui::include_image!("../../../assets/icons/PlayCircle.svg"))
        }
    }
}

#[must_use]
pub fn top_bar_icon_button(ui: &mut Ui, image: Image<'static>, checked: bool) -> Response {
    let tint = if checked {
        ui.visuals().selection.stroke.color
    } else {
        ui.visuals().widgets.inactive.fg_stroke.color
    };
    let button = Button::image(
        image
            .fit_to_exact_size(vec2(ICON_GLYPH_SIZE_PX, ICON_GLYPH_SIZE_PX))
            .tint(tint),
    )
    .selected(checked)
    .frame(true)
    .frame_when_inactive(false)
    .corner_radius(ICON_BUTTON_SIZE_PX / 2.0)
    .min_size(vec2(ICON_BUTTON_SIZE_PX, ICON_BUTTON_SIZE_PX))
    .image_tint_follows_text_color(false);
    ui.add(button)
}
