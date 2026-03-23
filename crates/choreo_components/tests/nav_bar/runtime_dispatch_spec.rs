use choreo_components::nav_bar::reducer::NavBarEffect;
use choreo_components::nav_bar::runtime::NavBarRuntimeHandlers;
use choreo_components::nav_bar::runtime::dispatch_effects;
use choreo_components::nav_bar::state::InteractionMode;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

#[test]
fn runtime_dispatch_consumes_each_effect_once() {
    let open_audio_count = Arc::new(AtomicUsize::new(0));
    let open_image_count = Arc::new(AtomicUsize::new(0));
    let reset_count = Arc::new(AtomicUsize::new(0));
    let mode_count = Arc::new(AtomicUsize::new(0));

    let mut handlers = NavBarRuntimeHandlers {
        open_audio_requested: Some(Box::new({
            let count = Arc::clone(&open_audio_count);
            move |_| {
                count.fetch_add(1, Ordering::SeqCst);
            }
        })),
        open_image_requested: Some(Box::new({
            let count = Arc::clone(&open_image_count);
            move |_| {
                count.fetch_add(1, Ordering::SeqCst);
            }
        })),
        reset_floor_viewport_requested: Some(Box::new({
            let count = Arc::clone(&reset_count);
            move |_| {
                count.fetch_add(1, Ordering::SeqCst);
            }
        })),
        interaction_mode_changed: Some(Box::new({
            let count = Arc::clone(&mode_count);
            move |_| {
                count.fetch_add(1, Ordering::SeqCst);
            }
        })),
    };

    dispatch_effects(
        vec![
            NavBarEffect::OpenAudioRequested,
            NavBarEffect::OpenImageRequested,
            NavBarEffect::ResetFloorViewportRequested,
            NavBarEffect::InteractionModeChanged {
                mode: InteractionMode::Move,
            },
        ],
        &mut handlers,
    );

    assert_eq!(open_audio_count.load(Ordering::SeqCst), 1);
    assert_eq!(open_image_count.load(Ordering::SeqCst), 1);
    assert_eq!(reset_count.load(Ordering::SeqCst), 1);
    assert_eq!(mode_count.load(Ordering::SeqCst), 1);
}
