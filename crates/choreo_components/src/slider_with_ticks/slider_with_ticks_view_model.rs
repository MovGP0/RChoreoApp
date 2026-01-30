use crate::behavior::{Behavior, CompositeDisposable};
use nject::injectable;
use slint::Color;

#[injectable]
#[inject(|behaviors: Vec<Box<dyn Behavior<SliderWithTicksViewModel>>>| Self::new(behaviors))]
pub struct SliderWithTicksViewModel {
    pub minimum: f64,
    pub maximum: f64,
    pub value: f64,
    pub tick_values: Vec<f64>,
    pub tick_color: Option<Color>,
    pub is_enabled: bool,
    disposables: CompositeDisposable,
}

impl SliderWithTicksViewModel {
    pub fn new(mut behaviors: Vec<Box<dyn Behavior<SliderWithTicksViewModel>>>) -> Self {
        let mut view_model = Self {
            minimum: 0.0,
            maximum: 1.0,
            value: 0.0,
            tick_values: Vec::new(),
            tick_color: None,
            is_enabled: true,
            disposables: CompositeDisposable::new(),
        };

        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors.drain(..) {
            behavior.initialize(&mut view_model, &mut disposables);
        }

        view_model.disposables = disposables;
        view_model
    }

    pub fn set_range(&mut self, minimum: f64, maximum: f64) {
        self.minimum = minimum;
        self.maximum = maximum;
        self.value = self.value.clamp(self.minimum, self.maximum);
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(self.minimum, self.maximum);
    }

    pub fn dispose(&mut self) {
        self.disposables.dispose_all();
    }
}

impl Default for SliderWithTicksViewModel {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}
