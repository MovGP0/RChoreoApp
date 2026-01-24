use std::fmt::Debug;

pub trait Disposable: Debug {
    fn dispose(&mut self);
}

#[derive(Debug, Default)]
pub struct CompositeDisposable {
    items: Vec<Box<dyn Disposable>>,
}

impl CompositeDisposable {
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
}

pub trait Behavior<T> {
    fn activate(&self, view_model: &mut T, disposables: &mut CompositeDisposable);
}
