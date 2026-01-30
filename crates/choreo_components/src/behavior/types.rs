use rxrust::prelude::SubscriptionLike;

pub trait Disposable {
    fn dispose(&mut self);
}

#[derive(Default)]
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
    fn activate(&self, _view_model: &mut T, _disposables: &mut CompositeDisposable)
    {
    }
}

pub struct SubscriptionDisposable<T>
where
    T: SubscriptionLike,
{
    subscription: T,
}

impl<T> SubscriptionDisposable<T>
where
    T: SubscriptionLike,
{
    pub fn new(subscription: T) -> Self
    {
        Self { subscription }
    }
}

impl<T> Disposable for SubscriptionDisposable<T>
where
    T: SubscriptionLike,
{
    fn dispose(&mut self)
    {
        self.subscription.unsubscribe();
    }
}

pub struct TimerDisposable {
    timer: slint::Timer,
}

impl TimerDisposable {
    pub fn new(timer: slint::Timer) -> Self
    {
        Self { timer }
    }
}

impl Disposable for TimerDisposable {
    fn dispose(&mut self)
    {
        self.timer.stop();
    }
}
