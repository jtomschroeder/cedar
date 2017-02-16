
use cocoa::base::{id, nil, class};
use cocoa::foundation::NSString;

use cacao::view::View;
use property::Property;

enum Attribute {
    Text(Box<Property<String>>),
}

pub struct Label {
    id: id,
    attributes: Vec<Attribute>,
}

impl Label {
    pub fn new() -> Self {
        unsafe {
            let string = NSString::alloc(nil).init_str("");
            let label: id = msg_send![class("NSTextField"), labelWithString: string];

            Label {
                id: label,
                attributes: vec![],
            }
        }
    }

    fn set_text(&mut self, text: &str) {
        unsafe {
            let string = NSString::alloc(nil).init_str(text);
            msg_send![self.id, setStringValue: string];

            msg_send![self.id, sizeToFit];
        }
    }

    pub fn position(self, x: f64, y: f64) -> Self {
        use cocoa::foundation::NSRect;

        let mut frame: NSRect = unsafe { msg_send![self.id, frame] };
        frame.origin.x = x;
        frame.origin.y = y;
        unsafe { msg_send![self.id, setFrame: frame] };

        self
    }

    pub fn text<P: Property<String> + 'static>(mut self, attribute: P) -> Self {
        self.attributes.push(Attribute::Text(Box::new(attribute)));
        self
    }
}

impl View for Label {
    fn view(&self) -> id {
        self.id
    }

    fn update(&mut self, model: i32) {
        enum Attr {
            Text(String),
        }

        let mut attrs: Vec<_> = self.attributes
            .iter_mut()
            .map(|attr| match attr {
                &mut Attribute::Text(ref mut prop) => Attr::Text(prop.process(model)),
            })
            .collect();

        for attr in attrs.drain(..) {
            match attr {
                Attr::Text(s) => self.set_text(&s),
            }
        }
    }
}
