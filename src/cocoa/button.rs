
use cocoa::base::{id, nil, class};

use super::id::Id;
use super::widget::Widget;
use super::action;

use property::Property;
use stream::Stream;

enum Attribute<M> {
    Text(Box<Property<M, String>>),
}

#[repr(u64)]
enum BezelStyle {
    Rounded = 1,
}

pub struct Button {
    id: Id,
    // attributes: Vec<Attribute<M>>,
    // stream: Stream<S>,
}

impl Button {
    pub fn new() -> Self {
        unsafe {
            let button: id = msg_send![class("NSButton"), alloc];
            let button: id = msg_send![button, init];

            msg_send![button, setBezelStyle: BezelStyle::Rounded];

            let mut button = Button {
                id: button.into(),
                // attributes: vec![],
                // stream: stream,
            };

            button.set_text("TEST!");

            button
        }
    }

    fn set_text(&mut self, text: &str) {
        use cocoa::foundation::NSString;

        unsafe {
            let title = NSString::alloc(nil).init_str(text);
            msg_send![*self.id, setTitle: title];
        }
    }

    // pub fn text<P: Property<M, String> + 'static>(mut self, attribute: P) -> Self {
    //     self.attributes
    //         .push(Attribute::Text(Box::new(attribute)));
    //     self
    // }

    // pub fn click<F: Fn() -> S + 'static>(self, action: F) -> Self {
    //     let stream = self.stream.clone();
    //     let action = action::create(move || stream.push(action()));

    //     unsafe {
    //         msg_send![*self.id, setAction: sel!(act)];
    //         msg_send![*self.id, setTarget: action];
    //     }

    //     self
    // }
}

impl<S> Widget<S> for Button {
    fn id(&self) -> &Id {
        &self.id
    }

    // fn update(&mut self, model: &M) {
    //     enum Attr {
    //         Text(String),
    //     }

    //     let mut attrs: Vec<_> = self.attributes
    //         .iter_mut()
    //         .map(|attr| match attr {
    //                  &mut Attribute::Text(ref mut prop) => Attr::Text(prop.process(model)),
    //              })
    //         .collect();

    //     for attr in attrs.drain(..) {
    //         match attr {
    //             Attr::Text(s) => self.set_text(&s),
    //         }
    //     }
    // }
}
