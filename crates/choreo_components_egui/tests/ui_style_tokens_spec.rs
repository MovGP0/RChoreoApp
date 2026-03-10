use std::io;
use std::sync::Arc;

use choreo_components_egui::choreo_main;
use choreo_components_egui::dancer_settings_page;
use choreo_components_egui::dancers_pane_view;
use choreo_components_egui::dialog_host;
use choreo_components_egui::material::styling::material_style_metrics::material_style_metrics;
use choreo_components_egui::nav_bar;
use choreo_components_egui::settings;
use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;
use rspec::report::Report;

fn run_suite<T>(suite: &rspec::block::Suite<T>) -> rspec::report::SuiteReport
where
    T: Clone + Send + Sync + std::fmt::Debug,
{
    let configuration = ConfigurationBuilder::default()
        .parallel(false)
        .exit_on_failure(false)
        .build()
        .expect("rspec configuration should build");
    let logger = Arc::new(Logger::new(io::stdout()));
    let runner = Runner::new(configuration, vec![logger]);
    runner.run(suite)
}

#[test]
fn ui_style_tokens_spec() {
    let suite = rspec::describe("ui style metrics token parity", (), |spec| {
        spec.it(
            "maps source Slint size, padding, and corner-radius tokens",
            |_| {
                let metrics = material_style_metrics();
                assert_eq!(metrics.sizes.size_1, 1.0);
                assert_eq!(metrics.sizes.size_56, 56.0);
                assert_eq!(metrics.sizes.size_90, 96.0);
                assert_eq!(metrics.paddings.padding_12, 12.0);
                assert_eq!(metrics.paddings.padding_24, 24.0);
                assert_eq!(metrics.corner_radii.border_radius_12, 12.0);
                assert_eq!(metrics.corner_radii.border_radius_28, 28.0);
            },
        );

        spec.it("maps stroke, elevation, and state-opacity tokens", |_| {
            let metrics = material_style_metrics();
            assert_eq!(metrics.strokes.outline, 1.0);
            assert_eq!(metrics.strokes.focus, 2.0);
            assert_eq!(metrics.elevations.level_4.outer.offset_y, 6.0);
            assert_eq!(metrics.elevations.level_4.outer.blur, 10.0);
            assert!((metrics.state_opacities.hover - 0.08).abs() < f32::EPSILON);
            assert!((metrics.state_opacities.pressed - 0.10).abs() < f32::EPSILON);
            assert!((metrics.state_opacities.disabled - 0.12).abs() < f32::EPSILON);
            assert!((metrics.state_opacities.drag - 0.16).abs() < f32::EPSILON);
            assert!((metrics.state_opacities.content_disabled - 0.38).abs() < f32::EPSILON);
        });

        spec.it("dialog host consumes shared dialog metrics tokens", |_| {
            let metrics = material_style_metrics();
            let dialog_metrics = dialog_host::ui::dialog_metrics_tokens();
            assert_eq!(
                dialog_metrics.dialog_padding,
                metrics.paddings.padding_24 as i8
            );
            assert_eq!(dialog_metrics.dialog_margin, metrics.paddings.padding_24);
            assert_eq!(
                dialog_metrics.dialog_corner_radius,
                metrics.corner_radii.border_radius_12 as u8
            );
        });

        spec.it(
            "component UIs consume shared spacing/radius/state-opacity tokens",
            |_| {
                let metrics = material_style_metrics();

                assert_eq!(
                    choreo_main::ui::content_spacing_token(),
                    metrics.spacings.spacing_12
                );
                assert_eq!(
                    dancers_pane_view::ui::pane_corner_radius_token(),
                    metrics.corner_radii.border_radius_12
                );
                assert_eq!(
                    dancer_settings_page::ui::card_corner_radius_token(),
                    metrics.corner_radii.border_radius_12
                );

                let state_opacity_tokens = nav_bar::hamburger_toggle_button::state_opacity_tokens();
                assert!(
                    (state_opacity_tokens.disabled - metrics.state_opacities.content_disabled)
                        .abs()
                        < f32::EPSILON
                );
                assert!(
                    (state_opacity_tokens.hover - metrics.state_opacities.hover).abs()
                        < f32::EPSILON
                );
                assert!(
                    (state_opacity_tokens.pressed - metrics.state_opacities.pressed).abs()
                        < f32::EPSILON
                );
                assert_eq!(
                    settings::ui::content_spacing_token(),
                    metrics.spacings.spacing_12
                );
                assert_eq!(
                    nav_bar::hamburger_toggle_button::minimum_button_size_token(),
                    metrics.sizes.size_40
                );
                assert_eq!(
                    nav_bar::hamburger_toggle_button::content_padding_token(),
                    metrics.paddings.padding_10
                );
            },
        );
    });

    let report = run_suite(&suite);
    assert!(report.is_success());
}
