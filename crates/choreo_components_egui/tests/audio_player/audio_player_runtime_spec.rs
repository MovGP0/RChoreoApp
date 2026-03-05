use choreo_components_egui::audio_player::AudioPlayerBackend;
use choreo_components_egui::audio_player::runtime::AudioPlayerRuntime;
use choreo_components_egui::audio_player::runtime::apply_player_sample;
use choreo_components_egui::audio_player::state::AudioPlayerState;

#[test]
fn runtime_creates_player_for_platform_backend() {
    let mut runtime = AudioPlayerRuntime::new(AudioPlayerBackend::Rodio);
    runtime.open_file("C:\\temp\\song.mp3".to_string());
    assert!(runtime.has_player());
}

#[test]
fn runtime_sample_syncs_audio_state_from_player() {
    let mut runtime = AudioPlayerRuntime::new(AudioPlayerBackend::Rodio);
    runtime.open_file("C:\\temp\\song.mp3".to_string());
    runtime.toggle_play_pause();
    runtime.seek_and_play(0.5);
    runtime.set_speed(1.05);
    runtime.set_volume(0.8);
    runtime.set_balance(-0.2);
    runtime.set_loop(true);

    let sample = runtime.sample().expect("runtime should have a sample");
    let mut state = AudioPlayerState::default();
    apply_player_sample(&mut state, sample);

    assert!(state.has_player);
    assert!(state.is_playing);
    assert!(state.can_set_speed);
    assert_eq!(state.speed, 1.05);
    assert_eq!(state.volume, 0.8);
    assert_eq!(state.balance, -0.2);
    assert!(state.loop_enabled);
}
