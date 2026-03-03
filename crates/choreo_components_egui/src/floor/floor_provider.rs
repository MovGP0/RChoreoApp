use super::actions::FloorAction;
use super::reducer::reduce;
use super::state::FloorState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloorBehaviorKind {
    Draw,
    Redraw,
    Gesture,
    Move,
    Place,
    Rotate,
    Scale,
}

pub trait FloorBehavior {
    fn kind(&self) -> FloorBehaviorKind;
    fn activate(&mut self, _state: &mut FloorState) {}
    fn on_tick(&mut self, _state: &mut FloorState) {}
    fn deactivate(&mut self, _state: &mut FloorState) {}
}

pub trait FloorBehaviorFactory {
    fn create(&self) -> Box<dyn FloorBehavior>;
}

pub struct FloorProviderDependencies {
    pub behavior_factories: Vec<Box<dyn FloorBehaviorFactory>>,
}

pub struct FloorProvider {
    state: FloorState,
    dependencies: FloorProviderDependencies,
    active_behaviors: Vec<Box<dyn FloorBehavior>>,
    pub activation_order: Vec<FloorBehaviorKind>,
}

impl FloorProvider {
    #[must_use]
    pub fn new(state: FloorState, dependencies: FloorProviderDependencies) -> Self {
        Self {
            state,
            dependencies,
            active_behaviors: Vec::new(),
            activation_order: Vec::new(),
        }
    }

    pub fn activate(&mut self) {
        if !self.active_behaviors.is_empty() {
            return;
        }

        for factory in &self.dependencies.behavior_factories {
            let mut behavior = factory.create();
            self.activation_order.push(behavior.kind());
            behavior.activate(&mut self.state);
            self.active_behaviors.push(behavior);
        }
        reduce(&mut self.state, FloorAction::Initialize);
        reduce(&mut self.state, FloorAction::DrawFloor);
    }

    pub fn tick(&mut self) {
        for behavior in &mut self.active_behaviors {
            behavior.on_tick(&mut self.state);
        }
    }

    pub fn deactivate(&mut self) {
        for behavior in &mut self.active_behaviors {
            behavior.deactivate(&mut self.state);
        }
        self.active_behaviors.clear();
    }

    #[must_use]
    pub fn state(&self) -> &FloorState {
        &self.state
    }

    #[must_use]
    pub fn state_mut(&mut self) -> &mut FloorState {
        &mut self.state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickRedrawBehavior;

impl FloorBehavior for TickRedrawBehavior {
    fn kind(&self) -> FloorBehaviorKind {
        FloorBehaviorKind::Redraw
    }

    fn on_tick(&mut self, state: &mut FloorState) {
        reduce(state, FloorAction::RedrawFloor);
    }
}

pub struct TickRedrawBehaviorFactory;

impl FloorBehaviorFactory for TickRedrawBehaviorFactory {
    fn create(&self) -> Box<dyn FloorBehavior> {
        Box::new(TickRedrawBehavior)
    }
}
