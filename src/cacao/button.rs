
use cocoa::base::{id, nil, class};

use cacao::view::View;
use cacao::action;

#[repr(u64)]
enum NSBezelStyle {
    NSRoundedBezelStyle = 1,
}

pub struct Button(id);

impl Button {
    pub fn new() -> Self {
        unsafe {
            let button: id = msg_send![class("NSButton"), alloc];
            let button: id = msg_send![button, init];

            msg_send![button, setBezelStyle: NSBezelStyle::NSRoundedBezelStyle];

            Button(button)
        }
    }

    pub fn text(self, text: &str) -> Self {
        use cocoa::foundation::NSString;

        unsafe {
            let title = NSString::alloc(nil).init_str(text);
            msg_send![self.0, setTitle: title];

            msg_send![self.0, sizeToFit];
        }

        self
    }

    pub fn position(self, x: f64, y: f64) -> Self {
        use cocoa::foundation::NSRect;

        let mut frame: NSRect = unsafe { msg_send![self.0, frame] };
        frame.origin.x = x;
        frame.origin.y = y;
        unsafe { msg_send![self.0, setFrame: frame] };

        self
    }

    pub fn click<F: FnMut() + 'static>(self, action: F) -> Self {
        let target = action::create(action);
        unsafe {
            msg_send![self.0, setAction: sel!(act)];
            msg_send![self.0, setTarget: target];
        }

        self
    }
}

impl View for Button {
    fn view(&self) -> id {
        self.0
    }
}
