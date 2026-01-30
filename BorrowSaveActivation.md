# Borrow-Safe Behavior Activation Options

This document shows refactor examples for each option to avoid `RefCell` borrow conflicts when behaviors subscribe to reactive properties.

Each option includes:
- Example view model shape
- Example behavior activation
- Pros / Cons

All code is illustrative and uses Allman braces.

---

## Option A: Two-Phase Activation With Pre-Cloned Signals (Preferred)

### ViewModel Example

```rust
use rxrust::prelude::LocalSubject;

pub struct DancerSettingsSignals {
    pub selected_dancer_changed: LocalSubject<'static, (), ()>,
    pub selected_role_changed: LocalSubject<'static, (), ()>,
}

pub struct DancerSettingsViewModel {
    signals: DancerSettingsSignals,
    disposables: CompositeDisposable,
    pub selected_dancer: Option<Rc<DancerModel>>,
    pub selected_role: Option<Rc<RoleModel>>,
}

impl DancerSettingsViewModel {
    pub fn new() -> Self
    {
        Self {
            signals: DancerSettingsSignals {
                selected_dancer_changed: LocalSubject::new(),
                selected_role_changed: LocalSubject::new(),
            },
            disposables: CompositeDisposable::new(),
            selected_dancer: None,
            selected_role: None,
        }
    }

    pub fn signals(&self) -> DancerSettingsSignals
    {
        DancerSettingsSignals {
            selected_dancer_changed: self.signals.selected_dancer_changed.clone(),
            selected_role_changed: self.signals.selected_role_changed.clone(),
        }
    }

    pub fn set_selected_dancer(&mut self, value: Option<Rc<DancerModel>>)
    {
        self.selected_dancer = value;
        self.signals.selected_dancer_changed.next(());
    }

    pub fn set_selected_role(&mut self, value: Option<Rc<RoleModel>>)
    {
        self.selected_role = value;
        self.signals.selected_role_changed.next(());
    }
}
```

### Behavior Example

```rust
use rxrust::observable::SubscribeNext;

pub trait Behavior<T> {
    fn activate(&self, view_model: &mut T, disposables: &mut CompositeDisposable);
    fn bind(&self, signals: DancerSettingsSignals, disposables: &mut CompositeDisposable);
}

pub struct SelectedRoleBehavior;

impl Behavior<DancerSettingsViewModel> for SelectedRoleBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        SelectedRoleBehavior::update_selected_role(view_model);
    }

    fn bind(&self, signals: DancerSettingsSignals, disposables: &mut CompositeDisposable)
    {
        let subscription = signals.selected_role_changed.subscribe(move |_| {
            // borrow happens at call site, not here
        });
        disposables.add(Box::new(SubscriptionDisposable::new(subscription)));
    }
}
```

### Pros / Cons

- Pros: avoids borrow conflicts structurally; keeps rxrust; easy to reason about.
- Cons: adds a signals struct and an extra `bind` step in wiring.

---

## Option B: Activate Behaviors Before Adapter Attaches

### ViewModel Wiring Example

```rust
pub fn build_dancer_settings_view_model(
    behaviors: Vec<Box<dyn Behavior<DancerSettingsViewModel>>>,
) -> Rc<RefCell<DancerSettingsViewModel>> {
    let view_model = Rc::new(RefCell::new(DancerSettingsViewModel::new()));
    {
        let mut vm = view_model.borrow_mut();
        vm.activate_behaviors(behaviors);
    }

    // Adapter attaches after behaviors are activated.
    view_model
}
```

### Behavior Example (unchanged)

```rust
impl Behavior<DancerSettingsViewModel> for SelectedRoleBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        let signals = view_model.signals();
        let subscription = signals.selected_role_changed.subscribe(move |_| { });
        disposables.add(Box::new(SubscriptionDisposable::new(subscription)));
    }
}
```

### Pros / Cons

- Pros: smallest code change; keeps API as-is.
- Cons: brittle ordering; regressions likely if activation happens under a borrow again.

