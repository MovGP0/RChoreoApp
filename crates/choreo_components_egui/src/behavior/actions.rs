pub enum BehaviorAction {
    Initialize,
    AddDisposable {
        disposable: Box<dyn super::state::Disposable>,
    },
    DisposeAll,
}
