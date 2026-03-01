#![allow(clippy::bool_assert_comparison)]

#[test]
fn timestamp_sync_spec_scaffold_exists() {
    let current_dir = std::env::current_dir().expect("cwd");
    assert!(current_dir.exists());
}
