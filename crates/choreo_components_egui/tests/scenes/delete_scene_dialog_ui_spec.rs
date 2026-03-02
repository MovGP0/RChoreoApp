use super::state::SceneItemState;
use super::state::ScenesState;
use super::ui::DeleteSceneDialogAction;
use super::ui::build_delete_scene_dialog_view_model;
use super::ui::draw;
use super::ui::draw_delete_scene_dialog;

#[test]
fn delete_scene_dialog_view_model_formats_message_with_scene_name() {
    let scene = SceneItemState::new(
        choreo_master_mobile_json::SceneId(7),
        "Bridge",
        choreo_master_mobile_json::Color {
            r: 0x11,
            g: 0x22,
            b: 0x33,
            a: 0xFF,
        },
    );

    let view_model = build_delete_scene_dialog_view_model(&scene, "en");

    assert_eq!(view_model.title_text, "Delete scene");
    assert_eq!(view_model.scene_name, "Bridge");
    assert_eq!(view_model.message_text, "Delete scene \"Bridge\"?");
    assert_eq!(view_model.confirm_text, "Yes");
    assert_eq!(view_model.cancel_text, "No");
}

#[test]
fn scenes_ui_draw_and_delete_dialog_render_without_panicking() {
    let context = egui::Context::default();
    let state = ScenesState {
        selected_scene: Some(SceneItemState::new(
            choreo_master_mobile_json::SceneId(8),
            "Outro",
            choreo_master_mobile_json::Color::transparent(),
        )),
        show_delete_scene_dialog: true,
        ..ScenesState::default()
    };

    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = draw(ui, &state);
            if let Some(selected_scene) = state.selected_scene.as_ref() {
                let _ = draw_delete_scene_dialog(ui, selected_scene, "en");
            }
        });
    });
}

#[test]
fn draw_delete_scene_dialog_returns_no_action_without_click() {
    let scene = SceneItemState::new(
        choreo_master_mobile_json::SceneId(1),
        "Scene",
        choreo_master_mobile_json::Color {
            r: 0x00,
            g: 0x00,
            b: 0x00,
            a: 0xFF,
        },
    );

    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let action: Option<DeleteSceneDialogAction> =
                draw_delete_scene_dialog(ui, &scene, "en");
            assert_eq!(action, None);
        });
    });
}
