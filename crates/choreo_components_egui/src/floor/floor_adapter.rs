use super::actions::FloorAction;
use super::reducer::reduce;
use super::state::FloorPosition;
use super::state::FloorState;

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
}

#[derive(Debug, Default)]
pub struct FloorAdapter;

impl FloorAdapter {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    pub fn apply(&self, state: &mut FloorState, input: FloorAdapterInput) {
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
    }
}
