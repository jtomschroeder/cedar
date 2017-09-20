
use dom::Attributes;
use super::{TextField, Button, Label, Stack};

pub trait Widgeted<S> {
    fn update(&mut self, Attributes<S>) {}
}

pub enum Widget<S> {
    Button(Button<S>),
    Stack(Stack),
    Label(Label),
    Field(TextField<S>),
}

impl<S: Clone + 'static> Widget<S> {
    pub fn add(&self, widget: &Widget<S>) {
        match self {
            &Widget::Stack(ref stack) => stack.add(widget),
            _ => {}
        }
    }

    pub fn widget(&mut self) -> &mut Widgeted<S> {
        match self {
            &mut Widget::Button(ref mut b) => b,
            &mut Widget::Stack(ref mut s) => s,
            &mut Widget::Label(ref mut l) => l,
            &mut Widget::Field(ref mut f) => f,
        }
    }

    pub fn update(&mut self, attrs: Attributes<S>) {
        self.widget().update(attrs);
    }
}
