use crossbeam_channel::Receiver;
use slint::Color;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::settings::MaterialScheme;

use super::audio_player_view_model::AudioPlayerViewModel;

#[derive(Debug, Clone)]
pub struct AudioPlayerStyle {
    pub surface_color: Color,
    pub outline_color: Color,
    pub on_surface_variant_color: Color,
}

impl AudioPlayerStyle {
    pub fn from_scheme(scheme: &MaterialScheme) -> Self {
        Self {
            surface_color: to_slint_color(&scheme.surface_container),
            outline_color: to_slint_color(&scheme.outline_variant),
            on_surface_variant_color: to_slint_color(&scheme.on_surface_variant),
        }
    }
}

impl Default for AudioPlayerStyle {
    fn default() -> Self {
        Self {
            surface_color: Color::from_rgb_u8(0xF2, 0xF2, 0xF2),
            outline_color: Color::from_rgb_u8(0xD1, 0xD1, 0xD1),
            on_surface_variant_color: Color::from_rgb_u8(0x5E, 0x5E, 0x5E),
        }
    }
}

pub struct AudioPlayerStyleBehavior {
    style: AudioPlayerStyle,
    receiver: Option<Receiver<MaterialScheme>>,
}

impl AudioPlayerStyleBehavior {
    pub fn new(style: AudioPlayerStyle, receiver: Option<Receiver<MaterialScheme>>) -> Self {
        Self { style, receiver }
    }

    pub fn apply(&self, view_model: &mut AudioPlayerViewModel) {
        view_model.surface_color = self.style.surface_color;
        view_model.outline_color = self.style.outline_color;
        view_model.on_surface_variant_color = self.style.on_surface_variant_color;
    }

    pub fn try_handle(&mut self, view_model: &mut AudioPlayerViewModel) -> bool {
        let Some(receiver) = &self.receiver else {
            return false;
        };

        match receiver.try_recv() {
            Ok(scheme) => {
                self.style = AudioPlayerStyle::from_scheme(&scheme);
                self.apply(view_model);
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerStyleBehavior {
    fn activate(&self, view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        self.apply(view_model);
    }
}

fn to_slint_color(color: &choreo_master_mobile_json::Color) -> Color {
    Color::from_argb_u8(color.a, color.r, color.g, color.b)
}
