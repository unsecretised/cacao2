# Approach C: Thread-Local Registry

## Concept

Store all callbacks in a `thread_local!` map keyed by the button's `NSView` pointer. A single global Objective-C trampoline class looks up the sender's address and invokes the matching closure. No per-button ObjC objects needed.

## How It Works

```
Button::new("Click").on_press(|| println!("clicked"))
    → button.setTarget(GlobalTarget::shared())
    → button.setAction(sel!(globalAction:))
    → CALLBACKS.insert(&*button, Box::new(|| println!("clicked")))
    → user clicks → AppKit sends globalAction: to GlobalTarget
    → GlobalTarget::global_action gets sender, looks up &*sender in CALLBACKS
    → invokes the closure
```

## Code Sketch

### Thread-local callback map

```rust
use std::cell::RefCell;
use std::collections::HashMap;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject, Sel};

thread_local! {
    static CALLBACKS: RefCell<HashMap<*const AnyObject, Box<dyn FnMut()>>> =
        RefCell::new(HashMap::new());
}
```

### Shared trampoline class (one instance, registered once)

```rust
use objc2::define_class;
use objc2::rc::Retained;

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "CacaoGlobalTarget"]
    struct GlobalTarget;

    impl GlobalTarget {
        #[unsafe(method(globalAction:))]
        fn global_action(&self, sender: Option<&AnyObject>) {
            let sender = sender.expect("nil sender");
            let ptr: *const AnyObject = sender as *const AnyObject;
            CALLBACKS.with(|map| {
                let mut map = map.borrow_mut();
                if let Some(cb) = map.get_mut(&ptr) {
                    (cb)();
                }
            });
        }
    }
);

impl GlobalTarget {
    fn shared() -> &'static Self {
        static INSTANCE: once_cell::sync::Lazy<Retained<GlobalTarget>> =
            once_cell::sync::Lazy::new(|| unsafe {
                msg_send![GlobalTarget::alloc(), init]
            });
        &INSTANCE
    }
}
```

### Button registers callback

```rust
pub struct Button {
    pub(crate) button: Retained<NSButton>,
}

impl Button {
    pub fn new(text: &str) -> Self { /* ... */ }

    pub fn on_press<F: FnMut() + 'static>(mut self, f: F) -> Self {
        CALLBACKS.with(|map| {
            map.borrow_mut().insert(&*self.button as *const _ as *const AnyObject, Box::new(f));
        });
        unsafe {
            self.button.setTarget(Some(GlobalTarget::shared()));
            self.button.setAction(Some(sel!(globalAction:)));
        }
        self
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        CALLBACKS.with(|map| {
            map.borrow_mut().remove(&(&*self.button as *const _ as *const AnyObject));
        });
    }
}
```

### User code

```rust
Button::new("Click")
    .on_press(|| println!("works!"))
    .set_pos(100., 100.);
```

## Pros

- **No per-button ObjC objects** — single shared target for all buttons
- **Closure is `FnMut()`, not `FnMut(&mut T)`** — clean, no application reference needed
- **Automatic cleanup** — `Drop` impl removes the entry from the map
- **Simple API** — `.on_press(|| ...)` with no extra arguments
- **Building-block friendly** — can be extended to support `&mut T` by also storing a raw pointer to the data

## Cons

- **Global mutable state** — thread-local is better than a full static, but still introduces hidden mutable state
- **Safety of pointer keys** — if an object is deallocated and its address reused, a stale entry could fire on the wrong button. (Mitigated by `Drop` cleanup.)
- **`once_cell` / `LazyLock` dependency** — needed for the shared target singleton
- **One global action selector** — all buttons share the same selector, so you can't distinguish button types at the ObjC level
- **Closure is `'static`** — same limitation as Approach A

## When To Use

Good lightweight approach for apps that don't need application-level state in their callbacks. Best for simple "fire and forget" actions. The thread-local keeps it safer than a plain `static Mutex`.
