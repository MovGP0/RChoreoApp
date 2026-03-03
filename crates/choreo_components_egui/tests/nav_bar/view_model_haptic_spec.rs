use choreo_components_egui::nav_bar::actions::NavBarAction;
use choreo_components_egui::nav_bar::runtime::NavBarRuntimeHandlers;
use choreo_components_egui::nav_bar::state::NavBarState;
use choreo_components_egui::nav_bar::view_model::NavBarHapticFeedback;
use choreo_components_egui::nav_bar::view_model::NavBarViewModel;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

struct TestHaptics {
    count: Arc<AtomicUsize>,
}

impl NavBarHapticFeedback for TestHaptics {
    fn is_supported(&self) -> bool {
        true
    }

    fn perform_click(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn nav_view_model_triggers_haptic_feedback_for_click_actions() {
    let count = Arc::new(AtomicUsize::new(0));
    let mut view_model = NavBarViewModel::new(
        NavBarState::default(),
        NavBarRuntimeHandlers::default(),
        Some(Box::new(TestHaptics { count: Arc::clone(&count) })),
    );

    view_model.dispatch(NavBarAction::OpenAudio);
    view_model.dispatch(NavBarAction::OpenImage);
    view_model.dispatch(NavBarAction::ResetFloorViewport);
    view_model.dispatch(NavBarAction::ToggleChoreographySettings);

    assert_eq!(count.load(Ordering::SeqCst), 4);
}
