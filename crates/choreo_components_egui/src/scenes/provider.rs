use super::actions::ScenesAction;
use super::reducer::reduce;
use super::state::ScenesState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScenesBehaviorKind {
    Load,
    Filter,
    Insert,
    Select,
    SelectFromAudio,
    ShowTimestamps,
    Open,
    Save,
}

pub trait ScenesBehavior {
    fn kind(&self) -> ScenesBehaviorKind;
    fn activate(&mut self, _state: &mut ScenesState) {}
    fn on_tick(&mut self, _state: &mut ScenesState) {}
    fn deactivate(&mut self, _state: &mut ScenesState) {}
}

pub trait ScenesBehaviorFactory {
    fn create(&self) -> Box<dyn ScenesBehavior>;
}

pub struct ScenesProviderDependencies {
    pub behavior_factories: Vec<Box<dyn ScenesBehaviorFactory>>,
}

pub struct ScenesProvider {
    state: ScenesState,
    dependencies: ScenesProviderDependencies,
    active_behaviors: Vec<Box<dyn ScenesBehavior>>,
    pub activation_order: Vec<ScenesBehaviorKind>,
}

impl ScenesProvider {
    #[must_use]
    pub fn new(state: ScenesState, dependencies: ScenesProviderDependencies) -> Self {
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
    }

    pub fn dispatch(&mut self, action: ScenesAction) {
        reduce(&mut self.state, action);
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
    pub fn state(&self) -> &ScenesState {
        &self.state
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyncShowTimestampsBehavior;

impl ScenesBehavior for SyncShowTimestampsBehavior {
    fn kind(&self) -> ScenesBehaviorKind {
        ScenesBehaviorKind::ShowTimestamps
    }

    fn on_tick(&mut self, state: &mut ScenesState) {
        reduce(state, ScenesAction::SyncShowTimestampsFromChoreography);
    }
}

pub struct SyncShowTimestampsBehaviorFactory;

impl ScenesBehaviorFactory for SyncShowTimestampsBehaviorFactory {
    fn create(&self) -> Box<dyn ScenesBehavior> {
        Box::new(SyncShowTimestampsBehavior)
    }
}
