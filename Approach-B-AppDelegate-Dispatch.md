# Approach B: App-Delegate Dispatch

## Concept

Route all button actions through a central target (the `Application` object). Each button stores a closure ID. When any button is pressed, `Application::handleAction:` is called, which looks up the sender's ID and dispatches the correct closure.

## How It Works

```
Button::new("Click").on_press(app, |data| data.do_thing())
    → app.register_callback(Box::new(|data| data.do_thing())) → returns ID
    → button.setTag(id as NSInteger)
    → button.setTarget(Some(app.as_ref()))
    → button.setAction(sel!(cacaoAction:))
    → user clicks → AppKit sends cacaoAction: to Application
    → Application::cacaoAction looks up sender's tag, finds closure, calls it
```

## Code Sketch

### Application holds a callback registry

```rust
use std::cell::RefCell;
use objc2::{define_class, sel};
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Sel};

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<AppIvars>]
    #[name = "CacaoApplication"]
    struct CacaoApplication;

    impl CacaoApplication {
        #[unsafe(method(cacaoAction:))]
        fn cacao_action(&self, sender: Option<&AnyObject>) {
            let sender = sender.expect("nil sender in action");
            let tag: NSInteger = unsafe { msg_send![sender, tag] };
            let mut ivars = self.ivars().borrow_mut();
            if let Some(cb) = ivars.callbacks.get_mut(&tag) {
                (cb)(&mut ivars.user_data);
            }
        }
    }
);

struct AppIvars<T> {
    callbacks: Vec<Option<Box<dyn FnMut(&mut T)>>>,
    next_id: NSInteger,
    user_data: T,
}

pub struct Application<T: 'static> {
    inner: Retained<CacaoApplication>,
}

impl<T> Application<T> {
    pub fn new(data: T) -> Self { /* ... */ }

    pub fn register_callback<F: FnMut(&mut T) + 'static>(&self, f: F) -> NSInteger {
        let mut ivars = self.inner.ivars().borrow_mut();
        let id = ivars.next_id;
        ivars.next_id += 1;
        ivars.callbacks.push(Some(Box::new(f)));
        id
    }
}
```

### Button stores an ID

```rust
pub struct Button {
    pub(crate) button: Retained<NSButton>,
    callback_id: NSInteger,
}

impl Button {
    pub fn on_press<F: FnMut(&mut T) + 'static>(
        mut self,
        app: &Application<T>,
        f: F,
    ) -> Self {
        self.callback_id = app.register_callback(Box::new(f));
        self.button.setTag(self.callback_id);
        unsafe {
            self.button.setTarget(Some(app.inner.as_ref()));
            self.button.setAction(Some(sel!(cacaoAction:)));
        }
        self
    }
}
```

### User code

```rust
let mut app = Application::new(MyData { count: 0 });

let btn = Button::new("+")
    .on_press(&app, |data: &mut MyData| data.count += 1);
```

## Pros

- **One target object** — only one extra ObjC object (the Application), regardless of button count
- **Can pass `&mut T`** — the closure receives mutable access to application data
- **Tag-based lookup** — simple integer dispatch, no extra ObjC allocations per button

## Cons

- **`Application` must be the target** — ties the action system to `Application`, making it harder to have independent windows/view controllers
- **Requires `&Application<T>` in `on_press`** — the API is less ergonomic than a standalone `.on_press(|| ...)`
- **Tag collision risk** — `NSInteger` tag must be unique across all controls
- **Closure registry must be cleaned up** — no automatic cleanup when a button is dropped (unless you add deregistration)
- **`Application`'s ivars become complex** — mixing `T`, callback storage, and future state in one struct

## When To Use

Good for simple apps with one window and a single data model. Avoid if you want modular windows, view controllers, or buttons that don't share a central data context.
