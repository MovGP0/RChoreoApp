use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;

const EPSILON: f64 = 0.0001;
const ZOOM_BEFORE: f64 = 1.0;
const ZOOM_AFTER: f64 = 1.6;
const ZOOM_OUT: f64 = 0.6;

fn assert_close(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "expected {expected} +/- {epsilon}, got {actual}"
    );
}

fn measure_at_zoom(state: &mut FloorState, zoom: f64, measure: impl Fn(&FloorState) -> f64) -> f64 {
    reduce(state, FloorAction::SetZoom { zoom });
    measure(state)
}

fn assert_zoom_scaled_measurement_between(
    state: &mut FloorState,
    label: &str,
    measure: impl Fn(&FloorState) -> f64,
    zoom_from: f64,
    zoom_to: f64,
    epsilon: f64,
) {
    let before = measure_at_zoom(state, zoom_from, &measure);
    let after = measure_at_zoom(state, zoom_to, &measure);

    assert!(
        before > 0.0,
        "{label} baseline must be positive, got {before}"
    );

    let expected_scale = zoom_to / zoom_from;
    let actual_scale = after / before;
    assert_close(actual_scale, expected_scale, epsilon);
}

fn assert_zoom_scaled_measurement(
    state: &mut FloorState,
    label: &str,
    measure: impl Fn(&FloorState) -> f64,
    epsilon: f64,
) {
    assert_zoom_scaled_measurement_between(state, label, measure, ZOOM_BEFORE, ZOOM_AFTER, epsilon);
}

fn assert_measurement_scales_like_panel_dimension_between(
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

    assert!(
        before > 0.0,
        "{label} baseline must be positive, got {before}"
    );
    assert!(
        panel_before > 0.0,
        "{label} panel baseline must be positive, got {panel_before}"
    );

    let measurement_scale = after / before;
    let panel_scale = panel_after / panel_before;
    assert_close(measurement_scale, panel_scale, epsilon);
}

#[test]
fn floor_canvas_zoom_layout_scales_layout_metrics_for_zoom_in_and_out() {
    let mut state = FloorState::default();

    assert_zoom_scaled_measurement(
        &mut state,
        "header_bottom",
        |s| s.metrics.header_bottom,
        EPSILON,
    );
    assert_zoom_scaled_measurement(&mut state, "floor_top", |s| s.metrics.floor_top, EPSILON);
    assert_zoom_scaled_measurement(
        &mut state,
        "floor_width",
        |s| s.metrics.floor_width,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "floor_height",
        |s| s.metrics.floor_height,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_panel_width",
        |s| s.metrics.legend_panel_width,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_panel_height",
        |s| s.metrics.legend_panel_height,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "side_label_left_gap",
        |s| s.metrics.side_label_left_gap,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "side_label_right_gap",
        |s| s.metrics.side_label_right_gap,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "top_label_vertical_gap",
        |s| s.metrics.top_label_vertical_gap,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "bottom_label_vertical_gap",
        |s| s.metrics.bottom_label_vertical_gap,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_content_padding_left",
        |s| s.metrics.legend_content_padding_left,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_content_padding_top",
        |s| s.metrics.legend_content_padding_top,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_content_padding_right",
        |s| s.metrics.legend_content_padding_right,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_content_padding_bottom",
        |s| s.metrics.legend_content_padding_bottom,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_first_square_offset_x",
        |s| s.metrics.legend_first_square_offset_x,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_first_square_offset_y",
        |s| s.metrics.legend_first_square_offset_y,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_first_row_offset_x",
        |s| s.metrics.legend_first_row_offset_x,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_first_row_offset_y",
        |s| s.metrics.legend_first_row_offset_y,
        EPSILON,
    );
    assert_zoom_scaled_measurement(
        &mut state,
        "legend_last_row_bottom_gap",
        |s| s.metrics.legend_last_row_bottom_gap,
        EPSILON,
    );

    assert_measurement_scales_like_panel_dimension_between(
        &mut state,
        "legend_content_padding_left_vs_width",
        |s| s.metrics.legend_content_padding_left,
        |s| s.metrics.legend_panel_width,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        EPSILON,
    );
    assert_measurement_scales_like_panel_dimension_between(
        &mut state,
        "legend_content_padding_left_vs_width_zoom_out",
        |s| s.metrics.legend_content_padding_left,
        |s| s.metrics.legend_panel_width,
        ZOOM_BEFORE,
        ZOOM_OUT,
        EPSILON,
    );
    assert_measurement_scales_like_panel_dimension_between(
        &mut state,
        "legend_content_padding_top_vs_height",
        |s| s.metrics.legend_content_padding_top,
        |s| s.metrics.legend_panel_height,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        EPSILON,
    );
    assert_measurement_scales_like_panel_dimension_between(
        &mut state,
        "legend_content_padding_top_vs_height_zoom_out",
        |s| s.metrics.legend_content_padding_top,
        |s| s.metrics.legend_panel_height,
        ZOOM_BEFORE,
        ZOOM_OUT,
        EPSILON,
    );
    assert_measurement_scales_like_panel_dimension_between(
        &mut state,
        "legend_content_padding_right_vs_width",
        |s| s.metrics.legend_content_padding_right,
        |s| s.metrics.legend_panel_width,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        EPSILON,
    );
    assert_measurement_scales_like_panel_dimension_between(
        &mut state,
        "legend_content_padding_right_vs_width_zoom_out",
        |s| s.metrics.legend_content_padding_right,
        |s| s.metrics.legend_panel_width,
        ZOOM_BEFORE,
        ZOOM_OUT,
        EPSILON,
    );
    assert_measurement_scales_like_panel_dimension_between(
        &mut state,
        "legend_content_padding_bottom_vs_height",
        |s| s.metrics.legend_content_padding_bottom,
        |s| s.metrics.legend_panel_height,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        EPSILON,
    );
    assert_measurement_scales_like_panel_dimension_between(
        &mut state,
        "legend_content_padding_bottom_vs_height_zoom_out",
        |s| s.metrics.legend_content_padding_bottom,
        |s| s.metrics.legend_panel_height,
        ZOOM_BEFORE,
        ZOOM_OUT,
        EPSILON,
    );

    let top_padding = measure_at_zoom(&mut state, ZOOM_OUT, |s| {
        s.metrics.legend_content_padding_top
    });
    let bottom_padding = measure_at_zoom(&mut state, ZOOM_OUT, |s| {
        s.metrics.legend_content_padding_bottom
    });
    assert_close(top_padding, bottom_padding, EPSILON);
}
