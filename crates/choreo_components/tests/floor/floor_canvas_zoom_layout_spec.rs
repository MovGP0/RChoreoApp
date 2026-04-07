use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;

const EPSILON: f64 = 0.0001;
const ZOOM_BEFORE: f64 = 1.0;
const ZOOM_AFTER: f64 = 1.6;
const ZOOM_OUT: f64 = 0.6;

fn measure_at_zoom(state: &mut FloorState, zoom: f64, measure: impl Fn(&FloorState) -> f64) -> f64 {
    reduce(state, FloorAction::SetZoom { zoom });
    measure(state)
}

fn check_close(
    errors: &mut Vec<String>,
    label: &str,
    actual: f64,
    expected: f64,
    epsilon: f64,
) {
    if (actual - expected).abs() > epsilon {
        errors.push(format!(
            "{label}: expected {expected} +/- {epsilon}, got {actual}"
        ));
    }
}

fn check_zoom_scaled_measurement_between(
    errors: &mut Vec<String>,
    state: &mut FloorState,
    label: &str,
    measure: impl Fn(&FloorState) -> f64,
    zoom_from: f64,
    zoom_to: f64,
    epsilon: f64,
) {
    let before = measure_at_zoom(state, zoom_from, &measure);
    let after = measure_at_zoom(state, zoom_to, &measure);

    if before <= 0.0 {
        errors.push(format!("{label} baseline must be positive, got {before}"));
        return;
    }

    let expected_scale = zoom_to / zoom_from;
    let actual_scale = after / before;
    check_close(errors, label, actual_scale, expected_scale, epsilon);
}

fn check_zoom_scaled_measurement(
    errors: &mut Vec<String>,
    state: &mut FloorState,
    label: &str,
    measure: impl Fn(&FloorState) -> f64,
    epsilon: f64,
) {
    check_zoom_scaled_measurement_between(
        errors,
        state,
        label,
        measure,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        epsilon,
    );
}

fn check_measurement_scales_like_panel_dimension_between(
    errors: &mut Vec<String>,
    state: &mut FloorState,
    label: &str,
    measure: impl Fn(&FloorState) -> f64,
    panel_measure: impl Fn(&FloorState) -> f64,
    zoom_from: f64,
    zoom_to: f64,
    epsilon: f64,
) {
    let before = measure_at_zoom(state, zoom_from, &measure);
    let after = measure_at_zoom(state, zoom_to, &measure);
    let panel_before = measure_at_zoom(state, zoom_from, &panel_measure);
    let panel_after = measure_at_zoom(state, zoom_to, &panel_measure);

    if before <= 0.0 {
        errors.push(format!("{label} baseline must be positive, got {before}"));
        return;
    }
    if panel_before <= 0.0 {
        errors.push(format!(
            "{label} panel baseline must be positive, got {panel_before}"
        ));
        return;
    }

    let measurement_scale = after / before;
    let panel_scale = panel_after / panel_before;
    check_close(errors, label, measurement_scale, panel_scale, epsilon);
}

