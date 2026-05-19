use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::NSApplication;

use crate::window::Window;

pub struct Application<'a, T> {
    pub(crate) app: Retained<NSApplication>,
    pub windows: Vec<&'a Window>,
    pub data: T,
}

impl<'a, T> Application<'a, T> {
    pub fn new(data: T) -> Self {
        let app = NSApplication::sharedApplication(
            MainThreadMarker::new().expect("Must be on main thread"),
        );
        Self {
            app,
            windows: vec![],
            data,
        }
    }

    /// Adds a window to the application
    pub fn add_window(&mut self, window: &'a Window) {
        self.windows.push(window);
    }

    /// Returns a reference to the windows
    pub fn windows(&self) -> &Vec<&'a Window> {
        &self.windows
    }

    pub fn run(&self) {
        self.app.run();
    }
}
