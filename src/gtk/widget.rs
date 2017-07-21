
use gtk;

use dom::Attributes;
use super::{TextField, Button, Label, Stack};

pub trait Widget<S> {
    fn update(&mut self, Attributes<S>) {}
}

pub enum NWidget<S> {
    Button(Button<S>),
    Stack(Stack),
    Label(Label),
    Field(TextField<S>),
}

impl<S: Clone + 'static> NWidget<S> {
    pub fn add(&self, widget: &NWidget<S>) {
        match self {
            &NWidget::Stack(ref stack) => stack.add(widget),
            _ => {}
        }
    }

    pub fn widget(&mut self) -> &mut Widget<S> {
        match self {
            &mut NWidget::Button(ref mut b) => b,
            &mut NWidget::Stack(ref mut s) => s,
            &mut NWidget::Label(ref mut l) => l,
            &mut NWidget::Field(ref mut f) => f,
        }
    }

    pub fn update(&mut self, attrs: Attributes<S>) {
        self.widget().update(attrs);
    }
}