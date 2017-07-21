
use std::marker::PhantomData;

use super::widget::Widget;

use property::Property;
use stream::Stream;

use gtk;
use gtk::prelude::*;

// enum Attribute<M> {
//     Text(Box<Property<M, String>>),
// }

pub struct Button<S> {
    pub button: gtk::Button,
    // attributes: Vec<Attribute<M>>,
    stream: Stream<S>, 
    // phantom: PhantomData<M>,
}

impl<S: 'static> Button<S> {
    pub fn new(stream: Stream<S>) -> Self {
        Button {
            button: gtk::Button::new(),
            // attributes: vec![],
            stream: stream, 
            // phantom: PhantomData,
        }
    }

    // pub fn text<P: Property<M, String> + 'static>(mut self, attribute: P) -> Self {
    //     self.attributes
    //         .push(Attribute::Text(Box::new(attribute)));
    //     self
    // }

    // pub fn click<F: Fn() -> S + 'static>(self, action: F) -> Self {
    //     let stream = self.stream.clone();
    //     self.button
    //         .connect_clicked(move |_| stream.push(action()));
    //     self
    // }

    // fn widget(&self) -> &gtk::Widget {
    //     &self.button as &gtk::Widget
    // }
}

use dom::Attributes;

impl<S: 'static> Widget<S> for Button<S> {
    // fn add(&self, container: &gtk::Box) {
    //     container.add(&self.button);
    //     self.button.show();
    // }

    fn update(&mut self, attrs: Attributes<S>) {}

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
    //             Attr::Text(s) => self.button.set_label(&s),
    //         }
    //     }
    // }
}
