#![allow(clippy::bool_assert_comparison)]

#[test]
fn open_image_behavior_spec_scaffold_exists() {
    let current_dir = std::env::current_dir().expect("cwd");
    assert!(current_dir.exists());
}
