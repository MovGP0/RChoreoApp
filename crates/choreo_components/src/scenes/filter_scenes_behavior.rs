use crate::preferences::Preferences;

use super::scenes_view_model::ScenesPaneViewModel;

pub struct FilterScenesBehavior;

impl FilterScenesBehavior {
    pub fn apply<P: Preferences>(view_model: &mut ScenesPaneViewModel<P>) {
        view_model.refresh_scenes();
    }
}
