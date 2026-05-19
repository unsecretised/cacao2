# Cacao2 API Restructure Proposal

## Target API Pattern

Consuming-builder API where every setter returns `Self`, widgets are `NSView`-backed directly (no `into()` loss), and `Application` owns nothing (AppKit retains windows).

```rust
use cacao2::prelude::*;

fn main() {
    let app = Application::new((), ActivationPolicy::Accessory);

    let text = Text::new("Something")
        .font_size(30.)
        .align(Alignment::Center)
        .selectable(true);

    let view = View::new()
        .bg_color(Color::new(0.5, 0.5, 0.5, 1.))
        .corner_radius(10.)
        .size(500., 500.)
        .add_centered(text);

    Window::new()
        .title("Someone")
        .titlebar(|t| t.visible(false).transparent(true).traffic_lights(true))
        .bg_color(Color::new(0., 0., 0., 0.3))
        .center_on_screen()
        .child(view)
        .show();

    app.run();
}
```

## Source Layout

```
src/
  lib.rs          # pub mods + prelude
  prelude.rs      # Re-exports everything user-facing
  color.rs        # Color struct
  app.rs          # Application (no lifetime, no window vec)
  window.rs       # Window with consuming builder
  window/
    titlebar.rs   # TitlebarConfig
  view.rs         # View, Widget trait, Anchor, Alignment
  view/
    text.rs       # Text implements Widget, AsRef<NSView>
    button.rs     # Button implements Widget, AsRef<NSView>
  layout.rs       # Layout containers (HStack, VStack)
```

## Key Design Decisions

| Decision | Choice |
|---|---|
| Application ownership | Standalone windows — AppKit retains them |
| Widget → View conversion | `AsRef<NSView>` + `add_subview(impl AsRef<NSView>)` — no widget data loss |
| Color | Typed struct, not raw tuples |
| Window builder | Consuming builder (`fn title(mut self, ...) -> Self`) |
| Titlebar | Closure-based: `.titlebar(\|t\| t.visible(false)...)` |
| Layout | `.add_centered()`, `.add_anchored()` helpers + optional layout containers |
| Widget trait | Shared `set_bg_color`, `set_size`, `set_pos`, `set_corner_radius` methods |

## Implementation Order

1. `Color` struct + `prelude.rs`
2. `Widget` trait — implement on `View`, `Text`, `Button`
3. Refactor `Text`/`Button` to expose `AsRef<NSView>` (via `NSTextField`/`NSButton` → `NSView` upcast), remove `From<*>` for `View`
4. `Window` consuming builder — inline titlebar config, remove `prelaunch` fn ptr
5. `Application` — strip lifetime + window vec
6. `View` layout helpers — `.add_centered()`, `.add_anchored()`
7. Example — rewrite to match the new API

## Current Problems Being Solved

- **Widget ownership loss** — `From<Text> for View` consumes `Text`, losing the ability to call text-specific methods afterward. `NSTextField` *is* an `NSView` — no wrapping needed.
- **Lifetime-heavy Application** — `Application<'a, T>` storing `&'a Window` forces awkward `let window = app.new_window(); app.add_window(&window)` dance.
- **Verbose imperative setup** — Every property requires a separate `.set_*()` call. No chaining.
- **Color as raw tuples** — `(f64, f64, f64, f64)` everywhere, no type safety.
- **Fragmented layout** — `anchor_in_window`, `anchor_in_view`, `add_subview` are separate manual steps.
- **`prelaunch` fn pointer** — Deferred config via a fn ptr in `Window` is unusual and opaque.
