use choreo_components_egui::material::styling::material_style_metrics::MATERIAL_STYLE_METRICS;
use choreo_components_egui::material::styling::material_style_metrics::material_style_metrics;

#[test]
fn material_style_metrics_slint_parity_spec() {
    let metrics = material_style_metrics();

    assert_eq!(metrics, MATERIAL_STYLE_METRICS);

    assert_eq!(metrics.sizes.size_1, 1.0);
    assert_eq!(metrics.sizes.size_90, 96.0);
    assert_eq!(metrics.sizes.size_640, 640.0);

    assert_eq!(metrics.icon_sizes.icon_size_18, 18.0);
    assert_eq!(metrics.icon_sizes.icon_size_90, 27.0);

    assert_eq!(metrics.paddings.padding_4, 4.0);
    assert_eq!(metrics.paddings.padding_6, 6.0);
    assert_eq!(metrics.paddings.padding_10, 10.0);
    assert_eq!(metrics.paddings.padding_12, 12.0);
    assert_eq!(metrics.paddings.padding_8, 8.0);
    assert_eq!(metrics.paddings.padding_56, 56.0);

    assert_eq!(metrics.spacings.spacing_2, 2.0);
    assert_eq!(metrics.spacings.spacing_6, 6.0);
    assert_eq!(metrics.spacings.spacing_4, 4.0);
    assert_eq!(metrics.spacings.spacing_52, 52.0);

    assert_eq!(metrics.corner_radii.border_radius_2, 2.0);
    assert_eq!(metrics.corner_radii.border_radius_12, 12.0);
    assert_eq!(metrics.corner_radii.border_radius_28, 28.0);
}
