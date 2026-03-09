use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use egui::Color32;
use egui::Context;

use crate::material::styling::material_palette::material_palette_for_visuals;

static DISABLE_HOVER: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MaterialWindow {
    pub disable_hover: bool,
}

impl MaterialWindow {
    pub fn show<R>(self, _ctx: &Context, add_contents: impl FnOnce() -> R) -> R {
        let previous = DISABLE_HOVER.swap(self.disable_hover, Ordering::Relaxed);
        let result = add_contents();
        DISABLE_HOVER.store(previous, Ordering::Relaxed);
        result
    }
}

#[must_use]
pub fn material_window_disable_hover() -> bool {
    DISABLE_HOVER.load(Ordering::Relaxed)
}

#[must_use]
pub fn material_window_background(ctx: &Context) -> Color32 {
    material_palette_for_visuals(&ctx.style().visuals).background
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::MaterialWindow;
    use super::material_window_background;
    use super::material_window_disable_hover;

    #[test]
    fn material_window_scopes_disable_hover_flag() {
        let context = Context::default();
        assert!(!material_window_disable_hover());
        let observed = MaterialWindow { disable_hover: true }.show(&context, material_window_disable_hover);
        assert!(observed);
        assert!(!material_window_disable_hover());
    }

    #[test]
    fn material_window_background_matches_palette() {
        let context = Context::default();
        assert_ne!(material_window_background(&context), egui::Color32::TRANSPARENT);
    }
}
