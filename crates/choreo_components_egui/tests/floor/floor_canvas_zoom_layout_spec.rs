use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;

#[test]
fn floor_canvas_zoom_layout_scales_metrics_proportionally() {
    let mut state = FloorState::default();
    reduce(&mut state, FloorAction::SetZoom { zoom: 1.0 });
    let baseline = state.metrics;

    reduce(&mut state, FloorAction::SetZoom { zoom: 1.6 });
    let zoomed = state.metrics;

    let scale = 1.6;
    assert!((zoomed.header_bottom / baseline.header_bottom - scale).abs() < 0.0001);
    assert!((zoomed.floor_width / baseline.floor_width - scale).abs() < 0.0001);
    assert!((zoomed.floor_height / baseline.floor_height - scale).abs() < 0.0001);
    assert!((zoomed.legend_panel_width / baseline.legend_panel_width - scale).abs() < 0.0001);
    assert!(
        (zoomed.legend_content_padding_top / baseline.legend_content_padding_top - scale).abs()
            < 0.0001
    );
    assert!(
        (zoomed.legend_content_padding_bottom / baseline.legend_content_padding_bottom - scale)
            .abs()
            < 0.0001
    );
}
