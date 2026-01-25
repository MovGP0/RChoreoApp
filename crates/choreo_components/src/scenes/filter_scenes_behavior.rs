use crate::preferences::Preferences;
use nject::injectable;

use super::scenes_view_model::ScenesPaneViewModel;

#[injectable]
#[inject(|| Self)]
pub struct FilterScenesBehavior;

impl FilterScenesBehavior {
    pub fn apply<P: Preferences>(view_model: &mut ScenesPaneViewModel<P>) {
        view_model.refresh_scenes();
    }
}
