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

## Behaviors

- Behaviors are reusable components that encapsulate specific functionalities or interactions.
- They react to property or state changes in the ViewModel, the Model, or global events and execute business logic.
- Behaviors implement the `Behavior<TViewModel>` trait and are registered in the dependency injection container using `nject`.
- Behaviors are linked to a specific ViewModel instance using the `activate` method, which takes a mutable reference to the ViewModel and a `CompositeDisposable` for managing subscriptions and resources.
- When the disposable is disposed of, the behavior then no longer reacts to changes in the ViewModel or events.

## Messages

- Messages are published and received using [crossbeam-channel](https://crates.io/crates/crossbeam-channel),
  allowing behaviors to communicate with each other and with other parts of the application.
- Messages can be Events, Commands, Queries, or Responses, depending on their purpose and usage.

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

        for behavior in &view_model.behaviors {
            behavior.activate(&mut view_model);
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
        for behavior in &self.behaviors {
            behavior.activate(self, disposables);
        }
    }
}
```

**Behavior**
```rust
#[export(Box<dyn Behavior<MainViewModel>>, Box::new(self.behavior))]
struct MainViewModelLoggingBehavior;
impl Behavior<MainViewModel> for MainViewModelLoggingBehavior {
    fn activate(&self, view_model: &mut MainViewModel, disposables: &mut CompositeDisposable) {
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
    let mainViewModel = provider.resolve::<MainViewModel>();
    mainViewModel.activate(&mut activations);

    // later, when the view model is no longer needed
    activations.dispose_all();
}
```

## RxRust ViewModel and Slint Adapter Example

This example keeps the Model and ViewModel independent of Slint and uses an Adapter to bind to the view.
RxRust provides observable "properties" with a current value (BehaviorSubject style).

**Slint view**
- UI owns properties + callbacks
```slint
export component AppWindow {
    in-out property<string> title;
    in-out property<int> counter;

    callback increment_clicked();
    callback title_edited(string);

    VerticalLayout {
        Text { text: "Title:" }
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
- RxRust subjects
- no Slint types
```rust
use nject::inject;
use rxrust::prelude::*;
use rxrust::subject::LocalBehaviorSubject;
use std::cell::RefCell;

#[inject(|model: DomainModel| Self::from_model(model))]
pub struct AppViewModel
{
    model: RefCell<DomainModel>,

    title: RefCell<LocalBehaviorSubject<'static, String, ()>>,
    counter: RefCell<LocalBehaviorSubject<'static, i32, ()>>,
}

impl AppViewModel
{
    fn from_model(model: DomainModel) -> Self
    {
        let title_seed = model.title().to_owned();
        let counter_seed = model.counter();

        Self {
            model: RefCell::new(model),
            title: RefCell::new(LocalBehaviorSubject::<'static, _, ()>::new(title_seed)),
            counter: RefCell::new(LocalBehaviorSubject::<'static, _, ()>::new(counter_seed)),
        }
    }

    pub fn title_stream(&self) -> LocalBehaviorSubject<'static, String, ()>
    {
        self.title.borrow().clone()
    }

    pub fn counter_stream(&self) -> LocalBehaviorSubject<'static, i32, ()>
    {
        self.counter.borrow().clone()
    }

    pub fn set_title(&self, new_title: String)
    {
        self.model.borrow_mut().set_title(new_title.clone());
        self.title.borrow_mut().next(new_title);
    }

    pub fn increment(&self)
    {
        self.model.borrow_mut().increment();
        let new_value = self.model.borrow().counter();
        self.counter.borrow_mut().next(new_value);
    }
}
```

**Adapter**
- binds UI and ViewModel
```rust
use nject::injectable;
use std::rc::Rc;

use crate::ui::AppWindow;

#[injectable]
pub struct AppAdapter
{
    ui: AppWindow,
    view_model: Rc<AppViewModel>,
}

impl AppAdapter
{
    pub fn wire_up(&self)
    {
        // ViewModel -> View
        {
            let ui_weak = self.ui.as_weak();
            self.view_model.title_stream().subscribe(move |title_value| {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_title(title_value.into());
                }
            });

            let ui_weak = self.ui.as_weak();
            self.view_model.counter_stream().subscribe(move |counter_value| {
                if let Some(ui) = ui_weak.upgrade() {
                    ui.set_counter(counter_value);
                }
            });
        }

        // View -> ViewModel
        {
            let view_model = self.view_model.clone();
            self.ui.on_increment_clicked(move || view_model.increment());

            let view_model = self.view_model.clone();
            self.ui.on_title_edited(move |new_title| view_model.set_title(new_title.to_string()));
        }
    }
}
```

**Startup with nject**
```rust
use nject::provider;
use std::rc::Rc;

#[provider]
struct AppProvider
{
    #[provide]
    ui: AppWindow,

    #[provide]
    model: DomainModel,

    #[provide(Rc<AppViewModel>, |vm: AppViewModel| Rc::new(vm))]
    _vm_rc: (),
}

fn main() -> Result<(), slint::PlatformError>
{
    let ui = AppWindow::new()?;

    let provider = AppProvider {
        ui: ui.clone(),
        model: DomainModel::new(),
        _vm_rc: (),
    };

    let adapter: AppAdapter = provider.provide();
    adapter.wire_up();

    ui.run()
}
```

**Notes**
- If ViewModel updates can originate from worker threads, marshal updates to the Slint event loop
  (e.g., `slint::invoke_from_event_loop`).
- Avoid feedback loops: if you use `<=>` in Slint, avoid also setting the same property on every edit
  unless you deduplicate values.
