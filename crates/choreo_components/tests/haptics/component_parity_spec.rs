use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use choreo_components::audio_player::HapticFeedback as AudioPlayerHapticFeedback;
use choreo_components::audio_player::NoopHapticFeedback as AudioPlayerNoopHapticFeedback;
use choreo_components::audio_player::PlatformHapticFeedback as AudioPlayerPlatformHapticFeedback;
use choreo_components::audio_player::actions::AudioPlayerAction;
use choreo_components::audio_player::reduce_with_haptics;
use choreo_components::audio_player::state::AudioPlayerState;
use choreo_components::haptics::HapticFeedback;
use choreo_components::nav_bar::actions::NavBarAction;
use choreo_components::nav_bar::runtime::NavBarRuntimeHandlers;
use choreo_components::nav_bar::state::NavBarState;
use choreo_components::nav_bar::view_model::NavBarViewModel;

struct RecordingHaptics {
    count: Arc<AtomicUsize>,
}

impl HapticFeedback for RecordingHaptics {
    fn is_supported(&self) -> bool {
        true
    }

    fn perform_click(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn audio_player_module_reexports_shared_haptics_surface() {
    let noop = AudioPlayerNoopHapticFeedback::new();
    let _platform = AudioPlayerPlatformHapticFeedback::default();
    let shared_contract: &dyn AudioPlayerHapticFeedback = &noop;

    assert!(!shared_contract.is_supported());
}

#[test]
fn shared_haptics_trait_drives_audio_player_and_nav_bar() {
    let count = Arc::new(AtomicUsize::new(0));
    let mut audio_state = AudioPlayerState {
        has_player: true,
        ..AudioPlayerState::default()
    };
    let audio_haptics = RecordingHaptics {
        count: Arc::clone(&count),
    };

    reduce_with_haptics(
        &mut audio_state,
        AudioPlayerAction::TogglePlayPause,
        Some(&audio_haptics),
    );

    let mut nav_view_model = NavBarViewModel::new(
        NavBarState::default(),
        NavBarRuntimeHandlers::default(),
        Some(Box::new(RecordingHaptics {
            count: Arc::clone(&count),
        })),
    );

    nav_view_model.dispatch(NavBarAction::OpenAudio);

    assert!(audio_state.is_playing);
    assert_eq!(count.load(Ordering::SeqCst), 2);
}
