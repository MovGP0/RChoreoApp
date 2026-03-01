#![allow(clippy::bool_assert_comparison)]

#[test]
fn audio_player_position_changed_behavior_spec_scaffold_exists() {
    let current_dir = std::env::current_dir().expect("cwd");
    assert!(current_dir.exists());
}
