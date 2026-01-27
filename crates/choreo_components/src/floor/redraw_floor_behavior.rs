use crate::behavior::{Behavior, CompositeDisposable};
use nject::injectable;

use super::floor_view_model::FloorCanvasViewModel;

#[derive(Debug, Default)]
#[injectable]
#[inject(|| Self::default())]
pub struct RedrawFloorBehavior;

impl RedrawFloorBehavior {
    pub fn handle_choreography_changed(&self, view_model: &FloorCanvasViewModel) {
        view_model.draw_floor();
    }

    pub fn handle_selected_scene_changed(&self, view_model: &FloorCanvasViewModel) {
        view_model.draw_floor();
    }

    pub fn handle_redraw_command(&self, view_model: &FloorCanvasViewModel) {
        view_model.draw_floor();
    }
}

impl Behavior<FloorCanvasViewModel> for RedrawFloorBehavior {
    fn activate(&self, _view_model: &mut FloorCanvasViewModel, _disposables: &mut CompositeDisposable) {
    }
}

