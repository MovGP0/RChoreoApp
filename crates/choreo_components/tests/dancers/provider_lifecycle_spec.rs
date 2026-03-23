use choreo_components::behavior::Behavior;
use choreo_components::behavior::CompositeDisposable;
use choreo_components::behavior::Disposable;
use choreo_components::dancers::DancersProvider;
use choreo_components::dancers::actions::DancersAction;
use choreo_components::dancers::state::DancersState;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

struct CounterDisposable {
    counter: Arc<AtomicUsize>,
}

impl Disposable for CounterDisposable {
    fn dispose(&mut self) {
        self.counter.fetch_add(1, Ordering::SeqCst);
    }
}

struct LifecycleBehavior {
    activate_counter: Arc<AtomicUsize>,
    dispose_counter: Arc<AtomicUsize>,
}

impl Behavior<DancersState> for LifecycleBehavior {
    fn activate(&self, _view_model: &mut DancersState, disposables: &mut CompositeDisposable) {
        self.activate_counter.fetch_add(1, Ordering::SeqCst);
        disposables.add(Box::new(CounterDisposable {
            counter: Arc::clone(&self.dispose_counter),
        }));
    }
}

#[test]
fn provider_activates_behaviors_and_disposes_on_drop() {
    let activate_counter = Arc::new(AtomicUsize::new(0));
    let dispose_counter = Arc::new(AtomicUsize::new(0));
    {
        let mut provider = DancersProvider::new(vec![Box::new(LifecycleBehavior {
            activate_counter: Arc::clone(&activate_counter),
            dispose_counter: Arc::clone(&dispose_counter),
        })]);
        provider.dispatch(DancersAction::LoadFromGlobal);
        provider.reload();
    }

    assert_eq!(activate_counter.load(Ordering::SeqCst), 1);
    assert_eq!(dispose_counter.load(Ordering::SeqCst), 1);
}
