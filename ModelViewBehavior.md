# Model View Behavior Pattern

We are using the "Model View Behavior" Pattern (`MVB`) for the UI.
The components are structured into the following parts:

**ViewModel**: 
- This part acts as an intermediary between the Model and the View, handling data presentation logic and user interactions.

**View**:
- This part is responsible for the UI representation and user interface elements.
- The View binds to the ViewModel to display data and capture user input.

**Adapter**:
- This part serves as a bridge between the View and the ViewModel, facilitating data binding and event handling.

**Behavior**:
- This part encapsulates the logic that defines how the ViewModel and View interact, managing state changes and user actions.
- The behaviors are associated with the ViewModel to handle specific functionalities.

**Model**:
- Models represent the shared state of the application.

We also have additional components:

**MessageBus**
- This part facilitates communication between different parts of the application using a message-passing system.

**StateMachine**: 
- This part manages the different states of the application and transitions between them.
- The transitions are triggered by trigger events, which may or may not cause a state transition.

## ViewModels
- ViewModels only know about Models and Behaviors.
- ViewModels get the behaviors injected, but do not know any specific behavior.
- ViewModels do not know about Views or Adapters.
- do not make injected behaviors mutable and use `behaviors.drain(..)` for enumeration; use `behaviors.iter()` for non-mutable access.

## Behaviors

- Behaviors are reusable components that encapsulate specific functionalities or interactions.
- They react to property or state changes in the ViewModel, the Model, or global events and execute business logic.
- Behaviors implement the `Behavior<TViewModel>` trait and are registered in the dependency injection container using `nject`.
- Behaviors are linked to a specific ViewModel instance using `activate(&mut view_model, &mut disposables)`.
- When the disposable is disposed of, the behavior then no longer reacts to changes in the ViewModel or events.
- Behaviors may not have a ViewModel in the constructor; only shared models are allowed.
- The behavior is linked with the ViewModel during activation.
- Activate behaviors before attaching the View-ViewModel adapter because the adapter borrows the ViewModel mutably.
- Do not borrow the ViewModel inside subscription callbacks; the adapter is still attached with a mutable borrow.

**Behavior trait**
```rust
pub trait Behavior<T> {
    fn activate(&self, _view_model: &mut T, _disposables: &mut CompositeDisposable)
    {
    }
}
```

## Messages

