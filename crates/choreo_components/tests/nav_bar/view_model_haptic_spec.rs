use choreo_components::nav_bar::actions::NavBarAction;
use choreo_components::nav_bar::runtime::NavBarRuntimeHandlers;
use choreo_components::nav_bar::state::NavBarState;
use choreo_components::nav_bar::view_model::NavBarHapticFeedback;
use choreo_components::nav_bar::view_model::NavBarViewModel;
use choreo_components::nav_bar::view_model::emits_click_feedback;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
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
        Some(Box::new(TestHaptics {
            count: Arc::clone(&count),
        })),
    );

    view_model.dispatch(NavBarAction::OpenAudio);
    view_model.dispatch(NavBarAction::OpenImage);
    view_model.dispatch(NavBarAction::ResetFloorViewport);
    view_model.dispatch(NavBarAction::ToggleChoreographySettings);
    view_model.dispatch(NavBarAction::CloseChoreographySettings);

    assert_eq!(count.load(Ordering::SeqCst), 5);
}

#[test]
fn nav_view_model_documents_which_actions_emit_click_feedback_for_parity() {
    let mut errors = Vec::new();

    check!(errors, emits_click_feedback(&NavBarAction::OpenAudio));
    check!(errors, emits_click_feedback(&NavBarAction::OpenImage));
    check!(
        errors,
        emits_click_feedback(&NavBarAction::ToggleChoreographySettings)
    );
    check!(
        errors,
        emits_click_feedback(&NavBarAction::CloseChoreographySettings)
    );
    check!(errors, emits_click_feedback(&NavBarAction::ResetFloorViewport));

    check!(errors, !emits_click_feedback(&NavBarAction::ToggleNavigation));
    check!(errors, !emits_click_feedback(&NavBarAction::CloseNavigation));
    check!(errors, !emits_click_feedback(&NavBarAction::Initialize));
    check!(
        errors,
        !emits_click_feedback(&NavBarAction::SetModeSelectionEnabled { enabled: true })
    );
    check!(
        errors,
        !emits_click_feedback(&NavBarAction::SetAudioPlayerOpened {
            is_open: true,
        })
    );
    check!(
        errors,
        !emits_click_feedback(&NavBarAction::SetSelectedMode {
            mode: choreo_components::nav_bar::state::InteractionMode::View,
        })
    );

    assert_no_errors(errors);
}
