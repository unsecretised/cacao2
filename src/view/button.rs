use objc2::{MainThreadMarker, MainThreadOnly, rc::Retained};
use objc2_app_kit::NSButton;
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_core_graphics::CGColor;
use objc2_foundation::NSString;

use crate::view::View;

pub struct Button {
    pub(crate) button: Retained<NSButton>,
}

impl Button {
    pub fn new(text: &str) -> Self {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");

        let button = NSButton::initWithFrame(
            NSButton::alloc(mtm),
            CGRect::new(CGPoint::new(0., 0.), CGSize::new(200., 40.)),
        );

        button.setTitle(&NSString::from_str(text));
        button.setWantsLayer(true);

        Self { button }
    }

    pub fn set_text(&self, text: &str) {
        self.button.setTitle(&NSString::from_str(text));
    }

    pub fn set_size(&self, width: f64, height: f64) {
        let frame = self.button.frame();
        self.button
            .setFrame(CGRect::new(frame.origin, CGSize::new(width, height)));
    }

    pub fn set_pos(&self, x: f64, y: f64) {
        let frame = self.button.frame();
        self.button
            .setFrame(CGRect::new(CGPoint::new(x, y), frame.size));
    }

    pub fn set_bg_color(&self, clr: (f64, f64, f64, f64)) {
        self.button.setWantsLayer(true);
        let Some(layer) = self.button.layer() else {
            return;
        };

        layer.setBackgroundColor(Some(&CGColor::new_srgb(clr.0, clr.1, clr.2, clr.3)));
    }
}

impl From<Button> for View {
    fn from(val: Button) -> Self {
        let view = View::new();
        view.set_content(val.button.downcast().unwrap());
        view
    }
}
