
use cocoa::base::nil;
use cocoa::foundation::{NSRect, NSPoint, NSSize, NSString, NSAutoreleasePool};
use cocoa::appkit::{NSButton, NSBezelStyle};

use super::id::Id;
use super::widget::Widget;
use super::action;

use stream::Stream;
use dom::Attributes;

pub struct Button<S> {
    id: Id,
    stream: Stream<S>,
}

impl<S: Clone + 'static> Button<S> {
    pub fn new(stream: Stream<S>) -> Self {
        let frame = NSRect::new(NSPoint::new(0., 0.), NSSize::new(100., 100.));

        let button = unsafe { NSButton::alloc(nil).initWithFrame_(frame).autorelease() };

        unsafe { button.setBezelStyle_(NSBezelStyle::NSRoundedBezelStyle) };

        Button {
            id: button.into(),
            stream: stream,
        }
    }

    fn set_text(&mut self, text: &str) {
        unsafe {
            let title = NSString::alloc(nil).init_str(text);
            NSButton::setTitle_(*self.id, title);
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
