use std::io;
use std::sync::Arc;

use choreo_components::choreo_main;
use choreo_components::dancer_settings_page;
use choreo_components::dancers_pane_view;
use choreo_components::material::styling::material_style_metrics::material_style_metrics;
use choreo_components::nav_bar;
use choreo_components::settings;
use material3::components::dialog_host;
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

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

#[test]
fn ui_style_tokens_spec() {
    let suite = rspec::describe("ui style metrics token parity", (), |spec| {
        spec.it(
            "maps source Slint size, padding, and corner-radius tokens",
            |_| {
                let metrics = material_style_metrics();
                let mut errors = Vec::new();

                check_eq!(errors, metrics.sizes.size_1, 1.0);
                check_eq!(errors, metrics.sizes.size_56, 56.0);
                check_eq!(errors, metrics.sizes.size_90, 96.0);
                check_eq!(errors, metrics.paddings.padding_12, 12.0);
                check_eq!(errors, metrics.paddings.padding_24, 24.0);
                check_eq!(errors, metrics.corner_radii.border_radius_12, 12.0);
                check_eq!(errors, metrics.corner_radii.border_radius_28, 28.0);

                assert!(
                    errors.is_empty(),
                    "Assertion failures:\n{}",
                    errors.join("\n")
                );
            },
        );

        spec.it("maps stroke, elevation, and state-opacity tokens", |_| {
            let metrics = material_style_metrics();
            let mut errors = Vec::new();

            check_eq!(errors, metrics.strokes.outline, 1.0);
            check_eq!(errors, metrics.strokes.focus, 2.0);
            check_eq!(errors, metrics.elevations.level_4.outer.offset_y, 6.0);
            check_eq!(errors, metrics.elevations.level_4.outer.blur, 10.0);
            check!(
                errors,
                (metrics.state_opacities.hover - 0.08).abs() < f32::EPSILON
            );
            check!(
                errors,
                (metrics.state_opacities.pressed - 0.10).abs() < f32::EPSILON
            );
            check!(
                errors,
                (metrics.state_opacities.disabled - 0.12).abs() < f32::EPSILON
            );
            check!(
                errors,
                (metrics.state_opacities.drag - 0.16).abs() < f32::EPSILON
            );
            check!(
                errors,
                (metrics.state_opacities.content_disabled - 0.38).abs() < f32::EPSILON
            );

            assert!(
                errors.is_empty(),
                "Assertion failures:\n{}",
                errors.join("\n")
            );
        });

        spec.it("dialog host consumes shared dialog metrics tokens", |_| {
            let metrics = material_style_metrics();
            let dialog_metrics = dialog_host::dialog_metrics_tokens();
            let mut errors = Vec::new();

            check_eq!(
                errors,
                dialog_metrics.dialog_padding,
                metrics.paddings.padding_24 as i8
            );
            check_eq!(errors, dialog_metrics.dialog_margin, metrics.paddings.padding_24);
            check_eq!(
                errors,
                dialog_metrics.dialog_corner_radius,
                metrics.corner_radii.border_radius_12 as u8
            );

            assert!(
                errors.is_empty(),
                "Assertion failures:\n{}",
                errors.join("\n")
            );
        });

        spec.it(
            "component UIs consume shared spacing/radius/state-opacity tokens",
            |_| {
                let metrics = material_style_metrics();
                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    choreo_main::ui::content_spacing_token(),
                    metrics.spacings.spacing_12
                );
                check_eq!(
                    errors,
                    dancers_pane_view::ui::pane_corner_radius_token(),
                    metrics.corner_radii.border_radius_12
                );
                check_eq!(
                    errors,
                    dancer_settings_page::ui::card_corner_radius_token(),
                    metrics.corner_radii.border_radius_12
                );

                let state_opacity_tokens = nav_bar::hamburger_toggle_button::state_opacity_tokens();
                check!(
                    errors,
                    (state_opacity_tokens.disabled - metrics.state_opacities.content_disabled)
                        .abs()
                        < f32::EPSILON
                );
                check!(
                    errors,
                    (state_opacity_tokens.hover - metrics.state_opacities.hover).abs()
                        < f32::EPSILON
                );
                check!(
                    errors,
                    (state_opacity_tokens.pressed - metrics.state_opacities.pressed).abs()
                        < f32::EPSILON
                );
                check_eq!(
                    errors,
                    settings::ui::content_spacing_token(),
                    metrics.spacings.spacing_12
                );
                check_eq!(
                    errors,
                    nav_bar::hamburger_toggle_button::minimum_button_size_token(),
                    metrics.sizes.size_40
                );
                check_eq!(
                    errors,
                    nav_bar::hamburger_toggle_button::content_padding_token(),
                    metrics.paddings.padding_10
                );

                assert!(
                    errors.is_empty(),
                    "Assertion failures:\n{}",
                    errors.join("\n")
                );
            },
        );
    });

    let report = run_suite(&suite);
    assert!(report.is_success());
}
