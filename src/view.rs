use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSColor, NSView};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_core_graphics::CGColor;

pub mod button;
pub mod text;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Into<Retained<NSColor>> for Color {
    fn into(self) -> Retained<NSColor> {
        NSColor::colorWithRed_green_blue_alpha(self.r, self.g, self.b, self.a)
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Frame {
    pub(crate) origin: (f64, f64),
    pub(crate) size: (f64, f64),
}

#[derive(Debug, Clone, Copy)]
pub struct Radius {
    pub radius: f64,
}

impl Radius {
    pub fn new(rad: f64) -> Self {
        Self { radius: rad }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Start,
    Center,
    End,
}

pub struct View {
    pub(crate) view: Retained<NSView>,
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

impl View {
    pub fn new() -> Self {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");
        let view = NSView::new(mtm);
        view.setWantsLayer(true);

        Self { view }
    }

    pub fn frame(&self) -> Frame {
        let frame = self.view.frame();
        Frame {
            origin: (frame.origin.x, frame.origin.y),
            size: (frame.size.width, frame.size.height),
        }
    }

    pub fn set_bg_color(&self, clr: (f64, f64, f64, f64)) -> &Self {
        self.view.setWantsLayer(true);
        let Some(layer) = self.view.layer() else {
            return self;
        };

        layer.setBackgroundColor(Some(&CGColor::new_srgb(clr.0, clr.1, clr.2, clr.3)));
        self
    }

    pub fn blur_view(&self) -> &Self {
        let Some(layer) = self.view.layer() else {
            return self;
        };

        layer.setMasksToBounds(true);
        self
    }

    pub fn set_corner_radius(&self, rad: Radius) -> &Self {
        let Some(layer) = self.view.layer() else {
            return self;
        };

        layer.setCornerRadius(rad.radius);
        layer.setMasksToBounds(true);
        self
    }

    pub fn set_size(&self, dims: (f64, f64)) -> &Self {
        let frame = self.view.frame();
        self.view
            .setFrame(CGRect::new(frame.origin, CGSize::new(dims.0, dims.1)));
        self
    }

    pub(crate) fn set_content(&self, view: Retained<NSView>) -> &Self {
        self.view.addSubview(&view);
        self
    }

    pub fn set_pos(&self, pos_coords: (f64, f64)) -> &Self {
        let frame = self.view.frame();
        self.view.setFrame(CGRect::new(
            CGPoint::new(pos_coords.0, pos_coords.1),
            frame.size,
        ));
        self
    }

    pub fn add_subview(&self, view: &View) -> &Self {
        self.view.addSubview(&view.view);
        self
    }

    pub fn align_x(&self, parent_frame: Frame, alignment: Alignment) -> &Self {
        let view_frame = self.view.frame();

        let x = match alignment {
            Alignment::Start => 0.0,
            Alignment::Center => (parent_frame.size.0 - view_frame.size.width) / 2.0,
            Alignment::End => parent_frame.size.0 - view_frame.size.width,
        };

        self.view.setFrame(CGRect::new(
            CGPoint::new(x, view_frame.origin.y),
            view_frame.size,
        ));

        self
    }

    pub fn align_y(&self, parent_frame: Frame, alignment: Alignment) -> &Self {
        let view_frame = self.view.frame();

        let y = match alignment {
            Alignment::Start => parent_frame.size.1 - view_frame.size.height, // top (flipped)
            Alignment::Center => (parent_frame.size.1 - view_frame.size.height) / 2.0,
            Alignment::End => 0.0, // bottom (flipped)
        };

        self.view.setFrame(CGRect::new(
            CGPoint::new(view_frame.origin.x, y),
            view_frame.size,
        ));

        self
    }
}

pub trait Widget {
    fn new() -> Self;
    fn set_bg_color(&self, clr: Color) -> &Self;
    fn add_subview(&self, view: &View) -> &Self;
}
