
use gtk;
use gtk::prelude::*;

use super::widget::Widget;
use stream::Stream;

pub struct TextField<S> {
    entry: gtk::Entry,
    stream: Stream<S>,
}

impl<S: 'static> TextField<S> {
    pub fn new(stream: Stream<S>) -> Self {
        TextField {
            entry: gtk::Entry::new(),
            stream: stream,
        }
    }

    pub fn placeholder(self, text: &str) -> Self {
        self.entry.set_placeholder_text(text);
        self
    }

    pub fn change<F: Fn(&str) -> S + 'static>(self, delegate: F) -> Self {
        let stream = self.stream.clone();
        self.entry
            .connect_event(move |entry, _| {
                if let Some(ref text) = entry.get_text() {
                    stream.push(delegate(text));
                }

                gtk::Inhibit(false)
            });

        self
    }
}

impl<M, S: 'static> Widget<M> for TextField<S> {
    fn add(&self, container: &gtk::Box) {
        container.add(&self.entry);
        self.entry.show();
    }

    fn update(&mut self, _: &M) {}
}
