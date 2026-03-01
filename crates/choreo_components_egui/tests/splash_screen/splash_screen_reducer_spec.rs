use egui::Color32;

use super::actions::SplashScreenAction;
use super::reducer::reduce;
use super::state::SplashScreenState;

#[test]
fn splash_screen_defaults_match_source_intent() {
    let state = SplashScreenState::new();

    assert_eq!(state.background_color, Color32::from_rgb(0xF8, 0xFA, 0xFD));
    assert_eq!(state.splash_image_path, "splash.svg");
}

#[test]
fn splash_screen_reducer_updates_background_color() {
    let mut state = SplashScreenState::new();

    reduce(
        &mut state,
        SplashScreenAction::SetBackgroundColor {
            color: Color32::from_rgb(0x10, 0x20, 0x30),
        },
    );

    assert_eq!(state.background_color, Color32::from_rgb(0x10, 0x20, 0x30));
}

#[test]
fn splash_screen_reducer_updates_splash_image_path() {
    let mut state = SplashScreenState::new();

    reduce(
        &mut state,
        SplashScreenAction::SetSplashImagePath {
            path: "custom.svg".to_owned(),
        },
    );

    assert_eq!(state.splash_image_path, "custom.svg");
}

#[test]
fn initialize_action_is_supported() {
    let mut state = SplashScreenState::new();
    reduce(&mut state, SplashScreenAction::Initialize);
    assert_eq!(state.splash_image_path, "splash.svg");
}
