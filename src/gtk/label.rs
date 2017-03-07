
use gtk;
use gtk::prelude::*;

use super::widget::Widget;
use property::Property;

enum Attribute<M> {
    Text(Box<Property<M, String>>),
}

pub struct Label<M> {
    label: gtk::Label,
    attributes: Vec<Attribute<M>>,
}

impl<M> Label<M> {
    pub fn new() -> Self {
        Label {
            label: gtk::Label::new(None),
            attributes: vec![],
        }
    }

    pub fn text<P: Property<M, String> + 'static>(mut self, attribute: P) -> Self {
        self.attributes.push(Attribute::Text(Box::new(attribute)));
        self
    }
}

impl<M> Widget<M> for Label<M> {
    fn add(&self, container: &gtk::Box) {
        container.add(&self.label);
        self.label.show();
    }

    fn update(&mut self, model: &M) {
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
                Attr::Text(s) => self.label.set_text(&s),
            }
        }
    }
}
