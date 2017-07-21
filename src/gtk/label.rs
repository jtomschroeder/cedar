
use gtk;

use super::widget::Widget;

pub struct Label {
    pub label: gtk::Label,
}

impl Label {
    pub fn new() -> Self {
        Label { label: gtk::Label::new(None) }
    }
}

use dom::Attributes;

impl<S> Widget<S> for Label {
    fn update(&mut self, attributes: Attributes<S>) {
        use dom::Attribute::*;
        for attr in attributes.into_iter() {
            match attr {
                Text(text) => self.label.set_text(&text),
                _ => {}
            }
        }
    }
}
