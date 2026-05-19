use cacao2::window::Window;
use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;

fn main() {
    let mtm = MainThreadMarker::new().expect("Must be on main thread");

    let app = NSApplication::sharedApplication(mtm);

    let window = Window::new((100., 100.), ());
    window.show();

    app.run();
}
