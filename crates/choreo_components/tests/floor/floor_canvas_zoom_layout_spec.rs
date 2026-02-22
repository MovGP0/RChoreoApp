use std::cell::Cell;
use std::time::Duration;

use crate::floor;

use choreo_components::AxisLabel;
use choreo_components::LegendEntry;
use choreo_components::ShellHost;
use floor::Report;
use slint::Color;
use slint::ComponentHandle;
use slint::LogicalSize;
use slint::ModelRc;
use slint::VecModel;

thread_local! {
    static INIT_TEST_BACKEND: Cell<bool> = const { Cell::new(false) };
}

const ZOOM_BEFORE: f32 = 1.0;
const ZOOM_AFTER: f32 = 1.6;
const ZOOM_OUT: f32 = 0.6;

fn ensure_slint_test_backend() {
    INIT_TEST_BACKEND.with(|initialized| {
        if initialized.get() {
            return;
        }
        i_slint_backend_testing::init_no_event_loop();
        initialized.set(true);
    });
}

fn pump_ui() {
    i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(20));
    slint::platform::update_timers_and_animations();
}

fn assert_close(actual: f32, expected: f32, epsilon: f32) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "expected {expected} +/- {epsilon}, got {actual}"
    );
}

fn configure_floor_for_layout_measurement(view: &ShellHost) {
    view.window().set_size(LogicalSize::new(1800.0, 1200.0));
    view.set_floor_front(6);
    view.set_floor_back(6);
    view.set_floor_left(6);
    view.set_floor_right(6);
    view.set_floor_pan_x(0.0);
    view.set_floor_pan_y(0.0);
    view.set_floor_zoom_factor(ZOOM_BEFORE);

    view.set_floor_show_axis_labels(true);
    view.set_floor_axis_labels_x(ModelRc::new(VecModel::from(vec![AxisLabel {
        value: 0.0,
        text: "0".into(),
    }])));
    view.set_floor_axis_labels_y(ModelRc::new(VecModel::from(vec![AxisLabel {
        value: 0.0,
        text: "0".into(),
    }])));

    view.set_floor_show_legend(true);
    let legend_entries: Vec<LegendEntry> = (0..20)
        .map(|index| LegendEntry {
            color: Color::from_rgb_u8(
                (32 + (index * 7) % 160) as u8,
                (64 + (index * 11) % 120) as u8,
                (96 + (index * 13) % 100) as u8,
            ),
            shortcut: format!("D{index:02}").into(),
            name: format!("Dancer {index:02}").into(),
            position_text: format!("({index}, {})", index + 1).into(),
        })
        .collect();
    view.set_floor_legend_entries(ModelRc::new(VecModel::from(legend_entries)));

    pump_ui();
}

fn measure_at_zoom(view: &ShellHost, zoom: f32, measure: impl Fn(&ShellHost) -> f32) -> f32 {
    view.set_floor_zoom_factor(zoom);
    pump_ui();
    measure(view)
}

fn assert_zoom_scaled_measurement(
    view: &ShellHost,
    label: &str,
    measure: impl Fn(&ShellHost) -> f32,
    epsilon: f32,
) {
    assert_zoom_scaled_measurement_between(
        view,
        label,
        measure,
        ZOOM_BEFORE,
        ZOOM_AFTER,
        epsilon,
    );
}

fn assert_zoom_scaled_measurement_between(
    view: &ShellHost,
    label: &str,
    measure: impl Fn(&ShellHost) -> f32,
    zoom_from: f32,
    zoom_to: f32,
    epsilon: f32,
) {
    let before = measure_at_zoom(view, zoom_from, &measure);
    let after = measure_at_zoom(view, zoom_to, &measure);

    assert!(before > 0.0, "{label} baseline must be positive, got {before}");

    let expected_scale = zoom_to / zoom_from;
    let actual_scale = after / before;
    assert_close(actual_scale, expected_scale, epsilon);
}

fn assert_measurement_scales_like_panel_dimension_between(
    view: &ShellHost,
    label: &str,
    measure: impl Fn(&ShellHost) -> f32,
    panel_measure: impl Fn(&ShellHost) -> f32,
    zoom_from: f32,
    zoom_to: f32,
    epsilon: f32,
) {
    let before = measure_at_zoom(view, zoom_from, &measure);
    let after = measure_at_zoom(view, zoom_to, &measure);
    let panel_before = measure_at_zoom(view, zoom_from, &panel_measure);
    let panel_after = measure_at_zoom(view, zoom_to, &panel_measure);

    assert!(before > 0.0, "{label} baseline must be positive, got {before}");
    assert!(
        panel_before > 0.0,
        "{label} panel baseline must be positive, got {panel_before}"
    );

    let measurement_scale = after / before;
    let panel_scale = panel_after / panel_before;
    assert_close(measurement_scale, panel_scale, epsilon);
}

