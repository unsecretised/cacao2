# Approach Comparison

## At a Glance

| Criterion | A: Trampoline | B: Delegate Dispatch | C: TLS Registry | D: Tag Enum |
|---|---|---|---|---|
| API shape | `.on_press(\|\| ...)` | `.on_press(&app, \|data\| ...)` | `.on_press(\|\| ...)` | `.with_id(X).target(&d)` |
| Closure type | `FnMut()` | `FnMut(&mut T)` | `FnMut()` | No closures |
| Per-button allocation | 1 ObjC object | 1 integer | None | None |
| Global state | No | Yes (Application) | Yes (thread_local) | No |
| Cleanup on drop | Automatic | Manual deregister | Automatic (Drop) | N/A |
| Passes app data | No (capture only) | Yes (first-class) | No (capture only) | Yes (via delegate) |
| `unsafe` exposure | Low (2 calls) | Low (2 calls + registry) | Low (2 calls) | Low (2 calls) |
| Complexity | Low | Medium | Medium | Low |
| Max buttons | Unlimited | Unlimited | Unlimited | Fixed enum |

## User-Facing API Comparison

**Approach A** (Trampoline class):
```rust
Button::new("Click").on_press(|| println!("hi"));
```
- Closure captures what it needs. No app reference needed.
- Best DX for simple actions.

**Approach B** (App-Delegate dispatch):
```rust
Button::new("Click").on_press(&app, |data: &mut MyData| data.count += 1);
```
- Closure receives `&mut T` from the Application.
- Good for apps where all state is in one struct.

**Approach C** (TLS registry):
```rust
Button::new("Click").on_press(|| println!("hi"));
```
- Same API as A, but no per-button ObjC objects.
- Best runtime efficiency for many buttons.

**Approach D** (Tag enum):
```rust
Button::new("+").with_id(Action::Add).target(&delegate);
```
- No closures, pure ObjC-style dispatch.
- Best when action set is fixed and known.

## Can They Coexist?

Yes. The approaches aren't mutually exclusive. A common pattern:

1. Use **A (Trampoline)** as the general-purpose default — works for 90% of cases
2. Use **D (Tag enum)** for menu items and toolbar buttons where the action set is fixed
3. Use **B (Delegate dispatch)** if you want to give buttons direct `&mut AppData` access

The library can provide both `.on_press()` (approach A/C) and `.with_id()` + `.target()` (approach D) on the same `Button` type.

## Recommendation

| If you want... | Use |
|---|---|
| Simplest user-facing API | **A (Trampoline)** |
| Maximum performance | **C (TLS Registry)** |
| App-data-in-closure ergonomics | **B (Delegate Dispatch)** |
| Cocoa-familiar pattern | **D (Tag Enum)** |
| A hybrid that covers most cases | **A for callbacks + D for menus/toolbar** |
