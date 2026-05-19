# Approach D: Tag-Based Enum Dispatch

## Concept

Don't use closures at all. Instead, use a central handler that matches on an enum to decide what to do. This is closer to how AppKit works natively (IBAction). The button carries an action ID (via its `tag` property), and the app's delegate handles the action in a single method.

## How It Works

```
Button::new("Add").with_id(Action::AddTemplate)
    → button.setTag(Action::AddTemplate as NSInteger)
    → button.setTarget(Some(delegate))
    → button.setAction(sel!(handleAction:))
    → user clicks → AppKit sends handleAction: to delegate
    → delegate matches sender.tag → Action::AddTemplate → add_new_template()
```

## Code Sketch

### Define an action enum

```rust
#[repr(NSInteger)]
pub enum Action {
    AddTemplate = 1,
    RemoveTemplate = 2,
    Save = 3,
    // ...
}
```

### Central handler (could be on Application, Window, or a separate Delegate)

```rust
use objc2::define_class;

define_class!(
    #[unsafe(super(NSResponder))]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<MyAppData>]
    #[name = "MyAppDelegate"]
    struct MyAppDelegate;

    impl MyAppDelegate {
        #[unsafe(method(handleAction:))]
        fn handle_action(&self, sender: Option<&AnyObject>) {
            let sender = sender.expect("nil sender");
            let action: Action = unsafe { msg_send![sender, tag] }.into();
            let mut data = self.ivars().borrow_mut();
            match action {
                Action::AddTemplate => data.add_template(),
                Action::RemoveTemplate => data.remove_template(),
                Action::Save => data.save(),
            }
        }
    }
);
```

### Button API

```rust
pub struct Button {
    pub(crate) button: Retained<NSButton>,
}

impl Button {
    pub fn with_id(mut self, action: Action) -> Self {
        self.button.setTag(action as NSInteger);
        self
    }

    /// Set the target for this button to the given object.
    /// The target should respond to `sel!(handleAction:)`.
    pub fn target(mut self, target: &AnyObject) -> Self {
        unsafe { self.button.setTarget(Some(target)) };
        unsafe { self.button.setAction(Some(sel!(handleAction:))) };
        self
    }
}
```

### User code

```rust
let delegate = MyAppDelegate::new(MyAppData { ... });

Button::new("+")
    .with_id(Action::AddTemplate)
    .target(delegate.as_ref())
    .set_pos(100., 100.);
```

## Pros

- **No closures or allocations** — purely integer dispatch, zero overhead
- **Familiar Cocoa pattern** — this is exactly how `IBAction` and `NSMenuItem` work
- **Full control in one place** — all actions are visible in a single `match` block
- **Type-safe** — the enum guarantees handled cases at compile time
- **Works with state** — the delegate owns `&mut T` directly

## Cons

- **Not a builder-friendly `.on_press()`** — requires a separate `.target()` call and an external delegate object
- **Boilerplate** — every new action needs an enum variant and a match arm
- **No per-instance customization** — two buttons with the same `Action` always do the same thing (unless you also match on the sender pointer)
- **Separate delegate class per app** — the user must define an ObjC class for their delegate, which requires `define_class!`

## When To Use

Best for apps with a fixed set of known actions (toolbar buttons, menu items, etc.). Overkill for dynamic UIs where buttons are created and configured at runtime. Works well as a complementary mechanism alongside one of the closure-based approaches.
