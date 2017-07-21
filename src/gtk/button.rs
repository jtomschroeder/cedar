
use super::widget::Widget;

use stream::Stream;
use dom::Attributes;

use gtk;
use gtk::prelude::*;

pub struct Button<S> {
    pub button: gtk::Button,
    stream: Stream<S>,
}

impl<S: Clone + 'static> Button<S> {
    pub fn new(stream: Stream<S>) -> Self {
        Button {
            button: gtk::Button::new(),
            stream: stream,
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.button.set_label(text);
    }

    pub fn register_click(&mut self, message: S) {
        let stream = self.stream.clone();
        self.button
            .connect_clicked(move |_| stream.push(message.clone()));
    }
}


impl<S: Clone + 'static> Widget<S> for Button<S> {
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
