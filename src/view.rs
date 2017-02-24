
use super::backend;
use stream::Stream;

pub struct View<M, S> {
    window: backend::Window<M>,
    stream: Stream<S>,
}

impl<M: 'static, S: 'static> View<M, S> {
    pub fn new() -> Self {
        View {
            window: backend::Window::new("cedar"),
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
        where F: FnOnce(backend::Button<M, S>) -> backend::Button<M, S>
    {
        let button = f(backend::Button::new(self.stream.clone()));
        self.window.add(button);
        self
    }

    pub fn label<F>(mut self, f: F) -> Self
        where F: FnOnce(backend::Label<M>) -> backend::Label<M>
    {
        let label = f(backend::Label::new());
        self.window.add(label);
        self
    }

    pub fn field<F>(mut self, f: F) -> Self
        where F: FnOnce(backend::TextField<S>) -> backend::TextField<S>
    {
        let field = f(backend::TextField::new(self.stream.clone()));
        self.window.add(field);
        self
    }
}
