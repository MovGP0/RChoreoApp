use super::state::SceneItemState;
use super::state::ScenesState;
use super::ui::delete_scene_dialog_scene;
use super::ui::draw;
use crate::delete_scene_dialog::ui::DeleteSceneDialogAction;
use crate::delete_scene_dialog::ui::build_delete_scene_dialog_view_model;
use crate::delete_scene_dialog::ui::draw_delete_scene_dialog;

#[test]
fn delete_scene_dialog_view_model_formats_message_with_scene_name() {
    macro_rules! check_eq {
        ($errors:expr, $left:expr, $right:expr) => {
            if $left != $right {
                $errors.push(format!(
                    "{} != {} (left = {:?}, right = {:?})",
                    stringify!($left),
                    stringify!($right),
                    $left,
                    $right
                ));
            }
        };
    }

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
    let mut errors = Vec::new();

    check_eq!(errors, view_model.title_text, "Delete scene");
    check_eq!(errors, view_model.scene_name, "Bridge");
    check_eq!(errors, view_model.message_text, "Delete scene \"Bridge\"?");
    check_eq!(errors, view_model.confirm_text, "Yes");
    check_eq!(errors, view_model.cancel_text, "No");

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn delete_scene_dialog_view_model_uses_default_name_when_scene_name_is_blank() {
    macro_rules! check_eq {
        ($errors:expr, $left:expr, $right:expr) => {
            if $left != $right {
                $errors.push(format!(
                    "{} != {} (left = {:?}, right = {:?})",
                    stringify!($left),
                    stringify!($right),
                    $left,
                    $right
                ));
            }
        };
    }

    let scene = SceneItemState::new(
        choreo_master_mobile_json::SceneId(9),
        "   ",
        choreo_master_mobile_json::Color::transparent(),
    );

    let view_model = build_delete_scene_dialog_view_model(&scene, "en");
    let mut errors = Vec::new();

    check_eq!(errors, view_model.scene_name, "this scene");
    check_eq!(errors, view_model.message_text, "Delete scene \"this scene\"?");

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
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
        delete_scene_dialog_scene: Some(SceneItemState::new(
            choreo_master_mobile_json::SceneId(8),
            "Outro",
            choreo_master_mobile_json::Color::transparent(),
        )),
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

#[test]
fn delete_scene_dialog_uses_captured_scene_snapshot() {
    let state = ScenesState {
        selected_scene: Some(SceneItemState::new(
            choreo_master_mobile_json::SceneId(10),
            "Current",
            choreo_master_mobile_json::Color::transparent(),
        )),
        show_delete_scene_dialog: true,
        delete_scene_dialog_scene: Some(SceneItemState::new(
            choreo_master_mobile_json::SceneId(11),
            "Captured",
            choreo_master_mobile_json::Color::transparent(),
        )),
        ..ScenesState::default()
    };

    assert_eq!(
        delete_scene_dialog_scene(&state).map(|scene| scene.name.as_str()),
        Some("Captured")
    );
}
