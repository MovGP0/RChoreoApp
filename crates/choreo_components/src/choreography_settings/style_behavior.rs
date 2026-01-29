use crossbeam_channel::Receiver;
use nject::injectable;
use slint::Color;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::settings::MaterialScheme;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;

#[derive(Debug, Clone)]
pub struct ChoreographySettingsStyle {
    pub surface_color: Color,
    pub surface_container_color: Color,
    pub outline_variant_color: Color,
}

impl ChoreographySettingsStyle {
    pub fn from_scheme(scheme: &MaterialScheme) -> Self {
        Self {
            surface_color: to_slint_color(&scheme.surface_variant),
            surface_container_color: to_slint_color(&scheme.surface_container),
            outline_variant_color: to_slint_color(&scheme.outline_variant),
        }
    }
}

impl Default for ChoreographySettingsStyle {
    fn default() -> Self {
        Self {
            surface_color: Color::from_rgb_u8(0xF2, 0xF2, 0xF2),
            surface_container_color: Color::from_rgb_u8(0xFF, 0xFF, 0xFF),
            outline_variant_color: Color::from_rgb_u8(0xD1, 0xD1, 0xD1),
        }
    }
}

#[injectable]
#[inject(
    |style: ChoreographySettingsStyle, receiver: Option<Receiver<MaterialScheme>>| Self::new(style, receiver)
)]
pub struct ChoreographySettingsStyleBehavior {
    style: ChoreographySettingsStyle,
    receiver: Option<Receiver<MaterialScheme>>,
}

impl ChoreographySettingsStyleBehavior {
    pub fn new(style: ChoreographySettingsStyle, receiver: Option<Receiver<MaterialScheme>>) -> Self {
        Self { style, receiver }
    }

    pub fn apply(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.surface_color = self.style.surface_color;
        view_model.surface_container_color = self.style.surface_container_color;
        view_model.outline_variant_color = self.style.outline_variant_color;
    }

    pub fn try_handle(&mut self, view_model: &mut ChoreographySettingsViewModel) -> bool {
        let Some(receiver) = &self.receiver else {
            return false;
        };

        match receiver.try_recv() {
            Ok(scheme) => {
                self.style = ChoreographySettingsStyle::from_scheme(&scheme);
                self.apply(view_model);
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<ChoreographySettingsViewModel> for ChoreographySettingsStyleBehavior {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        self.apply(view_model);
    }
}

fn to_slint_color(color: &choreo_master_mobile_json::Color) -> Color {
    Color::from_argb_u8(color.a, color.r, color.g, color.b)
}
