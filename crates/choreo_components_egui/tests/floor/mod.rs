#[path = "../../src/floor/mod.rs"]
pub mod floor_component;

use std::io;
use std::sync::Arc;

use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

pub use rspec::report::Report;

pub fn run_suite<T>(suite: &rspec::block::Suite<T>) -> rspec::report::SuiteReport
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

pub mod audio_position_interpolation_spec;
pub mod draw_floor_behavior_spec;
pub mod floor_canvas_zoom_layout_spec;
pub mod floor_public_api_parity_spec;
pub mod gesture_handling_behavior_spec;
pub mod move_positions_behavior_spec;
pub mod move_positions_feature_spec;
pub mod place_position_behavior_spec;
pub mod provider_adapter_parity_spec;
pub mod redraw_floor_behavior_spec;
pub mod render_layer_parity_spec;
pub mod rotate_around_center_behavior_spec;
pub mod scale_around_dancer_behavior_spec;
pub mod scale_positions_behavior_spec;
pub mod test_pointer_event_args_spec;
