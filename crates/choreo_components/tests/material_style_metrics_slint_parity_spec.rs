use choreo_components::material::styling::material_style_metrics::MATERIAL_STYLE_METRICS;
use choreo_components::material::styling::material_style_metrics::material_style_metrics;

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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn material_style_metrics_slint_parity_spec() {
    let metrics = material_style_metrics();
    let mut errors = Vec::new();

    check_eq!(errors, metrics, MATERIAL_STYLE_METRICS);

    check_eq!(errors, metrics.sizes.size_1, 1.0);
    check_eq!(errors, metrics.sizes.size_90, 96.0);
    check_eq!(errors, metrics.sizes.size_640, 640.0);

    check_eq!(errors, metrics.icon_sizes.icon_size_18, 18.0);
    check_eq!(errors, metrics.icon_sizes.icon_size_90, 27.0);

    check_eq!(errors, metrics.paddings.padding_4, 4.0);
    check_eq!(errors, metrics.paddings.padding_6, 6.0);
    check_eq!(errors, metrics.paddings.padding_10, 10.0);
    check_eq!(errors, metrics.paddings.padding_12, 12.0);
    check_eq!(errors, metrics.paddings.padding_8, 8.0);
    check_eq!(errors, metrics.paddings.padding_56, 56.0);

    check_eq!(errors, metrics.spacings.spacing_2, 2.0);
    check_eq!(errors, metrics.spacings.spacing_6, 6.0);
    check_eq!(errors, metrics.spacings.spacing_4, 4.0);
    check_eq!(errors, metrics.spacings.spacing_52, 52.0);

    check_eq!(errors, metrics.corner_radii.border_radius_2, 2.0);
    check_eq!(errors, metrics.corner_radii.border_radius_12, 12.0);
    check_eq!(errors, metrics.corner_radii.border_radius_28, 28.0);

    assert_no_errors(errors);
}
