#![allow(clippy::bool_assert_comparison)]

use std::cell::Cell;
use std::rc::Rc;

use crate::behavior_component::actions::BehaviorAction;
use crate::behavior_component::reducer::reduce;
use crate::behavior_component::state::{BehaviorState, Disposable};

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

    let mut state = BehaviorState::default();

    reduce(
        &mut state,
        BehaviorAction::AddDisposable {
            disposable: Box::new(CountingDisposable::new(Rc::clone(&first_count))),
        },
    );
    reduce(
        &mut state,
        BehaviorAction::AddDisposable {
            disposable: Box::new(CountingDisposable::new(Rc::clone(&second_count))),
        },
    );

    assert_eq!(state.disposable_count(), 2);

    reduce(&mut state, BehaviorAction::DisposeAll);

    assert_eq!(first_count.get(), 1);
    assert_eq!(second_count.get(), 1);
    assert_eq!(state.disposable_count(), 0);
}

#[test]
fn initialize_clears_registered_disposables() {
    let dispose_count = Rc::new(Cell::new(0));
    let mut state = BehaviorState::default();

    reduce(
        &mut state,
        BehaviorAction::AddDisposable {
            disposable: Box::new(CountingDisposable::new(Rc::clone(&dispose_count))),
        },
    );

    reduce(&mut state, BehaviorAction::Initialize);

    assert_eq!(dispose_count.get(), 1);
    assert_eq!(state.disposable_count(), 0);
}
