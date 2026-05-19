use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSScreen};

use crate::window::Window;

pub struct Application<'a, T> {
    pub(crate) app: Retained<NSApplication>,
    pub windows: Vec<&'a Window>,
    pub data: T,
}

pub enum ActivationPolicy {
    Regular,
    Accessory,
    Prohibited,
}

impl<'a, T> Application<'a, T> {
    pub fn new(data: T, activation_policy: ActivationPolicy) -> Self {
        let app = NSApplication::sharedApplication(
            MainThreadMarker::new().expect("Must be on main thread"),
        );

        let raw_act_policy = match activation_policy {
            ActivationPolicy::Regular => NSApplicationActivationPolicy::Regular,
            ActivationPolicy::Accessory => NSApplicationActivationPolicy::Accessory,
            ActivationPolicy::Prohibited => NSApplicationActivationPolicy::Prohibited,
        };

        app.setActivationPolicy(raw_act_policy);

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

    pub fn set_activation_policy(&self, policy: ActivationPolicy) {
        let raw_act_policy = match policy {
            ActivationPolicy::Regular => NSApplicationActivationPolicy::Regular,
            ActivationPolicy::Accessory => NSApplicationActivationPolicy::Accessory,
            ActivationPolicy::Prohibited => NSApplicationActivationPolicy::Prohibited,
        };

        self.app.setActivationPolicy(raw_act_policy);
    }

    pub fn new_window(&self) -> Window {
        Window::new()
    }

    pub fn monitor_dimensions(&self) -> Option<(f64, f64)> {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");
        NSScreen::mainScreen(mtm).map(|ns_screen| {
            let screen_size = ns_screen.frame().size;
            (screen_size.width, screen_size.height)
        })
    }

    pub fn run(&self) {
        self.app.run();
    }
}
