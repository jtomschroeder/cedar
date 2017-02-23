
use super::cacao;
use stream::Stream;

pub struct View<M, S> {
    window: cacao::Window<M>,
    stream: Stream<S>,
}

impl<M: 'static, S: 'static> View<M, S> {
    pub fn new() -> Self {
        View {
            window: cacao::Window::new("buttons"),
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
        where F: FnOnce(cacao::Button<M, S>) -> cacao::Button<M, S>
    {
        let button = f(cacao::Button::new(self.stream.clone()));
        self.window.add(button);
        self
    }

    pub fn label<F>(mut self, f: F) -> Self
        where F: FnOnce(cacao::Label<M>) -> cacao::Label<M>
    {
        let label = f(cacao::Label::new());
        self.window.add(label);
        self
    }

    pub fn field<F>(mut self, f: F) -> Self
        where F: FnOnce(cacao::TextField<S>) -> cacao::TextField<S>
    {
        let field = f(cacao::TextField::new(self.stream.clone()));
        self.window.add(field);
        self
    }
}
