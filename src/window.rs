use objc2::rc::Retained;
use objc2_app_kit::{
    NSBackingStoreType, NSColor, NSWindow, NSWindowButton, NSWindowStyleMask,
    NSWindowTitleVisibility,
};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::{MainThreadMarker, NSString};

use crate::view::View;

pub mod titlebar;

pub struct Window {
    pub(crate) window: Retained<NSWindow>,
    pub(crate) prelaunch: fn(&Retained<NSWindow>),
}

impl Window {
    pub fn new() -> Self {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");

        let rect = CGRect::new(
            CGPoint::new(0., 0.),
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

    pub fn set_visibility(&self, visible: bool) {
        self.window.setIsVisible(visible);
    }

    pub fn set_bg_color(&self, clr: (f64, f64, f64, f64)) {
        self.window
            .setBackgroundColor(Some(&NSColor::colorWithRed_green_blue_alpha(
                clr.0, clr.1, clr.2, clr.3,
            )));
    }

    pub fn toggle_visibility(&self) {
        self.window.setIsVisible(!self.window.isVisible());
    }

    pub fn set_titlebar_config(&self, config: &titlebar::TitlebarConfig) {
        self.set_title(&config.title);
        self.window
            .setTitlebarAppearsTransparent(config.appears_transparent);
        self.window.setTitleVisibility(match config.show_titlebar {
            true => NSWindowTitleVisibility::Visible,
            false => NSWindowTitleVisibility::Hidden,
        });

        if !config.show_traffic_lights {
            if let Some(btn) = self
                .window
                .standardWindowButton(NSWindowButton::CloseButton)
            {
                btn.setHidden(true);
            }
            if let Some(btn) = self
                .window
                .standardWindowButton(NSWindowButton::MiniaturizeButton)
            {
                btn.setHidden(true);
            }
            if let Some(btn) = self.window.standardWindowButton(NSWindowButton::ZoomButton) {
                btn.setHidden(true);
            }
        }
    }

    pub fn view(&self, view: &View) {
        self.window.contentView().map(|x| x.addSubview(&view.view));
    }

    fn default_window_prelaunch(window: &Retained<NSWindow>) {
        window.setMovableByWindowBackground(true);
        window.setBackgroundColor(Some(&NSColor::whiteColor()));
        window.setStyleMask(
            NSWindowStyleMask::Titled
                | NSWindowStyleMask::Closable
                | NSWindowStyleMask::Miniaturizable
                | NSWindowStyleMask::Resizable,
        );
    }
}
