
use stream::Stream;
use atomic_box::AtomicBox;

use super::{Window, Button, Label, TextField};

pub struct View<M, S> {
    window: AtomicBox<Window<M>>,
    stream: Stream<S>,
}

impl<M: 'static, S: 'static> View<M, S> {
    pub fn new() -> Self {
        View {
            window: AtomicBox::new(Window::new("cedar")),
            stream: Stream::new(),
        }
    }

    pub fn update(&mut self, model: &M) {
        self.window.update(model)
    }

    pub fn stream(&mut self) -> &Stream<S> {
        &self.stream
    }

    pub fn button<F>(mut self, f: F) -> Self
        where F: FnOnce(Button<M, S>) -> Button<M, S>
    {
        let button = f(Button::new(self.stream.clone()));
        self.window.add(button);
        self
    }

    pub fn label<F>(mut self, f: F) -> Self
        where F: FnOnce(Label<M>) -> Label<M>
    {
        let label = f(Label::new());
        self.window.add(label);
        self
    }

    pub fn field<F>(mut self, f: F) -> Self
        where F: FnOnce(TextField<S>) -> TextField<S>
    {
        let field = f(TextField::new(self.stream.clone()));
        self.window.add(field);
        self
    }
}
