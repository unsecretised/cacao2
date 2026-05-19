use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSColor, NSFont, NSTextAlignment, NSTextField};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::NSString;

use crate::view::{Alignment, Color, View};

pub struct Text {
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

        Self { text_field }
    }

    pub fn text_align(&self, alignment: Alignment) -> &Self {
        match alignment {
            Alignment::Start => {
                self.text_field.setAlignment(NSTextAlignment::Left);
            }
            Alignment::Center => {
                self.text_field.setAlignment(NSTextAlignment::Center);
            }
            Alignment::End => {
                self.text_field.setAlignment(NSTextAlignment::Right);
            }
        }

        self
    }

    pub fn set_text(&self, text: &str) -> &Self {
        self.text_field.setStringValue(&NSString::from_str(text));
        self
    }

    pub fn set_size(&self, width: f64, height: f64) -> &Self {
        let frame = self.text_field.frame();
        self.text_field
            .setFrame(CGRect::new(frame.origin, CGSize::new(width, height)));
        self
    }

    pub fn set_font_size(&self, size: f64) -> &Self {
        self.text_field
            .setFont(Some(&NSFont::systemFontOfSize(size)));
        self
    }

    pub fn set_pos(&self, x: f64, y: f64) -> &Self {
        let frame = self.text_field.frame();
        self.text_field
            .setFrame(CGRect::new(CGPoint::new(x, y), frame.size));
        self
    }

    pub fn set_editable(&self, editable: bool) -> &Self {
        self.text_field.setEditable(editable);
        self
    }

    pub fn set_selectable(&self, selectable: bool) -> &Self {
        self.text_field.setSelectable(selectable);
        self
    }

    pub fn set_text_color(&self, clr: Color) -> &Self {
        let clr: Retained<NSColor> = clr.into();
        self.text_field.setTextColor(Some(&clr));
        self
    }

    pub fn set_font(&self, font: &str) -> &Self {
        let Some(font) = &NSFont::fontWithName_size(&NSString::from_str(font), 14.) else {
            return self;
        };

        self.text_field.setFont(Some(font));
        self
    }
}

impl From<&Text> for View {
    fn from(val: &Text) -> Self {
        let view = View::new();
        let frame = val.text_field.frame();
        view.view.setFrame(frame);
        view.set_content(val.text_field.clone().downcast().unwrap());
        view
    }
}
