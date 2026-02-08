#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use nject::injectable;
use std::fmt::Debug;
use std::sync::Once;

pub trait GlobalStateModel: Debug {}

pub trait ApplicationState: Debug {
    fn kind(&self) -> StateKind;
}

pub trait ApplicationTrigger: Debug {
    fn kind(&self) -> TriggerKind;
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TriggerKind {
    ApplicationTrigger,
    MovePositionsCompletedTrigger,
    MovePositionsDragCompletedTrigger,
    MovePositionsDragStartedTrigger,
    MovePositionsSelectionCompletedTrigger,
    MovePositionsSelectionStartedTrigger,
    MovePositionsStartedTrigger,
    PanCompletedTrigger,
    PanStartedTrigger,
    PlacePositionsCanceledTrigger,
    PlacePositionsCompletedTrigger,
    PlacePositionsStartedTrigger,
    RotateAroundCenterCompletedTrigger,
    RotateAroundCenterRotationCompletedTrigger,
    RotateAroundCenterRotationStartedTrigger,
    RotateAroundCenterSelectionCompletedTrigger,
    RotateAroundCenterSelectionStartedTrigger,
    RotateAroundCenterStartedTrigger,
    ScaleAroundDancerCompletedTrigger,
    ScaleAroundDancerDragCompletedTrigger,
    ScaleAroundDancerDragStartedTrigger,
    ScaleAroundDancerSelectionCompletedTrigger,
    ScaleAroundDancerSelectionStartedTrigger,
    ScaleAroundDancerStartedTrigger,
    ScalePositionsCompletedTrigger,
    ScalePositionsDragCompletedTrigger,
    ScalePositionsDragStartedTrigger,
    ScalePositionsSelectionCompletedTrigger,
    ScalePositionsSelectionStartedTrigger,
    ScalePositionsStartedTrigger,
    ZoomCompletedTrigger,
    ZoomStartedTrigger,
}

impl TriggerKind {
    pub fn is_assignable_from(self, other: TriggerKind) -> bool {
        self == TriggerKind::ApplicationTrigger || self == other
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

macro_rules! trigger_struct {
    ($name:ident, $kind:expr) => {
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
        pub struct $name;

        impl ApplicationTrigger for $name {
            fn kind(&self) -> TriggerKind {
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
state_struct!(
    MovePositionsSelectionState,
    StateKind::MovePositionsSelectionState
);
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

trigger_struct!(ApplicationTriggerBase, TriggerKind::ApplicationTrigger);
trigger_struct!(
    MovePositionsCompletedTrigger,
    TriggerKind::MovePositionsCompletedTrigger
);
trigger_struct!(
    MovePositionsDragCompletedTrigger,
    TriggerKind::MovePositionsDragCompletedTrigger
);
trigger_struct!(
    MovePositionsDragStartedTrigger,
    TriggerKind::MovePositionsDragStartedTrigger
);
trigger_struct!(
    MovePositionsSelectionCompletedTrigger,
    TriggerKind::MovePositionsSelectionCompletedTrigger
);
trigger_struct!(
    MovePositionsSelectionStartedTrigger,
    TriggerKind::MovePositionsSelectionStartedTrigger
);
trigger_struct!(
    MovePositionsStartedTrigger,
    TriggerKind::MovePositionsStartedTrigger
);
trigger_struct!(PanCompletedTrigger, TriggerKind::PanCompletedTrigger);
trigger_struct!(PanStartedTrigger, TriggerKind::PanStartedTrigger);
trigger_struct!(
    PlacePositionsCanceledTrigger,
    TriggerKind::PlacePositionsCanceledTrigger
);
trigger_struct!(
    PlacePositionsCompletedTrigger,
    TriggerKind::PlacePositionsCompletedTrigger
);
trigger_struct!(
    PlacePositionsStartedTrigger,
    TriggerKind::PlacePositionsStartedTrigger
);
trigger_struct!(
    RotateAroundCenterCompletedTrigger,
    TriggerKind::RotateAroundCenterCompletedTrigger
);
trigger_struct!(
    RotateAroundCenterRotationCompletedTrigger,
    TriggerKind::RotateAroundCenterRotationCompletedTrigger
);
trigger_struct!(
    RotateAroundCenterRotationStartedTrigger,
    TriggerKind::RotateAroundCenterRotationStartedTrigger
);
trigger_struct!(
    RotateAroundCenterSelectionCompletedTrigger,
    TriggerKind::RotateAroundCenterSelectionCompletedTrigger
);
trigger_struct!(
    RotateAroundCenterSelectionStartedTrigger,
    TriggerKind::RotateAroundCenterSelectionStartedTrigger
);
trigger_struct!(
    RotateAroundCenterStartedTrigger,
    TriggerKind::RotateAroundCenterStartedTrigger
);
trigger_struct!(
    ScaleAroundDancerCompletedTrigger,
    TriggerKind::ScaleAroundDancerCompletedTrigger
);
trigger_struct!(
    ScaleAroundDancerDragCompletedTrigger,
    TriggerKind::ScaleAroundDancerDragCompletedTrigger
);
trigger_struct!(
    ScaleAroundDancerDragStartedTrigger,
    TriggerKind::ScaleAroundDancerDragStartedTrigger
);
trigger_struct!(
    ScaleAroundDancerSelectionCompletedTrigger,
    TriggerKind::ScaleAroundDancerSelectionCompletedTrigger
);
trigger_struct!(
    ScaleAroundDancerSelectionStartedTrigger,
    TriggerKind::ScaleAroundDancerSelectionStartedTrigger
);
trigger_struct!(
    ScaleAroundDancerStartedTrigger,
    TriggerKind::ScaleAroundDancerStartedTrigger
);
trigger_struct!(
    ScalePositionsCompletedTrigger,
    TriggerKind::ScalePositionsCompletedTrigger
);
trigger_struct!(
    ScalePositionsDragCompletedTrigger,
    TriggerKind::ScalePositionsDragCompletedTrigger
);
trigger_struct!(
    ScalePositionsDragStartedTrigger,
    TriggerKind::ScalePositionsDragStartedTrigger
);
trigger_struct!(
    ScalePositionsSelectionCompletedTrigger,
    TriggerKind::ScalePositionsSelectionCompletedTrigger
);
trigger_struct!(
    ScalePositionsSelectionStartedTrigger,
    TriggerKind::ScalePositionsSelectionStartedTrigger
);
trigger_struct!(
    ScalePositionsStartedTrigger,
    TriggerKind::ScalePositionsStartedTrigger
);
trigger_struct!(ZoomCompletedTrigger, TriggerKind::ZoomCompletedTrigger);
trigger_struct!(ZoomStartedTrigger, TriggerKind::ZoomStartedTrigger);

pub type Preconditions = Vec<Precondition>;
pub type Precondition = Box<
    dyn Fn(&dyn GlobalStateModel, &dyn ApplicationState, &dyn ApplicationTrigger) -> bool
        + Send
        + Sync,
>;
pub type ApplyFn = Box<
    dyn Fn(
            &dyn GlobalStateModel,
            &dyn ApplicationState,
            &dyn ApplicationTrigger,
        ) -> Box<dyn ApplicationState>
        + Send
        + Sync,
>;

pub struct StateTransition {
    pub from_state: StateKind,
    pub trigger: TriggerKind,
    pub preconditions: Preconditions,
    pub apply: ApplyFn,
}

impl StateTransition {
    pub fn can_apply(
        &self,
        global_state: &dyn GlobalStateModel,
        state: &dyn ApplicationState,
        trigger: &dyn ApplicationTrigger,
    ) -> bool {
        self.preconditions.is_empty()
            || self
                .preconditions
                .iter()
                .all(|precondition| precondition(global_state, state, trigger))
    }
}

#[injectable]
#[inject(|global_state: Box<dyn GlobalStateModel>, transitions: Vec<StateTransition>| Self {
    global_state,
    transitions,
    state: Box::new(ViewSceneState),
})]
pub struct ApplicationStateMachine {
    global_state: Box<dyn GlobalStateModel>,
    transitions: Vec<StateTransition>,
    state: Box<dyn ApplicationState>,
}

fn ensure_logger() {
    static LOGGER_INIT: Once = Once::new();

    LOGGER_INIT.call_once(|| {
        let _ = env_logger::builder().is_test(cfg!(test)).try_init();
    });
}

impl ApplicationStateMachine {
    pub fn new(global_state: Box<dyn GlobalStateModel>, transitions: Vec<StateTransition>) -> Self {
        Self {
            global_state,
            transitions,
            state: Box::new(ViewSceneState),
        }
    }

    pub fn with_default_transitions(global_state: Box<dyn GlobalStateModel>) -> Self {
        Self::new(global_state, default_transitions())
    }

    pub fn state(&self) -> &dyn ApplicationState {
        self.state.as_ref()
    }

    pub fn try_apply(&mut self, trigger: &dyn ApplicationTrigger) -> bool {
        let state_kind = self.state.kind();
        let trigger_kind = trigger.kind();

        for transition in &self.transitions {
            if !transition.from_state.is_assignable_from(state_kind)
                || !transition.trigger.is_assignable_from(trigger_kind)
            {
                continue;
            }

            if !transition.can_apply(self.global_state.as_ref(), self.state.as_ref(), trigger) {
                continue;
            }

            let next_state =
                (transition.apply)(self.global_state.as_ref(), self.state.as_ref(), trigger);
            let next_kind = next_state.kind();

            if next_kind != state_kind {
                ensure_logger();
                log::info!(
                    "ApplicationStateMachine transition: {:?} -> {:?} (trigger: {:?})",
                    state_kind,
                    next_kind,
                    trigger_kind
                );
            }

            self.state = next_state;
            return true;
        }

        false
    }
}

pub fn default_transitions() -> Vec<StateTransition> {
    let transitions = vec![
        transition(
            StateKind::ViewSceneState,
            TriggerKind::PanStartedTrigger,
            StateKind::ViewScenePanState,
        ),
        transition(
            StateKind::ViewScenePanState,
            TriggerKind::PanCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ViewSceneState,
            TriggerKind::ZoomStartedTrigger,
            StateKind::ViewSceneZoomState,
        ),
        transition(
            StateKind::ViewSceneZoomState,
            TriggerKind::ZoomCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ViewScenePanState,
            TriggerKind::ZoomStartedTrigger,
            StateKind::ViewSceneZoomState,
        ),
        transition(
            StateKind::ViewSceneZoomState,
            TriggerKind::PanStartedTrigger,
            StateKind::ViewScenePanState,
        ),
        transition(
            StateKind::ViewSceneState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::ViewScenePanState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::ViewSceneZoomState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::ViewSceneState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ViewScenePanState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ViewSceneZoomState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ViewSceneState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::ViewScenePanState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::ViewSceneZoomState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::ViewSceneState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::ViewScenePanState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::ViewSceneZoomState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::MovePositionsState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::MovePositionsSelectionState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::MovePositionsDragState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::MovePositionsState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::MovePositionsSelectionState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::MovePositionsDragState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::MovePositionsState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::MovePositionsSelectionState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::MovePositionsDragState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::MovePositionsState,
            TriggerKind::MovePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::MovePositionsSelectionState,
            TriggerKind::MovePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::MovePositionsDragState,
            TriggerKind::MovePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::MovePositionsState,
            TriggerKind::MovePositionsSelectionStartedTrigger,
            StateKind::MovePositionsSelectionState,
        ),
        transition(
            StateKind::MovePositionsSelectionState,
            TriggerKind::MovePositionsSelectionCompletedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::MovePositionsSelectionState,
            TriggerKind::MovePositionsSelectionStartedTrigger,
            StateKind::MovePositionsSelectionState,
        ),
        transition(
            StateKind::MovePositionsSelectionState,
            TriggerKind::MovePositionsDragStartedTrigger,
            StateKind::MovePositionsDragState,
        ),
        transition(
            StateKind::MovePositionsDragState,
            TriggerKind::MovePositionsDragCompletedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::MovePositionsDragState,
            TriggerKind::MovePositionsDragStartedTrigger,
            StateKind::MovePositionsDragState,
        ),
        transition(
            StateKind::RotateAroundCenterState,
            TriggerKind::RotateAroundCenterCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionStartState,
            TriggerKind::RotateAroundCenterCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionEndState,
            TriggerKind::RotateAroundCenterCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationStartState,
            TriggerKind::RotateAroundCenterCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationEndState,
            TriggerKind::RotateAroundCenterCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::RotateAroundCenterState,
            TriggerKind::RotateAroundCenterSelectionStartedTrigger,
            StateKind::RotateAroundCenterSelectionStartState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionStartState,
            TriggerKind::RotateAroundCenterSelectionCompletedTrigger,
            StateKind::RotateAroundCenterSelectionEndState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionEndState,
            TriggerKind::RotateAroundCenterSelectionStartedTrigger,
            StateKind::RotateAroundCenterSelectionStartState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionStartState,
            TriggerKind::RotateAroundCenterRotationStartedTrigger,
            StateKind::RotateAroundCenterRotationStartState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationStartState,
            TriggerKind::RotateAroundCenterRotationCompletedTrigger,
            StateKind::RotateAroundCenterRotationEndState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationEndState,
            TriggerKind::RotateAroundCenterRotationStartedTrigger,
            StateKind::RotateAroundCenterRotationStartState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionEndState,
            TriggerKind::RotateAroundCenterRotationStartedTrigger,
            StateKind::RotateAroundCenterRotationStartState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationStartState,
            TriggerKind::RotateAroundCenterSelectionStartedTrigger,
            StateKind::RotateAroundCenterSelectionStartState,
        ),
        transition(
            StateKind::ScaleAroundDancerState,
            TriggerKind::ScaleAroundDancerCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionStartState,
            TriggerKind::ScaleAroundDancerCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionEndState,
            TriggerKind::ScaleAroundDancerCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScaleAroundDancerDragStartState,
            TriggerKind::ScaleAroundDancerCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScaleAroundDancerDragEndState,
            TriggerKind::ScaleAroundDancerCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScaleAroundDancerState,
            TriggerKind::ScaleAroundDancerSelectionStartedTrigger,
            StateKind::ScaleAroundDancerSelectionStartState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionStartState,
            TriggerKind::ScaleAroundDancerSelectionCompletedTrigger,
            StateKind::ScaleAroundDancerSelectionEndState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionEndState,
            TriggerKind::ScaleAroundDancerSelectionStartedTrigger,
            StateKind::ScaleAroundDancerSelectionStartState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionStartState,
            TriggerKind::ScaleAroundDancerDragStartedTrigger,
            StateKind::ScaleAroundDancerDragStartState,
        ),
        transition(
            StateKind::ScaleAroundDancerDragStartState,
            TriggerKind::ScaleAroundDancerDragCompletedTrigger,
            StateKind::ScaleAroundDancerDragEndState,
        ),
        transition(
            StateKind::ScaleAroundDancerDragEndState,
            TriggerKind::ScaleAroundDancerDragStartedTrigger,
            StateKind::ScaleAroundDancerDragStartState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionEndState,
            TriggerKind::ScaleAroundDancerDragStartedTrigger,
            StateKind::ScaleAroundDancerDragStartState,
        ),
        transition(
            StateKind::ScaleAroundDancerDragStartState,
            TriggerKind::ScaleAroundDancerSelectionStartedTrigger,
            StateKind::ScaleAroundDancerSelectionStartState,
        ),
        transition(
            StateKind::ScalePositionsState,
            TriggerKind::ScalePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScalePositionsSelectionStartState,
            TriggerKind::ScalePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScalePositionsSelectionEndState,
            TriggerKind::ScalePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScalePositionsDragStartState,
            TriggerKind::ScalePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScalePositionsDragEndState,
            TriggerKind::ScalePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::ScalePositionsState,
            TriggerKind::ScalePositionsSelectionStartedTrigger,
            StateKind::ScalePositionsSelectionStartState,
        ),
        transition(
            StateKind::ScalePositionsSelectionStartState,
            TriggerKind::ScalePositionsSelectionCompletedTrigger,
            StateKind::ScalePositionsSelectionEndState,
        ),
        transition(
            StateKind::ScalePositionsSelectionEndState,
            TriggerKind::ScalePositionsSelectionStartedTrigger,
            StateKind::ScalePositionsSelectionStartState,
        ),
        transition(
            StateKind::ScalePositionsSelectionStartState,
            TriggerKind::ScalePositionsDragStartedTrigger,
            StateKind::ScalePositionsDragStartState,
        ),
        transition(
            StateKind::ScalePositionsDragStartState,
            TriggerKind::ScalePositionsDragCompletedTrigger,
            StateKind::ScalePositionsDragEndState,
        ),
        transition(
            StateKind::ScalePositionsDragEndState,
            TriggerKind::ScalePositionsDragStartedTrigger,
            StateKind::ScalePositionsDragStartState,
        ),
        transition(
            StateKind::ScalePositionsSelectionEndState,
            TriggerKind::ScalePositionsDragStartedTrigger,
            StateKind::ScalePositionsDragStartState,
        ),
        transition(
            StateKind::ScalePositionsDragStartState,
            TriggerKind::ScalePositionsSelectionStartedTrigger,
            StateKind::ScalePositionsSelectionStartState,
        ),
        transition(
            StateKind::RotateAroundCenterState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionStartState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionEndState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationStartState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationEndState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionStartState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionEndState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationStartState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationEndState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::RotateAroundCenterState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionStartState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionEndState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationStartState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationEndState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::RotateAroundCenterState,
            TriggerKind::RotateAroundCenterSelectionStartedTrigger,
            StateKind::RotateAroundCenterSelectionStartState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionEndState,
            TriggerKind::RotateAroundCenterSelectionStartedTrigger,
            StateKind::RotateAroundCenterSelectionStartState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationEndState,
            TriggerKind::RotateAroundCenterRotationStartedTrigger,
            StateKind::RotateAroundCenterRotationStartState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionEndState,
            TriggerKind::RotateAroundCenterRotationStartedTrigger,
            StateKind::RotateAroundCenterRotationStartState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationStartState,
            TriggerKind::RotateAroundCenterSelectionStartedTrigger,
            StateKind::RotateAroundCenterSelectionStartState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationEndState,
            TriggerKind::RotateAroundCenterSelectionStartedTrigger,
            StateKind::RotateAroundCenterSelectionStartState,
        ),
        transition(
            StateKind::RotateAroundCenterSelectionStartState,
            TriggerKind::RotateAroundCenterRotationStartedTrigger,
            StateKind::RotateAroundCenterRotationStartState,
        ),
        transition(
            StateKind::RotateAroundCenterRotationStartState,
            TriggerKind::RotateAroundCenterSelectionStartedTrigger,
            StateKind::RotateAroundCenterSelectionStartState,
        ),
        transition(
            StateKind::ScalePositionsState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::ScalePositionsSelectionStartState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::ScalePositionsSelectionEndState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::ScalePositionsDragStartState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::ScalePositionsDragEndState,
            TriggerKind::MovePositionsStartedTrigger,
            StateKind::MovePositionsState,
        ),
        transition(
            StateKind::ScalePositionsState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScalePositionsSelectionStartState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScalePositionsSelectionEndState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScalePositionsDragStartState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScalePositionsDragEndState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScalePositionsState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::ScalePositionsSelectionStartState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::ScalePositionsSelectionEndState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::ScalePositionsDragStartState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::ScalePositionsDragEndState,
            TriggerKind::ScaleAroundDancerStartedTrigger,
            StateKind::ScaleAroundDancerState,
        ),
        transition(
            StateKind::ScaleAroundDancerState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionStartState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionEndState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScaleAroundDancerDragStartState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScaleAroundDancerDragEndState,
            TriggerKind::RotateAroundCenterStartedTrigger,
            StateKind::RotateAroundCenterState,
        ),
        transition(
            StateKind::ScaleAroundDancerState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionStartState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::ScaleAroundDancerSelectionEndState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::ScaleAroundDancerDragStartState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::ScaleAroundDancerDragEndState,
            TriggerKind::ScalePositionsStartedTrigger,
            StateKind::ScalePositionsState,
        ),
        transition(
            StateKind::ViewSceneState,
            TriggerKind::PlacePositionsStartedTrigger,
            StateKind::PlacePositionsState,
        ),
        transition(
            StateKind::ViewScenePanState,
            TriggerKind::PlacePositionsStartedTrigger,
            StateKind::PlacePositionsState,
        ),
        transition(
            StateKind::ViewSceneZoomState,
            TriggerKind::PlacePositionsStartedTrigger,
            StateKind::PlacePositionsState,
        ),
        transition(
            StateKind::PlacePositionsState,
            TriggerKind::PlacePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::PlacePositionsPanState,
            TriggerKind::PlacePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::PlacePositionsZoomState,
            TriggerKind::PlacePositionsCompletedTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::PlacePositionsState,
            TriggerKind::PlacePositionsCanceledTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::PlacePositionsPanState,
            TriggerKind::PlacePositionsCanceledTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::PlacePositionsZoomState,
            TriggerKind::PlacePositionsCanceledTrigger,
            StateKind::ViewSceneState,
        ),
        transition(
            StateKind::PlacePositionsState,
            TriggerKind::PanStartedTrigger,
            StateKind::PlacePositionsPanState,
        ),
        transition(
            StateKind::PlacePositionsPanState,
            TriggerKind::PanCompletedTrigger,
            StateKind::PlacePositionsState,
        ),
        transition(
            StateKind::PlacePositionsState,
            TriggerKind::ZoomStartedTrigger,
            StateKind::PlacePositionsZoomState,
        ),
        transition(
            StateKind::PlacePositionsZoomState,
            TriggerKind::ZoomCompletedTrigger,
            StateKind::PlacePositionsState,
        ),
        transition(
            StateKind::PlacePositionsPanState,
            TriggerKind::ZoomStartedTrigger,
            StateKind::PlacePositionsZoomState,
        ),
        transition(
            StateKind::PlacePositionsZoomState,
            TriggerKind::PanStartedTrigger,
            StateKind::PlacePositionsPanState,
        ),
    ];

    transitions
}

fn transition(from_state: StateKind, trigger: TriggerKind, to_state: StateKind) -> StateTransition {
    StateTransition {
        from_state,
        trigger,
        preconditions: Vec::new(),
        apply: Box::new(move |_, _, _| create_state(to_state)),
    }
}

fn create_state(kind: StateKind) -> Box<dyn ApplicationState> {
    match kind {
        StateKind::InitialApplicationState => Box::new(InitialApplicationState),
        StateKind::ViewSceneState => Box::new(ViewSceneState),
        StateKind::ViewScenePanState => Box::new(ViewScenePanState),
        StateKind::ViewSceneZoomState => Box::new(ViewSceneZoomState),
        StateKind::PlacePositionsState => Box::new(PlacePositionsState),
        StateKind::PlacePositionsPanState => Box::new(PlacePositionsPanState),
        StateKind::PlacePositionsZoomState => Box::new(PlacePositionsZoomState),
        StateKind::MovePositionsState => Box::new(MovePositionsState),
        StateKind::MovePositionsSelectionState => Box::new(MovePositionsSelectionState),
        StateKind::MovePositionsDragState => Box::new(MovePositionsDragState),
        StateKind::RotateAroundCenterState => Box::new(RotateAroundCenterState),
        StateKind::RotateAroundCenterSelectionStartState => {
            Box::new(RotateAroundCenterSelectionStartState)
        }
        StateKind::RotateAroundCenterSelectionEndState => {
            Box::new(RotateAroundCenterSelectionEndState)
        }
        StateKind::RotateAroundCenterRotationStartState => {
            Box::new(RotateAroundCenterRotationStartState)
        }
        StateKind::RotateAroundCenterRotationEndState => {
            Box::new(RotateAroundCenterRotationEndState)
        }
        StateKind::ScalePositionsState => Box::new(ScalePositionsState),
        StateKind::ScalePositionsSelectionStartState => Box::new(ScalePositionsSelectionStartState),
        StateKind::ScalePositionsSelectionEndState => Box::new(ScalePositionsSelectionEndState),
        StateKind::ScalePositionsDragStartState => Box::new(ScalePositionsDragStartState),
        StateKind::ScalePositionsDragEndState => Box::new(ScalePositionsDragEndState),
        StateKind::ScaleAroundDancerState => Box::new(ScaleAroundDancerState),
        StateKind::ScaleAroundDancerSelectionStartState => {
            Box::new(ScaleAroundDancerSelectionStartState)
        }
        StateKind::ScaleAroundDancerSelectionEndState => {
            Box::new(ScaleAroundDancerSelectionEndState)
        }
        StateKind::ScaleAroundDancerDragStartState => Box::new(ScaleAroundDancerDragStartState),
        StateKind::ScaleAroundDancerDragEndState => Box::new(ScaleAroundDancerDragEndState),
    }
}
