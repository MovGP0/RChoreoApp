const REFERENCE_FONT_SIZE_PX: f32 = 64.0;
const REFERENCE_HEIGHT_FACTOR_AT_64PX: f32 = 0.9472333;
const HEIGHT_FACTOR_SLOPE_PER_PX: f32 = -0.0006916667;
const MIN_HEIGHT_FACTOR: f32 = 0.70;

/// Returns a corrected font size in pixels so that the rendered text height
/// is approximately `font_size_px * zoom_factor`.
///
/// This uses a single linear correction model for the rendered height factor:
/// `factor(size) = reference + slope * (size - 64)`, anchored at 64px.
pub fn corrected_font_size_for_zoom(font_size_px: f32, zoom_factor: f32) -> f32 {
    if font_size_px <= 0.0 || zoom_factor <= 0.0 {
        return 0.0;
    }

    let desired_visual_size = font_size_px * zoom_factor;
    let correction_factor = linear_height_factor(desired_visual_size);
    desired_visual_size / correction_factor
}

fn linear_height_factor(size_px: f32) -> f32 {
    let factor = REFERENCE_HEIGHT_FACTOR_AT_64PX
        + HEIGHT_FACTOR_SLOPE_PER_PX * (size_px - REFERENCE_FONT_SIZE_PX);
    factor.max(MIN_HEIGHT_FACTOR)
}

#[cfg(test)]
mod tests {
    use super::corrected_font_size_for_zoom;

    const EPSILON: f32 = 0.0001;

    fn assert_close(actual: f32, expected: f32, epsilon: f32) {
        assert!(
            (actual - expected).abs() <= epsilon,
            "expected {expected} +/- {epsilon}, got {actual}"
        );
    }

    #[test]
    fn returns_zero_for_non_positive_inputs() {
        assert_eq!(corrected_font_size_for_zoom(0.0, 1.0), 0.0);
        assert_eq!(corrected_font_size_for_zoom(16.0, 0.0), 0.0);
        assert_eq!(corrected_font_size_for_zoom(-1.0, 1.0), 0.0);
    }

    #[test]
    fn uses_reference_factor_for_64px_visual_size() {
        let corrected = corrected_font_size_for_zoom(64.0, 1.0);
        let expected = 64.0 / 0.9472333;
        assert_close(corrected, expected, EPSILON);
    }

    #[test]
    fn scales_with_zoom_after_linear_correction() {
        let corrected_zoom_1 = corrected_font_size_for_zoom(24.0, 1.0);
        let corrected_zoom_2 = corrected_font_size_for_zoom(24.0, 2.0);
        assert!(corrected_zoom_2 > corrected_zoom_1);
    }
}
