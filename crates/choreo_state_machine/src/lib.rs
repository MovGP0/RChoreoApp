#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use std::fmt::Debug;
use thiserror::Error;

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
pub type Precondition =
    Box<dyn Fn(&dyn GlobalStateModel, &dyn ApplicationState, &dyn ApplicationTrigger) -> bool + Send + Sync>;
pub type ApplyFn = Box<
    dyn Fn(&dyn GlobalStateModel, &dyn ApplicationState, &dyn ApplicationTrigger) -> Box<dyn ApplicationState>
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

pub struct ApplicationStateMachine {
    global_state: Box<dyn GlobalStateModel>,
    transitions: Vec<StateTransition>,
    state: Box<dyn ApplicationState>,
}

impl ApplicationStateMachine {
    pub fn new(global_state: Box<dyn GlobalStateModel>, transitions: Vec<StateTransition>) -> Self {
        Self {
            global_state,
            transitions,
            state: Box::new(ViewSceneState),
        }
    }

    pub fn with_default_transitions(
        global_state: Box<dyn GlobalStateModel>,
    ) -> Result<Self, StateMachineError> {
        let transitions = default_transitions()?;
        Ok(Self::new(global_state, transitions))
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

            self.state = (transition.apply)(self.global_state.as_ref(), self.state.as_ref(), trigger);
            return true;
        }

        false
    }
}

#[derive(Debug, Error)]
pub enum StateMachineError {
    #[error("unknown state name: {0}")]
    UnknownState(String),
    #[error("unknown trigger name: {0}")]
    UnknownTrigger(String),
    #[error("invalid transition matrix header")]
    InvalidHeader,
}

pub fn default_transitions() -> Result<Vec<StateTransition>, StateMachineError> {
    let csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/StateMachine_TransitionMatrix.csv"
    ));
    parse_transition_matrix(csv)
}

fn parse_transition_matrix(csv: &str) -> Result<Vec<StateTransition>, StateMachineError> {
    let mut lines = csv.lines();
    let header = lines.next().ok_or(StateMachineError::InvalidHeader)?;
    let header_parts: Vec<&str> = header.split(',').collect();
    if header_parts.len() < 2 {
        return Err(StateMachineError::InvalidHeader);
    }

    let to_states: Vec<StateKind> = header_parts
        .iter()
        .skip(1)
        .map(|name| state_kind_from_name(name.trim()))
        .collect::<Result<_, _>>()?;

    let mut transitions = Vec::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();
        if parts.is_empty() {
            continue;
        }

        let from_state = state_kind_from_name(parts[0].trim())?;
        for (index, cell) in parts.iter().enumerate().skip(1) {
            let cell = cell.trim();
            if cell.is_empty() {
                continue;
            }

            let to_state = to_states
                .get(index - 1)
                .copied()
                .ok_or(StateMachineError::InvalidHeader)?;

            for trigger_name in cell.split(';') {
                let trigger_name = trigger_name.trim();
                if trigger_name.is_empty() {
                    continue;
                }

                let trigger = trigger_kind_from_name(trigger_name)?;
                transitions.push(default_transition(from_state, trigger, to_state));
            }
        }
    }

    Ok(transitions)
}

fn default_transition(from_state: StateKind, trigger: TriggerKind, to_state: StateKind) -> StateTransition {
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
        StateKind::RotateAroundCenterSelectionEndState => Box::new(RotateAroundCenterSelectionEndState),
        StateKind::RotateAroundCenterRotationStartState => Box::new(RotateAroundCenterRotationStartState),
        StateKind::RotateAroundCenterRotationEndState => Box::new(RotateAroundCenterRotationEndState),
        StateKind::ScalePositionsState => Box::new(ScalePositionsState),
        StateKind::ScalePositionsSelectionStartState => Box::new(ScalePositionsSelectionStartState),
        StateKind::ScalePositionsSelectionEndState => Box::new(ScalePositionsSelectionEndState),
        StateKind::ScalePositionsDragStartState => Box::new(ScalePositionsDragStartState),
        StateKind::ScalePositionsDragEndState => Box::new(ScalePositionsDragEndState),
        StateKind::ScaleAroundDancerState => Box::new(ScaleAroundDancerState),
        StateKind::ScaleAroundDancerSelectionStartState => Box::new(ScaleAroundDancerSelectionStartState),
        StateKind::ScaleAroundDancerSelectionEndState => Box::new(ScaleAroundDancerSelectionEndState),
        StateKind::ScaleAroundDancerDragStartState => Box::new(ScaleAroundDancerDragStartState),
        StateKind::ScaleAroundDancerDragEndState => Box::new(ScaleAroundDancerDragEndState),
    }
}

