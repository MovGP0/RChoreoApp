use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::{Rc, Weak};
use std::time::Duration;

use slint::{Timer, TimerMode};

use super::GlobalStateModel;

type GlobalStateCommand = Box<dyn FnOnce(&mut GlobalStateModel) + 'static>;

pub struct GlobalStateActor {
    state: Rc<RefCell<GlobalStateModel>>,
    queue: RefCell<VecDeque<GlobalStateCommand>>,
    subscribers: RefCell<Vec<Weak<dyn Fn()>>>,
    _timer: Timer,
}

impl GlobalStateActor {
    pub fn new() -> Rc<Self> {
        let store = Rc::new(Self {
            state: Rc::new(RefCell::new(GlobalStateModel::default())),
            queue: RefCell::new(VecDeque::new()),
            subscribers: RefCell::new(Vec::new()),
            _timer: Timer::default(),
        });

        let store_weak = Rc::downgrade(&store);
        store
            ._timer
            .start(TimerMode::Repeated, Duration::from_millis(16), move || {
                if let Some(store) = store_weak.upgrade() {
                    store.drain();
                }
            });

        store
    }

    pub fn state_handle(&self) -> Rc<RefCell<GlobalStateModel>> {
        Rc::clone(&self.state)
    }

    pub fn try_with_state<T>(&self, handler: impl FnOnce(&GlobalStateModel) -> T) -> Option<T> {
        self.state.try_borrow().ok().map(|state| handler(&state))
    }

    pub fn try_update(&self, handler: impl FnOnce(&mut GlobalStateModel)) -> bool {
        let Ok(mut state) = self.state.try_borrow_mut() else {
            return false;
        };
        handler(&mut state);
        true
    }

    pub fn dispatch<F>(&self, command: F)
    where
        F: FnOnce(&mut GlobalStateModel) + 'static,
    {
        self.queue.borrow_mut().push_back(Box::new(command));
    }

    pub fn subscribe(&self, handler: Rc<dyn Fn()>) {
        self.subscribers.borrow_mut().push(Rc::downgrade(&handler));
    }

    fn drain(&self) {
        let mut queue = self.queue.borrow_mut();
        if queue.is_empty() {
            return;
        }

        while let Some(command) = queue.pop_front() {
            let mut state = self.state.borrow_mut();
            command(&mut state);
        }

        let mut subscribers = self.subscribers.borrow_mut();
        subscribers.retain(|handler| handler.upgrade().is_some());
        for handler in subscribers.iter() {
            if let Some(handler) = handler.upgrade() {
                handler();
            }
        }
    }
}
