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

    pub fn add(&mut self, item: Box<dyn Disposable>) {
        self.items.push(item);
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

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

pub trait Behavior<T> {
    fn activate(&self, _view_model: &mut T, _disposables: &mut CompositeDisposable) {}
}

pub struct TimerDisposable {
    stop: Box<dyn FnMut()>,
}

impl TimerDisposable {
    pub fn new<F>(stop: F) -> Self
    where
        F: FnMut() + 'static,
    {
        Self {
            stop: Box::new(stop),
        }
    }
}

impl Disposable for TimerDisposable {
    fn dispose(&mut self) {
        (self.stop)();
    }
}
