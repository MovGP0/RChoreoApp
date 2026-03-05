use choreo_components_egui::shell;

#[test]
fn app_icon_spec() {
    let app_icon_svg = shell::app_icon_svg();
    let wasm_favicon_svg = include_str!("../../../../apps/wasm_egui/app_icon.svg");

    assert!(app_icon_svg.contains("id=\"background\""));
    assert!(app_icon_svg.contains("id=\"dancers\""));
    assert!(app_icon_svg.contains("fill=\"#B0BEC5\""));
    assert_eq!(app_icon_svg, wasm_favicon_svg);
}