- Messages are published and received using [crossbeam-channel](https://crates.io/crates/crossbeam-channel),
  allowing behaviors to communicate with each other and with other parts of the application.
- Messages can be Events, Commands, Queries, or Responses, depending on their purpose and usage.
- Prefer Sender/Receiver for message passing instead of global state or callbacks.
- Prefer bounded channels to avoid unbounded memory growth

Example:
```rust
use crossbeam_channel::{Sender, Receiver, unbounded, bounded};
```

## Shared Global State (Actor Model)

We use an actor-style wrapper for shared global state to avoid borrow conflicts and to keep
mutations centralized.

**Key points**
- The shared state is owned by `GlobalStateActor`, created by `GlobalProvider`.
- Read access uses `try_with_state` (non-blocking, returns `Option<T>`).
- Write access uses `try_update` (non-blocking, returns `bool`).
- For queued work, use `dispatch` to push mutations onto the actor queue.
- Behaviors should prefer `GlobalStateActor` over direct `Rc<RefCell<GlobalStateModel>>` access.

**Read example**
```rust
let Some(snapshot) = global_state_actor.try_with_state(|state| {
    (state.selected_scene.clone(), state.choreography.clone())
}) else {
    return;
};
```

**Write example**
```rust
let updated = global_state_actor.try_update(|state| {
    state.interaction_mode = InteractionMode::Move;
});
if !updated {
    return;
}
```

**Queued update example**
```rust
global_state_actor.dispatch(|state| {
    state.choreography.settings.show_timestamps = true;
});
```

## Slint Views and Adapters

Slint properties belong to the View. They are a UI-runtime reactive state and should not be used as ViewModel state.
The Adapter is the only layer that is allowed to know about both the Slint view and the ViewModel.

**Rules of thumb**
- Business state lives in Rust (Model / ViewModel), not in Slint properties.
- Slint properties are for view state (layout, selection, transient UI flags).
- The Adapter pushes state into the view and translates callbacks back into commands.
- Use Slint callbacks for user intent and keep data flow explicit.

**Minimal data flow**
```
Model <-- Behavior --> ViewModel <-- Adapter --> Slint View
```

## Example

**ViewModel**
```rust
pub struct MainViewModel {
    disposables: CompositeDisposable,
    behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
}

#[injectable]
impl MainViewModel {
    pub fn new(behaviors: Vec<Box<dyn Behavior<MainViewModel>>>) -> Self {
        let mut view_model = Self {
            disposables: CompositeDisposable::new(),
            behaviors,
        };

        for behavior in behaviors.iter() {
            behavior.activate(&mut view_model, &mut view_model.disposables);
        }

        view_model
    }
}

impl Drop for MainViewModel {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}
```

**Activatable ViewModel**
```rust
pub struct MainViewModel {
    // no disposables here, or keep it but don't pass &mut to behaviors
    behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
}

#[injectable]
impl MainViewModel {
    pub fn new(behaviors: Vec<Box<dyn Behavior<MainViewModel>>>) -> Self {
        Self { behaviors }
    }

    pub fn activate(&mut self, disposables: &mut CompositeDisposable) {
        for behavior in behaviors.iter() {
            behavior.activate(self, disposables);
        }
    }
}
```

**Behavior**
- Behaviors need to be registered in the DI container as `Behavior<TViewModel>` type.
- The only public methods are the constructor and the `activate` method. All other methods are private.
- Behaviors are not created by calling the constructor directly, but are injected into the ViewModel.
- When a behavior need to inform another behavior, it can use a shared Model or a MessageBus (e.g. using `crossbeam-channel`).

```rust
#[export(Box<dyn Behavior<MainViewModel>>, Box::new(self.behavior))]
struct MainViewModelLoggingBehavior;
impl Behavior<MainViewModel> for MainViewModelLoggingBehavior {
    fn activate(&self, view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("MainViewModelLoggingBehavior", "MainViewModel"); // log the activation
        let log_subscription = view_model.property_changed_channel().subscribe(move |property_name| {
            println!("Property changed: {}", property_name);
        });
        disposables.add(log_subscription);
    }
}
```

### Example Usage

Example with ViewModel:
```rust
fn main()
{
    #[provider]
    struct InitProvider;

    let provider = InitProvider.provide::<Provider>();
    let mainViewModel = provider.resolve::<MainViewModel>();
    // when mainViewModel goes out of scope, its Drop implementation will dispose all disposables
}
```

Example with Activatable ViewModel:
```rust
fn main()
{
    #[provider]
    struct InitProvider;

    let provider = InitProvider.provide::<Provider>();

    let mut activations = CompositeDisposable::new();
    let mut main_view_model = provider.resolve::<MainViewModel>();
    main_view_model.activate(&mut activations);

    // later, when the view model is no longer needed
    activations.dispose_all();
}
```

## Crossbeam Channel ViewModel and Slint Adapter Example

This example keeps the Model and ViewModel independent of Slint and uses an Adapter to bind to the view.
The ViewModel receives user events and publishes state updates through `crossbeam-channel`.

**Slint view**
- UI owns properties + callbacks
```slint
export component AppWindow {
    in-out property<string> title;
    in-out property<int> counter;

    callback increment_clicked();
    callback title_edited(string);

    VerticalLayout {
        Text { text: "Title:"; }
        LineEdit {
            text <=> root.title;
            edited(text) => { root.title_edited(text); }
        }

        Text { text: "Counter: " + root.counter; }

        Button {
            text: "Increment";
            clicked => { root.increment_clicked(); }
        }
    }
}
```

**Model**
- pure Rust
```rust
#[derive(Debug, Clone)]
pub struct DomainModel
{
    title: String,
    counter: i32,
}

impl DomainModel
{
    pub fn new() -> Self
    {
        Self {
            title: "Hello".to_owned(),
            counter: 0,
        }
    }

    pub fn title(&self) -> &str
    {
        &self.title
    }

    pub fn counter(&self) -> i32
    {
        self.counter
    }

    pub fn set_title(&mut self, new_title: String)
    {
        self.title = new_title;
    }

    pub fn increment(&mut self)
    {
        self.counter += 1;
    }
}
```

**ViewModel**
- crossbeam channels
- no Slint types
```rust
use crossbeam_channel::{Receiver, Sender};

pub enum ViewEvent
{
    Increment,
    TitleEdited(String),
}

#[derive(Debug, Clone)]
pub struct ViewState
{
    pub title: String,
    pub counter: i32,
}

pub struct AppViewModel
{
    model: DomainModel,
    event_rx: Receiver<ViewEvent>,
    state_tx: Sender<ViewState>,
}

impl AppViewModel
{
    pub fn new(
        model: DomainModel,
        event_rx: Receiver<ViewEvent>,
        state_tx: Sender<ViewState>,
    ) -> Self
    {
        Self
        {
            model,
            event_rx,
            state_tx,
        }
    }

    pub fn run(mut self)
    {
        self.publish_state();

        for event in self.event_rx.iter()
        {
            match event
            {
                ViewEvent::Increment => self.model.increment(),
                ViewEvent::TitleEdited(new_title) => self.model.set_title(new_title),
            }

            self.publish_state();
        }
    }

    fn publish_state(&self)
    {
        let _ = self.state_tx.send(ViewState
        {
            title: self.model.title().to_owned(),
            counter: self.model.counter(),
        });
    }
}
```

**Adapter**
- binds UI and ViewModel
```rust
use crossbeam_channel::{Receiver, Sender};

use crate::ui::AppWindow;

pub struct AppAdapter
{
    ui: AppWindow,
    event_tx: Sender<ViewEvent>,
    state_rx: Receiver<ViewState>,
}

impl AppAdapter
{
    pub fn new(ui: AppWindow, event_tx: Sender<ViewEvent>, state_rx: Receiver<ViewState>) -> Self
    {
        Self
        {
            ui,
            event_tx,
            state_rx,
        }
    }

    pub fn wire_up(&self)
    {
        // ViewModel -> View
        {
            let ui_weak = self.ui.as_weak();
            let state_rx = self.state_rx.clone();

            std::thread::spawn(move ||
            {
                for state in state_rx.iter()
                {
                    let ui_weak = ui_weak.clone();

                    let _ = slint::invoke_from_event_loop(move ||
                    {
                        if let Some(ui) = ui_weak.upgrade()
                        {
                            ui.set_title(state.title.into());
                            ui.set_counter(state.counter);
                        }
                    });
                }
            });
        }

        // View -> ViewModel
        {
            let event_tx = self.event_tx.clone();
            self.ui.on_increment_clicked(move ||
            {
                let _ = event_tx.send(ViewEvent::Increment);
            });

            let event_tx = self.event_tx.clone();
            self.ui.on_title_edited(move |new_title|
            {
                let _ = event_tx.send(ViewEvent::TitleEdited(new_title.to_string()));
            });
        }
    }
}
```

**Startup**
```rust
fn main() -> Result<(), slint::PlatformError>
{
    let ui = AppWindow::new()?;

    let (event_tx, event_rx) = crossbeam_channel::bounded::<ViewEvent>(32);
    let (state_tx, state_rx) = crossbeam_channel::bounded::<ViewState>(32);

    let view_model = AppViewModel::new(DomainModel::new(), event_rx, state_tx);
    std::thread::spawn(move || view_model.run());

    let adapter = AppAdapter::new(ui.clone(), event_tx, state_rx);
    adapter.wire_up();

    ui.run()
}
```

**Notes**
- If ViewModel updates can originate from worker threads, marshal updates to the Slint event loop
  (e.g., `slint::invoke_from_event_loop`).
- Avoid feedback loops: if you use `<=>` in Slint, avoid also setting the same property on every edit
  unless you deduplicate values.
