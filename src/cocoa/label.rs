
use cocoa::base::{id, nil, class, NO};
use cocoa::foundation::NSString;

use super::id::Id;
use super::widget::Widget;
use property::Property;

// enum Attribute<M> {
//     Text(Box<Property<M, String>>),
// }

pub struct Label {
    id: Id, 
    // attributes: Vec<Attribute>,
}

impl Label {
    pub fn new() -> Self {
        unsafe {
            let string = NSString::alloc(nil).init_str("TEXT");

            let label: id = msg_send![class("NSTextField"), alloc];
            let label: id = msg_send![label, init];

            msg_send![label, setStringValue: string];
            msg_send![label, setBezeled: NO];
            msg_send![label, setDrawsBackground: NO];
            msg_send![label, setEditable: NO];
            msg_send![label, setSelectable: NO];

            Label {
                id: label.into(), 
                // attributes: vec![],
            }
        }
    }

    fn set_text(&mut self, text: &str) {
        unsafe {
            let string = NSString::alloc(nil).init_str(text);
            msg_send![*self.id, setStringValue: string];
        }
    }

    // pub fn text<P: Property<M, String> + 'static>(mut self, attribute: P) -> Self {
    //     self.attributes
    //         .push(Attribute::Text(Box::new(attribute)));
    //     self
    // }
}

impl Widget for Label {
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
