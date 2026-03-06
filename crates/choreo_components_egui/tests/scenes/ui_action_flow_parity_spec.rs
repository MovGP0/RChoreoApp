use super::actions::ScenesAction;
use super::create_state;
use super::ui::navigate_dancers_icon;
use super::ui::navigate_settings_icon;
use super::ui::open_choreography_icon;
use super::ui::save_choreography_icon;
use super::ui::scene_add_after_icon;
use super::ui::scene_add_before_icon;
use super::ui::scene_search_bar_content_width;
use super::ui::scene_delete_icon;
use super::ui::scene_list_panel_height;
use super::ui::scene_pane_action_flow;
use super::ui::scene_pane_controls_height;
use super::ui::scene_search_bar_text_edit_width;

#[test]
fn scene_pane_action_flow_matches_original_controls_and_caps() {
    let mut state = create_state();
    state.can_delete_scene = true;
    state.can_save_choreo = true;
    state.can_navigate_to_settings = true;
    state.can_navigate_to_dancer_settings = true;

    let actions = scene_pane_action_flow(&state);

    assert_eq!(
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
}

#[test]
fn scene_pane_action_flow_excludes_non_parity_controls() {
    let state = create_state();
    let actions = scene_pane_action_flow(&state);

    assert!(!actions.contains(&ScenesAction::OpenCopyScenePositionsDialog));
    assert!(
        !actions
            .iter()
            .any(|action| { matches!(action, ScenesAction::UpdateShowTimestamps(_)) })
    );
}

#[test]
fn scene_toolbar_icons_match_original_slint_catalog() {
    assert_eq!(scene_add_before_icon().slint_name, "TableRowPlusBefore");
    assert_eq!(scene_add_after_icon().slint_name, "TableRowPlusAfter");
    assert_eq!(scene_delete_icon().slint_name, "Delete");
    assert_eq!(open_choreography_icon().slint_name, "FolderOpen");
    assert_eq!(save_choreography_icon().slint_name, "ContentSave");
    assert_eq!(navigate_settings_icon().slint_name, "Cog");
    assert_eq!(navigate_dancers_icon().slint_name, "AccountGroup");
}

#[test]
fn scene_pane_layout_reserves_space_for_search_and_toolbar_rows() {
    assert_eq!(scene_pane_controls_height(12.0, 40.0), 160.0);
    assert_eq!(scene_list_panel_height(420.0, 12.0, 40.0), 260.0);
    assert_eq!(scene_list_panel_height(120.0, 12.0, 40.0), 0.0);
}

#[test]
fn scene_search_bar_text_width_is_anchored_to_panel_width() {
    assert_eq!(scene_search_bar_content_width(300.0, 10.0, 4.0), 286.0);
    assert_eq!(scene_search_bar_text_edit_width(286.0, 12.0, 24.0), 214.0);
    assert_eq!(scene_search_bar_content_width(180.0, 10.0, 4.0), 166.0);
    assert_eq!(scene_search_bar_text_edit_width(166.0, 12.0, 24.0), 94.0);
    assert_eq!(scene_search_bar_content_width(12.0, 10.0, 4.0), 0.0);
    assert_eq!(scene_search_bar_text_edit_width(60.0, 12.0, 24.0), 0.0);
}
