use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::NSView;
use objc2_core_foundation::{CGRect, CGSize};
use objc2_core_graphics::CGColor;

pub mod button;

#[derive(Debug, Clone, Copy)]
pub struct Radius {
    pub top_left: f64,
    pub top_right: f64,
    pub bottom_left: f64,
    pub bottom_right: f64,
}

impl Radius {
    pub fn new(rad: f64) -> Self {
        Self {
            top_left: rad,
            top_right: rad,
            bottom_left: rad,
            bottom_right: rad,
        }
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

        layer.setCornerRadius(rad.top_left);
        layer.setMasksToBounds(true);
    }

    pub fn set_position(&self, dims: (f64, f64)) {
        let frame = self.view.frame();
        self.view
            .setFrame(CGRect::new(frame.origin, CGSize::new(dims.0, dims.1)));
    }

    pub fn add_subview(&self, view: &View) {
        self.view.addSubview(&view.view);
    }
}
