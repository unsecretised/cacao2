use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSScreen};

use crate::{view::View, window::Window};

pub struct Application<'a, T, M> {
    pub(crate) app: Retained<NSApplication>,
    pub windows: Vec<&'a Window>,
    pub data: T,
    pub view_fn: Box<dyn Fn(&T) -> View>,
    pub update_fn: Box<dyn Fn(&mut T, M)>,
}

pub enum ActivationPolicy {
    Regular,
    Accessory,
    Prohibited,
}

impl<'a, T, M> Application<'a, T, M> {
    pub fn new(
        data: T,
        view_fn: Box<dyn Fn(&T) -> View>,
        update_fn: Box<dyn Fn(&mut T, M)>,
    ) -> Self {
        let app = NSApplication::sharedApplication(
            MainThreadMarker::new().expect("Must be on main thread"),
        );

        Self {
            app,
            windows: vec![],
            data,
            view_fn,
            update_fn,
        }
    }

    pub fn init_default_window(&self) {
        let new_window = Window::new();
        new_window.view(&(self.view_fn)(&self.data));
        new_window.show();
    }

    pub fn add_window(&self, window: Window) {
        window.view(&(self.view_fn)(&self.data));
        window.show();
    }

    pub fn set_activation_policy(&self, policy: ActivationPolicy) {
        let raw_act_policy = match policy {
            ActivationPolicy::Regular => NSApplicationActivationPolicy::Regular,
            ActivationPolicy::Accessory => NSApplicationActivationPolicy::Accessory,
            ActivationPolicy::Prohibited => NSApplicationActivationPolicy::Prohibited,
        };

        self.app.setActivationPolicy(raw_act_policy);
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
