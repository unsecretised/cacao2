# Approach A: Custom ObjC Trampoline Class

## Concept

Define a single custom Objective-C class (`ActionTarget`) that owns a `Box<dyn FnMut()>`. When the button is pressed, the ObjC runtime calls the trampoline's action method, which invokes the closure. Each `Button` stores its own `Retained<ActionTarget>`.

## How It Works

```
Button::new("Click").on_press(|| println!("clicked"))
    → ActionTarget stores Box<dyn FnMut()>
    → button.setTarget(actionTarget)
    → button.setAction(sel!(handleAction:))
    → user clicks → AppKit sends handleAction: to ActionTarget
    → ActionTarget::handleAction calls the closure
```

## Code Sketch

### The trampoline class (defined once in the library)

```rust
use std::cell::RefCell;
use objc2::define_class;
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject, Sel};

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[ivars = RefCell<Box<dyn FnMut()>>]
    #[name = "CacaoActionTarget"]
    struct ActionTarget;

    impl ActionTarget {
        #[unsafe(method(handleAction:))]
        fn handle_action(&self, _sender: Option<&AnyObject>) {
            (self.ivars().borrow_mut())();
        }
    }
);

impl ActionTarget {
    fn new(f: Box<dyn FnMut()>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(RefCell::new(f));
        unsafe { msg_send![super(this), init] }
    }
}
```

### Button stores the target

```rust
use objc2_app_kit::{NSButton, NSControl};
use objc2_foundation::NSString;
use objc2::{sel, MainThreadMarker};

pub struct Button {
    pub(crate) button: Retained<NSButton>,
    target: Retained<ActionTarget>,
}

impl Button {
    pub fn new(text: &str) -> Self {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");
        let button = NSButton::initWithFrame(
            NSButton::alloc(mtm),
            CGRect::new(CGPoint::new(0., 0.), CGSize::new(200., 40.)),
        );
        button.setTitle(&NSString::from_str(text));
        button.setWantsLayer(true);

        // Dummy target — the user will replace the callback via .on_press()
        let target = ActionTarget::new(Box::new(|| {}));

        Self { button, target }
    }

    pub fn on_press<F: FnMut() + 'static>(mut self, f: F) -> Self {
        *self.target.ivars().borrow_mut() = Box::new(f);
        unsafe {
            self.button.setTarget(Some(self.target.as_ref()));
            self.button.setAction(Some(sel!(handleAction:)));
        }
        self
    }
}
```

### User code

```rust
Button::new("Click me")
    .on_press(|| println!("clicked"))
    .set_pos(100., 100.);
```

## Pros

- **Direct mapping** — one ObjC object per button, nothing shared
- **Simple ownership** — `Button` owns its target; when `Button` drops, the target drops too
- **Minimal unsafe** — `setTarget`/`setAction` are the only `unsafe` calls; the trampoline itself is safe internally
- **No global state** — no statics, no registries, no thread-locals
- **Works with the consuming-builder pattern** — `.on_press()` returns `Self`

## Cons

- **Requires `define_class!`** — the trampoline class must be declared somewhere in the library crate (once)
- **Closure is `'static`** — can't capture non-`'static` references without `Rc`/`Arc`
- **One allocation per button** — each button allocates a separate ObjC object
- **Main thread only** — appropriate for AppKit, but the class must be `#[thread_kind = MainThreadOnly]`

## When To Use

Best general-purpose approach. Clean, no global state, works with any application architecture. Recommended as the primary mechanism.
