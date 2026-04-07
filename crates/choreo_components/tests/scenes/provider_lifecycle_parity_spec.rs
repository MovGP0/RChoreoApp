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

    fn assert_no_errors(errors: Vec<String>) {
        assert!(
            errors.is_empty(),
            "Assertion failures:\n{}",
            errors.join("\n")
        );
    }

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

    let mut errors = Vec::new();

    check_eq!(errors, provider.activation_order[0], ScenesBehaviorKind::Load);
    check_eq!(
        errors,
        provider.activation_order[1],
        ScenesBehaviorKind::ShowTimestamps
    );
    check_eq!(errors, provider.state().scenes.len(), 1);

    assert_no_errors(errors);

    provider.deactivate();
}
