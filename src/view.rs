use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::NSView;
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_core_graphics::CGColor;

use crate::window::Window;

pub mod button;
pub mod text;

#[derive(Debug, Clone, Copy)]
pub struct Radius {
    pub radius: f64,
}

impl Radius {
    pub fn new(rad: f64) -> Self {
        Self { radius: rad }
    }
}

pub struct View {
    pub(crate) view: Retained<NSView>,
}

impl View {
    pub fn new() -> Self {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");
        let view = NSView::new(mtm);
        view.setWantsLayer(true);

        Self { view }
    }

    pub fn set_bg_color(&self, clr: (f64, f64, f64, f64)) {
        self.view.setWantsLayer(true);
        let Some(layer) = self.view.layer() else {
            return;
        };

        #[allow(deprecated)]
        layer.setBackgroundColor(Some(&CGColor::new_srgb(clr.0, clr.1, clr.2, clr.3)));
    }

    pub fn blur_view(&self) {
        let Some(layer) = self.view.layer() else {
            return;
        };

        layer.setMasksToBounds(true);
    }

    pub fn set_corner_radius(&self, rad: Radius) {
        let Some(layer) = self.view.layer() else {
            return;
        };

        layer.setCornerRadius(rad.radius);
        layer.setMasksToBounds(true);
    }

    pub fn set_size(&self, dims: (f64, f64)) {
        let frame = self.view.frame();
        self.view
            .setFrame(CGRect::new(frame.origin, CGSize::new(dims.0, dims.1)));
    }

    pub(crate) fn set_content(&self, view: Retained<NSView>) {
        self.view.addSubview(&view);
    }

    pub fn set_pos(&self, pos_coords: (f64, f64)) {
        let frame = self.view.frame();
        self.view.setFrame(CGRect::new(
            CGPoint::new(pos_coords.0, pos_coords.1),
            frame.size,
        ));
    }

    pub fn add_subview(&self, view: &View) {
        self.view.addSubview(&view.view);
    }

    pub fn anchor_in_window(&self, window: &Window, anchor: Anchor) {
        let window_frame = window.window.frame();
        let view_frame = self.view.frame();

        let (x, y) = match anchor {
            Anchor::Center => (
                (window_frame.size.width - view_frame.size.width) / 2.0,
                (window_frame.size.height - view_frame.size.height) / 2.0,
            ),
            Anchor::CenterHorizontal => (
                (window_frame.size.width - view_frame.size.width) / 2.0,
                view_frame.origin.y,
            ),
            Anchor::CenterVertical => (
                view_frame.origin.x,
                (window_frame.size.height - view_frame.size.height) / 2.0,
            ),
        };

        self.view
            .setFrame(CGRect::new(CGPoint::new(x, y), view_frame.size));
    }

    pub fn anchor_in_view(&self, parent: &View, anchor: Anchor) {
        let parent_frame = parent.view.frame();
        let view_frame = self.view.frame();

        let (x, y) = match anchor {
            Anchor::Center => (
                (parent_frame.size.width - view_frame.size.width) / 2.0,
                (parent_frame.size.height - view_frame.size.height) / 2.0,
            ),
            Anchor::CenterHorizontal => (
                (parent_frame.size.width - view_frame.size.width) / 2.0,
                view_frame.origin.y,
            ),
            Anchor::CenterVertical => (
                view_frame.origin.x,
                (parent_frame.size.height - view_frame.size.height) / 2.0,
            ),
        };

        self.view
            .setFrame(CGRect::new(CGPoint::new(x, y), view_frame.size));
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Anchor {
    Center,
    CenterHorizontal,
    CenterVertical,
}
