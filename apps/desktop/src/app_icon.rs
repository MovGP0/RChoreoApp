use choreo_components::shell;
use egui::IconData;

const APP_ICON_SIZE_PX: u32 = 256;

pub(crate) fn load_window_icon() -> Option<IconData> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(shell::app_icon_svg(), &options).ok()?;

    let icon_size = APP_ICON_SIZE_PX;
    let mut pixmap = tiny_skia::Pixmap::new(icon_size, icon_size)?;
    let scale_x = icon_size as f32 / tree.size().width();
    let scale_y = icon_size as f32 / tree.size().height();
    let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    Some(IconData {
        rgba: pixmap.take(),
        width: icon_size,
        height: icon_size,
    })
}

#[cfg(test)]
mod tests {
    use super::APP_ICON_SIZE_PX;
    use super::load_window_icon;

    #[test]
    fn app_icon_rasterization_produces_rgba_pixels() {
        let icon = load_window_icon().expect("app icon svg should rasterize");

        assert_eq!(icon.width, APP_ICON_SIZE_PX);
        assert_eq!(icon.height, APP_ICON_SIZE_PX);
        assert_eq!(
            icon.rgba.len(),
            (APP_ICON_SIZE_PX as usize) * (APP_ICON_SIZE_PX as usize) * 4
        );
    }
}
