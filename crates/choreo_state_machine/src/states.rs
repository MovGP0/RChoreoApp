use crate::traits::ApplicationState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StateKind {
    InitialApplicationState,
    ViewSceneState,
    ViewScenePanState,
    ViewSceneZoomState,
    PlacePositionsState,
    PlacePositionsPanState,
    PlacePositionsZoomState,
    MovePositionsState,
    MovePositionsSelectionState,
    MovePositionsDragState,
    RotateAroundCenterState,
    RotateAroundCenterSelectionStartState,
    RotateAroundCenterSelectionEndState,
    RotateAroundCenterRotationStartState,
    RotateAroundCenterRotationEndState,
    ScalePositionsState,
    ScalePositionsSelectionStartState,
    ScalePositionsSelectionEndState,
    ScalePositionsDragStartState,
    ScalePositionsDragEndState,
    ScaleAroundDancerState,
    ScaleAroundDancerSelectionStartState,
    ScaleAroundDancerSelectionEndState,
    ScaleAroundDancerDragStartState,
    ScaleAroundDancerDragEndState,
}

impl StateKind {
    pub fn is_assignable_from(self, other: StateKind) -> bool {
        if self == other {
            return true;
        }

        match self {
            StateKind::ViewSceneState => matches!(
                other,
                StateKind::ViewScenePanState | StateKind::ViewSceneZoomState
            ),
            StateKind::PlacePositionsState => matches!(
                other,
                StateKind::PlacePositionsPanState | StateKind::PlacePositionsZoomState
            ),
            StateKind::MovePositionsState => matches!(
                other,
                StateKind::MovePositionsSelectionState | StateKind::MovePositionsDragState
            ),
            StateKind::RotateAroundCenterState => matches!(
                other,
                StateKind::RotateAroundCenterSelectionStartState
                    | StateKind::RotateAroundCenterSelectionEndState
                    | StateKind::RotateAroundCenterRotationStartState
                    | StateKind::RotateAroundCenterRotationEndState
            ),
            StateKind::ScalePositionsState => matches!(
                other,
                StateKind::ScalePositionsSelectionStartState
                    | StateKind::ScalePositionsSelectionEndState
                    | StateKind::ScalePositionsDragStartState
                    | StateKind::ScalePositionsDragEndState
            ),
            StateKind::ScaleAroundDancerState => matches!(
                other,
                StateKind::ScaleAroundDancerSelectionStartState
                    | StateKind::ScaleAroundDancerSelectionEndState
                    | StateKind::ScaleAroundDancerDragStartState
                    | StateKind::ScaleAroundDancerDragEndState
            ),
            _ => false,
        }
    }
}

macro_rules! state_struct {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl ApplicationState for $name {
            fn kind(&self) -> StateKind {
                $kind
            }
        }
    };
}

state_struct!(InitialApplicationState, StateKind::InitialApplicationState);
state_struct!(ViewSceneState, StateKind::ViewSceneState);
state_struct!(ViewScenePanState, StateKind::ViewScenePanState);
state_struct!(ViewSceneZoomState, StateKind::ViewSceneZoomState);
state_struct!(PlacePositionsState, StateKind::PlacePositionsState);
state_struct!(PlacePositionsPanState, StateKind::PlacePositionsPanState);
state_struct!(PlacePositionsZoomState, StateKind::PlacePositionsZoomState);
state_struct!(MovePositionsState, StateKind::MovePositionsState);
state_struct!(MovePositionsSelectionState, StateKind::MovePositionsSelectionState);
state_struct!(MovePositionsDragState, StateKind::MovePositionsDragState);
state_struct!(RotateAroundCenterState, StateKind::RotateAroundCenterState);
state_struct!(
    RotateAroundCenterSelectionStartState,
    StateKind::RotateAroundCenterSelectionStartState
);
state_struct!(
    RotateAroundCenterSelectionEndState,
    StateKind::RotateAroundCenterSelectionEndState
);
state_struct!(
    RotateAroundCenterRotationStartState,
    StateKind::RotateAroundCenterRotationStartState
);
state_struct!(
    RotateAroundCenterRotationEndState,
    StateKind::RotateAroundCenterRotationEndState
);
state_struct!(ScalePositionsState, StateKind::ScalePositionsState);
state_struct!(
    ScalePositionsSelectionStartState,
    StateKind::ScalePositionsSelectionStartState
);
state_struct!(
    ScalePositionsSelectionEndState,
    StateKind::ScalePositionsSelectionEndState
);
state_struct!(
    ScalePositionsDragStartState,
    StateKind::ScalePositionsDragStartState
);
state_struct!(
    ScalePositionsDragEndState,
    StateKind::ScalePositionsDragEndState
);
state_struct!(ScaleAroundDancerState, StateKind::ScaleAroundDancerState);
state_struct!(
    ScaleAroundDancerSelectionStartState,
    StateKind::ScaleAroundDancerSelectionStartState
);
state_struct!(
    ScaleAroundDancerSelectionEndState,
    StateKind::ScaleAroundDancerSelectionEndState
);
state_struct!(
    ScaleAroundDancerDragStartState,
    StateKind::ScaleAroundDancerDragStartState
);
state_struct!(
    ScaleAroundDancerDragEndState,
    StateKind::ScaleAroundDancerDragEndState
);
