use choreo_components_egui::shell;
use choreo_components_egui::shell_host::ShellHostViewModel;

#[test]
fn shell_host_component_module_spec() {
    let shell_host = shell::create_shell_host();
    let _typed_shell_host: ShellHostViewModel = shell_host;
}
