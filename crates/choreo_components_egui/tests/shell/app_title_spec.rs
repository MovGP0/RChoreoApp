use choreo_components_egui::shell;

#[test]
fn app_title_spec() {
    assert_eq!(shell::app_title(), "ChoreoApp");
    assert_eq!(shell::create_shell_host().title(), "ChoreoApp");
}
