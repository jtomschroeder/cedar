
use cocoa::base::{id, nil, class};

use cacao::view::View;
use cacao::action;

use property::Property;
use stream::Stream;

enum Attribute<M> {
    Text(Box<Property<M, String>>),
}

#[repr(u64)]
enum NSBezelStyle {
    NSRoundedBezelStyle = 1,
}

pub struct Button<M, S> {
    id: id,
    attributes: Vec<Attribute<M>>,
    stream: Stream<S>,
}

impl<M, S: 'static> Button<M, S> {
    pub fn new(stream: Stream<S>) -> Self {
        unsafe {
            let button: id = msg_send![class("NSButton"), alloc];
            let button: id = msg_send![button, init];

            msg_send![button, setBezelStyle: NSBezelStyle::NSRoundedBezelStyle];

            Button {
                id: button,
                attributes: vec![],
                stream: stream,
            }
        }
    }

    fn set_text(&mut self, text: &str) {
        use cocoa::foundation::NSString;

        unsafe {
            let title = NSString::alloc(nil).init_str(text);
            msg_send![self.id, setTitle: title];
            msg_send![self.id, sizeToFit];
        }
    }

    pub fn text<P: Property<M, String> + 'static>(mut self, attribute: P) -> Self {
        self.attributes.push(Attribute::Text(Box::new(attribute)));
        self
    }

    pub fn position(self, x: f64, y: f64) -> Self {
        use cocoa::foundation::NSRect;

        let mut frame: NSRect = unsafe { msg_send![self.id, frame] };
        frame.origin.x = x;
        frame.origin.y = y;
        unsafe { msg_send![self.id, setFrame: frame] };

        self
    }

    pub fn click<F: FnMut() -> S + 'static>(self, mut action: F) -> Self {
        let stream = self.stream.clone();
        let action = action::create(move || stream.push(action()));

        unsafe {
            msg_send![self.id, setAction: sel!(act)];
            msg_send![self.id, setTarget: action];
        }

        self
    }
}

impl<M: Clone, S: 'static> View<M> for Button<M, S> {
    fn id(&self) -> id {
        self.id
    }

    fn update(&mut self, model: M) {
        enum Attr {
            Text(String),
        }

        let mut attrs: Vec<_> = self.attributes
            .iter_mut()
            .map(|attr| match attr {
                &mut Attribute::Text(ref mut prop) => Attr::Text(prop.process(model.clone())),
            })
            .collect();

        for attr in attrs.drain(..) {
            match attr {
                Attr::Text(s) => self.set_text(&s),
            }
        }
    }
}
