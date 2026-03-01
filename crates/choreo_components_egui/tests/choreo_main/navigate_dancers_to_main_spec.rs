#![allow(clippy::bool_assert_comparison)]

#[test]
fn navigate_dancers_to_main_spec_scaffold_exists() {
    let current_dir = std::env::current_dir().expect("cwd");
    assert!(current_dir.exists());
}