---

## Option C: Event Hub With Crossbeam Channels

### Event Hub Example

```rust
use crossbeam_channel::{Receiver, Sender};

pub enum DancerSettingsEvent {
    SelectedDancerChanged,
    SelectedRoleChanged,
}

pub struct DancerSettingsEventHub {
    pub sender: Sender<DancerSettingsEvent>,
    pub receiver: Receiver<DancerSettingsEvent>,
}

impl DancerSettingsEventHub {
    pub fn new() -> Self
    {
        let (sender, receiver) = crossbeam_channel::unbounded();
        Self { sender, receiver }
    }
}
```

### ViewModel Example

```rust
pub struct DancerSettingsViewModel {
    events: DancerSettingsEventHub,
    disposables: CompositeDisposable,
}

impl DancerSettingsViewModel {
    pub fn new(events: DancerSettingsEventHub) -> Self
    {
        Self {
            events,
            disposables: CompositeDisposable::new(),
        }
    }

    pub fn set_selected_role(&mut self, value: Option<Rc<RoleModel>>)
    {
        self.selected_role = value;
        let _ = self.events.sender.send(DancerSettingsEvent::SelectedRoleChanged);
    }
}
```

### Behavior Example

```rust
pub struct SelectedRoleBehavior {
    receiver: Receiver<DancerSettingsEvent>,
}

impl Behavior<DancerSettingsViewModel> for SelectedRoleBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
        disposables: &mut CompositeDisposable,
    ) {
        let receiver = self.receiver.clone();
        let vm_handle = view_model.self_handle();
        let timer = slint::Timer::default();
        timer.start(
            slint::TimerMode::Repeated,
            std::time::Duration::from_millis(16),
            move || {
                let Some(vm) = vm_handle.as_ref().and_then(|h| h.upgrade()) else {
                    return;
                };
                for event in receiver.try_iter() {
                    if let DancerSettingsEvent::SelectedRoleChanged = event {
                        let mut vm = vm.borrow_mut();
                        SelectedRoleBehavior::update_selected_role(&mut vm);
                    }
                }
            },
        );
        disposables.add(Box::new(TimerDisposable::new(timer)));
    }
}
```

### Pros / Cons

- Pros: no borrow conflicts during subscription; clean separation of events.
- Cons: more plumbing; event loop or timer needed; slightly higher latency.

---

## Option D: Split Activate Into `initialize` and `bind`

### Trait Example

```rust
pub trait Behavior<T> {
    fn initialize(&self, view_model: &mut T, disposables: &mut CompositeDisposable);
    fn bind(&self, view_model: &T, disposables: &mut CompositeDisposable);
}
```

### Behavior Example

```rust
impl Behavior<DancerSettingsViewModel> for SelectedRoleBehavior {
    fn initialize(
        &self,
        view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        SelectedRoleBehavior::update_selected_role(view_model);
    }

    fn bind(&self, view_model: &DancerSettingsViewModel, disposables: &mut CompositeDisposable)
    {
        let subscription = view_model
            .signals()
            .selected_role_changed
            .subscribe(move |_| { });
        disposables.add(Box::new(SubscriptionDisposable::new(subscription)));
    }
}
```

### Pros / Cons

- Pros: clean separation of mutation vs subscription; fewer surprises.
- Cons: API churn across all behaviors; requires new wiring code.

---

## Option E: Defer Activation to the Event Loop

### Wiring Example

```rust
pub fn activate_later(view_model: Rc<RefCell<DancerSettingsViewModel>>, behaviors: Vec<Box<dyn Behavior<DancerSettingsViewModel>>>) {
    slint::Timer::single_shot(std::time::Duration::from_millis(0), move || {
        let mut vm = view_model.borrow_mut();
        vm.activate_behaviors(behaviors);
    });
}
```

### Pros / Cons

- Pros: tiny change; avoids immediate borrow conflict.
- Cons: timing-sensitive; harder to reason about; can mask ordering bugs.

