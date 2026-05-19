use std::sync::{Arc, Mutex};

use objc2::rc::Retained;
use objc2_app_kit::{NSBackingStoreType, NSColor, NSWindow, NSWindowStyleMask};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{MainThreadMarker, NSString, ns_string};

pub struct Window<T: Clone> {
    window: Retained<NSWindow>,
    prelaunch: fn(&Retained<NSWindow>),
    data: Arc<Mutex<T>>,
}

impl<T: Clone> Window<T> {
    pub fn new(origin: (f64, f64), data: T) -> Self {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");

        let rect = CGRect::new(
            CGPoint::new(origin.0, origin.1),
            CGSize::new(800.0, 600.0), // sensible default
        );

        let style = NSWindowStyleMask::Titled
            | NSWindowStyleMask::Closable
            | NSWindowStyleMask::Resizable
            | NSWindowStyleMask::Miniaturizable;

        let window = unsafe {
            NSWindow::initWithContentRect_styleMask_backing_defer(
                mtm.alloc(),
                rect,
                style,
                NSBackingStoreType::Buffered,
                false,
            )
        };

        Self {
            window,
            prelaunch: Self::default_window_prelaunch,
            data: Arc::new(Mutex::new(data)),
        }
    }

    pub fn resize(&mut self, width: f64, height: f64, animate: bool) {
        let frame = self.window.frame();
        self.window.setFrame_display(
            CGRect::new(
                CGPoint::new(frame.origin.x, frame.origin.y),
                CGSize::new(width, height),
            ),
            animate,
        );
    }

    pub fn show(&self) {
        (self.prelaunch)(&self.window);
        self.window.makeKeyAndOrderFront(None);
    }

    pub fn set_opaque(&self, opaque: bool) {
        self.window.setOpaque(opaque);
    }

    pub fn set_title(&self, title: &str) {
        self.window.setTitle(&NSString::from_str(title));
    }

    pub fn set_color(&self, color: (f64, f64, f64, f64)) {
        self.window
            .setBackgroundColor(Some(&&NSColor::colorWithRed_green_blue_alpha(
                color.0, color.1, color.2, color.3,
            )));
    }

    fn default_window_prelaunch(window: &Retained<NSWindow>) {
        window.setTitle(ns_string!("cacao2"));
        window.setMovableByWindowBackground(true);
        window.setStyleMask(
            NSWindowStyleMask::Titled
                | NSWindowStyleMask::Closable
                | NSWindowStyleMask::Miniaturizable,
        );
    }
}
