use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSFont, NSTextField};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::NSString;

use crate::view::View;

pub struct Text {
    pub(crate) text: String,
    pub text_field: Retained<NSTextField>,
}

impl Text {
    pub fn new(text: &str) -> Self {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");

        let text_field = NSTextField::initWithFrame(
            mtm.alloc(),
            CGRect::new(CGPoint::new(0., 0.), CGSize::new(200., 40.)),
        );

        text_field.setStringValue(&NSString::from_str(text));
        text_field.setEditable(false);
        text_field.setBezeled(false);
        text_field.setDrawsBackground(false);
        text_field.setWantsLayer(true);

        Self {
            text_field,
            text: text.to_string(),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.text_field.setStringValue(&NSString::from_str(text));
    }

    pub fn set_size(&self, width: f64, height: f64) {
        let frame = self.text_field.frame();
        self.text_field
            .setFrame(CGRect::new(frame.origin, CGSize::new(width, height)));
    }

    pub fn set_font_size(&self, size: f64) {
        self.text_field
            .setFont(Some(&NSFont::systemFontOfSize(size)));
    }

    pub fn set_pos(&self, x: f64, y: f64) {
        let frame = self.text_field.frame();
        self.text_field
            .setFrame(CGRect::new(CGPoint::new(x, y), frame.size));
    }
}

impl Into<View> for Text {
    fn into(self) -> View {
        let view = View::new();
        let frame = self.text_field.frame();
        view.view.setFrame(frame);
        view.set_content(self.text_field.downcast().unwrap());
        view
    }
}
