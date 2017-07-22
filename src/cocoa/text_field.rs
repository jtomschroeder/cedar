
use cocoa::base::{id, nil, class, YES};
use cocoa::foundation::NSString;

use super::id::Id;
use super::widget::Widget;
use super::delegate;

use stream::Stream;
use dom::Attributes;

pub struct TextField<S> {
    id: Id,
    stream: Stream<S>,
}

impl<S: 'static> TextField<S> {
    pub fn new(stream: Stream<S>) -> Self {
        unsafe {
            let string = NSString::alloc(nil).init_str("");

            let field: id = msg_send![class("NSTextField"), alloc];
            let field: id = msg_send![field, init];

            msg_send![field, setStringValue: string];
            msg_send![field, setBezeled: YES];
            msg_send![field, setDrawsBackground: YES];
            msg_send![field, setEditable: YES];
            msg_send![field, setSelectable: YES];

            TextField {
                id: field.into(),
                stream: stream,
            }
        }
    }

    pub fn set_placeholder(&mut self, text: &str) {
        unsafe {
            let text = NSString::alloc(nil).init_str(text);

            let string: id = msg_send![class("NSAttributedString"), alloc];
            let string: id = msg_send![string, initWithString: text];

            msg_send![*self.id, setPlaceholderAttributedString: string];

            // set "minimum size" through anchor constraint
            let anchor: id = msg_send![*self.id, widthAnchor];
            let constraint: id = msg_send![anchor, constraintGreaterThanOrEqualToConstant: 120.];
            msg_send![constraint, setActive: YES];
        }
    }

    pub fn register_change(&mut self, messenger: fn(String) -> S) {
        let stream = self.stream.clone();
        let delegate = delegate::create(move |s| stream.push(messenger(s.into())));

        unsafe { msg_send![*self.id, setDelegate: delegate] };
    }
}

impl<S: 'static> Widget<S> for TextField<S> {
    fn id(&self) -> &Id {
        &self.id
    }

    fn update(&mut self, attributes: Attributes<S>) {
        use dom::Attribute::*;
        for attr in attributes.into_iter() {
            match attr {
                Placeholder(text) => self.set_placeholder(&text),
                Change(messenger) => self.register_change(messenger),
                _ => {}
            }
        }
    }
}
