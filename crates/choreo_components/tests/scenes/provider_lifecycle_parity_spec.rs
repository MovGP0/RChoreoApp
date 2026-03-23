use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::provider::ScenesBehavior;
use super::provider::ScenesBehaviorFactory;
use super::provider::ScenesBehaviorKind;
use super::provider::ScenesProvider;
use super::provider::ScenesProviderDependencies;
use super::provider::SyncShowTimestampsBehaviorFactory;
use super::scene_model;

#[derive(Default)]
struct ProbeBehavior {
    ticks: usize,
}

impl ScenesBehavior for ProbeBehavior {
    fn kind(&self) -> ScenesBehaviorKind {
        ScenesBehaviorKind::Load
    }

    fn on_tick(&mut self, _state: &mut super::state::ScenesState) {
        self.ticks += 1;
    }
}

struct ProbeFactory;

impl ScenesBehaviorFactory for ProbeFactory {
    fn create(&self) -> Box<dyn ScenesBehavior> {
        Box::<ProbeBehavior>::default()
    }
}

#[test]
fn provider_activates_behaviors_and_dispatches_actions() {
    let _all_kinds = [
        ScenesBehaviorKind::Load,
        ScenesBehaviorKind::Filter,
        ScenesBehaviorKind::Insert,
        ScenesBehaviorKind::Select,
        ScenesBehaviorKind::SelectFromAudio,
        ScenesBehaviorKind::ShowTimestamps,
        ScenesBehaviorKind::Open,
        ScenesBehaviorKind::Save,
    ];

    let mut provider = ScenesProvider::new(
        create_state(),
        ScenesProviderDependencies {
            behavior_factories: vec![
                Box::new(ProbeFactory),
                Box::new(SyncShowTimestampsBehaviorFactory),
            ],
        },
    );

    provider.activate();
    provider.dispatch(ScenesAction::LoadScenes {
        choreography: Box::new(choreography_with_scenes(
            "Demo",
            vec![scene_model(1, "Scene", Some("1.2"), vec![])],
        )),
    });
    provider.tick();

    assert_eq!(provider.activation_order[0], ScenesBehaviorKind::Load);
    assert_eq!(
        provider.activation_order[1],
        ScenesBehaviorKind::ShowTimestamps
    );
    assert_eq!(provider.state().scenes.len(), 1);

    provider.deactivate();
}
