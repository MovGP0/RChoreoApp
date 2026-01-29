use crossbeam_channel::Receiver;
use nject::injectable;
use slint::Color;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::settings::MaterialScheme;

use super::main_view_model::MainViewModel;

#[derive(Debug, Clone)]
pub struct MainStyle {
    pub background_color: Color,
    pub top_bar_color: Color,
    pub drawer_background_color: Color,
    pub dialog_background_color: Color,
    pub overlay_color: Color,
}

impl MainStyle {
    pub fn from_scheme(scheme: &MaterialScheme) -> Self {
        Self {
            background_color: to_slint_color(&scheme.surface_container_low),
            top_bar_color: to_slint_color(&scheme.surface),
            drawer_background_color: to_slint_color(&scheme.surface_variant),
            dialog_background_color: to_slint_color(&scheme.surface_container_high),
            overlay_color: to_slint_color(&scheme.scrim),
        }
    }
}

impl Default for MainStyle {
    fn default() -> Self {
        Self {
            background_color: Color::from_rgb_u8(0xF2, 0xF2, 0xF2),
            top_bar_color: Color::from_rgb_u8(0xFF, 0xFF, 0xFF),
            drawer_background_color: Color::from_rgb_u8(0xF2, 0xF2, 0xF2),
            dialog_background_color: Color::from_rgb_u8(0xFF, 0xFF, 0xFF),
            overlay_color: Color::from_argb_u8(0x66, 0x00, 0x00, 0x00),
        }
    }
}

#[injectable]
#[inject(|style: MainStyle, receiver: Option<Receiver<MaterialScheme>>| Self::new(style, receiver))]
pub struct MainStyleBehavior {
    style: MainStyle,
    receiver: Option<Receiver<MaterialScheme>>,
}

impl MainStyleBehavior {
    pub fn new(style: MainStyle, receiver: Option<Receiver<MaterialScheme>>) -> Self {
        Self { style, receiver }
    }

    pub fn apply(&self, view_model: &mut MainViewModel) {
        view_model.background_color = self.style.background_color;
        view_model.top_bar_color = self.style.top_bar_color;
        view_model.drawer_background_color = self.style.drawer_background_color;
        view_model.dialog_background_color = self.style.dialog_background_color;
        view_model.overlay_color = self.style.overlay_color;
    }

    pub fn try_handle(&mut self, view_model: &mut MainViewModel) -> bool {
        let Some(receiver) = &self.receiver else {
            return false;
        };

        match receiver.try_recv() {
            Ok(scheme) => {
                self.style = MainStyle::from_scheme(&scheme);
                self.apply(view_model);
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<MainViewModel> for MainStyleBehavior {
    fn activate(&self, view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        self.apply(view_model);
    }
}

fn to_slint_color(color: &choreo_master_mobile_json::Color) -> Color {
    Color::from_argb_u8(color.a, color.r, color.g, color.b)
}
