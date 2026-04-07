use super::actions::ScenesAction;
use super::create_state;
use super::ui::navigate_dancers_icon;
use super::ui::navigate_settings_icon;
use super::ui::open_choreography_icon;
use super::ui::save_choreography_icon;
use super::ui::scene_add_after_icon;
use super::ui::scene_add_before_icon;
use super::ui::scene_delete_icon;
use super::ui::scene_list_panel_height;
use super::ui::scene_pane_action_flow;
use super::ui::scene_pane_controls_height;
use super::ui::scene_search_bar_content_width;
use super::ui::scene_search_bar_text_edit_width;

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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn scene_pane_action_flow_matches_original_controls_and_caps() {
    let mut state = create_state();
    state.can_delete_scene = true;
    state.can_save_choreo = true;
    state.can_navigate_to_settings = true;
    state.can_navigate_to_dancer_settings = true;

    let actions = scene_pane_action_flow(&state);

    let mut errors = Vec::new();

    check_eq!(
        errors,
        actions,
        vec![
            ScenesAction::InsertScene {
                insert_after: false
            },
            ScenesAction::InsertScene { insert_after: true },
            ScenesAction::OpenDeleteSceneDialog,
            ScenesAction::RequestOpenChoreography,
            ScenesAction::RequestSaveChoreography,
            ScenesAction::NavigateToSettings,
            ScenesAction::NavigateToDancerSettings,
        ]
    );

    assert_no_errors(errors);
}

#[test]
fn scene_pane_action_flow_excludes_non_parity_controls() {
    let state = create_state();
    let actions = scene_pane_action_flow(&state);

    let mut errors = Vec::new();

    check!(
        errors,
        !actions.contains(&ScenesAction::OpenCopyScenePositionsDialog)
    );
    check!(
        errors,
        !actions
            .iter()
            .any(|action| matches!(action, ScenesAction::UpdateShowTimestamps(_)))
    );

    assert_no_errors(errors);
}

#[test]
fn scene_toolbar_icons_match_original_slint_catalog() {
    let mut errors = Vec::new();

    check_eq!(errors, scene_add_before_icon().slint_name, "TableRowPlusBefore");
    check_eq!(errors, scene_add_after_icon().slint_name, "TableRowPlusAfter");
    check_eq!(errors, scene_delete_icon().slint_name, "Delete");
    check_eq!(errors, open_choreography_icon().slint_name, "FolderOpen");
    check_eq!(errors, save_choreography_icon().slint_name, "ContentSave");
    check_eq!(errors, navigate_settings_icon().slint_name, "Cog");
    check_eq!(errors, navigate_dancers_icon().slint_name, "AccountGroup");

    assert_no_errors(errors);
}

#[test]
fn scene_pane_layout_reserves_space_for_search_and_toolbar_rows() {
    let mut errors = Vec::new();

    check_eq!(errors, scene_pane_controls_height(12.0, 40.0), 160.0);
    check_eq!(errors, scene_list_panel_height(420.0, 12.0, 40.0), 260.0);
    check_eq!(errors, scene_list_panel_height(120.0, 12.0, 40.0), 0.0);

    assert_no_errors(errors);
}

#[test]
fn scene_search_bar_text_width_is_anchored_to_panel_width() {
    let mut errors = Vec::new();

    check_eq!(errors, scene_search_bar_content_width(300.0, 10.0, 4.0), 286.0);
    check_eq!(errors, scene_search_bar_text_edit_width(286.0, 12.0, 24.0), 214.0);
    check_eq!(errors, scene_search_bar_content_width(180.0, 10.0, 4.0), 166.0);
    check_eq!(errors, scene_search_bar_text_edit_width(166.0, 12.0, 24.0), 94.0);
    check_eq!(errors, scene_search_bar_content_width(12.0, 10.0, 4.0), 0.0);
    check_eq!(errors, scene_search_bar_text_edit_width(60.0, 12.0, 24.0), 0.0);

    assert_no_errors(errors);
}
