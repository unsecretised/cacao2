use objc2::{
    DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained, sel,
};
use objc2_app_kit::NSButton;
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_core_graphics::CGColor;
use objc2_foundation::{NSObject, NSString};

use crate::view::View;

pub struct Button {
    pub(crate) button: Retained<NSButton>,
    handler: Retained<NSButtonHandler>,
}

pub struct HandlerIvars {
    action: Box<dyn Fn()>,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "NSButtonHandler"]
    #[ivars = HandlerIvars]
    struct NSButtonHandler;

    impl NSButtonHandler {
        #[unsafe(method(buttonClicked:))]
        fn button_clicked(&self, _sender: &NSButton) {
            (self.ivars().action)();
        }
    }
);

impl NSButtonHandler {
    fn new(mtm: MainThreadMarker, action: Box<dyn Fn()>) -> Retained<Self> {
        let this = mtm.alloc::<Self>().set_ivars(HandlerIvars { action });
        unsafe { msg_send![super(this), init] }
    }
}

impl Button {
    pub fn new(text: &str, action: impl Fn() + 'static) -> Self {
        let mtm = MainThreadMarker::new().expect("Must be on main thread");

        let handler = NSButtonHandler::new(mtm, Box::new(action));

        let button = unsafe {
            NSButton::buttonWithTitle_target_action(
                &NSString::from_str(text),
                Some(&handler),
                Some(sel!(buttonClicked:)),
                mtm,
            )
        };

        button.setFrame(CGRect::new(CGPoint::new(0., 0.), CGSize::new(200., 40.)));
        button.setWantsLayer(true);

        Self { button, handler }
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

        let size = val.button.frame().size;

        view.set_size((size.width, size.height));
        val.button.setFrameOrigin(CGPoint::new(0., 0.));

        view.set_content(val.button.downcast().unwrap());
        view.keep_alive(Retained::into_super(val.handler));
        view
    }
}
