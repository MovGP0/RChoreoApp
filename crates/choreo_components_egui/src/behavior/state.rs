pub trait Disposable {
    fn dispose(&mut self);
}

#[derive(Default)]
pub struct CompositeDisposable {
    items: Vec<Box<dyn Disposable>>,
}

impl CompositeDisposable {
    #[must_use]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, disposable: Box<dyn Disposable>) {
        self.items.push(disposable);
    }

    pub fn dispose_all(&mut self) {
        for item in &mut self.items {
            item.dispose();
        }
        self.items.clear();
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[derive(Default)]
pub struct BehaviorState {
    disposables: CompositeDisposable,
}

impl BehaviorState {
    pub fn add_disposable(&mut self, disposable: Box<dyn Disposable>) {
        self.disposables.add(disposable);
    }

    pub fn dispose_all(&mut self) {
        self.disposables.dispose_all();
    }

    #[must_use]
    pub fn disposable_count(&self) -> usize {
        self.disposables.len()
    }
}
