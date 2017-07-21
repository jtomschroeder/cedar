
use gtk;
use gtk::prelude::*;

use super::widget::Widget;
use stream::Stream;
use dom::Attributes;

pub struct TextField<S> {
    pub entry: gtk::Entry,
    stream: Stream<S>,
}

impl<S: 'static> TextField<S> {
    pub fn new(stream: Stream<S>) -> Self {
        TextField {
            entry: gtk::Entry::new(),
            stream: stream,
        }
    }

    pub fn set_placeholder(&mut self, text: &str) {
        self.entry.set_placeholder_text(Some(text));
    }

    pub fn register_change(&mut self, delegate: fn(String) -> S) {
        let stream = self.stream.clone();
        self.entry
            .connect_event(move |entry, _| {
                               if let Some(ref text) = entry.get_text() {
                                   stream.push(delegate(text.clone()));
                               }
                               gtk::Inhibit(false)
                           });
    }
}

impl<S: 'static> Widget<S> for TextField<S> {
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
