# Model View Behavior Pattern

We are using the "Model View Behavior" Pattern (`MVB`) for the UI.
The components are structured into the following parts:

**ViewModel**: 
- This part acts as an intermediary between the Model and the View, handling data presentation logic and user interactions.

**View**:
- This part is responsible for the UI representation and user interface elements.
- The View binds to the ViewModel to display data and capture user input.

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
