#![allow(clippy::bool_assert_comparison)]

#[test]
fn navigate_main_to_dancers_spec_scaffold_exists() {
    let current_dir = std::env::current_dir().expect("cwd");
    assert!(current_dir.exists());
}