#[test]
fn floor_canvas_zoom_layout_scales_layout_metrics_for_zoom_in_and_out() {
    let mut state = FloorState::default();
    let mut errors = Vec::new();

    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "header_bottom",
        |s| s.metrics.header_bottom,
        EPSILON,
    );
    check_zoom_scaled_measurement(&mut errors, &mut state, "floor_top", |s| s.metrics.floor_top, EPSILON);
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "floor_width",
        |s| s.metrics.floor_width,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "floor_height",
        |s| s.metrics.floor_height,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_panel_width",
        |s| s.metrics.legend_panel_width,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_panel_height",
        |s| s.metrics.legend_panel_height,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "side_label_left_gap",
        |s| s.metrics.side_label_left_gap,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "side_label_right_gap",
        |s| s.metrics.side_label_right_gap,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "top_label_vertical_gap",
        |s| s.metrics.top_label_vertical_gap,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "bottom_label_vertical_gap",
        |s| s.metrics.bottom_label_vertical_gap,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_content_padding_left",
        |s| s.metrics.legend_content_padding_left,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_content_padding_top",
        |s| s.metrics.legend_content_padding_top,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_content_padding_right",
        |s| s.metrics.legend_content_padding_right,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_content_padding_bottom",
        |s| s.metrics.legend_content_padding_bottom,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_first_square_offset_x",
        |s| s.metrics.legend_first_square_offset_x,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_first_square_offset_y",
        |s| s.metrics.legend_first_square_offset_y,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_first_row_offset_x",
        |s| s.metrics.legend_first_row_offset_x,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_first_row_offset_y",
        |s| s.metrics.legend_first_row_offset_y,
        EPSILON,
    );
    check_zoom_scaled_measurement(
        &mut errors,
        &mut state,
        "legend_last_row_bottom_gap",
        |s| s.metrics.legend_last_row_bottom_gap,
        EPSILON,
    );

    check_measurement_scales_like_panel_dimension_between(
        &mut errors,
        &mut state,
        "legend_content_padding_left_vs_width",
        |s| s.metrics.legend_content_padding_left,
        |s| s.metrics.legend_panel_width,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        EPSILON,
    );
    check_measurement_scales_like_panel_dimension_between(
        &mut errors,
        &mut state,
        "legend_content_padding_left_vs_width_zoom_out",
        |s| s.metrics.legend_content_padding_left,
        |s| s.metrics.legend_panel_width,
        ZOOM_BEFORE,
        ZOOM_OUT,
        EPSILON,
    );
    check_measurement_scales_like_panel_dimension_between(
        &mut errors,
        &mut state,
        "legend_content_padding_top_vs_height",
        |s| s.metrics.legend_content_padding_top,
        |s| s.metrics.legend_panel_height,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        EPSILON,
    );
    check_measurement_scales_like_panel_dimension_between(
        &mut errors,
        &mut state,
        "legend_content_padding_top_vs_height_zoom_out",
        |s| s.metrics.legend_content_padding_top,
        |s| s.metrics.legend_panel_height,
        ZOOM_BEFORE,
        ZOOM_OUT,
        EPSILON,
    );
    check_measurement_scales_like_panel_dimension_between(
        &mut errors,
        &mut state,
        "legend_content_padding_right_vs_width",
        |s| s.metrics.legend_content_padding_right,
        |s| s.metrics.legend_panel_width,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        EPSILON,
    );
    check_measurement_scales_like_panel_dimension_between(
        &mut errors,
        &mut state,
        "legend_content_padding_right_vs_width_zoom_out",
        |s| s.metrics.legend_content_padding_right,
        |s| s.metrics.legend_panel_width,
        ZOOM_BEFORE,
        ZOOM_OUT,
        EPSILON,
    );
    check_measurement_scales_like_panel_dimension_between(
        &mut errors,
        &mut state,
        "legend_content_padding_bottom_vs_height",
        |s| s.metrics.legend_content_padding_bottom,
        |s| s.metrics.legend_panel_height,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        EPSILON,
    );
    check_measurement_scales_like_panel_dimension_between(
        &mut errors,
        &mut state,
        "legend_content_padding_bottom_vs_height_zoom_out",
        |s| s.metrics.legend_content_padding_bottom,
        |s| s.metrics.legend_panel_height,
        ZOOM_BEFORE,
        ZOOM_OUT,
        EPSILON,
    );

    let top_padding = measure_at_zoom(&mut state, ZOOM_OUT, |s| s.metrics.legend_content_padding_top);
    let bottom_padding = measure_at_zoom(&mut state, ZOOM_OUT, |s| s.metrics.legend_content_padding_bottom);
    check_close(
        &mut errors,
        "top_padding_vs_bottom_padding",
        top_padding,
        bottom_padding,
        EPSILON,
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
