#![allow(clippy::bool_assert_comparison)]

use crate::global_component::actions::GlobalAction;
use crate::global_component::reducer::reduce;
use crate::global_component::state::{GlobalState, InteractionMode, SelectionRectangle};

#[test]
fn default_state_uses_view_interaction_mode() {
    let state = GlobalState::default();

    assert_eq!(state.interaction_mode, InteractionMode::View);
    assert_eq!(state.is_place_mode, false);
    assert_eq!(state.selection_rectangle, None);
}

#[test]
fn reducer_updates_interaction_and_place_mode() {
    let mut state = GlobalState::default();

    reduce(
        &mut state,
        GlobalAction::SetInteractionMode {
            mode: InteractionMode::Move,
        },
    );
    reduce(
        &mut state,
        GlobalAction::SetPlaceMode {
            is_place_mode: true,
        },
    );

    assert_eq!(state.interaction_mode, InteractionMode::Move);
    assert_eq!(state.is_place_mode, true);
}

#[test]
fn reducer_tracks_selection_rectangle_and_scene_scroll_requests() {
    let mut state = GlobalState::default();

    reduce(
        &mut state,
        GlobalAction::SetSelectionRectangle {
            rectangle: Some(SelectionRectangle {
                start: (12.0, 24.0),
                end: (36.0, 48.0),
            }),
        },
    );
    reduce(&mut state, GlobalAction::RequestScrollToSelectedScene);

    assert_eq!(state.selection_rectangle.is_some(), true);
    assert_eq!(state.should_scroll_to_selected_scene, true);

    reduce(&mut state, GlobalAction::CompleteScrollToSelectedScene);

    assert_eq!(state.should_scroll_to_selected_scene, false);
}

#[test]
fn reducer_tracks_pending_drag_and_floor_redraw_flags() {
    let mut state = GlobalState::default();

    reduce(
        &mut state,
        GlobalAction::SetPendingDragSelection {
            has_pending_drag_selection: true,
            drag_selected_scene_id: Some("scene-7".to_owned()),
        },
    );
    reduce(&mut state, GlobalAction::RequestFloorRedraw);

    assert_eq!(state.has_pending_drag_selection, true);
    assert_eq!(state.drag_selected_scene_id.as_deref(), Some("scene-7"));
    assert_eq!(state.redraw_floor, true);

    reduce(&mut state, GlobalAction::CompleteFloorRedraw);

    assert_eq!(state.redraw_floor, false);
}

#[test]
fn reducer_updates_selected_scene_and_scroll_offset() {
    let mut state = GlobalState::default();

    reduce(
        &mut state,
        GlobalAction::SetSelectedSceneId {
            scene_id: Some("scene-2".to_owned()),
        },
    );
    reduce(
        &mut state,
        GlobalAction::SetSceneListScrollOffset {
            scroll_offset: 144.0,
        },
    );

    assert_eq!(state.selected_scene_id.as_deref(), Some("scene-2"));
    assert_eq!(state.scene_list_scroll_offset, 144.0);
}

#[test]
fn reducer_updates_floor_rendering_and_svg_path() {
    let mut state = GlobalState::default();

    reduce(
        &mut state,
        GlobalAction::SetFloorRendering {
            is_rendering_floor: true,
        },
    );
    reduce(
        &mut state,
        GlobalAction::SetSvgFilePath {
            svg_file_path: Some("C:/tmp/floor.svg".to_owned()),
        },
    );

    assert_eq!(state.is_rendering_floor, true);
    assert_eq!(state.svg_file_path.as_deref(), Some("C:/tmp/floor.svg"));
}

#[test]
fn initialize_restores_defaults() {
    let mut state = GlobalState {
        interaction_mode: InteractionMode::Scale,
        is_place_mode: true,
        ..GlobalState::default()
    };

    reduce(&mut state, GlobalAction::Initialize);

    assert_eq!(state.interaction_mode, InteractionMode::View);
    assert_eq!(state.is_place_mode, false);
}