#[test]
#[serial_test::serial]
fn floor_canvas_zoom_layout_spec() {
    let test_thread = std::thread::Builder::new()
        .name("floor-canvas-zoom-layout-spec".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            let suite = rspec::describe("floor canvas zoom layout", (), |spec| {
                spec.it("scales distance from control top to header bottom", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "header_bottom",
                        |v| v.get_floor_header_bottom(),
                        0.01,
                    );
                });

                spec.it("scales distance from control top to floor top", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "floor_top",
                        |v| v.get_floor_bounds_top(),
                        0.01,
                    );
                });

                spec.it("scales floor width", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(&view, "floor_width", |v| v.get_floor_width(), 0.01);
                });

                spec.it("scales floor height", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(&view, "floor_height", |v| v.get_floor_height(), 0.01);
                });

                spec.it("scales legend panel width", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_panel_width",
                        |v| v.get_floor_legend_panel_width(),
                        0.01,
                    );
                });

                spec.it("scales legend panel height", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_panel_height",
                        |v| v.get_floor_legend_panel_height(),
                        0.02,
                    );
                });

                spec.it("scales left side-label gap", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "side_label_left_gap",
                        |v| v.get_floor_side_label_left_gap(),
                        0.01,
                    );
                });

                spec.it("scales right side-label gap", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "side_label_right_gap",
                        |v| v.get_floor_side_label_right_gap(),
                        0.01,
                    );
                });

                spec.it("scales top label vertical gap", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "top_label_vertical_gap",
                        |v| v.get_floor_top_label_vertical_gap(),
                        0.01,
                    );
                });

                spec.it("scales bottom label vertical gap", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "bottom_label_vertical_gap",
                        |v| v.get_floor_bottom_label_vertical_gap(),
                        0.01,
                    );
                });

                spec.it("scales legend content left padding", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_content_padding_left",
                        |v| v.get_floor_legend_content_padding_left(),
                        0.01,
                    );
                });

                spec.it("scales legend left padding by same factor as legend width", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_measurement_scales_like_panel_dimension_between(
                        &view,
                        "legend_content_padding_left_vs_width",
                        |v| v.get_floor_legend_content_padding_left(),
                        |v| v.get_floor_legend_panel_width(),
                        ZOOM_BEFORE,
                        ZOOM_AFTER,
                        0.01,
                    );
                });

                spec.it("scales legend left padding by same factor as legend width when zooming out", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_measurement_scales_like_panel_dimension_between(
                        &view,
                        "legend_content_padding_left_vs_width_zoom_out",
                        |v| v.get_floor_legend_content_padding_left(),
                        |v| v.get_floor_legend_panel_width(),
                        ZOOM_BEFORE,
                        ZOOM_OUT,
                        0.01,
                    );
                });

                spec.it("scales legend content top padding", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_content_padding_top",
                        |v| v.get_floor_legend_content_padding_top(),
                        0.01,
                    );
                });

                spec.it("scales legend top padding by same factor as legend height", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_measurement_scales_like_panel_dimension_between(
                        &view,
                        "legend_content_padding_top_vs_height",
                        |v| v.get_floor_legend_content_padding_top(),
                        |v| v.get_floor_legend_panel_height(),
                        ZOOM_BEFORE,
                        ZOOM_AFTER,
                        0.02,
                    );
                });

                spec.it("scales legend top padding by same factor as legend height when zooming out", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_measurement_scales_like_panel_dimension_between(
                        &view,
                        "legend_content_padding_top_vs_height_zoom_out",
                        |v| v.get_floor_legend_content_padding_top(),
                        |v| v.get_floor_legend_panel_height(),
                        ZOOM_BEFORE,
                        ZOOM_OUT,
                        0.01,
                    );
                });

                spec.it("scales legend content top padding when zooming out", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement_between(
                        &view,
                        "legend_content_padding_top_zoom_out",
                        |v| v.get_floor_legend_content_padding_top(),
                        ZOOM_BEFORE,
                        ZOOM_OUT,
                        0.01,
                    );
                });

                spec.it("keeps legend top and bottom padding equal when zooming out", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    let top_padding = measure_at_zoom(&view, ZOOM_OUT, |v| {
                        v.get_floor_legend_content_padding_top()
                    });
                    let bottom_padding = measure_at_zoom(&view, ZOOM_OUT, |v| {
                        v.get_floor_legend_content_padding_bottom()
                    });
                    assert_close(top_padding, bottom_padding, 0.01);
                });

                spec.it("scales legend content right padding", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_content_padding_right",
                        |v| v.get_floor_legend_content_padding_right(),
                        0.01,
                    );
                });

                spec.it("scales legend right padding by same factor as legend width", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_measurement_scales_like_panel_dimension_between(
                        &view,
                        "legend_content_padding_right_vs_width",
                        |v| v.get_floor_legend_content_padding_right(),
                        |v| v.get_floor_legend_panel_width(),
                        ZOOM_BEFORE,
                        ZOOM_AFTER,
                        0.01,
                    );
                });

                spec.it(
                    "scales legend right padding by same factor as legend width when zooming out",
                    |_| {
                        ensure_slint_test_backend();
                        let view =
                            ShellHost::new().expect("shell host should be created in test backend");
                        configure_floor_for_layout_measurement(&view);

                        assert_measurement_scales_like_panel_dimension_between(
                            &view,
                            "legend_content_padding_right_vs_width_zoom_out",
                            |v| v.get_floor_legend_content_padding_right(),
                            |v| v.get_floor_legend_panel_width(),
                            ZOOM_BEFORE,
                            ZOOM_OUT,
                            0.01,
                        );
                    },
                );

                spec.it("scales legend content bottom padding", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_content_padding_bottom",
                        |v| v.get_floor_legend_content_padding_bottom(),
                        0.01,
                    );
                });

                spec.it("scales legend bottom padding by same factor as legend height", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_measurement_scales_like_panel_dimension_between(
                        &view,
                        "legend_content_padding_bottom_vs_height",
                        |v| v.get_floor_legend_content_padding_bottom(),
                        |v| v.get_floor_legend_panel_height(),
                        ZOOM_BEFORE,
                        ZOOM_AFTER,
                        0.02,
                    );
                });

                spec.it("scales legend bottom padding by same factor as legend height when zooming out", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_measurement_scales_like_panel_dimension_between(
                        &view,
                        "legend_content_padding_bottom_vs_height_zoom_out",
                        |v| v.get_floor_legend_content_padding_bottom(),
                        |v| v.get_floor_legend_panel_height(),
                        ZOOM_BEFORE,
                        ZOOM_OUT,
                        0.01,
                    );
                });

                spec.it("scales first legend color-square X offset from legend corner", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_first_square_offset_x",
                        |v| v.get_floor_legend_first_square_offset_x(),
                        0.01,
                    );
                });

                spec.it("scales first legend row-rectangle X offset from legend corner", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_first_row_offset_x",
                        |v| v.get_floor_legend_first_row_offset_x(),
                        0.01,
                    );
                });

                spec.it("scales first legend color-square Y offset from legend corner", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_first_square_offset_y",
                        |v| v.get_floor_legend_first_square_offset_y(),
                        0.02,
                    );
                });

                spec.it("scales first legend row-rectangle Y offset from legend corner", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_first_row_offset_y",
                        |v| v.get_floor_legend_first_row_offset_y(),
                        0.01,
                    );
                });

                spec.it("scales last legend row bottom gap to legend border", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    assert_zoom_scaled_measurement(
                        &view,
                        "legend_last_row_bottom_gap",
                        |v| v.get_floor_legend_last_row_bottom_gap(),
                        0.01,
                    );
                });

                spec.it("keeps text-height scaling aligned with font-size scaling", |_| {
                    ensure_slint_test_backend();
                    let view = ShellHost::new().expect("shell host should be created in test backend");
                    configure_floor_for_layout_measurement(&view);

                    let font_size_before =
                        measure_at_zoom(&view, ZOOM_BEFORE, |v| v.get_floor_legend_text_probe_font_size());
                    let text_height_before =
                        measure_at_zoom(&view, ZOOM_BEFORE, |v| v.get_floor_legend_text_probe_height());

                    let font_size_after =
                        measure_at_zoom(&view, ZOOM_AFTER, |v| v.get_floor_legend_text_probe_font_size());
                    let text_height_after =
                        measure_at_zoom(&view, ZOOM_AFTER, |v| v.get_floor_legend_text_probe_height());

                    assert!(font_size_before > 0.0, "font size baseline must be positive");
                    assert!(text_height_before > 0.0, "text height baseline must be positive");

                    let font_scale = font_size_after / font_size_before;
                    let text_height_scale = text_height_after / text_height_before;
                    assert_close(text_height_scale, font_scale, 0.02);

                    let correction_factor_before = text_height_before / font_size_before;
                    let correction_factor_after = text_height_after / font_size_after;
                    assert_close(correction_factor_after, correction_factor_before, 0.02);
                });
            });

            let report = floor::run_suite(&suite);
            assert!(report.is_success());
        })
        .expect("floor canvas zoom layout spec thread should start");

    match test_thread.join() {
        Ok(()) => {}
        Err(payload) => std::panic::resume_unwind(payload),
    }
}
