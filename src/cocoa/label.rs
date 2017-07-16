
use cocoa::base::{id, nil, class, NO};
use cocoa::foundation::NSString;

use super::dom::Attributes;

use super::id::Id;
use super::widget::Widget;

pub struct Label {
    id: Id,
}

impl Label {
    pub fn new() -> Self {
        unsafe {
            let string = NSString::alloc(nil).init_str("");

            let label: id = msg_send![class("NSTextField"), alloc];
            let label: id = msg_send![label, init];

            msg_send![label, setStringValue: string];
            msg_send![label, setBezeled: NO];
            msg_send![label, setDrawsBackground: NO];
            msg_send![label, setEditable: NO];
            msg_send![label, setSelectable: NO];

            Label { id: label.into() }
        }
    }

    fn set_text(&mut self, text: &str) {
        unsafe {
            let string = NSString::alloc(nil).init_str(text);
            msg_send![*self.id, setStringValue: string];
        }
    }
}

impl<S> Widget<S> for Label {
    fn id(&self) -> &Id {
        &self.id
    }

    fn update(&mut self, attributes: Attributes<S>) {
        use super::dom::Attribute::*;
        for attr in attributes.into_iter() {
            match attr {
                Text(text) => self.set_text(&text),
                _ => {}
            }
        }
    }
}
