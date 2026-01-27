# Slint

## Slint UI Architecture (What Pattern Is This?)

Slint does **not** enforce MVC / MVP / MVVM explicitly.

What it provides instead is:

- Declarative, reactive UI (`.slint`)
- Properties + bindings
- Callbacks into host language (Rust, C++, …)

In practice, most Slint applications end up **MVVM-like**:

| Role      | In Slint                                          |
| --------- | ------------------------------------------------- |
| View      | `.slint` components                               |
| ViewModel | `global` or root component properties + callbacks |
| Model     | Rust domain structs, services, persistence        |

Key difference to classic MVVM:
- No `INotifyPropertyChanged`
- No observers
- Dependency tracking is automatic

## Property Directions (Critical)

| Property kind | Who writes       | Who reads | Typical use           |
| ------------- | ---------------- | --------- | --------------------- |
| `in`          | Parent / binding | Component | Inputs, configuration |
| `out`         | Component        | Parent    | Outputs, signals      |
| `in-out`      | Both             | Both      | Shared state          |

Slint **enforces directionality at compile time**.

## Binding vs Assignment (Most Common Pitfall)

### Reactive binding (live, automatic)
```slint
child.value: parent.count;
````

* Establishes a **data binding**
* Updates propagate automatically
* This is the default and recommended approach

### One-time assignment (no updates)

```slint
child.value := parent.count;
```

* Copies the value once
* No further updates
* Rarely needed

**Rule of thumb**

| Operator | Meaning             |
| -------- | ------------------- |
| `:`      | Reactive binding    |
| `=`      | One-time assignment |
| `<=>`    | Two-way binding     |

## `out → in` Is Fully Data-Bound

Given:
```slint
out property<int> count;
Child {
    value: count;
}
```

* `Child.value` is **live-bound** to `count`
* No manual updates required
* No Rust glue code required
* Binding is removed automatically if the child is destroyed

This is **not** a copy.

## How Change Notification Works (Internals)

* `Property<T>` is a **reactive cell**
* Slint tracks **which UI expressions read which properties**
* When a property changes:
    * Dependents are invalidated
    * Only affected bindings/layout/paint nodes recompute
* No polling
* No full redraws

Important implications:

* Equality (`PartialEq`) matters
* Mutating interior data does **not** trigger updates
* Replace values or use `Model` for collections

## Properties vs Models

### Use `Property<T>` when:

* Value is scalar or small struct
* Replaced as a whole

### Use `Model` (`VecModel`, custom) when:

* Lists, tables, collections
* Incremental updates (`row_added`, `row_removed`)
* Large data sets

Mutating a `Vec` inside a property does **not** notify the UI.

## No Property Change Events (By Design)

Slint intentionally does **not** expose:

```rust
property.on_changed(...)
```

Reasons:

* Prevents observer ordering bugs
* Keeps reactive graph deterministic
* Enforces one-way data flow

If logic must run when state changes:

* Run it **before** setting the property
* Or **after** setting the property, explicitly, in Rust

## Dynamic / Responsive UI (Standard Pattern)

Do **not** replace the UI from Rust.

Instead:

1. Keep one root `Window`
2. Compute breakpoint from window size
3. Conditionally instantiate sub-components
4. Route all state through root / `global`

Example:

```slint
if (AppVm.is_compact) : CompactView { }
else : RegularView { }
```

Bindings are created/destroyed automatically.

## Mental Model (Keep This)

> Slint does not observe properties.
> Properties observe the UI.

The UI declares dependencies; the runtime enforces them.

# Short Primer: How to Use Slint (End-to-End)

## Define UI in `.slint`

```slint
export global AppVm {
    in-out property<string> title;
    callback refresh();
}

export component AppWindow inherits Window {
    Text { text: AppVm.title; }
    Button {
        text: "Refresh";
        clicked => { AppVm.refresh(); }
    }
}
```

## Implement Logic in Rust

```rust
use slint::{ComponentHandle, SharedString};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let vm = ui.global::<AppVm>();

    vm.set_title(SharedString::from("Hello"));

    vm.on_refresh({
        let ui = ui.as_weak();
        move || {
            if let Some(ui) = ui.upgrade() {
                ui.global::<AppVm>()
                    .set_title(SharedString::from("Updated"));
            }
        }
    });

    ui.run()
}
```

## Key Rules to Follow

* Bind properties in `.slint`, not in Rust
* Use `global` or root component as ViewModel façade
* Never store references to conditional UI elements in Rust
* Prefer many small properties over one large state blob
* Use `Model` for collections

## When Things Go Wrong, Check This First

* Did you use `:` instead of `:=`?
* Did you mutate interior data instead of replacing it?
* Did you try to write to an `in` property?
* Did you try to observe a property instead of structuring logic explicitly?

## Color binding

Colors are defined as `out` properties:
```slint
export global MaterialPalette {
    out property<color> surface_container_lowest;
    // ...
}
```
So we can just bind them to an `in` or `in-out` property of a control:
```slint
in property<color> background: MaterialPalette.surface_container_low;
```

## Translations (Global Properties)

Use a global translation object with `in-out` string properties, then bind UI text directly.

```slint
export global Translations {
    in-out property<string> settings_title;
    in-out property<string> dark_mode_label;
    in-out property<string> app_title;
}
```

Bind in views:
```slint
import { Translations } from "translations.slint";

Text { text: Translations.app_title; }
```

Set values from Rust once at startup (or whenever the locale changes):
```rust
let translations = view.global::<Translations<'_>>();
translations.set_app_title("ChoreoApp".into());
```

## Material Palette Binding

Material colors are exposed as `out` properties on `MaterialPalette`. Bind them directly to `in`/`in-out` color properties to keep the UI reactive:

```slint
background: MaterialPalette.surface_container_low;
```
