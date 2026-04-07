#![allow(clippy::bool_assert_comparison)]

use std::cell::Cell;
use std::rc::Rc;

use choreo_components::behavior::Behavior;
use choreo_components::behavior::CompositeDisposable;
use choreo_components::behavior::Disposable;
use choreo_components::behavior::TimerDisposable;

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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
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

#[derive(Default)]
struct StubViewModel {
    activated: bool,
}

struct StubBehavior;

impl Behavior<StubViewModel> for StubBehavior {
    fn activate(&self, view_model: &mut StubViewModel, _disposables: &mut CompositeDisposable) {
        view_model.activated = true;
    }
}

struct CountingDisposable {
    dispose_count: Rc<Cell<usize>>,
}

impl CountingDisposable {
    fn new(dispose_count: Rc<Cell<usize>>) -> Self {
        Self { dispose_count }
    }
}

impl Disposable for CountingDisposable {
    fn dispose(&mut self) {
        self.dispose_count.set(self.dispose_count.get() + 1);
    }
}

#[test]
fn dispose_all_disposes_every_registered_item_once() {
    let first_count = Rc::new(Cell::new(0));
    let second_count = Rc::new(Cell::new(0));

    let mut disposables = CompositeDisposable::new();

    disposables.add(Box::new(CountingDisposable::new(Rc::clone(&first_count))));
    disposables.add(Box::new(CountingDisposable::new(Rc::clone(&second_count))));

    let mut errors = Vec::new();

    check_eq!(errors, disposables.len(), 2);

    disposables.dispose_all();

    check_eq!(errors, first_count.get(), 1);
    check_eq!(errors, second_count.get(), 1);
    check!(errors, disposables.is_empty());

    assert_no_errors(errors);
}

#[test]
fn timer_disposable_invokes_stop_on_dispose() {
    let dispose_count = Rc::new(Cell::new(0));
    let dispose_count_for_timer = Rc::clone(&dispose_count);

    let mut timer = TimerDisposable::new(move || {
        dispose_count_for_timer.set(dispose_count_for_timer.get() + 1);
    });
    timer.dispose();

    assert_eq!(dispose_count.get(), 1);
}

#[test]
fn behavior_activation_uses_lifecycle_contract() {
    let behavior = StubBehavior;
    let mut view_model = StubViewModel::default();
    let mut disposables = CompositeDisposable::new();

    behavior.activate(&mut view_model, &mut disposables);

    assert!(view_model.activated);
}