fn state_kind_from_name(name: &str) -> Result<StateKind, StateMachineError> {
    match name {
        "InitialApplicationState" => Ok(StateKind::InitialApplicationState),
        "ViewSceneState" => Ok(StateKind::ViewSceneState),
        "ViewScenePanState" => Ok(StateKind::ViewScenePanState),
        "ViewSceneZoomState" => Ok(StateKind::ViewSceneZoomState),
        "PlacePositionsState" => Ok(StateKind::PlacePositionsState),
        "PlacePositionsPanState" => Ok(StateKind::PlacePositionsPanState),
        "PlacePositionsZoomState" => Ok(StateKind::PlacePositionsZoomState),
        "MovePositionsState" => Ok(StateKind::MovePositionsState),
        "MovePositionsSelectionState" => Ok(StateKind::MovePositionsSelectionState),
        "MovePositionsDragState" => Ok(StateKind::MovePositionsDragState),
        "RotateAroundCenterState" => Ok(StateKind::RotateAroundCenterState),
        "RotateAroundCenterSelectionStartState" => Ok(StateKind::RotateAroundCenterSelectionStartState),
        "RotateAroundCenterSelectionEndState" => Ok(StateKind::RotateAroundCenterSelectionEndState),
        "RotateAroundCenterRotationStartState" => Ok(StateKind::RotateAroundCenterRotationStartState),
        "RotateAroundCenterRotationEndState" => Ok(StateKind::RotateAroundCenterRotationEndState),
        "ScalePositionsState" => Ok(StateKind::ScalePositionsState),
        "ScalePositionsSelectionStartState" => Ok(StateKind::ScalePositionsSelectionStartState),
        "ScalePositionsSelectionEndState" => Ok(StateKind::ScalePositionsSelectionEndState),
        "ScalePositionsDragStartState" => Ok(StateKind::ScalePositionsDragStartState),
        "ScalePositionsDragEndState" => Ok(StateKind::ScalePositionsDragEndState),
        "ScaleAroundDancerState" => Ok(StateKind::ScaleAroundDancerState),
        "ScaleAroundDancerSelectionStartState" => Ok(StateKind::ScaleAroundDancerSelectionStartState),
        "ScaleAroundDancerSelectionEndState" => Ok(StateKind::ScaleAroundDancerSelectionEndState),
        "ScaleAroundDancerDragStartState" => Ok(StateKind::ScaleAroundDancerDragStartState),
        "ScaleAroundDancerDragEndState" => Ok(StateKind::ScaleAroundDancerDragEndState),
        _ => Err(StateMachineError::UnknownState(name.to_string())),
    }
}

fn trigger_kind_from_name(name: &str) -> Result<TriggerKind, StateMachineError> {
    match name {
        "MovePositionsCompletedTrigger" => Ok(TriggerKind::MovePositionsCompletedTrigger),
        "MovePositionsDragCompletedTrigger" => Ok(TriggerKind::MovePositionsDragCompletedTrigger),
        "MovePositionsDragStartedTrigger" => Ok(TriggerKind::MovePositionsDragStartedTrigger),
        "MovePositionsSelectionCompletedTrigger" => Ok(TriggerKind::MovePositionsSelectionCompletedTrigger),
        "MovePositionsSelectionStartedTrigger" => Ok(TriggerKind::MovePositionsSelectionStartedTrigger),
        "MovePositionsStartedTrigger" => Ok(TriggerKind::MovePositionsStartedTrigger),
        "PanCompletedTrigger" => Ok(TriggerKind::PanCompletedTrigger),
        "PanStartedTrigger" => Ok(TriggerKind::PanStartedTrigger),
        "PlacePositionsCanceledTrigger" => Ok(TriggerKind::PlacePositionsCanceledTrigger),
        "PlacePositionsCompletedTrigger" => Ok(TriggerKind::PlacePositionsCompletedTrigger),
        "PlacePositionsStartedTrigger" => Ok(TriggerKind::PlacePositionsStartedTrigger),
        "RotateAroundCenterCompletedTrigger" => Ok(TriggerKind::RotateAroundCenterCompletedTrigger),
        "RotateAroundCenterRotationCompletedTrigger" => {
            Ok(TriggerKind::RotateAroundCenterRotationCompletedTrigger)
        }
        "RotateAroundCenterRotationStartedTrigger" => {
            Ok(TriggerKind::RotateAroundCenterRotationStartedTrigger)
        }
        "RotateAroundCenterSelectionCompletedTrigger" => {
            Ok(TriggerKind::RotateAroundCenterSelectionCompletedTrigger)
        }
        "RotateAroundCenterSelectionStartedTrigger" => {
            Ok(TriggerKind::RotateAroundCenterSelectionStartedTrigger)
        }
        "RotateAroundCenterStartedTrigger" => Ok(TriggerKind::RotateAroundCenterStartedTrigger),
        "ScaleAroundDancerCompletedTrigger" => Ok(TriggerKind::ScaleAroundDancerCompletedTrigger),
        "ScaleAroundDancerDragCompletedTrigger" => {
            Ok(TriggerKind::ScaleAroundDancerDragCompletedTrigger)
        }
        "ScaleAroundDancerDragStartedTrigger" => Ok(TriggerKind::ScaleAroundDancerDragStartedTrigger),
        "ScaleAroundDancerSelectionCompletedTrigger" => {
            Ok(TriggerKind::ScaleAroundDancerSelectionCompletedTrigger)
        }
        "ScaleAroundDancerSelectionStartedTrigger" => {
            Ok(TriggerKind::ScaleAroundDancerSelectionStartedTrigger)
        }
        "ScaleAroundDancerStartedTrigger" => Ok(TriggerKind::ScaleAroundDancerStartedTrigger),
        "ScalePositionsCompletedTrigger" => Ok(TriggerKind::ScalePositionsCompletedTrigger),
        "ScalePositionsDragCompletedTrigger" => Ok(TriggerKind::ScalePositionsDragCompletedTrigger),
        "ScalePositionsDragStartedTrigger" => Ok(TriggerKind::ScalePositionsDragStartedTrigger),
        "ScalePositionsSelectionCompletedTrigger" => {
            Ok(TriggerKind::ScalePositionsSelectionCompletedTrigger)
        }
        "ScalePositionsSelectionStartedTrigger" => Ok(TriggerKind::ScalePositionsSelectionStartedTrigger),
        "ScalePositionsStartedTrigger" => Ok(TriggerKind::ScalePositionsStartedTrigger),
        "ZoomCompletedTrigger" => Ok(TriggerKind::ZoomCompletedTrigger),
        "ZoomStartedTrigger" => Ok(TriggerKind::ZoomStartedTrigger),
        _ => Err(StateMachineError::UnknownTrigger(name.to_string())),
    }
}
