
use cocoa::base::{id, nil, class};

use super::id::Id;
use super::widget::Widget;
use super::action;

use stream::Stream;
use dom::Attributes;

#[repr(u64)]
enum BezelStyle {
    Rounded = 1,
}

pub struct Button<S> {
    id: Id,
    stream: Stream<S>,
}

impl<S: Clone + 'static> Button<S> {
    pub fn new(stream: Stream<S>) -> Self {
        unsafe {
            let button: id = msg_send![class("NSButton"), alloc];

            use cocoa::foundation::{NSRect, NSPoint, NSSize};
            let rect = NSRect::new(NSPoint::new(100., 100.), NSSize::new(100., 100.));
            let button: id = msg_send![button, initWithFrame: rect];

            msg_send![button, setBezelStyle: BezelStyle::Rounded];

            Button {
                id: button.into(),
                stream: stream,
            }
        }
    }

    fn set_text(&mut self, text: &str) {
        use cocoa::foundation::NSString;
        unsafe {
            let title = NSString::alloc(nil).init_str(text);
            msg_send![*self.id, setTitle: title];
        }
    }

    pub fn register_click(&mut self, message: S) {
        let stream = self.stream.clone();
        let action = action::create(move || stream.push(message.clone()));

        unsafe {
            msg_send![*self.id, setAction: sel!(act)];
            msg_send![*self.id, setTarget: action];
        }
    }
}

impl<S: Clone + 'static> Widget<S> for Button<S> {
    fn id(&self) -> &Id {
        &self.id
    }

    fn update(&mut self, attributes: Attributes<S>) {
        use dom::Attribute::*;
        for attr in attributes.into_iter() {
            match attr {
                Text(text) => self.set_text(&text),
                Click(message) => self.register_click(message),
                _ => {}
            }
        }
    }
}
