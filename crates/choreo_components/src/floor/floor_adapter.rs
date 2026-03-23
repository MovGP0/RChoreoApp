use super::actions::FloorAction;
use super::floor_view_model::FloorCanvasViewModel;
use super::reducer::reduce;
use super::state::FloorPosition;
use super::state::FloorState;
use super::types::Rect;
use super::types::Size;

#[derive(Debug, Clone, PartialEq)]
pub struct AudioInterpolationInput {
    pub from: Vec<FloorPosition>,
    pub to: Vec<FloorPosition>,
    pub progress: f64,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FloorAdapterInput {
    pub scene_positions: Vec<FloorPosition>,
    pub axis_x_label: String,
    pub axis_y_label: String,
    pub legend_entries: Vec<(String, [u8; 4])>,
    pub svg_path: Option<String>,
    pub placement_remaining: Option<u32>,
    pub interpolation: Option<AudioInterpolationInput>,
    pub layout_size: Option<(f64, f64)>,
}

#[derive(Debug, Default)]
pub struct FloorAdapter;

impl FloorAdapter {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn apply(
        &self,
        state: &mut FloorState,
        view_model: &mut FloorCanvasViewModel,
        input: FloorAdapterInput,
    ) {
        if let Some((width_px, height_px)) = input.layout_size {
            reduce(
                state,
                FloorAction::SetLayout {
                    width_px,
                    height_px,
                },
            );
        }
        reduce(
            state,
            FloorAction::SetPositions {
                positions: input.scene_positions,
            },
        );
        reduce(
            state,
            FloorAction::SetAxisLabels {
                x_axis: input.axis_x_label,
                y_axis: input.axis_y_label,
            },
        );
        reduce(
            state,
            FloorAction::SetLegendEntries {
                entries: input.legend_entries,
            },
        );
        reduce(
            state,
            FloorAction::SetPlacementRemaining {
                count: input.placement_remaining,
            },
        );
        reduce(
            state,
            FloorAction::SetSvgOverlay {
                svg_path: input.svg_path,
            },
        );

        if let Some(interpolation) = input.interpolation {
            reduce(
                state,
                FloorAction::InterpolateAudioPosition {
                    from: interpolation.from,
                    to: interpolation.to,
                    progress: interpolation.progress,
                },
            );
        }

        self.sync_floor_metrics(state, view_model);
        self.sync_path_commands(state);
    }

    pub fn sync_floor_metrics(&self, state: &FloorState, view_model: &mut FloorCanvasViewModel) {
        let floor_bounds = Rect::new(
            state.floor_x as f32,
            state.floor_y as f32,
            (state.floor_x + state.floor_width) as f32,
            (state.floor_y + state.floor_height) as f32,
        );
        view_model.set_floor_bounds(floor_bounds);
        view_model.set_canvas_size(Size::new(
            state.layout_width_px as f32,
            state.layout_height_px as f32,
        ));
        view_model.set_transformation_matrix(state.transformation_matrix);
    }

    pub fn sync_path_commands(&self, state: &mut FloorState) {
        state.path_commands = state
            .path_segments
            .iter()
            .map(|segment| {
                format!(
                    "M {:.3} {:.3} L {:.3} {:.3}",
                    segment.from.x, segment.from.y, segment.to.x, segment.to.y
                )
            })
            .collect();
        state.dashed_path_commands = state
            .dashed_path_segments
            .iter()
            .map(|segment| {
                format!(
                    "M {:.3} {:.3} L {:.3} {:.3}",
                    segment.from.x, segment.from.y, segment.to.x, segment.to.y
                )
            })
            .collect();
    }
}
