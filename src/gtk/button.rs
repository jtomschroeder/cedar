
use super::widget::Widget;

use property::Property;
use stream::Stream;

use gtk;
use gtk::prelude::*;

enum Attribute<M> {
    Text(Box<Property<M, String>>),
}

pub struct Button<M, S> {
    button: gtk::Button,
    attributes: Vec<Attribute<M>>,
    stream: Stream<S>,
}

impl<M, S: 'static> Button<M, S> {
    pub fn new(stream: Stream<S>) -> Self {
        Button {
            button: gtk::Button::new(),
            attributes: vec![],
            stream: stream,
        }
    }

    pub fn text<P: Property<M, String> + 'static>(mut self, attribute: P) -> Self {
        self.attributes.push(Attribute::Text(Box::new(attribute)));
        self
    }

    pub fn click<F: Fn() -> S + 'static>(self, action: F) -> Self {
        let stream = self.stream.clone();
        self.button.connect_clicked(move |_| stream.push(action()));
        self
    }
}

impl<M, S: 'static> Widget<M> for Button<M, S> {
    fn add(&self, container: &gtk::Box) {
        container.add(&self.button);
        self.button.show();
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
                Attr::Text(s) => self.button.set_label(&s),
            }
        }
    }
}
