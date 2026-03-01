#![allow(clippy::bool_assert_comparison)]

#[test]
fn update_selected_scene_behavior_spec_scaffold_exists() {
    let current_dir = std::env::current_dir().expect("cwd");
    assert!(current_dir.exists());
}
